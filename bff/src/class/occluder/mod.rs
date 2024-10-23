use bff_derive::bff_class;

mod v2_256_38_19_pc;

use v2_256_38_19_pc::OccluderV2_256_38_19PC;

bff_class!(Occluder {
    (Asobo(2, 256, 38, 19), PC) => OccluderV2_256_38_19PC,
});