use std::process::Output;

pub(crate) mod process;

pub(crate) struct OsAbstraction;

impl OsAbstraction {
  pub fn read_output(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout)
      .to_string()
  }
}