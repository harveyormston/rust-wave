use std::io::prelude::*;
use std::str;
use std::fs::File;
use std::path::Path;
use std::string::String;
use std::vec::Vec;
use std::f64::consts::PI;

fn vec_bytes(value: u32, num_bytes: u32) -> Vec<u8> {

    let mut byte_vector: Vec<u8> = vec![];
    let mut buf = value;
    for _ in 0..num_bytes {
        let tmp: u8 = buf as u8;
        byte_vector.push(tmp);
        buf = buf >> 8;
    }

    byte_vector
}

/// # fn create_header(ns: u32, nc: u16, sr: u32, bps: u16) -> Vec<u8> {
/// #     ns: number of samples
/// #     nc: number of channels
/// #     sr: sample rate in Hz
/// #     bps: bits per sample
/// # }

fn create_header(ns: u32, nc: u16, sr: u32, bps: u16) -> Vec<u8> {

    let chunk_id = String::from("RIFF");
    let format = String::from("WAVE");
    let subchunk_1_id = String::from("fmt ");
    let subchunk_2_id = String::from("data");

    let subchunk_1_size: u32 = 16;
    let audio_format: u16 = 1;

    let block_align: u16 = nc * bps / 8;
    let byte_rate: u32 = sr * (block_align as u32);
    let subchunk_2_size: u32 = ns * (block_align as u32);
    let chunk_size: u32 = subchunk_2_size + 36;

    let mut header = vec![];

    header.extend(chunk_id.as_bytes());
    header.extend(vec_bytes(chunk_size, 4));
    header.extend(format.as_bytes());
    header.extend(subchunk_1_id.as_bytes());
    header.extend(vec_bytes(subchunk_1_size, 4));
    header.extend(vec_bytes((audio_format as u32), 2));
    header.extend(vec_bytes((nc as u32), 2));
    header.extend(vec_bytes(sr, 4));
    header.extend(vec_bytes(byte_rate, 4));
    header.extend(vec_bytes((block_align as u32), 2));
    header.extend(vec_bytes((bps as u32), 2));
    header.extend(subchunk_2_id.as_bytes());
    header.extend(vec_bytes(subchunk_2_size, 4));

    header
}

fn main() {

    let two: u32 = 2;
    let path = Path::new("test.wav");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(_) => panic!("couldn't create {}", display),
        Ok(file) => file,
    };

    let nc: u16 = 1;
    let sr: u32 = 96000;
    let bps: u16 = 32;
    let mut data: Vec<f64> = vec![];

    let sample_scale: f64 = (two.pow((bps as u32) - 1)) as f64;
    let float_scale: f64 = (sample_scale - 1.0) / sample_scale;

    for t in (0 .. sr).map(|x| x as f64 / (sr as f64)) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude: f64 = 1.0;
        let sample = sample * amplitude * float_scale;
        data.push(sample);
    }

    let ns = data.len() as u32;
    let header = create_header(ns, nc, sr, bps);
    file.write(&header).expect("Couldn't write");

    let bytes_per_sample: u32 = (bps as u32) / 8;

    for i in 0 .. data.len() {
        let svec = vec_bytes((data[i] * sample_scale) as u32, bytes_per_sample);
        file.write(&svec).expect("Couldn't write");
    }
}
