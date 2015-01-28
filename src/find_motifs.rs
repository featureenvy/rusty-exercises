use std::str;

pub fn run(input: &str, pattern: &str) -> String {
    slice_search(input, pattern).iter()
        .fold(String::new(),
              |mut str, next| {
                  str.push_str(next.to_string().as_slice());
                  str})
}

fn slice_search(input: &str, pattern: &str) -> Vec<u32> {
    let mut motifs: Vec<u32> = Vec::new();
    for (pos, slice) in input.as_bytes().windows(pattern.len()).enumerate() {
        if str::from_utf8(slice).unwrap() == pattern {
            motifs.push((pos as u32) + 1);
        }
    }

    motifs
}

#[cfg(test)]
mod test {
    use super::{slice_search};

    #[test]
    fn test_slice_search() {
        let result = slice_search("GATATATGCATATACTT", "ATAT");

        assert_eq!(2, result[0]);
        assert_eq!(4, result[1]);
        assert_eq!(10, result[2]);
    }

}
