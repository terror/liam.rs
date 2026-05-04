use super::*;

const DATE: &[FormatItem] = format_description!("[year]-[month]-[day]");

const RSS_DATE: &[FormatItem] = format_description!(
  "[weekday repr:short], [day padding:zero] [month repr:short] [year] [hour]:[minute]:00 +0000"
);

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct PostFrontmatter {
  pub(crate) date: String,
  pub(crate) title: String,
}

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

    let frontmatter = Frontmatter::<PostFrontmatter>::parse(&input)?;

    let modified = fs::metadata(path)?.modified()?;

    let filename = path
      .file_name()
      .context("post path has no filename")?
      .to_string_lossy()
      .to_string();

    Ok(
      Self::builder()
        .date(frontmatter.metadata.date.clone())
        .filename(filename)
        .height(format!(
          "{:.2}",
          f64::from(
            u32::try_from(frontmatter.content.lines().count())
              .unwrap_or(u32::MAX)
          ) * 18.0
            * 0.0222
        ))
        .html(Loader::markdown(frontmatter.content)?)
        .modified(modified)
        .path(path.to_path_buf())
        .read_time(format!(
          "{:.1}",
          f64::from(
            u32::try_from(frontmatter.content.split_whitespace().count())
              .unwrap_or(u32::MAX)
          ) / 150.0
        ))
        .rss_date(
          Date::parse(&frontmatter.metadata.date, DATE)?
            .with_hms(0, 0, 0)?
            .assume_utc()
            .format(RSS_DATE)?,
        )
        .rss_html(
          html_escape::encode_text(&Loader::pandoc(
            ["--quiet", "-t", "html"],
            frontmatter.content,
          )?)
          .trim_end()
          .to_string(),
        )
        .slug(Slug(frontmatter.metadata.title.clone()))
        .title(frontmatter.metadata.title)
        .build(),
    )
  }
}

#[cfg(test)]
mod tests {
  use {super::*, indoc::indoc};

  #[test]
  fn parse_frontmatter() {
    assert_eq!(
      Frontmatter::<PostFrontmatter>::parse(indoc! {
        "
        ---
        title: foo
        date: 2025-03-17
        ---
        bar
        "
      })
      .unwrap(),
      Frontmatter {
        content: "bar\n",
        metadata: PostFrontmatter {
          date: "2025-03-17".to_string(),
          title: "foo".to_string(),
        },
      }
    );
  }
}
