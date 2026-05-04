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

#[cfg(test)]
mod tests {
  use {super::*, indoc::indoc};

  #[derive(Debug, Deserialize, PartialEq)]
  struct Metadata {
    title: String,
  }

  #[test]
  fn parse() {
    assert_eq!(
      Frontmatter::<Metadata>::parse(indoc! {
        "
        ---
        title: foo
        ---
        bar
        "
      })
      .unwrap(),
      Frontmatter {
        content: "bar\n",
        metadata: Metadata {
          title: "foo".to_string(),
        },
      }
    );
  }
}
