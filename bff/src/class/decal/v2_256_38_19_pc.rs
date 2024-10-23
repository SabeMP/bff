use bff_derive::ReferencedNames;
use binrw::{binread, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
//TODO: Actually figure out the format
#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct DecalBodyV2_256_38_19PC {
    placeholder: u8,
}

pub type DecalV2_256_38_19PC = TrivialClass<(), DecalBodyV2_256_38_19PC>;