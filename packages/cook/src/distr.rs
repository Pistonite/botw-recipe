//! Module for simulating randomness in the cooking process

use std::ops::Range;

use num::{BigUint, Integer, One, ToPrimitive};

/// Distr<T> is a discreate distribution of type T
pub trait Distr: Sized {
    type T;

    /// Convert the distribution to a a vector of (outcome, count). The probability of each outcome is
    /// the count divided by the total count.
    fn into_counts(self) -> Vec<(Self::T, BigUint)>;

    #[inline]
    fn into_discrete(self) -> Discrete<Self::T> {
        Discrete {
            counts: self.into_counts()
        }
    }

    /// Convert the distribution to a probability mass function, represented
    /// as a vector of (outcome, probability). The order of outcome
    /// is not guarateed
    fn into_pmf(self) -> Vec<(Self::T, Prob)> {
        let counts = self.into_counts();
        let total = counts.iter().map(|(_, p)| p).sum::<BigUint>();
        counts.into_iter().map(|(t, p)| (t, Prob::new(&p, &total))).collect()
    }

    /// Operate on each outcome of the random event, and map the returned distribution
    /// as the new distribution of the random event from the initial state
    fn map<T2: PartialEq, D: Distr<T=T2>, F: Fn(Self::T) -> D>(self, f: F) -> Discrete<T2> {
        let mut counts = vec![];
        for (t, p) in self.into_counts() {
            let evt = f(t);
            for (t2, p2) in evt.into_counts() {
                counts.push((t2, &p * p2));
            }
        }
        let mut new_distr = Discrete {
            counts
        };
        new_distr.reduce();
        new_distr
    }
}

/// A distribution of a binary random event that can either be `true` or `false`
pub struct Binary {
    p: Option<Prob>,
}

impl Binary {
    pub fn never() -> Self {
        Binary {
            p: None
        }
    }
    pub fn always() -> Self {
        Binary {
            p: Some(Prob::one())
        }
    }
    pub fn with_prob(p: Prob) -> Self {
        Binary {
            p: Some(p)
        }
    }
    pub fn into_prob(self) -> Option<Prob> {
        self.p
    }
    pub fn is_never(&self) -> bool {
        self.p.is_none()
    }
    pub fn is_always(&self) -> bool {
        match &self.p {
            Some(p) => p.is_one(),
            None => false
        }
    }
}

pub fn coin_flip() -> Binary {
    Binary::with_prob(Prob::half())
}

impl Distr for Binary {
    type T = bool;
    
    fn into_counts(self) -> Vec<(Self::T, BigUint)> {
        match self.p {
            None => vec![(false, BigUint::one())],
            Some(p) => {
                if p.is_one() {
                    vec![(true, BigUint::one())]
                } else {
                    vec![(false, p.denominator - &p.numerator), (true, p.numerator)]
                }
            }
        }
    }
}

#[inline]
pub fn uniform(r: Range<u32>) -> Unif32 {
    Unif32 {
        lo: r.start,
        hi: r.end
    }
}

#[inline]
pub fn uniform_always(r: u32) -> Unif32 {
    Unif32 {
        lo: r,
        hi: r+1
    }
}

/// A distribution of a random event that can take discrete values in a range of u32
pub struct Unif32 {
    /// Lower bound, inclusive
    lo: u32,
    /// Upper bound, exclusive
    hi: u32,
}

impl Unif32 {
    pub fn less_than(self, t: u32) -> Binary {
        if t <= self.lo {
            Binary::never()
        } else if t >= self.hi {
            Binary::always()
        } else {
            Binary::with_prob(Prob::new(&BigUint::from(t - self.lo), &BigUint::from(self.hi - self.lo)))
        }
    }

    pub fn less_than_or_eq(self, t: u32) -> Binary {
        if t < self.lo {
            Binary::never()
        } else if t >= self.hi - 1 { // note when t == hi - 1, the expression below gives 1 as well
            Binary::always()
        } else {
            Binary::with_prob(Prob::new(&BigUint::from(t - self.lo + 1), &BigUint::from(self.hi - self.lo)))
        }
    }
}

impl Distr for Unif32 {
    type T = u32;

    fn into_counts(self) -> Vec<(Self::T, BigUint)> {
        (self.lo..self.hi).map(|t| (t, BigUint::one())).collect()
    }
}

#[inline]
pub fn always<T>(t: T) -> Always<T> {
    Always(t)
}

#[inline]
pub fn discrete_always<T>(t: T) -> Discrete<T> {
    Always(t).into_discrete()
}

pub struct Always<T>(T);
impl<T> Distr for Always<T> {
    type T = T;

    fn into_counts(self) -> Vec<(Self::T, BigUint)> {
        vec![(self.0, BigUint::one())]
    }
}


/// Discrete outcomes
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Discrete<T> {
    counts: Vec<(T, BigUint)>
}

impl<T> Distr for Discrete<T> {
    type T = T;

    fn into_counts(self) -> Vec<(Self::T, BigUint)> {
        self.counts
    }
}

impl<T> Discrete<T> {
    /// Iterate over the outcomes without worrying about the probabilities
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.counts.iter().map(|(t, _)| t)
    }
}

impl<T: PartialEq> Discrete<T> {
    /// Merge outcomes that are equal, adding their probability
    fn reduce(&mut self) {
        let mut i = 0;
        while i < self.counts.len() {
            //let t = &self.counts[i].0;
            let mut j = i + 1;
            while j < self.counts.len() {
                if &self.counts[j].0 == &self.counts[i].0 {
                    let j_p = &self.counts[j].1;
                    let i_p = &self.counts[i].1 + j_p;
                    self.counts[i].1 = i_p;
                    self.counts.swap_remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
}

/// A probability in a discrete event with arbitrary precision.
/// 
/// The value is stored as a ratio. It must be between 0 (exclusive) and 1 (inclusive),
/// and the numerator and denominator must be relatively prime.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prob {
    numerator: BigUint,
    denominator: BigUint
}

impl std::fmt::Display for Prob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl std::fmt::Debug for Prob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Prob {
    pub fn new(numerator: &BigUint, denominator: &BigUint) -> Self {
        let gcd = numerator.gcd(&denominator);
        Self {
            numerator: numerator / &gcd,
            denominator: denominator / &gcd
        }
    }

    pub fn one() -> Self {
        Self {
            numerator: BigUint::one(),
            denominator: BigUint::one()
        }
    }

    pub fn half() -> Self {
        Self {
            numerator: BigUint::one(),
            denominator: BigUint::from(2u32)
        }
    }

    pub fn is_one(&self) -> bool {
        self.numerator == self.denominator
    }

    /// Convert the ratio to between 0 and 1.
    ///
    /// Returns None if the denominator cannot be represented as a f64.
    pub fn to_f64(&self) -> Option<f64> {
        let d = self.denominator.to_f64()?;
        if f64::is_infinite(d) {
            None
        } else {
            Some(self.numerator.to_f64()? / d)
        }
    }

    /// Self::one_minus(other) gives (1 - other)
    pub fn one_minus(other: &Self) -> Self {
        Self::new(&(&other.denominator - &other.numerator), &other.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! r {
        ($n:literal / $d:literal) => {
            Prob::new(&BigUint::new(vec![$n]), &BigUint::new(vec![$d]))
        }
    }

    #[test]
    pub fn uniform_less_than() {
        assert!(uniform(0..5).less_than(0).is_never());
        assert!(uniform(0..5).less_than(5).is_always());
        assert!(uniform(0..5).less_than(6).is_always());
        assert_eq!(uniform(0..5).less_than(3).into_pmf(), vec![
            (false, r!(2/5)),
            (true, r!(3/5))
        ]);
    }

    #[test]
    pub fn uniform_less_than_eq() {
        assert!(uniform(1..6).less_than_or_eq(0).is_never());
        assert_eq!(uniform(1..6).less_than_or_eq(1).into_prob(), Some(r!(1/5)));
        assert!(uniform(1..6).less_than_or_eq(5).is_always());
        assert!(uniform(1..6).less_than_or_eq(6).is_always());
        assert_eq!(uniform(0..5).less_than_or_eq(3).into_pmf(), vec![
            (false, r!(1/5)),
            (true, r!(4/5))
        ]);
    }

    #[test]
    pub fn discrete_reduce() {
        let mut evt = Discrete {
            counts: vec![
                (1, BigUint::one()),
                (2, BigUint::one()),
                (1, BigUint::one()),
                (3, BigUint::one()),
                (2, BigUint::one()),
                (1, BigUint::one()),
            ]
        };
        evt.reduce();
        assert_eq!(evt.counts, vec![
            (1, BigUint::from(3u32)),
            (2, BigUint::from(2u32)),
            (3, BigUint::one())
        ]);
    }

    #[test]
    pub fn test_map() {
        let evt = uniform(0u32..10u32);
        // this is simualting:
        // let e = rand(0..10);
        // match e {
        //    0 => e += 1,
        //    1..3 => e += rand(0..2),
        //    _ => e += rand(0..3)
        // }
        let evt = evt.map(|x| {
            match x {
                0 => always(x + 1).into_discrete(), // just 1
                1..3 => {
                    // 50/50
                    uniform(0..2).map(|y| always(x + y))
                }
                _ => {
                    // 1/3 each
                    uniform(0..3).map(|y| always(x + y))
                }
            }
        });
        assert_eq!(evt.into_pmf(), vec![
            (1, r!(1/13)),
            (11, r!(1/26)),
            (2, r!(1/13)),
            (10, r!(1/13)),
            (3, r!(1/13)),
            (9, r!(3/26)),
            (4, r!(1/13)),
            (5, r!(3/26)),
            (8, r!(3/26)),
            (7, r!(3/26)),
            (6, r!(3/26)),
        ]);
    }

    #[test]
    pub fn test_if() {
        let evt = uniform(0u32..4u32);
        let evt = evt.map(|x| {
            if x % 2 == 0 {
                always(x * 2)
            } else  {
                always(x + 3)
            }
        });
        assert_eq!(evt.into_pmf(), vec![
            (0, r!(1/4)),
            (4, r!(1/2)),
            (6, r!(1/4)),
        ]);
    }
}
