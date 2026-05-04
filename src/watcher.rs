use super::*;

pub(crate) struct Watcher {
  reloader: Reloader,
  style: Style,
}

impl Watcher {
  const DEBOUNCE: Duration = Duration::from_millis(250);

  pub(crate) fn new(reloader: Reloader) -> Self {
    Self {
      reloader,
      style: Style::stdout(),
    }
  }

  pub(crate) fn watch(self) -> Result {
    let (sender, receiver) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(sender)?;

    for path in ["posts", "projects", "templates"] {
      notify::Watcher::watch(
        &mut watcher,
        Path::new(path),
        RecursiveMode::Recursive,
      )?;
    }

    println!(
      "{} posts, projects, templates",
      self.style.apply(GREEN, self.style.apply(BOLD, "[watch]"))
    );

    let mut pending = BTreeSet::new();
    let mut last_change = Instant::now();

    loop {
      match receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(event) => {
          let event = event?;

          if matches!(
            &event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
          ) {
            pending.extend(event.paths);
            last_change = Instant::now();
          }
        }
        Err(RecvTimeoutError::Timeout) => {
          if !pending.is_empty() && last_change.elapsed() >= Self::DEBOUNCE {
            let remaining = pending.len().saturating_sub(3);

            let changes = pending
              .iter()
              .take(3)
              .map(|path| path.display().to_string())
              .collect::<Vec<_>>()
              .join(", ");

            let changes = if remaining == 0 {
              changes
            } else {
              format!("{changes}, and {remaining} more")
            };

            println!(
              "{} {}",
              self.style.apply(YELLOW, self.style.apply(BOLD, "[reload]")),
              self.style.apply(DIM, changes)
            );

            pending.clear();

            match Generator::new().run() {
              Ok(()) => {
                self.reloader.reload();
              }
              Err(error) => {
                eprintln!("{} {error}", self.style.apply(RED, "error:"));
              }
            }
          }
        }
        Err(error) => return Err(error.into()),
      }
    }
  }
}
