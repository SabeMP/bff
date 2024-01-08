use bff_derive::bff_class;

pub mod v1_06_63_02_pc;
pub mod v1_291_03_06_pc;
pub mod v1_381_67_09_pc;
pub mod v1_634_78_10_ps2;

use v1_06_63_02_pc::MeshV1_06_63_02PC;
use v1_291_03_06_pc::MeshV1_291_03_06PC;
use v1_381_67_09_pc::MeshV1_381_67_09PC;
use v1_634_78_10_ps2::MeshV1_634_78_10PS2;

bff_class!(Mesh {
    (Asobo(1, 6, 63, 2), PC) => MeshV1_06_63_02PC,
    (Asobo(1, 291, 3, 6), PC) => MeshV1_291_03_06PC,
    (Asobo(1, 381, 67, 9), PC) => MeshV1_381_67_09PC,
    (Asobo(1, 634, 78, 10), PS2) => MeshV1_634_78_10PS2,
});
