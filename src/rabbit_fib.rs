pub fn run(generations: uint, offsprings: uint) -> uint {
    let result = range(2, generations).fold(vec![1u, 1], |mut data, _| {
        let next = data[data.len()-2] * offsprings + data[data.len()-1];
        data.push(next);

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
