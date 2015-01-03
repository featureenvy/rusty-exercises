use std::io::File;
use std::io::BufferedReader;

mod counting_dna;
mod dna_rna_transcription;
mod dna_reverse_complement;

fn read_file(path: &str) -> String {
    let mut file = BufferedReader::new(File::open(&Path::new(path)));

    // Read the file contents into a string, returns `IoResult<String>`
    match file.read_line() {
        Err(why) => std::string::as_string(why.desc).clone(),
        Ok(string) =>  string,
    }
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

fn run_test(file: &str, f: |&str| -> String) {
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
}

#[cfg(test)]
mod test {
    use super::{counting_dna, rna_transcription, dna_reverse_complement, read_file};
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
    fn can_read_file() {
        assert!(read_file("src/main.rs").contains("fn maain()"));
    }
}
