use super::*;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ProjectFrontmatter {
  pub(crate) date: String,
  pub(crate) image: String,
  pub(crate) lead: String,
  pub(crate) title: String,
  pub(crate) topics: Vec<String>,
}

#[derive(Clone, Serialize, TypedBuilder)]
pub(crate) struct Project {
  pub(crate) date: String,
  pub(crate) html: String,
  pub(crate) image: String,
  pub(crate) lead: String,
  pub(crate) month: String,
  pub(crate) slug: Slug,
  pub(crate) title: String,
  pub(crate) topics: Vec<String>,
}

impl Project {
  const DATE_FORMAT: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day]");

  pub(crate) fn load(converter: &Converter, path: &Path) -> Result<Self> {
    let input = fs::read_to_string(path).with_context(|| {
      format!("failed to read project `{}`", path.display())
    })?;

    let frontmatter = Frontmatter::<ProjectFrontmatter>::parse(&input)?;

    let date = Date::parse(&frontmatter.metadata.date, Self::DATE_FORMAT)?;

    Ok(
      Self::builder()
        .date(frontmatter.metadata.date.clone())
        .html(converter.run(
          ["--mathjax", "--syntax-highlighting", "monochrome"],
          frontmatter.content,
        )?)
        .image(frontmatter.metadata.image)
        .lead(frontmatter.metadata.lead)
        .month(format!(
          "{} {}",
          match date.month() {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
          },
          date.year()
        ))
        .slug(Slug(frontmatter.metadata.title.clone()))
        .title(frontmatter.metadata.title)
        .topics(frontmatter.metadata.topics)
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
      Frontmatter::<ProjectFrontmatter>::parse(indoc! {
        "
        ---
        title: foo
        date: 2025-03-17
        repo: foo/bar
        topics: ['foo', 'bar']
        lead: foo
        image: foo.png
        ---
        bar
        "
      })
      .unwrap(),
      Frontmatter {
        content: "bar\n",
        metadata: ProjectFrontmatter {
          date: "2025-03-17".to_string(),
          image: "foo.png".to_string(),
          lead: "foo".to_string(),
          title: "foo".to_string(),
          topics: vec!["foo".to_string(), "bar".to_string()],
        },
      }
    );
  }
}
