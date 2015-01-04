static ABSOLUTE_ERROR: f64 = 1000.0;

struct FastaData {
    id: String,
    dna_strand: String,
    percentage: i32,
}

fn calc_percentage(input: &str) -> i32 {
    let count = input.chars().filter(|&c| c == 'G' || c == 'C').count();

    (100.0 / input.len() as f64 * count as f64 * ABSOLUTE_ERROR) as i32
}

fn read_fasta(input: &str) -> Vec<FastaData> {
    let mut result: Vec<FastaData> = Vec::new();

    let mut lines = input.lines();

    loop {
        let id = match lines.next() {
            Some(line) => line.to_string(),
            None => break,
        };

        let strand = match lines.next() {
            Some(line) => line.to_string(),
            None => panic!("File structure is wrong."),
        };

        let percentage = calc_percentage(strand.as_slice());

        let next = FastaData {
            id: id,
            dna_strand: strand,
            percentage: percentage };
        result.push(next);
    }

    result
}

pub fn run(input: &str) -> (String, f64) {
    let fasta_data = read_fasta(input);

    let max_fasta: &FastaData = fasta_data.iter().max_by(|fasta| fasta.percentage ).unwrap();

    (max_fasta.id.clone(), max_fasta.percentage as f64 / ABSOLUTE_ERROR)
}

#[cfg(test)]
mod test {
    use super::{run, read_fasta, calc_percentage, ABSOLUTE_ERROR};

    #[test]
    fn run_works() {
        let input = "Rosalind_0001\nAGCTATAG\nRosalind_0002\nAGAATTAAT";
        let result = ("Rosalind_0001".to_string(), 37.5f64);

        assert_eq!(result, run(input));
    }

    #[test]
    fn can_read_fasta() {
        let input = "Rosalind_0001\nAGCTATAG\nRosalind_0002\nAGAATTAAT".as_slice();
        let result = read_fasta(input);

        assert_eq!(2, result.len());
        assert_eq!("Rosalind_0001", result.first().unwrap().id);
        assert_eq!("AGCTATAG", result.first().unwrap().dna_strand);
    }

    #[test]
    fn calculate_percentage() {
        let input = "AGCTATAG";
        assert_eq!((37.5 * ABSOLUTE_ERROR) as i32, calc_percentage(input));
    }
}
