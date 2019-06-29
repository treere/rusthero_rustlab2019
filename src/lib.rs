pub fn convert<T: AsRef<str>>(s: T) -> String {
    let mut chars = s.as_ref().chars();
    let init = chars.next().unwrap();

    let (res, l, c) = chars.fold((String::new(), init, 1), |(s, last, counter), c| {
        if last == c {
            (s, c, counter + 1)
        } else {
            let x = format!("{}{}", counter, last);
            (s + &x, c, 1)
        }
    });
    res + &format!("{}{}", c, l)
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
