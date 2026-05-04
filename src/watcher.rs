use super::*;

pub(crate) struct Watcher {
  reloader: Reloader,
}

impl Watcher {
  pub(crate) fn new(reloader: Reloader) -> Self {
    Self { reloader }
  }

  fn rebuild_and_reload(&self) {
    match Generator::new().build() {
      Ok(()) => {
        println!("[+] rebuilt");
        self.reloader.reload();
      }
      Err(error) => eprintln!("error: {error}"),
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

    println!("Watching posts, projects, and templates. Press Ctrl-C to stop.");

    let debounce = Duration::from_millis(250);
    let mut pending = false;
    let mut last_change = Instant::now();

    loop {
      match receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(event) => {
          if matches!(
            event?.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
          ) {
            pending = true;
            last_change = Instant::now();
          }
        }
        Err(RecvTimeoutError::Timeout) => {
          if pending && last_change.elapsed() >= debounce {
            pending = false;
            self.rebuild_and_reload();
          }
        }
        Err(error) => return Err(error.into()),
      }
    }
  }
}
