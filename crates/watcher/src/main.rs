use {
  anyhow::Error,
  notify::{Event, EventKind, RecursiveMode, Watcher},
  std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::Path,
    process::{self, Command, Stdio},
    sync::mpsc::channel,
    time::{Duration, Instant},
  },
};

fn run(script: &str, file_path: &str) -> Result<()> {
  let mut child = Command::new(script)
    .arg(file_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Failed to start script");

  let stdout_reader =
    BufReader::new(child.stdout.take().expect("Failed to capture stdout"));

  for line in stdout_reader.lines() {
    println!("{}", line?);
  }
  Ok(())
}

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() -> Result {
  let args = std::env::args().collect::<Vec<String>>();

  if args.len() != 3 {
    eprintln!("Usage: {} <directory> <script>", args[0]);
    process::exit(1);
  }

  let (directory, script) = (&args[1], &args[2]);

  let (tx, rx) = channel();

  let mut watcher = notify::recommended_watcher(tx)?;

  watcher.watch(Path::new(directory), RecursiveMode::Recursive)?;

  println!(
    "Watching for file modifications in {}. Press Ctrl-C to stop.",
    directory
  );

  let mut last_run_time = HashMap::new();

  let debounce_duration = Duration::from_millis(500);

  loop {
    match rx.recv_timeout(Duration::from_secs(1)) {
      Ok(event) => match event {
        Ok(Event { kind, paths, .. }) => {
          if let EventKind::Modify(_) = kind {
            for path in paths {
              let path_str = path.to_str().unwrap();

              let now = Instant::now();

              if let Some(&last_run) = last_run_time.get(path_str) {
                if now.duration_since(last_run) < debounce_duration {
                  continue;
                }
              }

              run(script, path_str)?;

              last_run_time.insert(path_str.to_string(), now);
            }
          }
        }
        Err(e) => eprintln!("error: {:?}", e),
      },
      Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
      Err(e) => eprintln!("error: {:?}", e),
    }
  }
}
