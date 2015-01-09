use std::default::Default;
use std::fmt;

#[derive(Eq, Default)]
pub struct Nucleotides {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

impl PartialEq for Nucleotides {
    fn eq(&self, other: &Nucleotides) -> bool {
        self.a == other.a &&
            self.t == other.t &&
            self.c == other.c &&
            self.g == other.g
    }
}

impl fmt::String for Nucleotides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

impl fmt::Show for Nucleotides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

fn count_nucelotide(count: &mut Nucleotides, next: char) -> &mut Nucleotides {
    match next {
        'A' => count.a += 1,
        'T' => count.t += 1,
        'C' => count.c += 1,
        'G' => count.g += 1,
        _ => {},
    }

    count
}

pub fn run(input: &str) -> Nucleotides {
    let mut count: Nucleotides = Default::default();
    input.chars().fold(&mut count, count_nucelotide);

    count
}

#[cfg(test)]
mod test {
    use super::{run, Nucleotides};

    #[test]
    fn counts_all_as() {
        let expected = Nucleotides { a: 3, t: 0, c: 0, g: 0 };
        assert_eq!(expected, run("AAA"));
    }

    #[test]
    fn counts_all_ts() {
        let expected = Nucleotides { a: 0, t: 2, c: 0, g: 0 };
        assert_eq!(expected, run("TT"));
    }

    #[test]
    fn counts_all_cs() {
        let expected = Nucleotides { a: 0, t: 0, c: 3, g: 0 };
        assert_eq!(expected, run("CCC"));
    }

    #[test]
    fn counts_all_gs() {
        let expected = Nucleotides { a: 0, t: 0, c: 0, g: 3 };
        assert_eq!(expected, run("GGG"));
    }
}
