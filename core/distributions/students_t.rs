use crate::distributions::{build, Continuous, Distribution, StudentsT};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous as Y, ContinuousCDF};
use statrs::statistics::Distribution as X;

impl StudentsT {
    pub fn new(freedom: u64) -> Result<StudentsT> {
        let core = build(SR::StudentsT::new(0.0, 1.0, freedom as f64))?;
        Ok(StudentsT { freedom, core })
    }
}

impl Distribution for StudentsT {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ t({f})", f = self.freedom)
    }
}

impl Continuous for StudentsT {
    fn pdf(&self, x: f64) -> f64 {
        self.core.pdf(x)
    }
    fn cdf(&self, x: f64) -> f64 {
        self.core.cdf(x)
    }
    fn inv_cdf(&self, x: f64) -> f64 {
        self.core.inverse_cdf(x)
    }
}

#[test]
fn test() -> Result<()> {
    // if X ~ t(f), then
    // * E(X) = 0
    // * var(X) = n/(n−2) for n > 2

    macro_rules! ttest {
        ($f:expr, $mean:expr, $variance:expr) => {
            let dist = StudentsT::new($f)?;
            assert_eq!(dist.mean(), $mean);
            assert_eq!(dist.variance(), $variance);
        };
    }

    assert_eq!(StudentsT::new(0).is_err(), true);
    ttest!(1, None, None);
    ttest!(2, Some(0.0), None);
    ttest!(3, Some(0.0), Some(3.0));
    ttest!(7, Some(0.0), Some(1.4));

    let t = |v| StudentsT::new(v).unwrap();
    float_eq!(t(8).pdf(8.0), 0.0000196463);
    float_eq!(t(9).cdf(1.2), 0.8696134013);

    Ok(())
}
