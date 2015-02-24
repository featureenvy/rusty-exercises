use utils::fasta::FastaData;
use std::str;

pub fn run(input: &str) -> String {
    let data = FastaData::read_file(input);
    let init_strand = data.first().expect("Not a single datapoint given");

    let mut longest_match = "";

    for i in 1..data.len() {
        let dna_strand = &*init_strand.dna_strand;
        let windows = dna_strand.as_bytes().windows(i);


        for win in windows {
            let win_str = str::from_utf8(win).ok().expect("win could't not be transformed.");
            let matching_subs = data[1..].iter().filter(|strand| {
                strand.dna_strand.contains(win_str)
            }).count();

            if matching_subs == data.len() - 1 {
                longest_match = win_str;
            }
        };
    }

    longest_match.to_string()
}
