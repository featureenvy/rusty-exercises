pub fn run(input: &str) -> String {
    input.chars().map(|c| if c == 'T' {'U'} else {c}).collect::<String>()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn leaves_a_alone() {
        assert_eq!("AGC", run("AGC"));
    }

    #[test]
    fn leaves_c_alone() {
        assert_eq!("ACG", run("ACG"));
    }
    #[test]
    fn leaves_g_alone() {
        assert_eq!("ACG", run("ACG"));
    }

    #[test]
    fn maps_t_to_u() {
        assert_eq!("AGCU", run("AGCT"));
    }
}
