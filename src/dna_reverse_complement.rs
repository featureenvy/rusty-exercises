pub fn run(input: &str) -> String {
    input.chars().rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => c,
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn reverses_the_input() {
        let expected = "sselesu";
        assert_eq!(expected, run("useless"));
    }

    #[test]
    fn complements_a_with_t() {
        let expected = "TGTG";
        let result = run("CACA");
        assert_eq!(expected.char_at(0), result.char_at(0));
        assert_eq!(expected.char_at(2), result.char_at(2));
    }

    #[test]
    fn complements_t_with_a() {
        let expected = "CACA";
        let result = run("TGTG");

        assert_eq!(expected.char_at(1), result.char_at(1));
        assert_eq!(expected.char_at(3), result.char_at(3));
    }

    #[test]
    fn complements_g_with_c() {
        let expected = "CACA";
        let result = run("AGAG");

        assert_eq!(expected.char_at(0), result.char_at(0));
        assert_eq!(expected.char_at(2), result.char_at(2));
    }

    #[test]
    fn complements_c_with_g() {
        let expected = "AGAG";
        let result = run("CACA");

        assert_eq!(expected.char_at(1), result.char_at(1));
        assert_eq!(expected.char_at(3), result.char_at(3));
    }
}
