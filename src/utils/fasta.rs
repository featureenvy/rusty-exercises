use std::default::Default;

#[derive(Default, Clone)]
pub struct FastaData {
    pub id: String,
    pub dna_strand: String,
}

impl FastaData {
    pub fn new(id: String, dna_strand: String) -> FastaData {
        FastaData { id: id, dna_strand: dna_strand }
    }

    pub fn read_file(input: &str) -> Vec<FastaData> {
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
}

#[cfg(test)]
mod test {
    mod read_fasta {
        use super::super::FastaData;

        #[test]
        fn finds_all_sequences() {
            let input = ">Rosalind_0001\nAGCTATAG\n>Rosalind_0002\nAGAATTAAT\nAGT";
            let result = FastaData::read_file(input);

            assert_eq!(2, result.len());
        }

        #[test]
        fn concats_the_full_sequence() {
            let input = ">Rosalind_0001\nAGCT\nAGCT\nAGC";
            let result = FastaData::read_file(input);

            assert_eq!("AGCTAGCTAGC", result.first().unwrap().dna_strand);
        }
    }
}
