use {super::*, serve::Serve};

mod generate;
mod serve;

#[derive(Parser)]
pub(crate) enum Subcommand {
  Generate,
  Serve(Serve),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Generate => generate::run(),
      Self::Serve(serve) => serve.run(),
    }
  }
}
