use {
  anyhow::Context,
  clap::Parser,
  regex::{Captures, Regex},
  std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process,
    sync::LazyLock,
  },
  walkdir::WalkDir,
};

pub static LINK_TRANSFORM: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(
    r"(?m)\[([^\]\n]*)\]\(([^()\n]*(?:\([^()\n]*\)[^()\n]*)*)\)([^{\n]|$)",
  )
  .unwrap()
});

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[derive(Debug, Parser)]
#[command(
  name = env!("CARGO_PKG_NAME"),
  about = "Add target=\"_blank\" to markdown links.",
)]
struct Arguments {
  /// One or more markdown files to process.
  #[arg(value_name = "FILE", num_args = 0..)]
  files: Vec<PathBuf>,
  /// Process all .md files recursively from the current directory.
  #[arg(short = 'r', long = "recursive", conflicts_with = "files")]
  recursive: bool,
}

fn replacement_for(captures: &Captures<'_>) -> String {
  let url = captures.get(2).map_or("", |match_| match_.as_str());

  let url_trimmed = url.trim();

  if url_trimmed.starts_with('#') {
    return captures
      .get(0)
      .map_or_else(String::new, |match_| match_.as_str().to_string());
  }

  let tail = captures.get(3).map_or("", |match_| match_.as_str());

  if tail.is_empty() {
    format!("[{}]({}){{target=\"_blank\"}}", &captures[1], &captures[2])
  } else {
    format!(
      "[{}]({}){{target=\"_blank\"}}{tail}",
      &captures[1], &captures[2],
    )
  }
}

fn process_file(path: &Path) -> Result {
  let input = fs::read_to_string(path)
    .with_context(|| format!("reading {}", path.display()))?;

  let output = LINK_TRANSFORM.replace_all(&input, replacement_for);

  fs::write(path, output.as_bytes())
    .with_context(|| format!("writing {}", path.display()))?;

  Ok(())
}

fn run() -> Result {
  let arguments = Arguments::parse();

  let is_markdown =
    |path: &Path| -> bool { path.extension() == Some(OsStr::new("md")) };

  if arguments.recursive {
    for entry in WalkDir::new(".") {
      let entry = entry?;

      if entry.file_type().is_file() && is_markdown(entry.path()) {
        process_file(entry.path())?;
      }
    }
  } else {
    for path in &arguments.files {
      if path.is_file() && is_markdown(path) {
        process_file(path)?;
      }
    }
  }

  Ok(())
}

fn main() {
  if let Err(error) = run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn adds_target_for_non_fragment_links() {
    let input = "- [Example](https://example.com)\n";

    assert_eq!(
      LINK_TRANSFORM.replace_all(input, replacement_for),
      "- [Example](https://example.com){target=\"_blank\"}\n"
    );
  }

  #[test]
  fn keeps_fragment_links_unchanged() {
    let input = "- [Panic! at the disco](#panic-at-the-disco)\n";

    assert_eq!(LINK_TRANSFORM.replace_all(input, replacement_for), input);
  }

  #[test]
  fn preserves_existing_target_attribute() {
    let input = "- [Example](https://example.com){target=\"_blank\"}\n";

    assert_eq!(LINK_TRANSFORM.replace_all(input, replacement_for), input);
  }

  #[test]
  fn adds_target_for_urls_with_parentheses() {
    let input = "- [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell))\n";

    assert_eq!(
      LINK_TRANSFORM.replace_all(input, replacement_for),
      "- [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)){target=\"_blank\"}\n"
    );
  }
}
