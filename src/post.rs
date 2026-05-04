use super::*;

const DATE: &[FormatItem] = format_description!("[year]-[month]-[day]");

const RSS_DATE: &[FormatItem] = format_description!(
  "[weekday repr:short], [day padding:zero] [month repr:short] [year] [hour]:[minute]:00 +0000"
);

#[derive(Clone, Serialize, TypedBuilder)]
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
  pub(crate) fn load(path: &Path) -> Result<Self> {
    let input = fs::read_to_string(path)
      .with_context(|| format!("failed to read post `{}`", path.display()))?;

    let modified = fs::metadata(path)?.modified()?;

    let filename = path
      .file_name()
      .context("post path has no filename")?
      .to_string_lossy()
      .to_string();

    let title = path
      .file_stem()
      .context("post path has no filename")?
      .to_string_lossy()
      .replace('-', " ");

    Ok(
      Self::builder()
        .date(OffsetDateTime::from(modified).format(DATE)?)
        .filename(filename)
        .height(format!(
          "{:.2}",
          f64::from(u32::try_from(input.lines().count()).unwrap_or(u32::MAX))
            * 18.0
            * 0.0222
        ))
        .html(Loader::markdown(&input)?)
        .modified(modified)
        .path(path.to_path_buf())
        .read_time(format!(
          "{:.1}",
          f64::from(
            u32::try_from(input.split_whitespace().count()).unwrap_or(u32::MAX)
          ) / 150.0
        ))
        .rss_date(OffsetDateTime::from(modified).format(RSS_DATE)?)
        .rss_html(
          html_escape::encode_text(&Loader::pandoc(
            ["--quiet", "-t", "html"],
            &input,
          )?)
          .trim_end()
          .to_string(),
        )
        .slug(Slug(title.clone()))
        .title(title)
        .build(),
    )
  }
}
