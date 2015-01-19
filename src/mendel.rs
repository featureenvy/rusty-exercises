fn calculate_mendel(data: &[((u32, char), (u32, char))], count: u32) -> f32 {
    let total = data.iter().map(|&((_, t1), (_, t2))| {
        match (t1, t2) {
            ('d', _) | (_, 'd') => 1.0,
            ('m', 'm')          => 0.75,
            ('m', 'r') | ('r', 'm') => 0.5,
            ('r', 'r')          => 0.0,
            _                   => -10000.0
        }
    }).fold(0.0, |sum, next| sum + next);

    total / count as f32
}

fn unique_pairs(data: &[(u32, char)]) -> Vec<((u32, char), (u32, char))> {
    if data.len() < 2 {
        Vec::new()
    } else {
        let last = data.last().expect("No last element found.");
        let mut pairs = Vec::new();
        pairs.push_all(data.init().iter()
                       .map(|next| (*last, *next))
                       .collect::<Vec<((u32, char), (u32, char))>>()
                       .as_slice());
        pairs.push_all(unique_pairs(data.init()).as_slice());

        pairs
    }
}

fn create_alleles(dominant: u32, mixed: u32, recessive: u32) -> Vec<(u32, char)> {
    let mut alleles: Vec<char> = Vec::new();

    for _ in range(0, dominant) {
        alleles.push('d');
    }

    for _ in range(0, mixed) {
        alleles.push('m');
    }

    for _ in range(0, recessive) {
        alleles.push('r');
    }

    return alleles.iter().enumerate().map(|(k,&v)| (k as u32, v)).collect();
}

pub fn run(dominant: u32, mixed: u32, recessive: u32) -> f32 {
    let alleles = create_alleles(dominant, mixed, recessive);
    let unique_pairs = unique_pairs(alleles.as_slice());

    calculate_mendel(unique_pairs.as_slice(), unique_pairs.len() as u32)
}

#[cfg(test)]
mod test {
    use super::{unique_pairs, create_alleles};

    #[test]
    fn creates_unique_pairings() {
        let input = vec![(1u32, 'a'), (2, 'a'), (3, 'b'), (4, 'b')];
        let result = unique_pairs(input.as_slice());

        let expected = vec![((4u32, 'b'), (1u32, 'a')),
                            ((4u32, 'b'), (2u32, 'a')),
                            ((4u32, 'b'), (3u32, 'b')),
                            ((3u32, 'b'), (1u32, 'a')),
                            ((3u32, 'b'), (2u32, 'a')),
                            ((2u32, 'a'), (1u32, 'a'))];

        assert_eq!(expected, result);
    }

    #[test]
    fn can_create_alleles() {
        let result = create_alleles(2, 2, 2);
        let expected = vec![(0u32, 'd'), (1, 'd'),
                            (2, 'm'), (3, 'm'),
                            (4, 'r'), (5, 'r')];

        assert_eq!(expected, result);
    }
}
