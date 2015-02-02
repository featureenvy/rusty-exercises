// each pair produces one offspring.
pub fn run(generations: u32, mortality: u32) -> f64 {
    let result = (2..generations).fold(vec![1f64, 1f64], |mut data, _| {
        let mature_rabbits = data[data.len()-2];
        let child_rabbits = data[data.len()-1] - mature_rabbits;

        let dying_rabbits = if data.len() >= (mortality + 1) as usize {
            data[data.len()-(mortality + 1) as usize]
        } else {
            0f64
        };
        let procreating_rabbits = mature_rabbits - dying_rabbits;

        data.push(child_rabbits + procreating_rabbits * 2f64);

        let temp = 0f64;

        println!("Value {:?}", data.last().unwrap());

        data
    });

    *result.last().expect("could not calculate the offsprings.")
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn calculates_properly() {
        assert_eq!(4f64, run(6,3));
    }
}