// each pair produces one offspring.
pub fn run(generations: u32, mortality: u32) -> f64 {
    //println!("1\t0\t0\n0\t1\t0");
    let result = (2..generations).fold(vec![(1f64,0f64,0f64), (0f64,1f64,0f64)], |mut data, _| {
        let child_rabbits = data[data.len()-1].1;
        let mature_rabbits = data[data.len()-1].0 + data[data.len()-1].1;
        let dying_rabbits = if data.len() >= mortality as usize {
            data[data.len()-mortality as usize].0
        } else {
            0f64
        };

        //println!("{}\t{}\t{}", child_rabbits, mature_rabbits - dying_rabbits, dying_rabbits);

        let res = (child_rabbits, mature_rabbits - dying_rabbits, dying_rabbits);
        data.push(res);

        data
    });

    let last = *result.last().expect("could not calculate the offsprings.");

    last.1 + last.0
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn calculates_properly() {
        assert_eq!(4f64, run(6,3));
    }
}
