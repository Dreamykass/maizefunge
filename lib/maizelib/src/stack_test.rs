#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn basics() {
        let stack = Stack::default();
        assert!(stack.container().is_empty());
    }
}
