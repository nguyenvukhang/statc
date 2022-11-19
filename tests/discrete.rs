#[test]
fn binomial() {
    statc_test!().statc("binom 10 0.2").expect_stdout(
        "
---
X ~ B(10, 0.2)
expected | 2
variance | 1.6
",
    );

    statc_test!().statc("binom 10 0.2 4").expect_stdout(
        "
---
X ~ B(10, 0.2)
expected  | 2
variance  | 1.6
P(X = 4)  | 0.088080384
P(X <= 4) | 0.9672065024
P(X > 4)  | 0.0327934976
",
    );

    statc_test!().statc("binom 10 0.2 4 10").expect_stdout(
        "
---
X ~ B(10, 0.2)
expected       | 2
variance       | 1.6
P(X = 4)       | 0.088080384
P(X = 10)      | 0.0000001024
P(X <= 4)      | 0.9672065024
P(4 < X <= 10) | 0.0327934976
P(X > 10)      | 0
",
    );

    statc_test!().statc("binom 10 0.2 0.4").expect_stderr(
        "
---
error: Invalid value '0.4' for '[WINS]...': Not an integer.

For more information try '--help'
",
    );
}

#[test]
fn negative_binomial() {
    statc_test!().statc("nbinom 4 0.3").expect_stdout(
        "
---
X ~ NB(4, 0.3)
expected | 13.3333333333
variance | 31.1111111111
",
    );

    statc_test!().statc("nbinom 4 0.3 3").expect_stdout(
        "
---
X ~ NB(4, 0.3)
expected  | 13.3333333333
variance  | 31.1111111111
P(X = 3)  | 0
P(X <= 3) | 0
P(X > 3)  | 1
",
    );

    statc_test!().statc("nbinom 4 0.3 6 8").expect_stdout(
        "
---
X ~ NB(4, 0.3)
expected      | 13.3333333333
variance      | 31.1111111111
P(X = 6)      | 0.03969
P(X = 8)      | 0.06806835
P(X <= 6)     | 0.07047
P(6 < X <= 8) | 0.12363435
P(X > 8)      | 0.80589565
",
    );
}

#[test]
fn geometric() {
    statc_test!().statc("geom 0.2").expect_stdout(
        "
---
X ~ G(0.2)
expected | 5
variance | 20
",
    );

    statc_test!().statc("geom 0.2 5").expect_stdout(
        "
---
X ~ G(0.2)
expected  | 5
variance  | 20
P(X = 5)  | 0.08192
P(X <= 5) | 0.67232
P(X > 5)  | 0.32768
",
    );

    statc_test!().statc("geom 0.2 5 9").expect_stdout(
        "
---
X ~ G(0.2)
expected      | 5
variance      | 20
P(X = 5)      | 0.08192
P(X = 9)      | 0.033554432
P(X <= 5)     | 0.67232
P(5 < X <= 9) | 0.193462272
P(X > 9)      | 0.134217728
",
    );
}

#[test]
fn poisson() {
    statc_test!().statc("pois 4.2").expect_stdout(
        "
---
X ~ Poisson(4.2)
expected | 4.2
variance | 4.2
",
    );

    statc_test!().statc("pois 4.2 3").expect_stdout(
        "
---
X ~ Poisson(4.2)
expected  | 4.2
variance  | 4.2
P(X = 3)  | 0.1851653826
P(X <= 3) | 0.3954033696
P(X > 3)  | 0.6045966304
",
    );

    statc_test!().statc("pois 4.2 3 5").expect_stdout(
        "
---
X ~ Poisson(4.2)
expected      | 4.2
variance      | 4.2
P(X = 3)      | 0.1851653826
P(X = 5)      | 0.1633158674
P(X <= 3)     | 0.3954033696
P(3 < X <= 5) | 0.3577395191
P(X > 5)      | 0.2468571113
",
    );
}
