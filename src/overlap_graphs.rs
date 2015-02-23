use utils::fasta::FastaData;

fn print_graph(graph: Vec<(String, String)>) -> String {
    let mut result = String::new();

    for data in graph.iter() {
        result.push_str(&format!("{} {}\n", data.0, data.1));
    }

    result
}

pub fn run(input: &str) -> String{
    let data  = FastaData::read_file(input);

    let graph = data.iter().fold(Vec::new(), |mut graph, prefix_data| {
        let prefix = &prefix_data.dna_strand[0..3];
        let mut subgraph = data.iter().fold(Vec::new(), |mut graph, suffix_data| {
            let suf_strand = &suffix_data.dna_strand;
            if &suf_strand.as_slice() != &prefix_data.dna_strand.as_slice() &&
                &suf_strand[suf_strand.len()-3..] == prefix {
                let res = (suffix_data.id.clone(), prefix_data.id.clone());
                graph.push(res);
            }

            graph
        });

        graph.append(&mut subgraph);

        graph
    });

    print_graph(graph)
}

#[cfg(test)]
mod test {

}
