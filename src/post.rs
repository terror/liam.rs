use super::*;

const DATE: &[FormatItem] = format_description!("[year]-[month]-[day]");

const RSS_DATE: &[FormatItem] = format_description!(
  "[weekday repr:short], [day padding:zero] [month repr:short] [year] [hour]:[minute]:00 +0000"
);

#[derive(Clone, Serialize)]
pub(crate) struct Post {
  pub(crate) date: String,
  pub(crate) filename: String,
  pub(crate) height: String,
  pub(crate) html: String,
  pub(crate) modified: SystemTime,
  pub(crate) path: PathBuf,
  pub(crate) read_time: String,
  pub(crate) rss_date: String,
  pub(crate) rss_html: String,
  pub(crate) slug: Slug,
  pub(crate) title: String,
}

impl Post {
  fn height(lines: usize) -> String {
    let lines = f64::from(u32::try_from(lines).unwrap_or(u32::MAX));

    format!("{:.2}", lines * 18.0 * 0.0222)
  }

  pub(crate) fn new(
    filename: String,
    html: String,
    input: &str,
    modified: SystemTime,
    path: PathBuf,
    rss_html: String,
  ) -> Result<Self> {
    let title = Self::title(&filename);

    Ok(Self {
      date: OffsetDateTime::from(modified).format(DATE)?,
      filename,
      height: Self::height(input.lines().count()),
      html,
      modified,
      path,
      read_time: Self::read_time(input.split_whitespace().count()),
      rss_date: OffsetDateTime::from(modified).format(RSS_DATE)?,
      rss_html,
      slug: Slug(title.clone()),
      title,
    })
  }

  fn read_time(words: usize) -> String {
    let words = f64::from(u32::try_from(words).unwrap_or(u32::MAX));

    format!("{:.1}", words / 150.0)
  }

  fn title(filename: &str) -> String {
    Path::new(filename)
      .file_stem()
      .unwrap_or_default()
      .to_string_lossy()
      .replace('-', " ")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn titles() {
    assert_eq!(
      Post::title("Powerful-`just`-features.md"),
      "Powerful `just` features"
    );
  }
}
