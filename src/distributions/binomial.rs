use crate::distributions::{DiscreteCdf, DiscretePdf, DistributionMeta};
use crate::math::{choose, MathOps};
use crate::prob::{Meta, PWrap, P};
use crate::utils::Result;

#[derive(PartialEq, Debug)]
pub struct Binomial {
    trials: u64,
    win_rate: P,
}

impl Binomial {
    pub fn new(trials: u64, win_rate: P) -> Result<Self> {
        Ok(Self { win_rate: P::new(win_rate)?, trials })
    }
}

impl DiscretePdf for Binomial {
    /// Returns the probability of winning `wins` times, when trying a
    /// total of `trials` times, with a win-rate of `win_rate`.
    fn pdf(&self, wins: u64) -> P {
        if wins > self.trials {
            return 0.0;
        }
        let mut r = 1.0;
        r *= self.win_rate.pow(wins);
        r *= (1.0 - self.win_rate).pow(self.trials - wins);
        r *= choose(self.trials, wins) as P;
        r
    }
}

impl DiscreteCdf for Binomial {
    /// Binomial Cumulative Distribution Function
    fn cdf(&self, wins: u64) -> P {
        (0..wins + 1).fold(0.0, |acc, x| acc + &self.pdf(x))
    }
}

impl DistributionMeta for Binomial {
    fn meta(&self) -> Meta {
        let expected = self.trials as f64 * self.win_rate;
        Meta { expected, variance: expected * (1.0 - self.win_rate) }
    }
}

#[test]
fn binomial_pdf_test() -> Result<()> {
    assert_eq!(Binomial::new(5, 0.2)?.pdf(2), 0.20480000000000023);
    assert_eq!(Binomial::new(7, 0.3)?.pdf(4), 0.09724049999999994);
    assert_eq!(Binomial::new(7, 1.0)?.pdf(3), 0.0);
    assert_eq!(Binomial::new(7, 0.0)?.pdf(9), 0.0);
    Ok(())
}

#[test]
fn binomial_cdf_test() -> Result<()> {
    assert_eq!(Binomial::new(7, 0.0)?.cdf(3), 1.0);
    assert_eq!(Binomial::new(7, 0.25)?.cdf(3), 0.929443359375);
    assert_eq!(Binomial::new(21, 0.73)?.cdf(8), 0.0008336333232230763);
    Ok(())
}
