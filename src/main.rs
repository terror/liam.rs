use {
  anyhow::{Context, Error, ensure},
  arguments::Arguments,
  axum::{
    Router,
    extract::OriginalUri,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
  },
  clap::Parser,
  environment::Environment,
  front_matter::FrontMatter,
  generator::Generator,
  loader::Loader,
  minijinja::context,
  notify::{EventKind, RecursiveMode},
  post::Post,
  project::{Project, ProjectFrontMatter},
  serde::{Deserialize, Serialize},
  server::Server,
  slug::Slug,
  std::{
    ffi::OsStr,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    string::String,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::{Duration, Instant, SystemTime},
  },
  subcommand::Subcommand,
  time::{
    Date, Month, OffsetDateTime, format_description::FormatItem,
    macros::format_description,
  },
  tower_livereload::{LiveReloadLayer, Reloader},
  watcher::Watcher,
};

mod arguments;
mod environment;
mod front_matter;
mod generator;
mod loader;
mod post;
mod project;
mod server;
mod slug;
mod subcommand;
mod watcher;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");

    let causes = error.chain().skip(1).count();

    for (i, error) in error.chain().skip(1).enumerate() {
      eprintln!("       {}─ {error}", if i < causes - 1 { '├' } else { '└' });
    }

    process::exit(1);
  }
}
