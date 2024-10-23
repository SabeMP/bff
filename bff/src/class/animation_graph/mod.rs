use bff_derive::bff_class;

mod v2_256_38_19_pc;

use v2_256_38_19_pc::AnimationGraphV2_256_38_19PC;

bff_class!(AnimationGraph {
    (Asobo(2, 256, 38, 19), PC) => AnimationGraphV2_256_38_19PC,
});