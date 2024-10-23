use bff_derive::bff_class;

mod v2_256_38_19_pc;

use v2_256_38_19_pc::HullSplineZoneV2_256_38_19PC;

bff_class!(HullSplineZone {
    (Asobo(2, 256, 38, 19), PC) => HullSplineZoneV2_256_38_19PC,
});