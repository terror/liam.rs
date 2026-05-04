use super::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Frontmatter<'a, T> {
  pub(crate) content: &'a str,
  pub(crate) metadata: T,
}

impl<'a, T> Frontmatter<'a, T>
where
  T: Deserialize<'a>,
{
  pub(crate) fn parse(input: &'a str) -> Result<Self> {
    let input = input
      .strip_prefix("---\n")
      .context("missing front matter")?;

    let (metadata, content) = input
      .split_once("\n---\n")
      .context("unclosed front matter")?;

    Ok(Self {
      content,
      metadata: serde_yaml::from_str(metadata)
        .context("invalid front matter")?,
    })
  }
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ProjectFrontmatter {
  pub(crate) date: String,
  pub(crate) image: String,
  pub(crate) lead: String,
  pub(crate) title: String,
  pub(crate) topics: Vec<String>,
}

#[cfg(test)]
mod tests {
  use {super::*, indoc::indoc};

  #[test]
  fn parse() {
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
