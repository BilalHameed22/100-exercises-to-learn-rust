pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::u32;

    use crate::factorial;

    #[test]
    fn twentieth() {
       
        assert_eq!(factorial(20).saturating_add(u32::MAX), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
