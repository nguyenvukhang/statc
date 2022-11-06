mod display;
mod distributions;
mod math;
mod prob;
mod r;
mod types;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use types::Distribution;
use utils::Result;

// refer to
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
// for help on subcommands

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
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
}

fn send(v: impl std::fmt::Display) {
    println!("{}", v);
}

fn run(cli: Cli) -> Result<()> {
    use distributions::{self as dist};
    match cli.command {
        Commands::Binom { n, p, x } => {
            let dist = dist::Binomial::new(n, p, x)?;
            send(dist.analyze().round());
        }
        Commands::Nbinom { k, p, x } => {
            let dist = dist::NegativeBinomial::new(k, p, x)?;
            send(dist.analyze().round());
        }
        Commands::Geom { p, x } => {
            let dist = dist::Geometric::new(p, x)?;
            send(dist.analyze().round());
        }
        Commands::Pois { l, x } => {
            let dist = dist::Poisson::new(l, x)?;
            send(dist.analyze().round());
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}
