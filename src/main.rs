//! `dove` — a swarm of RGB triangles simulated by `bevy_ecs` and drawn with
//! `citro3d`.
//!
//! Every entity is `(Body, Spin, Pulse)`. A `bevy_ecs` `Schedule` of four systems
//! moves them, bounces them off the screen edges, spins them and pulses their
//! size each frame; the render loop then just reads the resulting component
//! state back out and draws one triangle per entity.
//!
//! Controls: **A** spawn a triangle · **B** despawn one · **START** exit.
//!
//! Top screen renders the swarm in stereoscopic 3D (left/right eye projections
//! from the 3D slider); the bottom screen renders the same swarm centered.

// Under `cargo 3ds test`, swap the std test harness (which needs a hosted OS)
// for `test-runner`'s GDB-backed one. No effect on normal builds.
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]

use bevy_ecs::prelude::*;
use citro3d::macros::include_shader;
use citro3d::math::{AspectRatio, ClipPlanes, Matrix4, Projection, StereoDisplacement};
use citro3d::render::{ClearFlags, Frame, ScreenTarget, Target};
use citro3d::{attrib, buffer, shader, texenv};
use ctru::prelude::*;
use ctru::services::gfx::{RawFrameBuffer, Screen, TopScreen3D};

// --- geometry -------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: Vec3,
    color: Vec3,
}

/// One unit triangle centered on the origin; the per-entity model matrix does
/// the rest.
static VERTICES: &[Vertex] = &[
    Vertex {
        pos: Vec3::new(0.0, 0.45, 0.0),
        color: Vec3::new(1.0, 0.1, 0.1),
    },
    Vertex {
        pos: Vec3::new(-0.4, -0.35, 0.0),
        color: Vec3::new(0.1, 1.0, 0.1),
    },
    Vertex {
        pos: Vec3::new(0.4, -0.35, 0.0),
        color: Vec3::new(0.1, 0.1, 1.0),
    },
];

static SHADER_BYTES: &[u8] = include_shader!("vshader.pica");
const CLEAR_COLOR: u32 = 0x1A_1B_2E_FF;

/// How far apart the swarm bounces (world units, at z = `SCENE_Z`).
const BOUND_X: f32 = 1.7;
const BOUND_Y: f32 = 1.0;
const SCENE_Z: f32 = -4.0;
const MAX_TRIANGLES: usize = 24;

// --- ECS -----------------------------------------------------------------------

#[derive(Component)]
struct Body {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
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

/// Wall-clock-ish simulation time, advanced once per frame.
#[derive(Resource)]
struct SimTime {
    elapsed: f32,
    dt: f32,
}

fn drift(time: Res<SimTime>, mut bodies: Query<&mut Body>) {
    for mut b in &mut bodies {
        b.x += b.vx * time.dt;
        b.y += b.vy * time.dt;
    }
}

fn bounce(mut bodies: Query<&mut Body>) {
    for mut b in &mut bodies {
        if b.x.abs() > BOUND_X {
            b.x = b.x.clamp(-BOUND_X, BOUND_X);
            b.vx = -b.vx;
        }
        if b.y.abs() > BOUND_Y {
            b.y = b.y.clamp(-BOUND_Y, BOUND_Y);
            b.vy = -b.vy;
        }
    }
}

fn spin(time: Res<SimTime>, mut spins: Query<&mut Spin>) {
    for mut s in &mut spins {
        s.angle += s.rate * time.dt;
    }
}

/// Tiny xorshift RNG so each spawned triangle gets its own motion without
/// pulling in the `rand` crate.
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
                x: rng.range(-BOUND_X, BOUND_X),
                y: rng.range(-BOUND_Y, BOUND_Y),
                vx: rng.range(-1.1, 1.1),
                vy: rng.range(-1.1, 1.1),
            },
            Spin {
                angle: rng.range(0.0, 6.28),
                rate: rng.range(-3.5, 3.5),
            },
            Pulse {
                base: rng.range(0.35, 0.6),
                amp: rng.range(0.05, 0.22),
                freq: rng.range(1.0, 4.0),
                phase: rng.range(0.0, 6.28),
            },
        ))
        .id()
}

// --- main --------------------------------------------------------------------

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

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

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    let RawFrameBuffer { width, height, .. } = bottom_screen.raw_framebuffer();
    let mut bottom_target = instance
        .render_target(width, height, bottom_screen, None)
        .expect("failed to create bottom-screen render target");

    let shader = shader::Library::from_bytes(SHADER_BYTES).unwrap();
    let vertex_shader = shader.get(0).unwrap();
    let program = shader::Program::new(vertex_shader).unwrap();
    let projection_uniform_idx = program.get_vertex_uniform("projection").unwrap();

    let vbo_data = buffer::Buffer::new(VERTICES);
    let mut buf_info = buffer::Info::new();
    let attr_info = prepare_vbos(&mut buf_info, vbo_data);

    let stage0 = texenv::TexEnv::new()
        .src(texenv::Mode::BOTH, texenv::Source::PrimaryColor, None, None)
        .func(texenv::Mode::BOTH, texenv::CombineFunc::Replace);

    // --- ECS world setup ---
    let mut world = World::new();
    world.insert_resource(SimTime {
        elapsed: 0.0,
        dt: 1.0 / 60.0,
    });

    let mut rng = Rng(0x9E37_79B9);
    let mut triangles: Vec<Entity> = (0..6).map(|_| spawn_triangle(&mut world, &mut rng)).collect();

    // `drift` must run before `bounce` (bounce reflects whatever drift produced);
    // `spin` is independent. `.chain()` pins the order.
    let mut schedule = Schedule::default();
    schedule.add_systems(((drift, bounce).chain(), spin));

    // Scratch buffer of (x, y, angle, scale) filled from the ECS each frame.
    let mut scene: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(MAX_TRIANGLES);

    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();

        if keys.contains(KeyPad::START) {
            break;
        }
        if keys.contains(KeyPad::A) && triangles.len() < MAX_TRIANGLES {
            triangles.push(spawn_triangle(&mut world, &mut rng));
        }
        if keys.contains(KeyPad::B) {
            if let Some(entity) = triangles.pop() {
                world.despawn(entity);
            }
        }

        // Advance the simulation.
        {
            let mut time = world.resource_mut::<SimTime>();
            time.elapsed += time.dt;
        }
        schedule.run(&mut world);

        // Read the ECS state back out for rendering.
        let elapsed = world.resource::<SimTime>().elapsed;
        scene.clear();
        let mut query = world.query::<(&Body, &Spin, &Pulse)>();
        for (body, spin, pulse) in query.iter(&world) {
            let scale = pulse.base + pulse.amp * (elapsed * pulse.freq + pulse.phase).sin();
            scene.push((body.x, body.y, spin.angle, scale));
        }

        instance.render_frame_with(|mut frame| {
            fn cast_lifetime_to_closure<'frame, T>(x: T) -> T
            where
                T: Fn(&mut Frame<'frame>, &'frame mut ScreenTarget<'_>, &Matrix4),
            {
                x
            }

            let scene = &scene;
            let draw_swarm = cast_lifetime_to_closure(|frame, target, projection| {
                target.clear(ClearFlags::ALL, CLEAR_COLOR, 0);
                frame
                    .select_render_target(target)
                    .expect("failed to set render target");
                frame.set_texenvs(&[stage0]);
                frame.set_attr_info(&attr_info);

                for &(x, y, angle, scale) in scene {
                    let mut model = Matrix4::identity();
                    model.translate(x, y, SCENE_Z);
                    model.rotate_z(angle);
                    model.scale(scale, scale, 1.0);

                    frame.bind_vertex_uniform(projection_uniform_idx, &(projection * model));
                    frame
                        .draw_arrays(buffer::Primitive::Triangles, &buf_info, None)
                        .unwrap();
                }
            });

            frame.bind_program(&program);

            let Projections {
                left_eye,
                right_eye,
                center,
            } = calculate_projections();

            draw_swarm(&mut frame, &mut top_left_target, &left_eye);
            draw_swarm(&mut frame, &mut top_right_target, &right_eye);
            draw_swarm(&mut frame, &mut bottom_target, &center);

            frame
        });
    }
}

fn prepare_vbos(buf_info: &mut buffer::Info, vbo_data: buffer::Buffer) -> attrib::Info {
    let mut attr_info = attrib::Info::new();

    // v0 = position (vec3), v1 = colour (vec3)
    attr_info
        .add_loader(attrib::Register::V0, attrib::Format::Float, 3)
        .unwrap();
    attr_info
        .add_loader(attrib::Register::V1, attrib::Format::Float, 3)
        .unwrap();

    buf_info.add(vbo_data, attr_info.permutation()).unwrap();

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
                    x: 0.0,
                    y: 0.0,
                    vx: 1.0,
                    vy: 0.0,
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
        assert_eq!(body.x, 0.5);
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
                x: BOUND_X - 0.01,
                y: 0.0,
                vx: 5.0,
                vy: 0.0,
            })
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems((drift, bounce).chain());
        schedule.run(&mut world);

        let body = world.entity(e).get::<Body>().unwrap();
        assert!(body.x.abs() <= BOUND_X, "x = {}", body.x);
        assert!(body.vx < 0.0, "velocity should have flipped");
    }
}
