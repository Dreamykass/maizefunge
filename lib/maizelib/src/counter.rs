use crate::value::Value;

pub struct Counter {
    pub state: [Value; 16],
}

// default
impl Default for Counter {
    fn default() -> Self {
        Counter {
            state: [Value::default(); 16],
        }
    }
}

// new
impl Counter {
    pub fn new() -> Self {
        Counter::default()
    }
}

// state values
impl Counter {
    pub fn state_identifier(&mut self) -> &mut Value {
        &mut self.state[0]
    }

    pub fn state_coords(&mut self) -> (&mut Value, &mut Value) {
        // (&mut self.state[1], &mut self.state[2])
        let [_, x, y, ..] = &mut self.state;
        (x, y)
    }
    pub fn state_direction(&mut self) -> &mut Value {
        &mut self.state[3]
    }
    pub fn state_stringmode(&mut self) -> &mut Value {
        &mut self.state[4]
    }
    pub fn state_charmode(&mut self) -> &mut Value {
        &mut self.state[5]
    }
    pub fn state_stream_identifier(&mut self) -> &mut Value {
        &mut self.state[6]
    }
    pub fn state_7(&mut self) -> &mut Value {
        &mut self.state[7]
    }
    pub fn state_8(&mut self) -> &mut Value {
        &mut self.state[8]
    }
    pub fn state_9(&mut self) -> &mut Value {
        &mut self.state[9]
    }
    pub fn state_free_a(&mut self) -> &mut Value {
        &mut self.state[10]
    }
    pub fn state_free_b(&mut self) -> &mut Value {
        &mut self.state[11]
    }
    pub fn state_free_c(&mut self) -> &mut Value {
        &mut self.state[12]
    }
    pub fn state_free_d(&mut self) -> &mut Value {
        &mut self.state[13]
    }
    pub fn state_free_e(&mut self) -> &mut Value {
        &mut self.state[14]
    }
    pub fn state_free_f(&mut self) -> &mut Value {
        &mut self.state[15]
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::Counter;
    use crate::value::Value;

    #[test]
    fn state_coords() {
        let mut c = Counter::default();

        let (&mut x, &mut y) = c.state_coords();
        assert_eq!((x, y), (Value::from_i32(0), Value::from_i32(0)));

        *c.state_coords().0.as_i32_mut() = 2;
        *c.state_coords().1.as_i32_mut() = 7;

        let (&mut x, &mut y) = c.state_coords();
        assert_eq!((x, y), (Value::from_i32(2), Value::from_i32(7)));
    }
}
