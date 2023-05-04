use primal::Sieve;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Mul;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct WorryLevel {
    value: Vec<(usize, usize)>,
    sieve: Rc<Sieve>,
}

impl WorryLevel {
    pub(crate) fn divisible_by(&self, other: &WorryLevel) -> bool {
        let mut self_iter = self.value.iter();
        for (prime, pow) in &other.value {
            if let None = self_iter.find(|(pr, po)| pr == prime && pow <= po) {
                return false;
            }
        }
        true
    }
}

impl From<(usize, &Rc<Sieve>)> for WorryLevel {
    fn from((val, sieve): (usize, &Rc<Sieve>)) -> Self {
        Self {
            value: match sieve.factor(val) {
                Ok(val) => val,
                Err((_, val)) => val,
            },
            sieve: sieve.clone(),
        }
    }
}

impl PartialEq<Self> for WorryLevel {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for WorryLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl std::ops::Add for WorryLevel {
    type Output = WorryLevel;

    fn add(self, rhs: Self) -> Self::Output {
        let mut common_primes: Vec<(usize, usize)> = vec![];
        let mut lh_map: HashMap<usize, usize> = self.value.into_iter().collect();
        let mut rh_multiplied = 1;
        let mut lh_multiplied = 1;
        for (prime, pow) in rhs.value {
            match lh_map.get_mut(&prime) {
                None => rh_multiplied *= prime.pow(pow as u32),
                Some(&mut mut lh_pow) => {
                    match (lh_pow).cmp(&pow) {
                        Ordering::Less => {
                            let diff_pow = pow - lh_pow;
                            rh_multiplied *= prime.pow(diff_pow as u32);
                            common_primes.push((prime, lh_pow));
                        }
                        Ordering::Equal => {
                            common_primes.push((prime, pow));
                        }
                        Ordering::Greater => {
                            let diff_pow = lh_pow - pow;
                            common_primes.push((prime, pow));
                            lh_multiplied *= prime.pow(diff_pow as u32);
                        }
                    }
                    lh_pow = 0;
                }
            }
        }

        let lh_multiplied = lh_map
            .iter()
            .filter(|(_, &pow)| pow != 0)
            .fold(lh_multiplied, |mul, (&prime, &pow)| {
                mul * prime.pow(pow as u32)
            });

        let factors = match self.sieve.factor(lh_multiplied + rh_multiplied) {
            Ok(factors) => factors,
            // We can discard the rest of the number because we already have the part that could be divisible by the divisor
            Err((_, factors)) => factors,
        };
        WorryLevel {
            value: common_primes,
            sieve: self.sieve.clone(),
        } * WorryLevel {
            value: factors,
            sieve: self.sieve.clone(),
        }
    }
}

impl Mul for WorryLevel {
    type Output = WorryLevel;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut value: HashMap<_, _> = self.value.into_iter().collect();
        for (prime, pow) in rhs.value {
            value
                .entry(prime)
                .and_modify(|&mut mut e| e += pow)
                .or_insert(pow);
        }

        WorryLevel {
            value: value.into_iter().collect(),
            sieve: self.sieve,
        }
    }
}
