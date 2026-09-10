//! `dove` — a swarm of RGB triangles simulated by `bevy_ecs` and drawn with
//! `citro3d`, tuned for throughput.
//!
//! Every entity is `(Body, Spin, Pulse)` with `bevy_math` `Vec2` positions. A
//! `bevy_ecs` `Schedule` of `drift` → `bounce` + `spin` advances them each
//! frame. The render loop reads the component state back out, transforms every
//! triangle **on the CPU into one shared vertex buffer** (`Vec2::from_angle` /
//! `Vec2::rotate` for the spin), and then issues exactly **one draw call per
//! screen** (3 total) instead of one per triangle — so cost scales with
//! vertices pushed, not with draw-call overhead.
//!
//! Controls: **A/B** ±8 triangles · hold **X/Y** ±32 per frame · **SELECT**
//! reset · **L** toggle the bottom-screen metrics console · **START** exit.
//! Frame rate + triangle count are printed over `3dslink`
//! (`cargo 3ds run --server`), and — when open — on the bottom-screen console.

#![feature(allocator_api)]
// Under `cargo 3ds test`, swap the std test harness (which needs a hosted OS)
// for a 3DS one. Default: GDB-reporting (scripts/test-emulator.sh).
// `--features console`: interactive on-device runner with a bottom-screen menu.
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(all(test, not(feature = "console")), test_runner(test_runner::run_gdb))]
#![cfg_attr(all(test, feature = "console"), test_runner(test_console::run))]

use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};
use citro3d::macros::include_shader;
use citro3d::math::{AspectRatio, ClipPlanes, Matrix4, Projection, StereoDisplacement};
use citro3d::render::{ClearFlags, Frame, ScreenTarget, Target};
use citro3d::{attrib, buffer, shader, texenv};
use ctru::console::Console;
use ctru::linear::LinearAllocator;
use ctru::prelude::*;
use ctru::services::gfx::{RawFrameBuffer, Screen, TopScreen3D};

/// What the bottom screen is currently showing. Both variants borrow
/// `gfx.bottom_screen`, so only one can exist at a time — toggling drops one and
/// builds the other (see the `KeyPad::L` handler in `main`).
enum BottomView<'s> {
    /// A `citro3d` render target: the swarm, drawn with a centered projection.
    Swarm(ScreenTarget<'s>),
    /// A text console showing the live render metrics.
    Console(Console<'s>),
}

// --- geometry ----------------------------------------------------------------

/// GPU vertex: `glam::Vec3` is `#[repr(C)]` (3 × f32), so this is 24 bytes and
/// maps straight onto two `Float × 3` attribute loaders.
#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: Vec3,
    color: Vec3,
}

/// The unit triangle, centred on the origin: `(corner offset, colour)`. The
/// per-entity rotate/scale/translate is applied on the CPU each frame.
const BASE: [(Vec2, Vec3); 3] = [
    (Vec2::new(0.0, 0.45), Vec3::new(1.0, 0.1, 0.1)),
    (Vec2::new(-0.4, -0.35), Vec3::new(0.1, 1.0, 0.1)),
    (Vec2::new(0.4, -0.35), Vec3::new(0.1, 0.1, 1.0)),
];

static SHADER_BYTES: &[u8] = include_shader!("vshader.pica");
const CLEAR_COLOR: u32 = 0x1A_1B_2E_FF;

/// Bounds the swarm bounces within (world units, at z = `SCENE_Z`).
const BOUND_X: f32 = 1.7;
const BOUND_Y: f32 = 1.0;
const SCENE_Z: f32 = -4.0;
/// 8192 * 3 verts = 24576, comfortably under `C3D`'s 16-bit vertex count once
/// you account for the fact we draw the same buffer three times.
const MAX_TRIANGLES: usize = 8192;
const START_TRIANGLES: usize = 64;

// --- ECS -------------------------------------------------------------------

#[derive(Component)]
struct Body {
    pos: Vec2,
    vel: Vec2,
}

#[derive(Component)]
struct Spin {
    angle: f32,
    rate: f32,
}

#[derive(Component)]
struct Pulse {
    base: f32,
    amp: f32,
    freq: f32,
    phase: f32,
}

/// Simulation clock, advanced once per frame.
#[derive(Resource)]
struct SimTime {
    elapsed: f32,
    dt: f32,
}

fn drift(time: Res<SimTime>, mut bodies: Query<&mut Body>) {
    for mut b in &mut bodies {
        let step = b.vel * time.dt;
        b.pos += step;
    }
}

fn bounce(mut bodies: Query<&mut Body>) {
    for mut b in &mut bodies {
        if b.pos.x.abs() > BOUND_X {
            b.pos.x = b.pos.x.clamp(-BOUND_X, BOUND_X);
            b.vel.x = -b.vel.x;
        }
        if b.pos.y.abs() > BOUND_Y {
            b.pos.y = b.pos.y.clamp(-BOUND_Y, BOUND_Y);
            b.vel.y = -b.vel.y;
        }
    }
}

fn spin(time: Res<SimTime>, mut spins: Query<&mut Spin>) {
    for mut s in &mut spins {
        s.angle += s.rate * time.dt;
    }
}

/// Tiny xorshift RNG so each spawned triangle gets its own motion without the
/// `rand` crate.
struct Rng(u32);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (self.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }
}

fn spawn_triangle(world: &mut World, rng: &mut Rng) -> Entity {
    world
        .spawn((
            Body {
                pos: Vec2::new(rng.range(-BOUND_X, BOUND_X), rng.range(-BOUND_Y, BOUND_Y)),
                vel: Vec2::new(rng.range(-1.1, 1.1), rng.range(-1.1, 1.1)),
            },
            Spin {
                angle: rng.range(0.0, 6.28),
                rate: rng.range(-3.5, 3.5),
            },
            Pulse {
                base: rng.range(0.30, 0.55),
                amp: rng.range(0.04, 0.18),
                freq: rng.range(1.0, 4.0),
                phase: rng.range(0.0, 6.28),
            },
        ))
        .id()
}

// --- timing ---------------------------------------------------------------

fn ticks() -> u64 {
    unsafe { ctru_sys::svcGetSystemTick() }
}

const TICKS_PER_SEC: f64 = ctru_sys::SYSCLOCK_ARM11 as f64;

// --- main ----------------------------------------------------------------

fn main() {
    // On New 3DS this unlocks the 804 MHz clock + extra cache — free win for the
    // CPU-side ECS/transform work. No-op on Old 3DS.
    unsafe { ctru_sys::osSetSpeedupEnable(true) };

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    // Route stdout/stderr back over `3dslink --server` so we can watch the FPS.
    let mut soc = Soc::new().ok();
    if let Some(soc) = soc.as_mut() {
        let _ = soc.redirect_to_3dslink(true, true);
    }

    let mut instance = citro3d::Instance::new().expect("failed to initialize Citro3D");

    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut top_left, mut top_right) = top_screen.split_mut();

    let RawFrameBuffer { width, height, .. } = top_left.raw_framebuffer();
    let mut top_left_target = instance
        .render_target(width, height, top_left, None)
        .expect("failed to create top-left render target");

    let RawFrameBuffer { width, height, .. } = top_right.raw_framebuffer();
    let mut top_right_target = instance
        .render_target(width, height, top_right, None)
        .expect("failed to create top-right render target");

    // The bottom screen starts as a swarm render target; `L` toggles it to a
    // text console and back. Grab its framebuffer size once so we can rebuild
    // the target on demand.
    let (bottom_w, bottom_h) = {
        let mut fb = gfx.bottom_screen.borrow_mut();
        let RawFrameBuffer { width, height, .. } = fb.raw_framebuffer();
        (width, height)
    };
    let mut bottom = Some(BottomView::Swarm(
        instance
            .render_target(bottom_w, bottom_h, gfx.bottom_screen.borrow_mut(), None)
            .expect("failed to create bottom-screen render target"),
    ));

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();
    let program = shader::Program::new(vertex_shader).unwrap();
    let projection_uniform_idx = program.get_vertex_uniform("projection").unwrap();

    let attr_info = build_attr_info();
    let attr_perm = attr_info.permutation();

    let stage0 = texenv::TexEnv::new()
        .src(texenv::Mode::BOTH, texenv::Source::PrimaryColor, None, None)
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

    // --- ECS world ---
    let mut world = World::new();
    world.insert_resource(SimTime {
        elapsed: 0.0,
        dt: 1.0 / 60.0,
    });

    let mut rng = Rng(0x9E37_79B9);
    let mut triangles: Vec<Entity> = Vec::with_capacity(MAX_TRIANGLES);
    resize_swarm(&mut world, &mut rng, &mut triangles, START_TRIANGLES);

    let mut schedule = Schedule::default();
    schedule.add_systems(((drift, bounce).chain(), spin));

    // Hold the previous frame's vertex buffer alive until this frame's
    // `C3D_FrameBegin(SYNCDRAW)` has confirmed the GPU is done reading it.
    let mut prev_buffer: Option<buffer::Info> = None;

    let mut fps_mark = ticks();
    let mut fps_frames = 0u32;

    while apt.main_loop() {
        hid.scan_input();
        let down = hid.keys_down();
        let held = hid.keys_held();

        if down.contains(KeyPad::START) {
            break;
        }

        // Toggle the bottom screen between the swarm and the metrics console.
        // Dropping the old view releases its `RefMut<gfx.bottom_screen>` before
        // the replacement re-borrows it.
        if down.contains(KeyPad::L) {
            let was_console = matches!(bottom, Some(BottomView::Console(_)));
            drop(bottom.take());
            bottom = Some(if was_console {
                BottomView::Swarm(
                    instance
                        .render_target(bottom_w, bottom_h, gfx.bottom_screen.borrow_mut(), None)
                        .expect("failed to rebuild bottom-screen render target"),
                )
            } else {
                let console = Console::new(gfx.bottom_screen.borrow_mut());
                console.select();
                print!("\x1b[2J\x1b[H");
                BottomView::Console(console)
            });
        }

        let mut target_count = triangles.len();
        if down.contains(KeyPad::A) {
            target_count += 8;
        }
        if down.contains(KeyPad::B) {
            target_count = target_count.saturating_sub(8);
        }
        if held.contains(KeyPad::X) {
            target_count += 32;
        }
        if held.contains(KeyPad::Y) {
            target_count = target_count.saturating_sub(32);
        }
        if down.contains(KeyPad::SELECT) {
            target_count = START_TRIANGLES;
        }
        target_count = target_count.min(MAX_TRIANGLES);
        if target_count != triangles.len() {
            resize_swarm(&mut world, &mut rng, &mut triangles, target_count);
        }

        // Advance the simulation.
        {
            let mut time = world.resource_mut::<SimTime>();
            time.elapsed += time.dt;
        }
        schedule.run(&mut world);

        // Bake every triangle into one shared vertex buffer in linear memory.
        let elapsed = world.resource::<SimTime>().elapsed;
        let mut verts: Vec<Vertex, LinearAllocator> =
            Vec::with_capacity_in(triangles.len() * 3, LinearAllocator);
        let mut query = world.query::<(&Body, &Spin, &Pulse)>();
        for (body, spin, pulse) in query.iter(&world) {
            let scale = pulse.base + pulse.amp * (elapsed * pulse.freq + pulse.phase).sin();
            let rotation = Vec2::from_angle(spin.angle);
            for &(offset, color) in &BASE {
                let p = rotation.rotate(offset) * scale + body.pos;
                verts.push(Vertex {
                    pos: Vec3::new(p.x, p.y, SCENE_Z),
                    color,
                });
            }
        }

        let mut frame_buffer = buffer::Info::new();
        if !verts.is_empty() {
            frame_buffer
                .add(buffer::Buffer::new_in_linear(verts), attr_perm)
                .unwrap();
        }

        let projections = calculate_projections();

        instance.render_frame_with(|mut frame| {
            fn cast_lifetime_to_closure<'frame, T>(x: T) -> T
            where
                T: Fn(&mut Frame<'frame>, &'frame mut ScreenTarget<'_>, &Matrix4, &'frame buffer::Info),
            {
                x
            }

            let draw_screen = cast_lifetime_to_closure(|frame, target, projection, buf| {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);
                frame
                    .select_render_target(target)
                    .expect("failed to set render target");
                frame.set_texenvs(&[stage0]);
                frame.set_attr_info(&attr_info);
                frame.bind_vertex_uniform(projection_uniform_idx, projection);
                if !buf.is_empty() {
                    frame
                        .draw_arrays(buffer::Primitive::Triangles, buf, None)
                        .unwrap();
                }
            });

            frame.bind_program(&program);

            draw_screen(&mut frame, &mut top_left_target, &projections.left_eye, &frame_buffer);
            draw_screen(&mut frame, &mut top_right_target, &projections.right_eye, &frame_buffer);
            // The bottom screen only gets a draw call when it's a render target;
            // as a console it's owned by libctru and left alone here.
            if let Some(BottomView::Swarm(target)) = bottom.as_mut() {
                draw_screen(&mut frame, target, &projections.center, &frame_buffer);
            }

            frame
        });

        // The GPU is now done with the frame that was in flight when we entered
        // `render_frame_with`, so releasing its buffer here is safe.
        prev_buffer = Some(frame_buffer);

        fps_frames += 1;
        let now = ticks();
        let secs = (now - fps_mark) as f64 / TICKS_PER_SEC;
        if secs >= 0.5 {
            let fps = fps_frames as f64 / secs;
            fps_frames = 0;
            fps_mark = now;

            let metrics = format!(
                "{:>6.1} fps | {:>5} tris | {:>6} verts | 3 draws",
                fps,
                triangles.len(),
                triangles.len() * 3,
            );
            match bottom.as_ref() {
                // Console open: `stdout` is the on-screen console (libctru points
                // it there on the first `Console::new` and doesn't hand it back,
                // so the `3dslink` stream stops once the console has been used).
                Some(BottomView::Console(console)) => {
                    console.select();
                    print!("\x1b[H");
                    println!("{metrics}");
                    println!();
                    println!("L close   A/B +/-8   hold X/Y +/-32");
                    println!("SELECT reset        START exit");
                }
                _ => println!("{metrics}"),
            }
        }
    }

    drop(prev_buffer);
}

fn resize_swarm(world: &mut World, rng: &mut Rng, triangles: &mut Vec<Entity>, target: usize) {
    while triangles.len() < target {
        triangles.push(spawn_triangle(world, rng));
    }
    while triangles.len() > target {
        if let Some(entity) = triangles.pop() {
            world.despawn(entity);
        }
    }
}

fn build_attr_info() -> attrib::Info {
    let mut attr_info = attrib::Info::new();
    // v0 = position (vec3), v1 = colour (vec3)
    attr_info
        .add_loader(attrib::Register::V0, attrib::Format::Float, 3)
        .unwrap();
    attr_info
        .add_loader(attrib::Register::V1, attrib::Format::Float, 3)
        .unwrap();
    attr_info
}

struct Projections {
    left_eye: Matrix4,
    right_eye: Matrix4,
    center: Matrix4,
}

fn calculate_projections() -> Projections {
    // The hardware 3D slider controls how far apart the two eye views are.
    let slider_val = ctru::os::current_3d_slider_state();
    let interocular_distance = slider_val / 2.0;

    let vertical_fov = 40.0_f32.to_radians();
    let screen_depth = 2.0;

    let clip_planes = ClipPlanes {
        near: 0.01,
        far: 100.0,
    };

    let (left, right) = StereoDisplacement::new(interocular_distance, screen_depth);

    let (left_eye, right_eye) =
        Projection::perspective(vertical_fov, AspectRatio::TopScreen, clip_planes)
            .stereo_matrices(left, right);

    let center =
        Projection::perspective(vertical_fov, AspectRatio::BottomScreen, clip_planes).into();

    Projections {
        left_eye,
        right_eye,
        center,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ECS schedule actually mutates component state when run.
    #[test]
    fn schedule_moves_and_spins_bodies() {
        let mut world = World::new();
        world.insert_resource(SimTime {
            elapsed: 0.0,
            dt: 0.5,
        });
        let e = world
            .spawn((
                Body {
                    pos: Vec2::ZERO,
                    vel: Vec2::new(1.0, 0.0),
                },
                Spin {
                    angle: 0.0,
                    rate: 2.0,
                },
            ))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(((drift, bounce).chain(), spin));
        schedule.run(&mut world);

        let body = world.entity(e).get::<Body>().unwrap();
        assert_eq!(body.pos, Vec2::new(0.5, 0.0));
        assert_eq!(world.entity(e).get::<Spin>().unwrap().angle, 1.0);
    }

    /// Bodies that run past the edge get reflected back inside.
    #[test]
    fn bounce_keeps_bodies_in_bounds() {
        let mut world = World::new();
        world.insert_resource(SimTime {
            elapsed: 0.0,
            dt: 1.0,
        });
        let e = world
            .spawn(Body {
                pos: Vec2::new(BOUND_X - 0.01, 0.0),
                vel: Vec2::new(5.0, 0.0),
            })
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems((drift, bounce).chain());
        schedule.run(&mut world);

        let body = world.entity(e).get::<Body>().unwrap();
        assert!(body.pos.x.abs() <= BOUND_X, "x = {}", body.pos.x);
        assert!(body.vel.x < 0.0, "velocity should have flipped");
    }

    /// `resize_swarm` grows and shrinks the entity set to an exact count.
    #[test]
    fn resize_swarm_hits_target() {
        let mut world = World::new();
        let mut rng = Rng(1);
        let mut triangles = Vec::new();

        resize_swarm(&mut world, &mut rng, &mut triangles, 50);
        assert_eq!(triangles.len(), 50);
        assert_eq!(world.query::<&Body>().iter(&world).count(), 50);

        resize_swarm(&mut world, &mut rng, &mut triangles, 10);
        assert_eq!(triangles.len(), 10);
        assert_eq!(world.query::<&Body>().iter(&world).count(), 10);
    }
}
