#![allow(dead_code)]

use std::num::ParseIntError;
pub use std::{fs::File, io::BufReader};

pub use serde::{self, de::DeserializeOwned};

pub(crate) trait ReadFromFile {
    fn from_file<T: DeserializeOwned>(file: &'static str) -> T {
        let file = match File::open(file) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open file {}.", file),
        };
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                panic!("Error reading file.")
            }
        }
    }
}

pub(crate) fn hex_str_to_bytes(val: &str) -> Vec<u8> {
    let b: Result<Vec<u8>, ParseIntError> = (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}

pub(crate) fn hex_str_to_array<A>(val: &str) -> A
where
    A: Default + AsMut<[u8]>,
{
    let b: Result<Vec<u8>, ParseIntError> = (0..val.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&val[i..i + 2], 16))
        .collect();
    let b = b.expect("Error parsing hex string");
    let mut out = A::default();
    A::as_mut(&mut out).clone_from_slice(&b);
    out
}
