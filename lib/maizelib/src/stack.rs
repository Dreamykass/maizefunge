use crate::value::Value;
use smallvec::SmallVec;

pub struct Stack {
    container: SmallVec<[Value; 32]>,
}

// default
impl Default for Stack {
    fn default() -> Self {
        Stack {
            container: Default::default(),
        }
    }
}

// general
impl Stack {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn container(&self) -> &SmallVec<[Value; 32]> {
        &self.container
    }

    pub fn container_mut(&mut self) -> &mut SmallVec<[Value; 32]> {
        &mut self.container
    }
}

// operations
impl Stack {
    pub fn pop(&mut self) -> Value {
        self.container.pop().unwrap_or_default()
    }

    pub fn push(&mut self, v: Value) {
        self.container.push(v);
    }
}
