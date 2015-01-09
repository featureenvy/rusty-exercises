pub fn run(input: &str) -> u32 {
    let inputs = input.lines().collect::<Vec<&str>>();

    inputs[0].chars().zip(inputs[1].chars()).filter(|&(a, b)| a != b).count() as u32
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn calculates_distance() {
        let input = "GAGCCTACTAACGGGAT\nCATCGTAATGACGGCCT";

        assert_eq!(7, run(input));
    }
}
