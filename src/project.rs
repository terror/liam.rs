use super::*;

const PROJECT_DATE: &[FormatItem] = format_description!("[year]-[month]-[day]");

#[derive(Deserialize)]
pub(crate) struct ProjectFrontMatter {
  pub(crate) date: String,
  pub(crate) image: String,
  pub(crate) lead: String,
  pub(crate) title: String,
  pub(crate) topics: Vec<String>,
}

#[derive(Clone, Serialize)]
pub(crate) struct Project {
  pub(crate) date: String,
  pub(crate) html: String,
  pub(crate) image: String,
  pub(crate) lead: String,
  pub(crate) month: String,
  pub(crate) slug: String,
  pub(crate) title: String,
  pub(crate) topics: Vec<String>,
}

impl Project {
  fn month(date: Date) -> String {
    let month = match date.month() {
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
    };

    format!("{month} {}", date.year())
  }

  pub(crate) fn new(
    front_matter: ProjectFrontMatter,
    html: String,
  ) -> Result<Self> {
    let date = Date::parse(&front_matter.date, PROJECT_DATE)?;

    Ok(Self {
      date: front_matter.date,
      html,
      image: front_matter.image,
      lead: front_matter.lead,
      month: Self::month(date),
      slug: Slug::title(&front_matter.title),
      title: front_matter.title,
      topics: front_matter.topics,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn months() {
    assert_eq!(
      Project::month(Date::parse("2025-03-17", PROJECT_DATE).unwrap()),
      "March 2025"
    );
  }
}
