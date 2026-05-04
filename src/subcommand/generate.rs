use super::*;

pub(crate) fn run() -> Result {
  let style = Style::stdout();

  let start = Instant::now();

  Generator::new().run()?;

  println!(
    "{} done in {}ms",
    style.apply(GREEN, style.apply(BOLD, "[generate]")),
    start.elapsed().as_millis()
  );

  Ok(())
}
