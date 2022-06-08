fn calculate_sum(phrase: &str) -> Option<u32> {
    phrase
        .bytes()
        .map(|x| (x - 96) as u32)
        .fold(Option::from(0), |acc, y| acc.map(|sum| sum + y))
}

#[cfg(test)]
mod tests {
    use super::calculate_sum;

    #[test]
    fn test_0() {
        assert_eq!(calculate_sum(""), Some(0));
    }

    #[test]
    fn test_1() {
        assert_eq!(calculate_sum("a"), Some(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate_sum("z"), Some(26));
    }

    #[test]
    fn test_3() {
        assert_eq!(calculate_sum("cab"), Some(6));
    }

    #[test]
    fn test_4() {
        assert_eq!(calculate_sum("excellent"), Some(100));
    }

    #[test]
    fn test_5() {
        assert_eq!(calculate_sum("microspectrophotometries"), Some(317));
    }
}
