use bff_derive::bff_class;

mod v2_256_49_19_pc;

use v2_256_49_19_pc::MaterialCollectV2_256_49_19PC;

bff_class!(MaterialCollect {
    (Asobo(2, 256, 49, 19), PC) => MaterialCollectV2_256_49_19PC,
});