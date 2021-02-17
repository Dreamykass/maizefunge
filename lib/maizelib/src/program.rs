use crate::value::Value;
use smallvec::SmallVec;

pub struct Counter {}

pub struct Space {}

pub struct Program {
    program_space: SmallVec<[Value; 32]>,
    counters: SmallVec<[Counter; 4]>,
}
