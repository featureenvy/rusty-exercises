use num::{BigUint, Zero, One};

// each pair produces one offspring.
pub fn run(generations: u32, mortality: u32) -> BigUint {
    println!("1\t0\t0\n0\t1\t0");
    let acc: Vec<(BigUint, BigUint, BigUint)> = vec![(One::one(),Zero::zero(),Zero::zero()), (Zero::zero(),One::one(),Zero::zero())];
    let result = (2..generations).fold(acc, |mut data, _| {
        let child_rabbits: BigUint = data[data.len()-1].1.clone();
        let mature_rabbits: BigUint = data[data.len()-1].0.clone() + &data[data.len()-1].1;
        let dying_rabbits: BigUint = if data.len() >= mortality as usize {
            data[data.len()-mortality as usize].0.clone()
        } else {
            Zero::zero()
        };

        println!("{}\t{}\t{}\t{}", &child_rabbits, &mature_rabbits - &dying_rabbits, &dying_rabbits, &child_rabbits + &mature_rabbits - &dying_rabbits);

        let res = (child_rabbits, mature_rabbits - &dying_rabbits, dying_rabbits);
        data.push(res);

        data
    });

    let last = result.last().expect("could not calculate the offsprings.");

    last.1.clone() + &last.0
}

#[cfg(test)]
mod test {
    use super::run;
    use num::bigint::ToBigUint;

    #[test]
    fn calculates_properly() {
        assert_eq!(4.to_biguint().unwrap(), run(6,3));
    }
}
