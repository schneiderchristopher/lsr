use std::fmt::Write;

#[cfg(test)]
mod tests {
    use super::super::*;

    fn size_formatter_test(bytes: BYTE, expected: &str, si: bool) {
        let mut buf = String::new();
        // SAFETY: Writing to a string __cannot__ fail
        if si {
            write!(&mut buf, "{:?}", bytes.into_size()).unwrap();
        } else {
            write!(&mut buf, "{}", bytes.into_size()).unwrap();
        }
        assert_eq!(buf, expected);
    }

    #[test]
    fn size_formatter_under_1kib() {
        size_formatter_test(495, "495B", false)
    }

    #[test]
    fn size_formatter_exactly_1_kib() {
        size_formatter_test(1024, "1KiB", false)
    }

    #[test]
    fn size_formatter_under_1mib() {
        size_formatter_test(1024 * 512, "512KiB", false)
    }

    #[test]
    fn size_formatter_exactly_1mib() {
        size_formatter_test(1024 * 1024, "1MiB", false)
    }

    #[test]
    fn size_formatter_under_1gib() {
        size_formatter_test(299 * 1024 * 1024, "299MiB", false)
    }

    #[test]
    fn size_formatter_exactly_1gib() {
        size_formatter_test(KIB.pow(3), "1GiB", false)
    }

    #[test]
    fn size_formatter_under_1tib() {
        size_formatter_test(KIB.pow(3) * 128, "128GiB", false)
    }

    #[test]
    fn size_formatter_exactly_1tib() {
        size_formatter_test(KIB.pow(4), "1TiB", false)
    }

    #[test]
    fn size_formatter_under_1pib() {
        size_formatter_test(KIB.pow(4) * 256, "256TiB", false)
    }

    #[test]
    fn size_formatter_exactly_1pib() {
        size_formatter_test(KIB.pow(5), "1PiB", false)
    }

    #[test]
    fn exactsize_formatter_3pib_2gb_3b() {
        let mut buf = String::new();
        write!(&mut buf, "{}", (3 * PIB + 2 * GIB + 3 * B).into_longsize()).unwrap();
        assert_eq!(buf, "3PiB 2GiB 3B");
    }
}
