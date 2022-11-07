mod display;
mod distributions;
mod math;
mod r;
mod secret;
mod types;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use types::{PEval, PEvalList, Summary};
use utils::Result;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Area {
    Left,
    Center,
    Right,
}

fn about(sym: &str, desc: &str) -> String {
    let s = (0..16 - sym.chars().count()).fold(String::new(), |a, _| a + " ");
    format!("{sym}{s}{desc}")
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = about("X ~ B(n, p)", "P(win x times in n tries)"))]
    Binom {
        #[arg(value_name = "TRIALS")]
        n: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::is_probability)]
        p: f64,
        #[arg(value_name = "WINS")]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ NB(k, p)", "P(win kth time on the xth try)"))]
    Nbinom {
        #[arg(value_name = "WINS")]
        k: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::is_probability)]
        p: f64,
        #[arg(value_name = "TRIALS")]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ G(p)", "P(win once on the x+1th try)"))]
    Geom {
        #[arg(value_name = "WIN_RATE", value_parser = utils::is_probability)]
        p: f64,
        #[arg(value_name = "TRIALS")]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ Poisson(l)", "P(get x hits in interval)"))]
    Pois {
        #[arg(value_name = "EXPECTED")]
        l: f64,
        #[arg(value_name = "HITS")]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ U(a, b)", "Uniform distribution"))]
    Unif {
        #[arg(value_name = "MIN")]
        a: f64,
        #[arg(value_name = "MAX")]
        b: f64,
        #[arg(value_name = "KEY_POINTS")]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ Exp(l)", "Exponential distribution"))]
    Exp {
        #[arg(value_name = "RATE")]
        l: f64,
        #[arg(value_name = "KEY_POINTS")]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ N(m, s²)", "Normal distribution"))]
    Norm {
        #[arg(value_name = "MEAN")]
        m: f64,
        #[arg(value_name = "STD_DEV")]
        s: f64,
        #[arg(value_name = "KEY_POINTS")]
        x: Vec<f64>,
    },
    /// Reverse-engineer the Normal distribution
    Inorm {
        #[arg(value_name = "MEAN")]
        m: f64,
        #[arg(value_name = "STD_DEV")]
        s: f64,
        #[arg(value_name = "AREA", value_enum)]
        a: Area,
        #[arg(value_name = "PROBABILITY", value_parser = utils::is_probability)]
        x: f64,
    },
    #[command(hide = true)]
    Secret,
}

fn send(v: impl std::fmt::Display) {
    println!("{}", v);
}

fn run(cli: Cli) -> Result<()> {
    use distributions::{self as dist};
    match cli.command {
        Commands::Binom { n, p, x } => {
            let dist = dist::Binomial::new(n, p)?;
            send(dist.analyze(&x).round());
        }
        Commands::Nbinom { k, p, x } => {
            let dist = dist::NegativeBinomial::new(k, p)?;
            send(dist.analyze(&x).round());
        }
        Commands::Geom { p, x } => {
            let dist = dist::Geometric::new(p)?;
            send(dist.analyze(&x).round());
        }
        Commands::Pois { l, x } => {
            let dist = dist::Poisson::new(l)?;
            send(dist.analyze(&x).round());
        }
        Commands::Unif { a, b, x } => {
            let dist = dist::Uniform::new(a, b)?;
            send(dist.analyze(&x).round());
        }
        Commands::Exp { l, x } => {
            let dist = dist::Exponential::new(l)?;
            send(dist.analyze(&x).round());
        }
        Commands::Norm { m, s, x } => {
            let dist = dist::Normal::new(m, s)?;
            send(dist.analyze(&x).round());
        }
        Commands::Inorm { m, s, x, a } => {
            let dist = dist::Normal::new(m, s)?;
            use statrs::distribution::ContinuousCDF;
            send(dist.header());
            match a {
                Area::Left => {
                    let res = dist.inverse_cdf(x);
                    send(format!("P(X < {res}) = {x}"));
                }
                Area::Right => {
                    let res = -dist.inverse_cdf(x);
                    send(format!("P(X > {res}) = {x}"));
                }
                Area::Center => {
                    let mut plist = PEvalList::new();
                    let d = x / 2.0;
                    let lo = dist.inverse_cdf(0.5 - d);
                    let hi = dist.inverse_cdf(0.5 + d);
                    plist.push(PEval::new("a: left bound", lo));
                    plist.push(PEval::new("b: right bound", hi));
                    plist.push(PEval::new(&format!("P(a < X <= b)"), x));
                    send(plist);
                }
            }
        }
        _ => {
            println!("{}", secret::SECRET.trim());
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}
