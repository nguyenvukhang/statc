#[macro_use]
mod macros;
mod analyze;
mod data_set;
mod display;
mod distributions;
mod inverse;
mod math;
mod printer;
mod secret;
mod summary;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use display::{Analysis, LineList};
use inverse::Invert;
use summary::Summary;
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
pub enum Area {
    Left,
    Mid,
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
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "WINS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ NB(k, p)", "P(win kth time on the xth try)"))]
    Nbinom {
        #[arg(value_name = "WINS", value_parser = utils::eval_u64)]
        k: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ G(p)", "P(win once on the x+1th try)"))]
    Geom {
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ Poisson(l)", "P(get x hits in interval)"))]
    Pois {
        #[arg(value_name = "EXPECTED", value_parser = utils::eval)]
        l: f64,
        #[arg(value_name = "HITS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },
    #[command(about = about("X ~ U(a, b)", "Uniform distribution"))]
    Unif {
        #[arg(value_name = "MIN", value_parser = utils::eval)]
        a: f64,
        #[arg(value_name = "MAX", value_parser = utils::eval)]
        b: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ Exp(l)", "Exponential distribution"))]
    Exp {
        #[arg(value_name = "RATE", value_parser = utils::eval)]
        l: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ N(m, s²)", "Normal distribution"))]
    Norm {
        #[arg(value_name = "MEAN", value_parser = utils::eval)]
        m: f64,
        #[arg(value_name = "STD_DEV", value_parser = utils::eval)]
        s: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ t(n)", "Student's t-distribution"))]
    T {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        f: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ χ²(n)", "Chi-squared distribution"))]
    Chisq {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    #[command(about = about("X ~ F(m, n)", "Fisher-Snedecor distribution"))]
    F {
        #[arg(value_name = "FREEDOM_1", value_parser = utils::eval_u64)]
        m: u64,
        #[arg(value_name = "FREEDOM_2", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval)]
        x: Vec<f64>,
    },
    /// Reverse-engineer the Normal distribution
    Inorm {
        #[arg(value_name = "MEAN", value_parser = utils::eval)]
        m: f64,
        #[arg(value_name = "STD_DEV", value_parser = utils::eval)]
        s: f64,
        #[arg(value_name = "AREA", value_enum)]
        a: Area,
        #[arg(value_name = "PROBABILITY", value_parser = utils::eval_prob)]
        p: f64,
    },
    /// Reverse-engineer the Student's t-distribution
    It {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        f: u64,
        #[arg(value_name = "AREA", value_enum)]
        a: Area,
        #[arg(value_name = "PROBABILITY", value_parser = utils::eval_prob)]
        p: f64,
    },
    /// Reverse-engineer the Chi-squared distribution
    Ichisq {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "PROBABILITY", value_parser = utils::eval_prob)]
        p: f64,
    },
    #[command(about = "Pooled sample variance")]
    Vpool {
        #[arg(value_name = "SIZE_1", value_parser = utils::eval_u64)]
        n1: u64,
        #[arg(value_name = "VARIANCE_1", value_parser = utils::eval)]
        v1: f64,
        #[arg(value_name = "SIZE_2", value_parser = utils::eval_u64)]
        n2: u64,
        #[arg(value_name = "VARIANCE_2", value_parser = utils::eval)]
        v2: f64,
    },
    #[command(about = "Compare two data samples")]
    Compd {
        #[arg(value_name = "FILE_1")]
        f1: String,
        #[arg(value_name = "FILE_2")]
        f2: String,
    },
    #[command(about = "Compare two samples by summary")]
    Comps {
        #[arg(value_name = "SIZE_1")]
        n1: u64,
        #[arg(value_name = "MEAN_1")]
        m1: f64,
        #[arg(value_name = "STD_DEV_1")]
        s1: f64,
        #[arg(value_name = "SIZE_2")]
        n2: u64,
        #[arg(value_name = "MEAN_2")]
        m2: f64,
        #[arg(value_name = "STD_DEV_2")]
        s2: f64,
    },
    #[command(about = "Compare difference of two samples")]
    Diff {
        #[arg(value_name = "FILE")]
        file: String,
    },
    #[command(about = "Summarize data from a file")]
    Data {
        #[arg(value_name = "FILE")]
        file: String,
    },
    #[command(about = "Evaluate an expression")]
    Eval {
        #[arg(value_name = "EXPR")]
        expr: Vec<String>,
    },
    #[command(hide = true)]
    Secret,
}

fn send(v: impl std::fmt::Display) {
    println!("{}", v);
}

fn process(mut a: Analysis) {
    a.round();
    send(a);
}

fn run(cli: Cli) -> Result<()> {
    use distributions::{self as dist};
    match cli.command {
        Commands::Binom { n, p, x } => {
            let dist = dist::Binomial::new(n, p)?;
            process(dist.analyze(&x));
        }
        Commands::Nbinom { k, p, x } => {
            let dist = dist::NegativeBinomial::new(k, p)?;
            process(dist.analyze(&x));
        }
        Commands::Geom { p, x } => {
            let dist = dist::Geometric::new(p)?;
            process(dist.analyze(&x));
        }
        Commands::Pois { l, x } => {
            let dist = dist::Poisson::new(l)?;
            process(dist.analyze(&x));
        }
        Commands::Unif { a, b, x } => {
            let dist = dist::Uniform::new(a, b)?;
            process(dist.analyze(&x));
        }
        Commands::Exp { l, x } => {
            let dist = dist::Exponential::new(l)?;
            process(dist.analyze(&x));
        }
        Commands::Norm { m, s, x } => {
            let dist = dist::Normal::new(m, s)?;
            process(dist.analyze(&x));
        }
        Commands::T { f, x } => {
            let dist = dist::StudentsT::new(f)?;
            process(dist.analyze(&x));
        }
        Commands::Chisq { n, x } => {
            let dist = dist::ChiSquared::new(n)?;
            process(dist.analyze(&x));
        }
        Commands::F { m, n, x } => {
            let dist = dist::FisherSnedecor::new(m, n)?;
            process(dist.analyze(&x));
        }
        Commands::Ichisq { n, p } => {
            send(dist::ChiSquared::new(n)?.invert(Area::Right, p));
        }
        Commands::Inorm { a, m, s, p } => {
            send(dist::Normal::new(m, s)?.invert(a, p));
        }
        Commands::It { a, f, p } => {
            send(dist::StudentsT::new(f)?.invert(a, p));
        }
        Commands::Vpool { v1, v2, n1, n2 } => {
            send(math::pooled_variance(n1 as f64, v1, n2 as f64, v2))
        }
        Commands::Data { file } => {
            let data = data_set::analyze(&file, data_set::Parser::Single)?;
            send(data.export())
        }
        Commands::Compd { f1, f2 } => {
            let mut d1 = data_set::analyze(&f1, data_set::Parser::Single)?;
            let mut d2 = data_set::analyze(&f2, data_set::Parser::Single)?;
            let mut list = LineList::new();
            list.header(&f1);
            list.append(&d1.export());
            list.header(&f2);
            list.append(&d2.export());
            list.header("pooled sample");
            list.append(&math::pooled_variance(
                d1.n(),
                d1.var_s().unwrap(),
                d2.n(),
                d2.var_s().unwrap(),
            ));
            send(list);
        }
        #[allow(unused_variables)]
        Commands::Comps { n1, m1, s1, n2, m2, s2 } => {
            send("feature unsupported")
        }
        Commands::Diff { file } => {
            let data = data_set::analyze(&file, data_set::Parser::PairDiff)?;
            send(data.export())
        }
        Commands::Eval { expr } => match meval::eval_str(expr.join(" ")) {
            Ok(v) => send(v),
            Err(_) => send("Invalid expression."),
        },
        _ => {
            println!("{}", secret::rot13(secret::SECRET.trim()));
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}

#[test]
#[ignore]
fn subcommand_coverage() -> Result<()> {
    use crate::utils::ResultOps;
    use clap::CommandFactory;
    use std::collections::HashMap;
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};

    // complete list of subcommands
    let mut subcommands = Cli::command()
        .get_subcommands()
        .map(|v| v.get_name().to_string())
        .map(|v| (format!("integration::{}_test: test", v), v))
        .collect::<HashMap<_, _>>();

    // list of existing tests
    let mut cargo = Command::new("cargo");
    cargo.args(["test", "--", "--list", "--format=terse"]);
    let mut cargo = cargo.stdout(Stdio::piped()).spawn().serr("Can't spawn")?;
    let tests = BufReader::new(cargo.stdout.as_mut().ok_or("bopes")?)
        .lines()
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();

    tests.iter().for_each(|t| {
        subcommands.remove(t);
    });
    if !subcommands.is_empty() {
        panic!(
            "

Not all commands are tested.

Untested subcommands:
{:?}

({} untested)

",
            subcommands.values(),
            subcommands.len()
        );
    }
    Ok(())
}
