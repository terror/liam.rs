use super::*;

#[derive(Parser)]
pub(crate) struct Arguments {
  #[command(subcommand)]
  subcommand: Option<Subcommand>,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    self.subcommand.unwrap_or(Subcommand::Generate).run()
  }
}
