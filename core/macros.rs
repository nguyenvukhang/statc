#[macro_export]
macro_rules! pretty_panic {
    ($rec:expr, $exp:expr) => {
        let (rec, exp) = ($rec, $exp);
        panic!("\nexpected: {exp},\nreceived: {rec}\n");
    };
}

#[macro_export]
macro_rules! float_eq {
    ($rec:expr, $exp:expr) => {
        let (exp, rec) = ($exp as f64, $rec as f64);
        let diff = (exp - rec).abs();
        if diff > 1e-10 {
            pretty_panic!(rec, exp);
        }
    };
    ($rec:expr, $exp:expr, $precision:expr) => {
        let (exp, rec) = ($exp as f64, $rec as f64);
        let diff = (exp - rec).abs();
        let tolerance = 10.0_f64.powi(-$precision);
        if diff > tolerance {
            pretty_panic!(rec, exp);
        }
    };
}
