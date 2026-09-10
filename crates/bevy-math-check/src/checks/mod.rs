//! One submodule per `bevy_math` area. Every `#[test]` fn emits `ROW|` lines.

mod report;

const EPS: f32 = report::EPS;

#[track_caller]
fn near2(a: bevy_math::Vec2, b: bevy_math::Vec2) -> bool {
    a.abs_diff_eq(b, EPS)
}
#[track_caller]
fn near3(a: bevy_math::Vec3, b: bevy_math::Vec3) -> bool {
    a.abs_diff_eq(b, EPS)
}
#[track_caller]
fn near3a(a: bevy_math::Vec3A, b: bevy_math::Vec3A) -> bool {
    a.abs_diff_eq(b, EPS)
}
#[track_caller]
fn near4(a: bevy_math::Vec4, b: bevy_math::Vec4) -> bool {
    a.abs_diff_eq(b, EPS)
}
#[track_caller]
fn nearq(a: bevy_math::Quat, b: bevy_math::Quat) -> bool {
    a.abs_diff_eq(b, EPS)
}

/// A tiny deterministic RNG (`getrandom` has no 3DS backend, so no `OsRng`).
/// `rand_core` 0.10: implement `TryRng`; `Rng`/`RngExt` come via blanket impls.
#[derive(Clone)]
pub struct Xorshift(pub u64);
impl Xorshift {
    fn step(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
impl rand::rand_core::TryRng for Xorshift {
    type Error = core::convert::Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.step() as u32)
    }
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.step())
    }
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        for chunk in dst.chunks_mut(8) {
            let v = self.step().to_le_bytes();
            chunk.copy_from_slice(&v[..chunk.len()]);
        }
        Ok(())
    }
}

mod affine;
mod bounding;
mod compass_ord;
mod curves;
mod directions;
mod dvec;
mod float_ext;
mod ivec;
mod layout;
mod mat;
mod ops_mod;
mod primitives2d;
mod primitives3d;
mod quat;
mod rects;
mod sampling;
mod stable_interp;
mod vectors;
