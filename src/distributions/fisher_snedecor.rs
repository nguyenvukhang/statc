use crate::distributions::{build, FisherSnedecor, MyContinuous, MyDist};
use crate::utils::Result;
use statrs::distribution as SR;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

impl FisherSnedecor {
    pub fn new(f1: u64, f2: u64) -> Result<FisherSnedecor> {
        let core = build(SR::FisherSnedecor::new(f1 as f64, f2 as f64))?;
        Ok(FisherSnedecor { f1, f2, core })
    }
}

impl MyDist for FisherSnedecor {
    fn mean(&self) -> Option<f64> {
        self.core.mean()
    }
    fn variance(&self) -> Option<f64> {
        self.core.variance()
    }
    fn title(&self) -> String {
        format!("X ~ F({f1}, {f2})", f1 = self.f1, f2 = self.f2)
    }
}

impl MyContinuous for FisherSnedecor {
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
    // if X ~ F(m, n), then
    // * E(X) = n/n-2 for n > 2
    // * var(X) = {2n²(m+n-2)}/{m(n-2)²(n-4)}

    macro_rules! ftest {
        ($f1:expr, $f2:expr, $mean:expr, $variance:expr) => {
            let dist = FisherSnedecor::new($f1, $f2)?;
            assert_eq!(dist.mean(), $mean);
            assert_eq!(dist.variance(), $variance);
        };
    }

    assert_eq!(FisherSnedecor::new(0, 0).is_err(), true);
    assert_eq!(FisherSnedecor::new(0, 1).is_err(), true);
    assert_eq!(FisherSnedecor::new(1, 0).is_err(), true);
    assert_eq!(FisherSnedecor::new(1, 1).is_err(), false);

    ftest!(1, 1, None, None);
    ftest!(1, 2, None, None);
    ftest!(2, 1, None, None);
    ftest!(3, 1, None, None);
    ftest!(3, 2, None, None);
    ftest!(3, 3, Some(3.0), None);
    ftest!(4, 1, None, None);
    ftest!(4, 2, None, None);
    ftest!(4, 3, Some(3.0), None);
    ftest!(4, 4, Some(2.0), None);
    ftest!(5, 1, None, None);
    ftest!(5, 2, None, None);
    ftest!(5, 3, Some(3.0), None);
    ftest!(5, 4, Some(2.0), None);
    ftest!(5, 5, Some(1.6666666666666667), Some(8.88888888888889));

    let f = |f1, f2| FisherSnedecor::new(f1, f2).unwrap();
    float_eq!(f(4, 2).pdf(5.0), 0.0300525920);
    float_eq!(f(6, 9).cdf(4.0), 0.9687440402);

    Ok(())
}
