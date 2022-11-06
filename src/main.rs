mod descriptions;
mod distributions;
mod math;
mod prob;
mod utils;

use clap::{Parser, Subcommand};
use distributions::*;
use prob::P;
use utils::Result;

// refer to
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
// for help on subcommands

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Binom {
        // X ~ B(n, p)
        #[arg(short = 'n', long)]
        trials: u64,
        #[arg(short = 'p', long = "win-rate")]
        win_rate: P,
        #[arg(short = 'x', long = "wins")]
        wins: u64,
    },
    Nbinom {
        // X ~ NB(k, p)
        #[arg(short = 'k', long = "wins")]
        wins: u64,
        #[arg(short = 'p', long = "win-rate")]
        win_rate: P,
        #[arg(short = 'x', long)]
        trials: u64,
    },
}

fn run(command: Commands, verbose: bool) -> Result<()> {
    use Commands::*;
    let vprint = |v: &str| verbose.then(|| println!("{}", v));
    match command {
        Binom { trials, win_rate, wins } => {
            let binomial = Binomial::new(trials, win_rate)?;
            descriptions::title("binomial pdf");
            descriptions::binomial(
                trials,
                win_rate,
                wins,
                binomial.pdf(wins),
                binomial.cdf(wins),
            );
            descriptions::meta(binomial.meta());
            vprint(descriptions::BINOMIAL);
        }
        Nbinom { wins, win_rate, trials } => {
            let negative_binomial = NegativeBinomial::new(wins, win_rate)?;
            descriptions::title("negative binomial pdf");
            descriptions::negative_binomial(
                wins,
                win_rate,
                trials,
                negative_binomial.pdf(wins),
            );
            descriptions::meta(negative_binomial.meta());
            vprint(descriptions::NEGATIVE_BINOMIAL);
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    cli.command.map(|v| run(v, cli.verbose).map_err(|v| println!("{}", v)));
}
