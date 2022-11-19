use crate::utils::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const TEST_DIR: &str = "statc-tests";

#[derive(PartialEq)]
pub struct Test {
    name: String,
    bin_path: PathBuf,
    test_dir: PathBuf,
    received: ShellOutputs,
    expected: ShellOutputs,
    asserted_once: bool,
}

pub fn test(name: &str) -> Test {
    Test::new(name)
}

/// used for setting expected value of tests
/// "---" is used to prefix longer test outputs so that the string literal
/// can be flushed to the left.
fn set_expected(target: &mut String, val: &str) {
    if val.is_empty() {
        (*target).clear();
        return;
    }
    *target = val.split_once("---\n").map(|a| a.1).unwrap_or(val).to_string();
}

impl Test {
    fn new(name: &str) -> Test {
        let name = String::from(name);
        let test_dir = env::temp_dir().join(TEST_DIR).join(&name);
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }
        let bin_path = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf()
            .join(format!("../statc{}", env::consts::EXE_SUFFIX));
        fs::create_dir_all(&test_dir).unwrap();
        Test {
            bin_path,
            asserted_once: false,
            received: ShellOutputs::default(),
            expected: ShellOutputs::default(),
            name,
            test_dir,
        }
    }

    /// get path to statc's debug binary build
    fn bin(&self) -> Command {
        Command::new(&self.bin_path)
    }

    /// Runs a `statc` command at a relative path from the test
    /// directory and populates `self.received` with output
    pub fn statc(&mut self, args: &str) -> &mut Self {
        self.received = self.bin().args(args.split(' ')).outputs();
        self
    }

    /// Set expected stdout value.
    pub fn expect_stdout(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stdout, val);
        self
    }

    /// Set expected stderr value.
    pub fn expect_stderr(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stderr, val);
        self
    }

    pub fn assert(&mut self) -> &mut Self {
        self.asserted_once = true;
        assert_eq_pretty!(&self.expected.stdout, &self.received.stdout);
        assert_eq_pretty!(&self.expected.stderr, &self.received.stderr);
        self
    }
}

impl Drop for Test {
    /// asserts if hasn't, and then executes teardown
    fn drop(&mut self) {
        if !self.asserted_once {
            self.assert();
        }
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
