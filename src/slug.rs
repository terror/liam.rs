use super::*;

pub(crate) struct Slug;

impl Slug {
  pub(crate) fn title(input: &str) -> String {
    let mut output = String::new();
    let mut separator = false;

    for c in input.chars().flat_map(char::to_lowercase) {
      if c.is_ascii_alphanumeric() {
        if separator && !output.is_empty() {
          output.push('-');
        }

        output.push(c);
        separator = false;
      } else {
        separator = true;
      }
    }

    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn titles() {
    #[track_caller]
    fn case(input: &str, expected: &str) {
      assert_eq!(Slug::title(input), expected);
    }

    case("Powerful `just` features", "powerful-just-features");
    case("microeconomics.study", "microeconomics-study");
    case("  foo---bar  ", "foo-bar");
  }
}
