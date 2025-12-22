use {
  anyhow::{Context, Result, bail, ensure},
  arguments::Arguments,
  clap::Parser,
  client::Client,
  issue::Issue,
  pulldown_cmark::{Event, Parser as MarkdownParser, Tag},
  reqwest::{StatusCode, redirect::Policy},
  status::Status,
  std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fmt::{self, Display, Formatter},
    fs,
    path::{Path, PathBuf},
    process,
    time::Duration,
  },
  target::Target,
  url::Url,
  walkdir::WalkDir,
};

const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_YELLOW: &str = "\x1b[33m";
const DEFAULT_TIMEOUT_SECONDS: u64 = 10;

mod arguments;
mod client;
mod issue;
mod status;
mod target;

fn strip_link_attributes(destination: &str) -> &str {
  let Some(start) = destination.rfind('{') else {
    return destination;
  };

  if !destination.ends_with('}') {
    return destination;
  }

  let candidate = &destination[..start];

  if candidate.trim().is_empty() {
    destination
  } else {
    candidate
  }
}

fn unwrap_angle_brackets(destination: &str) -> &str {
  destination
    .strip_prefix('<')
    .and_then(|s| s.strip_suffix('>'))
    .unwrap_or(destination)
}

fn normalize_destination(destination: &str) -> String {
  strip_link_attributes(unwrap_angle_brackets(destination.trim()))
    .trim()
    .to_owned()
}

fn strip_fragment_and_query(destination: &str) -> &str {
  &destination[..destination
    .find(&['#', '?'][..])
    .unwrap_or(destination.len())]
}

fn has_skipped_scheme(destination: &str) -> bool {
  const SKIPPED_SCHEMES: &[&str] = &["mailto:", "tel:", "javascript:", "data:"];

  SKIPPED_SCHEMES.iter().any(|scheme| {
    destination.len() >= scheme.len()
      && destination[..scheme.len()].eq_ignore_ascii_case(scheme)
  })
}

fn main() {
  match Arguments::parse().run() {
    Ok(expired_count) => {
      if expired_count > 0 {
        process::exit(1);
      }
    }
    Err(error) => {
      eprintln!("error: {error}");
      process::exit(2);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn strip_link_attributes_removes_suffix_block() {
    assert_eq!(
      strip_link_attributes("https://example.com{.class}"),
      "https://example.com"
    );
  }

  #[test]
  fn strip_link_attributes_ignores_empty_candidate() {
    assert_eq!(strip_link_attributes("{#fragment}"), "{#fragment}");
  }

  #[test]
  fn unwrap_angle_brackets_strips_once() {
    assert_eq!(
      unwrap_angle_brackets("<https://example.com>"),
      "https://example.com"
    );

    assert_eq!(
      unwrap_angle_brackets("https://example.com"),
      "https://example.com"
    );
  }

  #[test]
  fn normalize_destination_trims_and_unwraps() {
    assert_eq!(
      normalize_destination(" <https://example.com> "),
      "https://example.com"
    );
  }

  #[test]
  fn normalize_destination_removes_attributes() {
    assert_eq!(
      normalize_destination("https://example.com{#id}"),
      "https://example.com"
    );
  }

  #[test]
  fn strip_fragment_and_query_stops_at_first_marker() {
    assert_eq!(
      strip_fragment_and_query("path/file.md?query#frag"),
      "path/file.md"
    );
  }

  #[test]
  fn has_skipped_scheme_detects_case_insensitive() {
    assert!(has_skipped_scheme("MAILTO:hello@example.com"));
    assert!(!has_skipped_scheme("https://example.com"));
  }
}
