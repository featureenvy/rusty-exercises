pub fn run(generations: u32, offsprings: u32) -> u64 {
    let result = (2..generations).fold(vec![1u64, 1], |mut data, _| {
        let next = data[data.len()-2] * offsprings as u64 + data[data.len()-1];
        data.push(next);
        println!("{:?}", next);

        data
    });

    *result.last().expect("could not calculate the offsprings.")
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn calculates_properly() {
        assert_eq!(19, run(5,3));
    }
}
