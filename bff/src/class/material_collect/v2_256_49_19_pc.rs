use bff_derive::ReferencedNames;
use binrw::{binread, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
//TODO: Actually figure out the format
#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct MaterialCollectBodyv2_256_49_19PC {
    placeholder: u8,
}

pub type MaterialCollectV2_256_49_19PC = TrivialClass<(), MaterialCollectBodyv2_256_49_19PC>;