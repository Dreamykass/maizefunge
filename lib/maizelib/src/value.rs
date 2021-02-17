pub struct Value(i32);

impl Default for Value {
    fn default() -> Self {
        Value(0)
    }
}

// i32 operations
impl Value {
    pub fn from_i32(i: i32) -> Self {
        Value(i)
    }

    pub fn into_i32(self) -> i32 {
        self.0
    }
}

// char operations
impl Value {
    // pub fn from_char(c: char) -> Self {
    //     Value(c)
    // }
    //
    // pub fn into_char(self) -> char {
    //     self.0
    // }
}
