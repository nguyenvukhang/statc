use crate::utils::Result;

#[test]
fn core() -> Result<()> {
    statc_test!().statc("binom 10 0.2").expect_stdout(
        "
---
X ~ B(10, 0.2)
expected | 2
variance | 1.6
",
    );
    Ok(())
}
