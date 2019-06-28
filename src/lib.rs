pub fn convert(s: &str) -> String {
    let mut result = String::new();

    let mut chars = s.chars();

    let mut last = chars.next().unwrap();
    let mut counter = 1;
    while let Some(c) = chars.next() {
        if last == c {
            counter += 1
        } else {
            result += &format!("{}{}", counter, last);
            last = c;
            counter = 1
        }
    }
    result += &format!("{}{}", counter, last);

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f1() {
        assert_eq!(convert("1"), "11".to_owned())
    }

    #[test]
    fn f2() {
        assert_eq!(convert("2"), "12".to_owned())
    }

    #[test]
    fn f3() {
        assert_eq!(convert("11"), "21".to_owned())
    }

    #[test]
    fn f4() {
        assert_eq!(convert("31"), "1311".to_owned())
    }

    #[test]
    fn f5() {
        assert_eq!(convert("3211"), "131221".to_owned())
    }

    #[test]
    fn f6() {
        assert_eq!(convert("111223"), "312213".to_owned())
    }

}
