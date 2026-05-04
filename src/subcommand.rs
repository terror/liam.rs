use super::*;

#[path = "subcommand/build.rs"]
mod build;

#[path = "subcommand/serve.rs"]
mod serve;

#[derive(Parser)]
pub(crate) enum Subcommand {
  Generate,
  Serve(serve::Serve),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Generate => build::Generate::run(),
      Self::Serve(serve) => serve.run(),
    }
  }
}
