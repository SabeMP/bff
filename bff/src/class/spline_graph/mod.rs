use bff_derive::bff_class;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::SplineGraphV1_381_67_09PC;

bff_class!(SplineGraph {
    (Asobo(1, 381, 67, 9), PC) => SplineGraphV1_381_67_09PC,
});
