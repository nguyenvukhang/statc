use crate::display::Line;
use std::fmt::Display;

/// Takes a list of points supplied by the user
/// and maps them to a list of p.d.f.-evaluted values
pub fn pdf_points<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    pdf: F,
    discrete: bool,
) -> Vec<Line> {
    let msg = |v: &T| match discrete {
        true => format!("P(X = {})", v),
        false => format!("pdf @ {}", v),
    };
    list.iter().map(|v| Line::new(&msg(v), Some(pdf(*v)))).collect()
}

/// Takes a list of n points supplied by the user
/// and maps them to n + 1 intervals calculated by c.d.f.
/// * starts from P(X <= first element)
/// * ends at P(X > last element)
pub fn cdf_intervals<T: Display + Copy, F: Fn(T) -> f64>(
    list: &Vec<T>,
    cdf: F,
) -> Vec<Line> {
    let mut result = vec![];

    // return empty list for an empty list
    if list.is_empty() {
        return result;
    };

    // first element: calculate P(X <= x)
    if let Some(x) = list.first() {
        result.push(Line::new(&format!("P(X <= {x})"), Some(cdf(*x))))
    }

    let mut iter = list.iter().peekable();
    // let mut send = |d: &str, v: f64| result.push(Line::new(&d, Some(v)));

    while let Some(lo) = iter.next() {
        let (desc, val) = match iter.peek() {
            // middle element: calculate P(left < X <= right)
            Some(hi) => (format!("P({lo} < X <= {hi})"), cdf(**hi) - cdf(*lo)),
            // last element: calculate P(X > x)
            None => (format!("P(X > {lo})"), 1.0 - cdf(*lo)),
        };
        result.push(Line::new(&desc, Some(val)));
    }
    return result;
}
