#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn basics() {
        let v = Value::from_i32(4);
        assert_eq!(v.into_i32(), 4);
    }
}
