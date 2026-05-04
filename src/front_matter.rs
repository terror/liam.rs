use super::*;

pub(crate) struct FrontMatter<'a, T> {
  pub(crate) content: &'a str,
  pub(crate) metadata: T,
}

impl<'a, T> FrontMatter<'a, T>
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
  use {super::*, crate::project::ProjectFrontMatter};

  #[test]
  fn parse() {
    let front_matter = FrontMatter::<ProjectFrontMatter>::parse(
      "---
title: foo
date: 2025-03-17
repo: foo/bar
topics: ['foo', 'bar']
lead: foo
image: foo.png
---
bar
",
    )
    .unwrap();

    assert_eq!(front_matter.metadata.title, "foo");
    assert_eq!(front_matter.metadata.topics, ["foo", "bar"]);
    assert_eq!(front_matter.content, "bar\n");
  }
}
