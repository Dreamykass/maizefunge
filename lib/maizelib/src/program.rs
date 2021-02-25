use crate::counter::Counter;
use crate::space::Space;
use smallvec::{smallvec, SmallVec};

pub struct Program {
    counters: SmallVec<[Counter; 1]>,
    spaces: SmallVec<[Space; 1]>,
}

// new
impl Program {
    pub fn new_from_initial_space(space: Space) -> Program {
        Program {
            counters: smallvec![Counter::default()],
            spaces: smallvec![space],
        }
    }
}

// accessors
impl Program {
    pub fn counters_and_spaces(&self) -> (&SmallVec<[Counter; 1]>, &SmallVec<[Space; 1]>) {
        (&self.counters, &self.spaces)
    }

    pub fn counters_and_spaces_mut(
        &mut self,
    ) -> (&mut SmallVec<[Counter; 1]>, &mut SmallVec<[Space; 1]>) {
        (&mut self.counters, &mut self.spaces)
    }
}
