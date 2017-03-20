use std::io::prelude::*;
use std::str;
use std::fs::File;
use std::path::Path;
use std::string::String;
use std::vec::Vec;
use std::i16;
use std::f32::consts::PI;

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

fn create_header(data: &Vec<i16>) -> Vec<u8> {

    let chunk_id = String::from("RIFF");
    let format = String::from("WAVE");
    let subchunk_1_id = String::from("fmt ");
    let subchunk_2_id = String::from("data");

    let num_samples = data.len();

    let subchunk_1_size: u32 = 16;
    let audio_format: u16 = 1;
    let num_channels: u16 = 1;
    let sample_rate: u32 = 44100;
    let bits_per_sample: u16 = 16;

    let block_align: u16 = num_channels * (bits_per_sample as u16) / 8;
    let byte_rate: u32 = sample_rate * (block_align as u32);
    let subchunk_2_size: u32 = (num_samples as u32) * (block_align as u32);
    let chunk_size: u32 = subchunk_2_size + 36;

    let mut header = vec![];

    header.extend(chunk_id.as_bytes());
    header.extend(vec_bytes(chunk_size, 4));
    header.extend(format.as_bytes());
    header.extend(subchunk_1_id.as_bytes());
    header.extend(vec_bytes(subchunk_1_size, 4));
    header.extend(vec_bytes((audio_format as u32), 2));
    header.extend(vec_bytes((num_channels as u32), 2));
    header.extend(vec_bytes(sample_rate, 4));
    header.extend(vec_bytes(byte_rate, 4));
    header.extend(vec_bytes((block_align as u32), 2));
    header.extend(vec_bytes((bits_per_sample as u32), 2));
    header.extend(subchunk_2_id.as_bytes());
    header.extend(vec_bytes(subchunk_2_size, 4));

    header
}

fn main() {

    let path = Path::new("test.wav");
    let display = path.display();

    let mut buffer = match File::create(&path) {
        Err(_) => panic!("couldn't create {}", display),
        Ok(file) => file,
    };

    let mut data: Vec<i16> = vec![];
    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        let sample = (sample * amplitude) as i16;
        data.push(sample);
    }

    let header = create_header(&data);
    buffer.write(&header).expect("Couldn't write");

    for i in 0..data.len() {
        let svec = vec_bytes(data[i] as u32, 2);
        buffer.write(&svec).expect("Couldn't write");
    }
}
