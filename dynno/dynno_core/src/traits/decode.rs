use std::io::{Read, Write};

pub trait Decode : Sized {
    fn decode(reader: &mut impl Read) -> std::io::Result<Self>;
}