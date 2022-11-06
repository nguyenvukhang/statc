#!/usr/bin/env Rscript

writeLines("X ~ B(10, 0.2)")
writeLines("P(X <= 4)")
pbinom(4, 10, 0.2, lower.tail = TRUE)
writeLines("P(X > 4)")
pbinom(4, 10, 0.2, lower.tail = FALSE)
writeLines("P(X = 4)")
dbinom(4, 10, 0.2)
