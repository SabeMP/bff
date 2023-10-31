use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use hound::{SampleFormat, WavSpec, WavWriter};
use three_d::CpuModel;

#[derive(Clone)]
pub enum Artifact {
    Bitmap {
        is_dds: bool, // use enum?
        data: Arc<Vec<u8>>,
    },
    Sound {
        data: Arc<Vec<i16>>,
        sample_rate: u32,
        channels: u16,
    },
    Mesh(Arc<CpuModel>),
    Skin(Arc<CpuModel>),
}

impl Artifact {
    //TODO: write to impl Write
    pub fn save(&self, path: &PathBuf) {
        match *self {
            Self::Bitmap {
                is_dds: _,
                ref data,
            } => {
                let mut file = File::create(path).unwrap();
                file.write_all(data).unwrap();
            }
            Self::Sound {
                ref data,
                channels,
                sample_rate,
            } => {
                let spec = WavSpec {
                    channels,
                    sample_rate,
                    bits_per_sample: 16,
                    sample_format: SampleFormat::Int,
                };
                let mut parent_writer = WavWriter::create(path, spec).unwrap();
                let mut sample_writer = parent_writer.get_i16_writer(data.len() as u32);
                for sample in data.iter() {
                    sample_writer.write_sample(*sample);
                }
                sample_writer.flush().unwrap();
                parent_writer.finalize().unwrap();
            }
            Self::Mesh(_) => todo!(),
            Self::Skin(_) => todo!(),
        };
    }
}
