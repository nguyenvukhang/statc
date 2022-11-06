use crate::Mode;

const WRAP_LEN: u64 = 69;

fn wrap(s: String) -> String {
    let mut count = 0;
    let mut result = String::new();
    for i in s.chars() {
        result.push(i);
        count += 1;
        if count == WRAP_LEN {
            count = 0;
            let split = result.rsplit_once(' ');
            if split.is_none() {
                continue;
            }
            let split = split.unwrap();
            let next = split.1.to_string();
            result = split.0.to_string();
            result.push('\n');
            result.push_str(&next);
        }
    }
    result
}

fn ord_string(n: u64) -> String {
    let mut n = n.to_string();
    if n.ends_with("1") {
        n.pop();
        n.push_str("1st");
    } else if n.ends_with("2") {
        n.pop();
        n.push_str("2nd");
    } else if n.ends_with("3") {
        n.pop();
        n.push_str("3rd");
    } else {
        n.push_str("th");
    }
    n
}

impl Mode {
    fn fmt(self, f: [&str; 5]) -> &str {
        match self {
            Mode::Lt => f[0],
            Mode::Le => f[1],
            Mode::Eq => f[2],
            Mode::Ge => f[3],
            Mode::Gt => f[4],
        }
    }
}

pub fn binomial(mode: Mode, trials: u64, win_rate: f64, wins: u64) -> String {
    let mode =
        mode.fmt(["less than", "at most", "exactly", "at least", "more than"]);
    let res = format!("Probability of winning {mode} {wins} times in {trials} trials given a win-rate of {win_rate}.");
    wrap(res)
}

pub fn negative_binomial(
    mode: Mode,
    wins: u64,
    win_rate: f64,
    trials: u64,
) -> String {
    let wins = ord_string(wins);
    let trials = ord_string(trials);
    let mode = mode.fmt([
        "before the",
        "before and including the",
        "on the",
        "after and including the",
        "after the",
    ]);
    let res = format!("Probability of winning for the {wins} time {mode} {trials} trial given a win-rate of {win_rate}.");
    wrap(res)
}
