use super::*;

#[derive(Parser)]
pub(crate) struct Arguments {
  #[command(subcommand)]
  subcommand: Subcommand,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    self.subcommand.run()
  }
}
