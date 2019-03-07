pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_two_() {
        assert_eq!(2 + 2, add_two(2));
    }
}
