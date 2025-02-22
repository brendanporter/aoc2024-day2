#[cfg(test)]
mod tests {
    #[test]
    fn safe() {
        assert_eq(check_safety("7 6 4 2 1"), true);
        assert_eq(check_safety("1 2 7 8 9"), false);
    }
}
