pub struct SizeFlag(u8);

impl SizeFlag {
    pub fn size(&self) -> usize {
        self.0 as usize
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for SizeFlag {
    fn from(value: u8) -> Self {
        Self(value)
    }
}