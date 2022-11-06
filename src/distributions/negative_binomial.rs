use crate::distributions::{DiscretePdf, DistributionMeta};
use crate::math::{choose, MathOps};
use crate::prob::{Meta, PWrap, P};
use crate::utils::Result;

#[derive(PartialEq, Debug)]
pub struct NegativeBinomial {
    wins: u64,
    win_rate: P,
}

impl NegativeBinomial {
    pub fn new(wins: u64, win_rate: P) -> Result<Self> {
        Ok(Self { win_rate: P::new(win_rate)?, wins })
    }
}

impl DiscretePdf for NegativeBinomial {
    fn pdf(&self, trials: u64) -> P {
        if self.wins > trials {
            return 0.0;
        }
        let mut r = 1.0;
        r *= self.win_rate.pow(self.wins);
        r *= (1.0 - self.win_rate).pow(trials - self.wins);
        r *= choose(trials - 1, self.wins - 1) as P;
        r
    }
}

impl DistributionMeta for NegativeBinomial {
    fn meta(&self) -> Meta {
        let expected = self.wins as f64 / self.win_rate;
        let (win_rate, loss_rate) = (self.win_rate, 1.0 - self.win_rate);
        Meta { expected, variance: expected * loss_rate / win_rate }
    }
}

#[test]
fn negative_binomial_pdf_test() -> Result<()> {
    assert_eq!(NegativeBinomial::new(2, 0.23)?.pdf(4), 0.0940932299999999);
    assert_eq!(NegativeBinomial::new(3, 0.69)?.pdf(9), 0.008163482508765649);
    assert_eq!(NegativeBinomial::new(1, 0.19)?.pdf(5), 0.08178876990000002);
    assert_eq!(NegativeBinomial::new(2, 1.0)?.pdf(5), 0.0);
    assert_eq!(NegativeBinomial::new(7, 0.0)?.pdf(3), 0.0);
    Ok(())
}
