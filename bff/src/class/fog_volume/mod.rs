use bff_derive::bff_class;

mod v2_256_38_19_pc;

use v2_256_38_19_pc::FogVolumeV2_256_38_19PC;

bff_class!(FogVolume {
    (Asobo(2, 256, 38, 19), PC) => FogVolumeV2_256_38_19PC,
});