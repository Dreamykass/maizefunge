use crate::stack::Stack;
use crate::value::Value;

pub struct Counter {
    state: [Value; 16],
    stack: Stack,
}

// default
impl Default for Counter {
    fn default() -> Self {
        Counter {
            state: [Value::default(); 16],
            stack: Stack::default(),
        }
    }
}

// new
impl Counter {
    pub fn new_forked_from(other: &Counter, identifier: Value, delta: Value) -> Self {
        let mut counter = Counter::default();
        counter.state = other.state; // copy
        *counter.state_identifier_mut() = identifier;
        delta; // TODO: implement advancing by delta -------------
        counter
    }
}

// state values non-mut
impl Counter {
    pub fn state_identifier(&self) -> &Value {
        &self.state[0]
    }
    pub fn state_coords(&self) -> (&Value, &Value) {
        let [_, x, y, ..] = &self.state;
        (x, y)
    }
    pub fn state_direction(&self) -> &Value {
        &self.state[3]
    }
    pub fn state_space_identifier(&self) -> &Value {
        &self.state[4]
    }
    pub fn state_stringmode(&self) -> &Value {
        &self.state[5]
    }
    pub fn state_charmode(&self) -> &Value {
        &self.state[6]
    }
    pub fn state_stream_identifier(&self) -> &Value {
        &self.state[7]
    }
    pub fn state_remaining_sleep(&self) -> &Value {
        &self.state[8]
    }
    pub fn state_9(&self) -> &Value {
        &self.state[9]
    }
    pub fn state_free_a(&self) -> &Value {
        &self.state[10]
    }
    pub fn state_free_b(&self) -> &Value {
        &self.state[11]
    }
    pub fn state_free_c(&self) -> &Value {
        &self.state[12]
    }
    pub fn state_free_d(&self) -> &Value {
        &self.state[13]
    }
    pub fn state_free_e(&self) -> &Value {
        &self.state[14]
    }
    pub fn state_free_f(&self) -> &Value {
        &self.state[15]
    }
}

// state values mut
impl Counter {
    pub fn state_identifier_mut(&mut self) -> &mut Value {
        &mut self.state[0]
    }
    pub fn state_coords_mut(&mut self) -> (&mut Value, &mut Value) {
        let [_, x, y, ..] = &mut self.state;
        (x, y)
    }
    pub fn state_direction_mut(&mut self) -> &mut Value {
        &mut self.state[3]
    }
    pub fn state_space_identifier_mut(&mut self) -> &mut Value {
        &mut self.state[4]
    }
    pub fn state_stringmode_mut(&mut self) -> &mut Value {
        &mut self.state[5]
    }
    pub fn state_charmode_mut(&mut self) -> &mut Value {
        &mut self.state[6]
    }
    pub fn state_stream_identifier_mut(&mut self) -> &mut Value {
        &mut self.state[7]
    }
    pub fn state_remaining_sleep_mut(&mut self) -> &mut Value {
        &mut self.state[8]
    }
    pub fn state_9_mut(&mut self) -> &mut Value {
        &mut self.state[9]
    }
    pub fn state_free_a_mut(&mut self) -> &mut Value {
        &mut self.state[10]
    }
    pub fn state_free_b_mut(&mut self) -> &mut Value {
        &mut self.state[11]
    }
    pub fn state_free_c_mut(&mut self) -> &mut Value {
        &mut self.state[12]
    }
    pub fn state_free_d_mut(&mut self) -> &mut Value {
        &mut self.state[13]
    }
    pub fn state_free_e_mut(&mut self) -> &mut Value {
        &mut self.state[14]
    }
    pub fn state_free_f_mut(&mut self) -> &mut Value {
        &mut self.state[15]
    }
}

// stack
impl Counter {
    pub fn stack(&self) -> &Stack {
        &self.stack
    }
    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::counter::Counter;
    use crate::value::Value;

    #[test]
    fn state_coords() {
        let mut c = Counter::default();

        let (&mut x, &mut y) = c.state_coords_mut();
        assert_eq!((x, y), (Value::from_i32(0), Value::from_i32(0)));

        *c.state_coords_mut().0.as_i32_mut() = 2;
        *c.state_coords_mut().1.as_i32_mut() = 7;

        let (&mut x, &mut y) = c.state_coords_mut();
        assert_eq!((x, y), (Value::from_i32(2), Value::from_i32(7)));
    }
}
