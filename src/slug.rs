use super::*;

#[derive(Clone)]
pub(crate) struct Slug(pub(crate) String);

impl Display for Slug {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    f.write_str(
      &self
        .0
        .chars()
        .flat_map(char::to_lowercase)
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-"),
    )
  }
}

impl Serialize for Slug {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn titles() {
    assert_eq!(
      Slug("Powerful `just` features".to_string()).to_string(),
      "powerful-just-features"
    );

    assert_eq!(
      Slug("microeconomics.study".to_string()).to_string(),
      "microeconomics-study"
    );

    assert_eq!(Slug("  foo---bar  ".to_string()).to_string(), "foo-bar");
  }
}
