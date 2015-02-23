use std::collections::HashMap;
use std::str;

use regex::Regex;

const CODON_STRING: &'static str = "
UUU F      CUU L      AUU I      GUU V
UUC F      CUC L      AUC I      GUC V
UUA L      CUA L      AUA I      GUA V
UUG L      CUG L      AUG M      GUG V
UCU S      CCU P      ACU T      GCU A
UCC S      CCC P      ACC T      GCC A
UCA S      CCA P      ACA T      GCA A
UCG S      CCG P      ACG T      GCG A
UAU Y      CAU H      AAU N      GAU D
UAC Y      CAC H      AAC N      GAC D
UAA Stop   CAA Q      AAA K      GAA E
UAG Stop   CAG Q      AAG K      GAG E
UGU C      CGU R      AGU S      GGU G
UGC C      CGC R      AGC S      GGC G
UGA Stop   CGA R      AGA R      GGA G
UGG W      CGG R      AGG R      GGG G";

pub fn run(input: &str) -> String {
    let table = create_codon_table();
    let mut output = String::new();

    for codon in input.as_bytes().chunks(3) {
        let codon_str = str::from_utf8(codon).unwrap();

        match &(table.get(codon_str).unwrap())[..] {
            "Stop" => break,
            i @ _ => output.push(i.char_at(0))
        }
    }

    output
}

fn create_codon_table() -> HashMap<String, String> {
    let mut table: HashMap<String, String> = HashMap::new();

    let re = match Regex::new("(\\w{3})\\s([^\\s]*)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err)
    };

    for mapping in re.captures_iter(CODON_STRING.trim()) {
        table.insert(mapping.at(1).unwrap().to_string(), mapping.at(2).unwrap().to_string());
    }

    table
}

#[cfg(test)]
mod test {
    use super::create_codon_table;

    #[test]
    fn can_create_codon_table() {
        let table = create_codon_table();

        println!("CODON: {}", table.get("UCC").unwrap());

        assert_eq!("F", table.get("UUU").unwrap().as_slice());
        assert_eq!("L".to_string(), table.get("CUC").unwrap().as_slice());
        assert_eq!("Stop", table.get("UAA").unwrap().as_slice());
    }
}
