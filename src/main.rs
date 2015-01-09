#![allow(unstable)]

use std::io::File;
use std::io::BufferedReader;

mod counting_dna;
mod dna_rna_transcription;
mod dna_reverse_complement;
mod rabbit_fib;
mod calculate_max_gc_content;
mod hamming_distance;

fn read_full_file(path: &str) -> String {
    let mut file = BufferedReader::new(File::open(&Path::new(path)));

    // Read the file contents into a string, returns `IoResult<String>`
    match file.read_to_string() {
        Err(why) => std::string::as_string(why.desc).clone(),
        Ok(string) =>  string,
    }
}

fn read_file(path: &str) -> String {
    let input = read_full_file(path);

    input.lines().take(1).collect()
}

#[allow(unused_must_use)]
fn write_output(path: &str, output: &str) {
    let mut file = match File::open_mode(&Path::new(path), std::io::Append, std::io::Write) {
        Err(why) => panic!("couldn't open: {}", why.desc),
        Ok(file) => file,
    };

    println!("Wrintng output {}", output);
    file.write_line("\n");
    file.write_line(output);
}

fn counting_dna(input: &str) -> counting_dna::Nucleotides {
    counting_dna::run(input)
}

fn rna_transcription(input: &str) -> String {
    dna_rna_transcription::run(input)
}

fn dna_reverse_complement(input: &str) -> String {
    dna_reverse_complement::run(input)
}

fn rabbit_fib(generations: u32, offsprings: u32) -> u64 {
    rabbit_fib::run(generations, offsprings)
}

fn calculate_max_gc_content(input: &str) -> (String, f64) {
    calculate_max_gc_content::run(input)
}

fn hamming_distance(input: &str) -> u32 {
    hamming_distance::run(input)
}

fn run_test<F>(file: &str, f: F)
    where F : FnOnce(&str) -> String {
    let input = read_file(file);
    let result = f(input.as_slice());
    write_output(file, result.as_slice());
}

#[allow(dead_code)]
fn main() {
    let input = read_file("/Users/zumda/Downloads/rosalind_dna(1).txt");
    println!("{}", counting_dna(input.as_slice()));

    let input = read_file("/Users/zumda/Downloads/rosalind_rna(1).txt");
    println!("{}", rna_transcription(input.as_slice()));

    run_test("/Users/zumda/Downloads/rosalind_revc(1).txt", dna_reverse_complement);

    println!("{}", rabbit_fib(32, 3));

    let input = read_full_file("/Users/zumda/Downloads/rosalind_gc.txt");
    let (identifier, percentage) = calculate_max_gc_content(input.as_slice());
    println!("{}\n{}", identifier, percentage);

    let input = read_full_file("/Users/zumda/Downloads/rosalind_hamming.txt");
    println!("Hamming distance: {}", hamming_distance(input.as_slice()));
}

#[cfg(test)]
mod test {
    use super::{counting_dna,
                rna_transcription,
                dna_reverse_complement,
                rabbit_fib,
                calculate_max_gc_content,
                hamming_distance,

                read_file};
    use counting_dna::Nucleotides;

    #[test]
    fn ex_1_counting_dna() {
        let expected = counting_dna::Nucleotides {a: 20, t: 21, c: 12, g: 17 };
        assert_eq!(expected, counting_dna("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"));
    }

    #[test]
    fn ex_2_rna_transcription() {
        let expected = "GAUGGAACUUGACUACGUAAAUU";
        assert_eq!(expected, rna_transcription("GATGGAACTTGACTACGTAAATT"));
    }

    #[test]
    fn ex_3_dna_reverse_complement() {
        let expected = "ACCGGGTTTT";
        assert_eq!(expected, dna_reverse_complement("AAAACCCGGT"));
    }

    #[test]
    fn ex_4_rabbit_fib() {
        assert_eq!(19, rabbit_fib(5, 3));
    }

    #[test]
    fn ex_5_calculate_gc_content() {
        let (ex_id, ex_perc) = calculate_max_gc_content(
            ">Rosalind_0001\nAGCTATAG\n>Rosalind_0002\nAGAATTAAT");
        assert_eq!("Rosalind_0001", ex_id);
        assert_eq!(37.5, ex_perc);
    }

    #[test]
    fn ex_6_hamming_distance() {
        let distance = hamming_distance("GAGCCTACTAACGGGAT\nCATCGTAATGACGGCCT");
        assert_eq!(7, distance);
    }

    #[test]
    fn can_read_file() {
        assert!(read_file("src/main.rs").contains("use std::io::File;"));
    }
}
