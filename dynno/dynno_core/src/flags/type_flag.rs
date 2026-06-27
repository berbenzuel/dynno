pub struct TypeFlag(u8);

impl From<u8> for TypeFlag {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

//represents 8bit num -> 1 option 1 required 11 general type matching 1111 precise
impl TypeFlag {
    pub fn new(option: bool, required: bool, type_code: usize) -> Self  {
        Self((option as u8) * 128 + (required as u8) * 64)
    }
    pub fn option(&self) -> bool {
        self.0 >= 0b1000_0000
    }
    pub fn required(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }
    pub fn type_code(&self) -> usize {
        (self.0 & 0b0011_1111) as usize
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}