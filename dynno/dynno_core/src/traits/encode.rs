use std::io::Write;

pub trait DbEncode {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()>;
}