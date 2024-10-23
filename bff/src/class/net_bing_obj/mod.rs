use bff_derive::bff_class;

mod v2_256_49_19_pc;

use v2_256_49_19_pc::NetBingObjV2_256_49_19PC;

bff_class!(NetBingObj {
    (Asobo(2, 256, 49, 19), PC) => NetBingObjV2_256_49_19PC,
});