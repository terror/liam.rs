use {
  anyhow::Context,
  clap::Parser,
  pulldown_cmark::{Event, LinkType, Options, Parser as MarkdownParser, Tag},
  std::{
    backtrace::BacktraceStatus,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process,
  },
  walkdir::WalkDir,
};

const TARGET_ATTRIBUTE: &str = "{target=\"_blank\"}";

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

fn should_add_target(
  link_type: LinkType,
  destination: &str,
  source: &str,
  link_end: usize,
) -> bool {
  if matches!(link_type, LinkType::WikiLink { .. }) {
    return false;
  }

  let destination_trimmed = destination.trim();

  if destination_trimmed.is_empty() || destination_trimmed.starts_with('#') {
    return false;
  }

  source.as_bytes().get(link_end) != Some(&b'{')
}

fn transform_links(input: &str) -> String {
  let options = Options::all();

  let parser = MarkdownParser::new_ext(input, options).into_offset_iter();

  let mut insertions = parser
    .filter(|(event, range)| {
      matches!(
          event,
          Event::Start(Tag::Link { link_type, dest_url, .. })
              if should_add_target(*link_type, dest_url, input, range.end)
      )
    })
    .map(|(_, range)| range.end)
    .collect::<Vec<_>>();

  if insertions.is_empty() {
    return input.to_string();
  }

  insertions.sort_unstable();
  insertions.dedup();

  let mut output = String::with_capacity(
    input.len() + insertions.len() * TARGET_ATTRIBUTE.len(),
  );

  let mut last_index = 0;

  for index in insertions {
    output.push_str(&input[last_index..index]);
    output.push_str(TARGET_ATTRIBUTE);
    last_index = index;
  }

  output.push_str(&input[last_index..]);
  output
}

fn process_file(path: &Path) -> Result {
  let input = fs::read_to_string(path)
    .with_context(|| format!("reading {}", path.display()))?;

  let output = transform_links(&input);

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

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = run() {
    eprintln!("error: {error}");

    for (i, error) in error.chain().skip(1).enumerate() {
      if i == 0 {
        eprintln!();
        eprintln!("because:");
      }

      eprintln!("- {error}");
    }

    let backtrace = error.backtrace();

    if backtrace.status() == BacktraceStatus::Captured {
      eprintln!("backtrace:");
      eprintln!("{backtrace}");
    }

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
      transform_links(input),
      "- [Example](https://example.com){target=\"_blank\"}\n"
    );
  }

  #[test]
  fn keeps_fragment_links_unchanged() {
    let input = "- [Panic! at the disco](#panic-at-the-disco)\n";

    assert_eq!(transform_links(input), input);
  }

  #[test]
  fn preserves_existing_target_attribute() {
    let input = "- [Example](https://example.com){target=\"_blank\"}\n";

    assert_eq!(transform_links(input), input);
  }

  #[test]
  fn adds_target_for_urls_with_parentheses() {
    let input = "- [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell))\n";

    assert_eq!(
      transform_links(input),
      "- [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)){target=\"_blank\"}\n"
    );
  }

  #[test]
  fn ignores_links_inside_inline_code() {
    let input = "Use `[Example](https://example.com)` as code.\n";

    assert_eq!(transform_links(input), input);
  }
}
