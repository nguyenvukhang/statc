/// Statistics calculator for entry-level university statistics modules
/// Distributions supported:
///
/// [discrete]
///   - binomial
///   - negative binomial
///   - geometric
///   - poisson
///
/// [continuous]
///   - uniform
///   - exponential
///   - normal
///   - t (Student's T)
///   - chi-squared
///   - f (Fisher-Snedecor)
///
/// Other operations supported:
///
/// [inversions]
///   - invert normal
///   - invert t
///   - invert chi-squared
///
/// [data crunching]
///   - calculate pooled sample variance from sample sizes and variances
///   - read a file of numbers
///   - read a file of value-probability pairs
///   - read a file of number pairs and analyze difference
///   - read two files of numbers and compare the samples
///   - evaluate a math expression

#[macro_use]
mod macros;
mod analyze;
mod data_set;
mod display;
mod distributions;
mod help;
mod inverse;
mod math;
mod printer;
mod secret;
mod utils;

use analyze::Analyze;
use clap::{Parser, Subcommand, ValueEnum};
use display::LineList;
use inverse::Invert;
use utils::Result;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
pub enum Area {
    Left,
    Mid,
    Right,
}

#[derive(Subcommand)]
enum Commands {
    /// X ~ B(n, p)     P(win x times in n tries)
    Binom {
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "WINS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },

    /// X ~ NB(k, p)    P(win kth time on the xth try)
    Nbinom {
        #[arg(value_name = "WINS", value_parser = utils::eval_u64)]
        k: u64,
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },

    /// X ~ G(p)        P(win once on the xth try)
    Geom {
        #[arg(value_name = "WIN_RATE", value_parser = utils::eval_prob)]
        p: f64,
        #[arg(value_name = "TRIALS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },

    /// X ~ Poisson(l)  P(get x hits in interval)
    Pois {
        #[arg(value_name = "EXPECTED", value_parser = utils::eval_f64)]
        l: f64,
        #[arg(value_name = "HITS", value_parser = utils::eval_u64)]
        x: Vec<u64>,
    },

    /// X ~ U(a, b)     Uniform distribution
    Unif {
        #[arg(value_name = "MIN", value_parser = utils::eval_f64)]
        a: f64,
        #[arg(value_name = "MAX", value_parser = utils::eval_f64)]
        b: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// X ~ Exp(l)      Exponential distribution
    Exp {
        #[arg(value_name = "RATE", value_parser = utils::eval_f64)]
        l: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// X ~ N(m, s²)    Normal distribution
    Norm {
        #[arg(value_name = "MEAN", value_parser = utils::eval_f64)]
        m: f64,
        #[arg(value_name = "STD_DEV", value_parser = utils::eval_f64)]
        s: f64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// X ~ t(n)        Student's t-distribution
    T {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        f: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// X ~ χ²(n)       Chi-squared distribution
    Chisq {
        /// degrees of freedom
        #[arg(value_name = "FREEDOM", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// X ~ F(m, n)     Fisher-Snedecor distribution
    F {
        #[arg(value_name = "FREEDOM_1", value_parser = utils::eval_u64)]
        m: u64,
        #[arg(value_name = "FREEDOM_2", value_parser = utils::eval_u64)]
        n: u64,
        #[arg(value_name = "KEY_POINTS", value_parser = utils::eval_f64)]
        x: Vec<f64>,
    },

    /// Reverse-engineer the Normal distribution
    Inorm {
        #[arg(value_name = "MEAN", value_parser = utils::eval_f64)]
        m: f64,
        #[arg(value_name = "STD_DEV", value_parser = utils::eval_f64)]
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

    /// Calculate pooled sample variance
    Vpool {
        #[arg(value_name = "SIZE_1", value_parser = utils::eval_u64)]
        n1: u64,
        #[arg(value_name = "VARIANCE_1", value_parser = utils::eval_f64)]
        v1: f64,
        #[arg(value_name = "SIZE_2", value_parser = utils::eval_u64)]
        n2: u64,
        #[arg(value_name = "VARIANCE_2", value_parser = utils::eval_f64)]
        v2: f64,
    },

    /// Summarize data from a file
    #[command(long_about = help::DATA)]
    Data {
        #[arg(value_name = "FILENAME")]
        file: String,
    },

    /// Compare difference of two samples
    Diff {
        #[arg(value_name = "FILE")]
        file: String,
    },

    /// Compare two data samples
    Comp {
        #[arg(value_name = "FILE_1")]
        f1: String,
        #[arg(value_name = "FILE_2")]
        f2: String,
    },

    /// Evaluate an expression
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

fn process<T>(data: impl Analyze<T>, x: &Vec<T>) {
    let mut analysis = data.analyze(x);
    analysis.round();
    send(analysis);
}

fn run(cli: Cli) -> Result<()> {
    use distributions::*;
    use Area::*;
    match cli.command {
        Commands::Binom { n, p, x } => process(Binomial::new(n, p)?, &x),
        Commands::Nbinom { k, p, x } => {
            process(NegativeBinomial::new(k, p)?, &x)
        }
        Commands::Geom { p, x } => process(Geometric::new(p)?, &x),
        Commands::Pois { l, x } => process(Poisson::new(l)?, &x),
        Commands::Unif { a, b, x } => process(Uniform::new(a, b)?, &x),
        Commands::Exp { l, x } => process(Exponential::new(l)?, &x),
        Commands::Norm { m, s, x } => process(Normal::new(m, s)?, &x),
        Commands::T { f, x } => process(StudentsT::new(f)?, &x),
        Commands::Chisq { n, x } => process(ChiSquared::new(n)?, &x),
        Commands::F { m, n, x } => process(FisherSnedecor::new(m, n)?, &x),
        Commands::Ichisq { n, p } => send(ChiSquared::new(n)?.invert(Right, p)),
        Commands::Inorm { a, m, s, p } => send(Normal::new(m, s)?.invert(a, p)),
        Commands::It { a, f, p } => send(StudentsT::new(f)?.invert(a, p)),
        Commands::Vpool { v1, v2, n1, n2 } => {
            send(math::pooled_variance(n1 as f64, v1, n2 as f64, v2))
        }
        Commands::Data { file } => {
            send(data_set::analyze(&file, data_set::Parser::Single)?.export())
        }
        Commands::Comp { f1, f2 } => {
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
        Commands::Diff { file } => {
            send(data_set::analyze(&file, data_set::Parser::PairDiff)?.export())
        }
        Commands::Eval { expr } => match utils::eval_f64(&expr.join(" ")) {
            Ok(v) => send(v),
            Err(_) => send("Invalid expression."),
        },
        _ => println!("{}", secret::rot13(secret::SECRET.trim())),
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    run(cli).map_err(|v| println!("{}", v)).ok();
}

#[test]
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
        .filter(|v| !v.eq("secret"))
        .map(|v| (format!("integration::{}_test: test", v), v))
        .collect::<HashMap<_, _>>();

    // list of existing tests
    let mut cargo = Command::new("cargo");
    cargo.args(["test", "--", "--list", "--format=terse"]);
    let mut cargo = cargo.stdout(Stdio::piped()).spawn().serr("bopes")?;
    BufReader::new(cargo.stdout.as_mut().ok_or("bopes")?)
        .lines()
        .filter_map(|v| v.ok())
        .for_each(|t| {
            subcommands.remove(&t);
        });

    if !subcommands.is_empty() {
        panic!(
            "\n\nNot all commands are tested.\n\nUntested subcommands:\n{:?}
\n({} untested)\n\n",
            subcommands.values(),
            subcommands.len()
        );
    }
    cargo.wait().ok();
    Ok(())
}
