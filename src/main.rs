mod display;
mod distributions;
mod math;
mod r;
mod types;
mod utils;

use clap::{Parser, Subcommand};
use types::Summary;
use utils::Result;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// X ~ B(n, p)     P(win x times in n tries)
    Binom {
        #[arg(value_name = "TRIALS")]
        n: u64,
        #[arg(value_name = "WIN_RATE")]
        p: f64,
        #[arg(value_name = "WINS")]
        x: Option<u64>,
    },
    /// X ~ NB(k, p)    P(win kth time on the xth try)
    Nbinom {
        #[arg(value_name = "WINS")]
        k: u64,
        #[arg(value_name = "WIN_RATE")]
        p: f64,
        #[arg(value_name = "TRIALS")]
        x: Option<u64>,
    },
    /// X ~ G(p)        P(win once on the x+1th try)
    Geom {
        #[arg(value_name = "WIN_RATE")]
        p: f64,
        #[arg(value_name = "TRIALS")]
        x: Option<u64>,
    },
    /// X ~ Poisson(l)  P(get x hits in interval)
    Pois {
        #[arg(value_name = "EXPECTED")]
        l: f64,
        #[arg(value_name = "HITS")]
        x: Option<u64>,
    },
    /// X ~ U(a, b)     Uniform distribution
    Unif {
        #[arg(value_name = "MIN")]
        a: f64,
        #[arg(value_name = "MAX")]
        b: f64,
        #[arg(value_name = "LOWER_BOUND")]
        lb: Option<f64>,
        #[arg(value_name = "UPPER_BOUND")]
        ub: Option<f64>,
    },
    /// X ~ Exp(l)      Exponential distribution
    Exp {
        #[arg(value_name = "RATE")]
        l: f64,
        #[arg(value_name = "LOWER_BOUND")]
        lb: Option<f64>,
        #[arg(value_name = "UPPER_BOUND")]
        ub: Option<f64>,
    },
}

fn send(v: impl std::fmt::Display) {
    println!("{}", v);
}

fn run(cli: Cli) -> Result<()> {
    use distributions::{self as dist};
    match cli.command {
        Commands::Binom { n, p, x } => {
            let dist = dist::Binomial::new(n, p)?;
            send(dist.analyze(x, None).round());
        }
        Commands::Nbinom { k, p, x } => {
            let dist = dist::NegativeBinomial::new(k, p)?;
            send(dist.analyze(x, None).round());
        }
        Commands::Geom { p, x } => {
            let dist = dist::Geometric::new(p)?;
            send(dist.analyze(x, None).round());
        }
        Commands::Pois { l, x } => {
            let dist = dist::Poisson::new(l)?;
            send(dist.analyze(x, None).round());
        }
        Commands::Unif { a, b, lb, ub } => {
            let dist = dist::Uniform::new(a, b)?;
            send(dist.analyze(lb, ub).round());
        }
        Commands::Exp { l, lb, ub } => {
            let dist = dist::Exponential::new(l)?;
            send(dist.analyze(lb, ub).round());
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}
