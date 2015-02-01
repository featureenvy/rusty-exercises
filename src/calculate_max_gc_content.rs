use std::default::Default;
use std::collections::HashMap;

static ABSOLUTE_ERROR: f64 = 1000000.0;

#[derive(Default, Clone)]
struct FastaData {
    id: String,
    dna_strand: String,
}

fn calc_percentage(input: &str) -> i32 {
    let count = input.chars().filter(|&c| c == 'G' || c == 'C').count();

    (100.0 / input.len() as f64 * count as f64 * ABSOLUTE_ERROR) as i32
}

fn read_fasta(input: &str) -> Vec<FastaData> {
    let mut result: Vec<FastaData> = Vec::new();

    for line in input.lines() {
        match &line[..1] {
            ">" => result.push(FastaData { id: String::from_str(&line[1..]), .. Default::default() }),
            "A" | "G" | "C" | "T" => {
                let data = result.last_mut().unwrap();
                let strand = data.dna_strand.clone();
                data.dna_strand = strand + line;
            },
            v @ _ => panic!("Wrong start index {}", v),
        }
    }

    result
}

pub fn run(input: &str) -> (String, f64) {
    let fasta_data = read_fasta(input);
    let id_and_percentages = fasta_data.iter()
        .fold(HashMap::new(),|mut map, data|
              {map.insert(data.id.clone(), calc_percentage(&data.dna_strand[])); map});
    let (max_strand, max_percentage) = id_and_percentages.iter().max_by(|&(_, value)| value).unwrap();

    (max_strand.clone(), *max_percentage as f64 / ABSOLUTE_ERROR)
}

#[cfg(test)]
mod test {
    use super::{run, calc_percentage, ABSOLUTE_ERROR};

    #[test]
    fn run_works() {
        let input = ">Rosalind_0001\nAGCTATAG\n>Rosalind_0002\nAGAATTAAT";
        let result = ("Rosalind_0001".to_string(), 37.5f64);

        assert_eq!(result, run(input));
    }

    mod read_fasta {
        use super::super::read_fasta;

        #[test]
        fn finds_all_sequences() {
            let input = ">Rosalind_0001\nAGCTATAG\n>Rosalind_0002\nAGAATTAAT\nAGT";
            let result = read_fasta(input);

            assert_eq!(2, result.len());
        }

        #[test]
        fn concats_the_full_sequence() {
            let input = ">Rosalind_0001\nAGCT\nAGCT\nAGC";
            let result = read_fasta(input);

            assert_eq!("AGCTAGCTAGC", result.first().unwrap().dna_strand);
        }
    }

    #[test]
    fn calculate_percentage() {
        let input = "AGCTATAG";
        assert_eq!((37.5 * ABSOLUTE_ERROR) as i32, calc_percentage(input));
    }
}
