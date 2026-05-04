use super::*;

pub(crate) struct Converter {
  command: &'static str,
}

impl Converter {
  pub(crate) fn new() -> Self {
    Self { command: "pandoc" }
  }

  pub(crate) fn run<I, S>(&self, arguments: I, input: &str) -> Result<String>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut child = Command::new(self.command)
      .args(["--quiet", "-t", "html"])
      .args(arguments)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .context("failed to spawn pandoc")?;

    child
      .stdin
      .take()
      .context("failed to open pandoc stdin")?
      .write_all(input.as_bytes())?;

    let output = child.wait_with_output()?;

    ensure!(
      output.status.success(),
      "pandoc failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    Ok(String::from_utf8(output.stdout)?)
  }
}
