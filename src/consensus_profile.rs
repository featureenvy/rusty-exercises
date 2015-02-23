use utils::fasta::FastaData;
use std::default::Default;

#[derive(Default, Debug)]
struct DnaProfile {
    a: u32,
    c: u32,
    t: u32,
    g: u32
}

impl DnaProfile {
    fn new(strand: &[char]) -> DnaProfile {
        strand.iter().fold(Default::default(), |mut dp, &c| {
            match c {
                'A' => dp.a += 1,
                'C' => dp.c += 1,
                'T' => dp.t += 1,
                'G' => dp.g += 1,
                _   => panic!("Invalid DNA element")
            };

            return dp
        })
    }

    fn max(&self) -> char {
        let mut max = self.a;
        let mut max_letter = 'A';

        if self.c > max {
            max = self.c;
            max_letter = 'C';
        }

        if self.t > max {
            max = self.t;
            max_letter = 'T';
        }

        if self.g > max {
            //max = self.g;
            max_letter = 'G';
        }

        max_letter
    }

    fn get_count_for(&self, n: char) -> u32 {
        match n {
            'A' => self.a,
            'C' => self.c,
            'G' => self.g,
            'T' => self.t,
            _   => panic!("Invalid"),
        }
    }
}

trait MatrixTransform<T> {
    fn transform(&self) -> Vec<Vec<T>>;
}

impl<T: Copy> MatrixTransform<T> for Vec<Vec<T>> {
    fn transform(&self) -> Vec<Vec<T>> {
        let mut trans_matrix: Vec<Vec<T>> = self[0].iter().map(|&base| vec![base]).collect();

        for strand in self[1..].iter() {
            for (mut vec, &base) in trans_matrix.iter_mut().zip(strand.iter()) {
                vec.push(base);
            }
        };

        trans_matrix
    }
}

fn print_profile_for_nucleotide(n: char, profiles: &[DnaProfile]) -> String {
    let mut line = n.to_string();
    line.push(' ');

    let data = profiles.iter().map( |p| {
        p.get_count_for(n).to_string()
    }).collect::<Vec<String>>().connect(" ");

    line.push_str(&data);

    line
}

pub fn print_profile(profiles: &[DnaProfile]) -> String {
    let matrix: String  = vec!['A', 'C', 'G', 'T'].iter().map( |&n| {
        print_profile_for_nucleotide(n, profiles)
    }).collect::<Vec<String>>().connect("\n");

    matrix
}

pub fn run(data: &str) -> (String, Vec<DnaProfile>) {
    let fasta_data = FastaData::read_file(data);
    let profile_matrix = create_profile_matrix(fasta_data);

    //let dna_profiles = transform(&profile_matrix[]).iter()
    let dna_profiles = profile_matrix.transform().iter()
        .map( |strand| { DnaProfile::new(strand) })
        .collect::<Vec<DnaProfile>>();

    let consensus_strand = dna_profiles.iter().map( |profile| {
        profile.max()
    }).collect();

    (consensus_strand, dna_profiles)
}

fn create_profile_matrix(data: Vec<FastaData>) -> Vec<Vec<char>> {
    data.iter().map(|d: &FastaData| {
        d.dna_strand.chars().collect()
    }).collect()
}

#[cfg(test)]
mod test {
    use super::{create_profile_matrix, print_profile, DnaProfile, MatrixTransform};
    use utils::fasta::FastaData;

    #[test]
    fn creates_profile_matrix() {
        let input = vec![FastaData::new("FASTA_1".to_string(), "AGCT".to_string()),
                         FastaData::new("FASTA_2".to_string(), "TGCT".to_string()),
                         FastaData::new("FASTA_3".to_string(), "TCGA".to_string())];
        let output = create_profile_matrix(input);

        assert_eq!(output[0][0], 'A');
        assert_eq!(output[1][1], 'G');
        assert_eq!(output[2][2], 'G');
    }

    #[test]
    fn can_transform() {
        let input = vec![vec!['a', 'b', 'c'],
                         vec!['a', 'b', 'c'],
                         vec!['c', 'c', 'c']];
        let expected = vec![vec!['a', 'a', 'c'],
                            vec!['b', 'b', 'c'],
                            vec!['c', 'c', 'c']];

        let output = input.transform();

        assert_eq!(expected, output);
    }

    #[test]
    fn create_dna_profile() {
        let input = ['A', 'C', 'C', 'T', 'G', 'T'];
        let output = DnaProfile::new(&input);

        assert_eq!(1, output.a);
        assert_eq!(2, output.c);
        assert_eq!(2, output.t);
        assert_eq!(1, output.g);
    }

    #[test]
    fn can_print_profile() {
        let input = vec![DnaProfile {a: 2, c: 3, t: 2, g: 4},
                         DnaProfile {a: 3, c: 1, t: 0, g: 5}];
        let expected = "A 2 3\nC 3 1\nG 4 5\nT 2 0";

        assert_eq!(expected, print_profile(&input));
    }
}
