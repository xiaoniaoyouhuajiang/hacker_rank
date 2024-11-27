pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_divisible(a: u32, b: u32) -> bool {
    b % a == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
