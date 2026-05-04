use super::*;

#[derive(Parser)]
pub(crate) struct Serve {
  #[arg(short, long, default_value_t = 8000)]
  port: u16,
}

impl Serve {
  pub(crate) fn run(self) -> Result {
    Server::new(self.port).serve()
  }
}
