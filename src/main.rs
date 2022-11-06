mod descriptions;
mod distributions;
mod prob;
mod r;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
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
    /// X ~ B(n, p)
    Binom {
        #[arg(value_enum)]
        mode: Mode,
        #[arg(value_name = "TRIALS")]
        n: u64,
        #[arg(value_name = "WIN_RATE")]
        p: f64,
        #[arg(value_name = "WINS")]
        x: u64,
    },
    /// X ~ NB(k, p)
    Nbinom {
        #[arg(value_enum)]
        mode: Mode,
        #[arg(value_name = "WINS")]
        k: u64,
        #[arg(value_name = "WIN_RATE")]
        p: f64,
        #[arg(value_name = "TRIALS")]
        x: u64,
    },
}
fn send(v: impl std::fmt::Display) {
    println!("{}", v);
}

fn run(cli: Cli) -> Result<()> {
    let vprint = |v: &str| (!cli.quiet).then(|| println!("{}", v));
    match cli.command {
        Commands::Binom { mode, n, p, x } => {
            let binom = distributions::Binomial::new(n, p)?;
            let result = binom.run(mode, x)?;
            send(result);
            vprint(&descriptions::binomial(mode, n, p, x));
        }
        Commands::Nbinom { mode, k, p, x } => {
            let nbinom = distributions::NegativeBinomial::new(k, p)?;
            let result = nbinom.run(mode, x)?;
            send(result);
            vprint(&descriptions::negative_binomial(mode, k, p, x));
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}
