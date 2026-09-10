//! Integer + boolean vectors: `IVec*`, `UVec*`, `I64Vec*`, `U64Vec*`, `BVec*`.

use super::report;
use bevy_math::{BVec3, I64Vec3, IVec2, IVec3, U64Vec3, UVec3, Vec3};

#[test]
fn ivec_math() {
    report::eq("IVec2 + IVec2", "(1,2)+(3,4)", IVec2::new(4, 6), IVec2::new(1, 2) + IVec2::new(3, 4));
    report::eq("IVec3::dot", "(1,2,3)·(4,5,6)", 32, IVec3::new(1, 2, 3).dot(IVec3::new(4, 5, 6)));
    report::eq("IVec3::cross", "X x Y", IVec3::Z, IVec3::X.cross(IVec3::Y));
    report::eq("IVec3::abs", "(-1,2,-3)", IVec3::new(1, 2, 3), IVec3::new(-1, 2, -3).abs());
    report::eq("IVec3::min", "(1,5,3),(4,2,6)", IVec3::new(1, 2, 3), IVec3::new(1, 5, 3).min(IVec3::new(4, 2, 6)));
    report::eq("IVec3::max", "(1,5,3),(4,2,6)", IVec3::new(4, 5, 6), IVec3::new(1, 5, 3).max(IVec3::new(4, 2, 6)));
    report::eq("IVec3 -> Vec3 (as_vec3)", "(1,2,3)", Vec3::new(1.0, 2.0, 3.0), IVec3::new(1, 2, 3).as_vec3());
    report::eq("Vec3 -> IVec3 (as_ivec3, truncating)", "(1.9,-2.9,3.1)", IVec3::new(1, -2, 3), Vec3::new(1.9, -2.9, 3.1).as_ivec3());
}

#[test]
fn uvec_i64_u64() {
    report::eq("UVec3 + UVec3", "(1,2,3)+(4,5,6)", UVec3::new(5, 7, 9), UVec3::new(1, 2, 3) + UVec3::new(4, 5, 6));
    report::eq("UVec3::element_sum", "(1,2,3)", 6u32, UVec3::new(1, 2, 3).element_sum());
    report::eq("I64Vec3::dot", "(1,2,3)·(4,5,6)", 32i64, I64Vec3::new(1, 2, 3).dot(I64Vec3::new(4, 5, 6)));
    report::eq("U64Vec3 max_element", "(3,9,1)", 9u64, U64Vec3::new(3, 9, 1).max_element());
}

#[test]
fn bvec() {
    report::eq("BVec3: size_of", "", core::mem::size_of::<BVec3>(), core::mem::size_of::<BVec3>());
    let m = IVec3::new(1, 5, 3).cmplt(IVec3::new(4, 2, 6));
    report::eq("IVec3::cmplt -> BVec3", "(1,5,3) < (4,2,6)", BVec3::new(true, false, true), m);
    report::is_true("BVec3::any", "(true,false,true)", m.any());
    report::eq("BVec3::all", "(true,false,true)", false, m.all());
    report::eq("BVec3::bitmask", "(true,false,true)", 0b101u32, m.bitmask());
    report::eq(
        "Vec3::select via mask",
        "mask(t,f,t) a=(1,1,1) b=(9,9,9)",
        Vec3::new(1.0, 9.0, 1.0),
        Vec3::select(m, Vec3::ONE, Vec3::splat(9.0)),
    );
}
