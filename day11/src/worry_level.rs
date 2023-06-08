use num_traits::Pow;
use primal::Sieve;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Mul;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct WorryLevel {
    value: HashMap<usize, usize>,
}

impl WorryLevel {
    pub(crate) fn divisible_by(&self, other: &usize) -> bool {
        self.value.get(other).map_or(false, |rem| *rem == 0)
    }

    pub(crate) fn pow(&self, exp: usize) -> Self {
        Self {
            value: self
                .value
                .iter()
                .map(|(&div, &rem)| (div, rem.pow(exp as u32) % div))
                .collect(),
        }
    }
}

impl From<(usize, &Vec<usize>)> for WorryLevel {
    fn from((val, divs): (usize, &Vec<usize>)) -> Self {
        Self {
            value: divs.iter().map(|&div| (div, val % div)).collect(),
        }
    }
}

impl std::ops::Add<usize> for WorryLevel {
    type Output = WorryLevel;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            value: self
                .value
                .iter()
                .map(|(&div, &rem)| (div, (rem + rhs) % div))
                .collect(),
        }
    }
}

impl Mul<usize> for WorryLevel {
    type Output = WorryLevel;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            value: self
                .value
                .iter()
                .map(|(&div, &rem)| (div, (rem * rhs) % div))
                .collect(),
        }
    }
}
