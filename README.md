# statc

A statistics calculator for that one open-book university module.

[![crates.io][badge]][crates_link]

## Install

statc can be installed by running `cargo install statc`.

Alternatively, you can download a copy of the [latest][latest] pre-built binary.

## Usage

Run `statc` to bring up the help message.

```
Usage: statc [OPTIONS] <COMMAND>

Commands:
  binom   X ~ B(n, p)     P(win x times in n tries)
  nbinom  X ~ NB(k, p)    P(win kth time on the xth try)
  geom    X ~ G(p)        P(win once on the xth try)
  pois    X ~ Poisson(l)  P(get x hits in interval)
  unif    X ~ U(a, b)     Uniform distribution
  exp     X ~ Exp(l)      Exponential distribution
  norm    X ~ N(m, s²)    Normal distribution
  t       X ~ t(n)        Student's t-distribution
  chisq   X ~ χ²(n)       Chi-squared distribution
  f       X ~ F(m, n)     Fisher-Snedecor distribution
  inorm   Reverse-engineer the Normal distribution
  it      Reverse-engineer the Student's t-distribution
  ichisq  Reverse-engineer the Chi-squared distribution
  vpool   Calculate pooled sample variance
  data    Summarize data from a file
  diff    Compare difference of two samples
  comp    Compare two data samples
  eval    Evaluate an expression
  help    Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet
  -h, --help   Print help information
```

Run `statc <subcommand>` such as `statc binom` to show more help on
the `binom` subcommand for operations with the Binomial Distribution.

Use the `--help` flag to see more info on any help page.

[latest]: https://github.com/nguyenvukhang/statc/releases/latest
[crates_link]: https://crates.io/crates/statc
[badge]: https://img.shields.io/crates/dr/statc?color=brightgreen
