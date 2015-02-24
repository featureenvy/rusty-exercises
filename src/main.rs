#![allow(dead_code)]
#![feature(core,collections,old_path,old_io)]

extern crate regex;
extern crate num;

use std::old_io::{File, BufferedReader, Append, Write};

mod counting_dna;
mod dna_rna_transcription;
mod dna_reverse_complement;
mod rabbit_fib;
mod calculate_max_gc_content;
mod hamming_distance;
mod mendel;
mod rna_translation;
mod find_motifs;
mod mortal_rabbits;
mod expected_offsprings;
mod consensus_profile;
mod overlap_graphs;
mod shared_motifs;
mod utils;

fn read_full_file(path: &str) -> String {
    let mut file = BufferedReader::new(File::open(&Path::new(path)));

    // Read the file contents into a string, returns `IoResult<String>`
    match file.read_to_string() {
        Err(why)   => panic!("Could not open path {}. Reason: {}", path, why),
        //Err(why) => std::string::as_string(why.desc).clone(),
        Ok(string) =>  string,
    }
}

fn read_file(path: &str) -> String {
    let input = read_full_file(path);

    input.lines().take(1).collect()
}

#[allow(unused_must_use)]
fn write_output(path: &str, output: &str) {
    let mut file = match File::open_mode(&Path::new(path), Append, Write) {
        Err(why) => panic!("couldn't open: {}", why.desc),
        Ok(file) => file,
    };

    //println!("Wrintng output {}", output);
    //file.write_line("\n");
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
    let result = f(&input);
    write_output(file, &result);
}

fn main() {
    //let input = read_file("/Users/zumda/Downloads/rosalind_dna(1).txt");
    //println!("{}", counting_dna(input.as_slice()));

    //let input = read_file("/Users/zumda/Downloads/rosalind_rna(1).txt");
    //println!("{}", rna_transcription(input.as_slice()));

    //run_test("/Users/zumda/Downloads/rosalind_revc(1).txt", dna_reverse_complement);

    //println!("{}", rabbit_fib(32, 3));

    //let input = read_full_file("/Users/zumda/Downloads/rosalind_gc.txt");
    //let (identifier, percentage) = calculate_max_gc_content(input.as_slice());
    //println!("{}\n{}", identifier, percentage);

    //let input = read_full_file("/Users/zumda/Downloads/rosalind_hamm.txt");
    //println!("Hamming distance: {}", hamming_distance(input.as_slice()));

    //println!("Mendel's number is {}", mendel::run(15, 24, 25));

    //run_test("/Users/zumda/Downloads/rosalind_prot.txt", rna_translation::run);

    //let input = read_full_file("/Users/zumda/Downloads/rosalind_prot.txt");
    //println!("RNA Translation: {}", rna_translation::run(input.as_slice()))

    //println!("Motif positions:");
    //let input = read_full_file("/Users/zumda/Downloads/rosalind_subs.txt");
    //let mut lines = input.lines();
    //let dna_seq = lines.next().expect("First line in subs not found.");
    //let motif = lines.next().expect("Second line in subs not found");
    //println!("{}", find_motifs::run(dna_seq, motif));

    println!("");
    println!("Mortal rabbits");
    println!("{}", mortal_rabbits::run(98, 17));
    //mortal_rabbits::run(6, 3);
    //println!("{}", mortal_rabbits::run(6, 3));

    //println!("");
    //println!("Expected offsprings");
    //println!("{}", expected_offsprings::run(18955, 16031, 17576, 16686, 19684, 18557))

    //println!("");
    //println!("Consensus Profile");
    //let consensus_profile = consensus_profile::run(&read_full_file("assets/consensus_profile_test.fasta"));
    //let consensus_profile = consensus_profile::run(&read_full_file("/Users/zumda/Downloads/rosalind_cons(3).txt"));

    //println!("{}", consensus_profile.0);
    //println!("{}", consensus_profile::print_profile(&consensus_profile.1));
    //let formatted_consensus = consensus_profile::print_profile(&consensus_profile.1);
    //write_output("/Users/zumda/Downloads/rosaling_cons_answer.txt", &format!("{}\n{}", consensus_profile.0, &formatted_consensus));

    //println!("");
    //println!("Overlap Graphs");
    //let overlap_graph = overlap_graphs::run(&read_full_file("/Users/zumda/Downloads/rosalind_grph(1).txt"));
    //println!("{}", overlap_graph);
    //write_output("/Users/zumda/Downloads/rosalind_overlap_graph_answer.txt", &overlap_graph);

    //println!("");
    //println!("Shared Motifs");
    //println!("{}", shared_motifs::run(&read_full_file("/Users/zumda/Downloads/rosalind_lcsm(1).txt")));
}

#[cfg(test)]
mod test {
    use super::{counting_dna,
                rna_transcription,
                dna_reverse_complement,
                rabbit_fib,
                calculate_max_gc_content,
                hamming_distance,

                read_full_file};
    use counting_dna::Nucleotides;
    use mendel;
    use rna_translation;
    use find_motifs;
    use mortal_rabbits;
    use expected_offsprings;
    use consensus_profile;
    use overlap_graphs;
    use shared_motifs;

    use num::bigint::ToBigUint;

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
    fn ex_7_mendel() {
        let result = mendel::run(2, 2, 2);
        assert_eq!(11.75/15.0, result);
    }

    #[test]
    fn ex_8_rna_translation() {
        let result = rna_translation::run("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA");
        assert_eq!("MAMAPRTEINSTRING", result);
    }

    #[test]
    fn ex_9_motif_serach() {
        let result = find_motifs::run("GATATATGCATATACTT", "ATAT");
        assert_eq!("2 4 10", result);
    }

    #[test]
    fn ex_10_mortal_rabbits() {
        let result = mortal_rabbits::run(6, 3);
        assert_eq!(4.to_biguint().unwrap(), result);
    }

    #[test]
    fn ex_11_expected_offsprings() {
        let result = expected_offsprings::run(1, 0, 0, 1, 0 ,1);
        assert_eq!(3.5, result);
    }

    #[test]
    fn ex_12_consensus_profile() {
        let result = consensus_profile::run(&read_full_file("assets/consensus_profile_test.fasta"));
        assert_eq!("ATGCAACT", result.0);
    }

    #[test]
    fn ex_13_shared_motifs() {
        let result = shared_motifs::run(&read_full_file("assets/shared_motifs.fasta"));
        assert_eq!("AC", result);
    }

    #[test]
    fn ex_13_overlap_graphs() {
        let result = overlap_graphs::run(&read_full_file("assets/overlap_graphs.fasta"));
        let expected = "Rosalind_0498 Rosalind_2391\nRosalind_0498 Rosalind_0442\nRosalind_2391 Rosalind_2323";
        let exp_tester: Vec<&str> = expected.lines().collect();
        let res_tester: Vec<&str> = result.lines().collect();

        assert!(exp_tester.contains(&res_tester[0]));
        assert!(exp_tester.contains(&res_tester[1]));
        assert!(exp_tester.contains(&res_tester[2]));
    }

    #[test]
    fn can_read_file() {
        assert!(read_full_file("src/main.rs").contains("use std::io::File;"));
    }
}
