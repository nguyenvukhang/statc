#[test]
fn binom_test() {
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
fn nbinom_test() {
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
fn geom_test() {
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
fn pois_test() {
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

#[test]
fn unif_test() {
    statc_test!().statc("unif 1 2").expect_stdout(
        "
---
X ~ U(1, 2)
expected | 1.5
variance | 0.0833333333
",
    );

    statc_test!().statc("unif 1 2 1.4").expect_stdout(
        "
---
X ~ U(1, 2)
expected    | 1.5
variance    | 0.0833333333
pdf @ 1.4   | 1
P(X <= 1.4) | 0.4
P(X > 1.4)  | 0.6
",
    );

    statc_test!().statc("unif 1 2 1.4 2.1").expect_stdout(
        "
---
X ~ U(1, 2)
expected          | 1.5
variance          | 0.0833333333
pdf @ 1.4         | 1
pdf @ 2.1         | 0
P(X <= 1.4)       | 0.4
P(1.4 < X <= 2.1) | 0.6
P(X > 2.1)        | 0
",
    );
}

#[test]
fn exp_test() {
    statc_test!().statc("exp 1.6").expect_stdout(
        "
---
X ~ Exp(1.6)
expected | 0.625
variance | 0.390625
",
    );

    statc_test!().statc("exp 1.6 3").expect_stdout(
        "
---
X ~ Exp(1.6)
expected  | 0.625
variance  | 0.390625
pdf @ 3   | 0.0131675953
P(X <= 3) | 0.991770253
P(X > 3)  | 0.008229747
",
    );

    statc_test!().statc("exp 1.6 3 5").expect_stdout(
        "
---
X ~ Exp(1.6)
expected      | 0.625
variance      | 0.390625
pdf @ 3       | 0.0131675953
pdf @ 5       | 0.0005367402
P(X <= 3)     | 0.991770253
P(3 < X <= 5) | 0.0078942844
P(X > 5)      | 0.0003354626
",
    );
}

#[test]
fn norm_test() {
    statc_test!().statc("norm 10 2").expect_stdout(
        "
---
X ~ N(10, 2²)
expected | 10
variance | 4
",
    );

    statc_test!().statc("norm 10 2 3").expect_stdout(
        "
---
X ~ N(10, 2²)
expected  | 10
variance  | 4
pdf @ 3   | 0.0004363413
P(X <= 3) | 0.0002326291
P(X > 3)  | 0.9997673709
",
    );

    statc_test!().statc("norm 10 2 0-1 3").expect_stdout(
        "
---
X ~ N(10, 2²)
expected       | 10
variance       | 4
pdf @ -1       | 0.0000000538
pdf @ 3        | 0.0004363413
P(X <= -1)     | 0.000000019
P(-1 < X <= 3) | 0.0002326101
P(X > 3)       | 0.9997673709
",
    );
}

#[test]
fn t_test() {
    statc_test!().statc("t 6").expect_stdout(
        "
---
X ~ t(6)
expected | 0
variance | 1.5
",
    );

    statc_test!().statc("t 6 0.3").expect_stdout(
        "
---
X ~ t(6)
expected    | 0
variance    | 1.5
pdf @ 0.3   | 0.3632992517
P(X <= 0.3) | 0.6128503895
P(X > 0.3)  | 0.3871496105
",
    );

    statc_test!().statc("t 6 0.3 0.7").expect_stdout(
        "
---
X ~ t(6)
expected          | 0
variance          | 1.5
pdf @ 0.3         | 0.3632992517
pdf @ 0.7         | 0.290782716
P(X <= 0.3)       | 0.6128503895
P(0.3 < X <= 0.7) | 0.13207792
P(X > 0.7)        | 0.2550716905
",
    );
}

#[test]
fn chisq_test() {
    statc_test!().statc("chisq 4").expect_stdout(
        "
---
X ~ χ²(4)
expected | 4
variance | 8
",
    );

    statc_test!().statc("chisq 4 1.5").expect_stdout(
        "
---
X ~ χ²(4)
expected    | 4
variance    | 8
pdf @ 1.5   | 0.1771374573
P(X <= 1.5) | 0.1733585327
P(X > 1.5)  | 0.8266414673
",
    );

    statc_test!().statc("chisq 4 1.5 2.7").expect_stdout(
        "
---
X ~ χ²(4)
expected          | 4
variance          | 8
pdf @ 1.5         | 0.1771374573
pdf @ 2.7         | 0.1749871759
P(X <= 1.5)       | 0.1733585327
P(1.5 < X <= 2.7) | 0.2174268548
P(X > 2.7)        | 0.6092146125
",
    );
}

#[test]
fn f_test() {
    statc_test!().statc("f 4 5").expect_stdout(
        "
---
X ~ F(4, 5)
expected | 1.6666666667
variance | 9.7222222222
",
    );

    statc_test!().statc("f 4 5 1").expect_stdout(
        "
---
X ~ F(4, 5)
expected  | 1.6666666667
variance  | 9.7222222222
pdf @ 1   | 0.3976140792
P(X <= 1) | 0.5143428032
P(X > 1)  | 0.4856571968
",
    );

    statc_test!().statc("f 4 5 1 1.2").expect_stdout(
        "
---
X ~ F(4, 5)
expected        | 1.6666666667
variance        | 9.7222222222
pdf @ 1         | 0.3976140792
pdf @ 1.2       | 0.3252497354
P(X <= 1)       | 0.5143428032
P(1 < X <= 1.2) | 0.0720479499
P(X > 1.2)      | 0.4136092469
",
    );
}

#[test]
fn inorm_test() {
    statc_test!().statc("inorm 0 1 mid 0.95").expect_stdout(
        "
---
X ~ N(0, 1²)
a: left bound  | -1.9599639845
b: right bound | 1.9599639845
P(a < X <= b)  | 0.95
",
    );

    statc_test!().statc("inorm 0 1 left 0.05").expect_stdout(
        "
---
X ~ N(0, 1²)
P(X < -1.644853627) = 0.05
",
    );

    statc_test!().statc("inorm 0 1 right 0.05").expect_stdout(
        "
---
X ~ N(0, 1²)
P(X > 1.644853627) = 0.05
",
    );
}

#[test]
fn it_test() {
    statc_test!().statc("it 5 mid 0.95").expect_stdout(
        "
---
X ~ t(5)
a: left bound  | -2.5705818356
b: right bound | 2.5705818356
P(a < X <= b)  | 0.95
",
    );

    statc_test!().statc("it 5 left 0.95").expect_stdout(
        "
---
X ~ t(5)
P(X < 2.0150483733) = 0.95
",
    );

    statc_test!().statc("it 5 right 0.9").expect_stdout(
        "
---
X ~ t(5)
P(X > -1.4758840488) = 0.9
",
    );
}

#[test]
fn ichisq_test() {
    statc_test!().statc("ichisq 5 0.9").expect_stdout(
        "
---
X ~ χ²(5)
P(X > 1.6103210449) = 0.9
",
    );
}

#[test]
#[ignore]
fn template() {
    statc_test!().statc("").expect_stdout(
        "
---
",
    );
}

#[test]
fn secret_test() {
    statc_test!().statc("secret").expect_stderr(" ").assert_ne();
}
