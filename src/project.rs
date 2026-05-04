use super::*;

const PROJECT_DATE: &[FormatItem] = format_description!("[year]-[month]-[day]");

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
  pub(crate) fn load(path: &Path) -> Result<Self> {
    let input = fs::read_to_string(path).with_context(|| {
      format!("failed to read project `{}`", path.display())
    })?;

    let frontmatter = Frontmatter::<ProjectFrontmatter>::parse(&input)?;

    let date = Date::parse(&frontmatter.metadata.date, PROJECT_DATE)?;

    Ok(
      Self::builder()
        .date(frontmatter.metadata.date.clone())
        .html(Loader::markdown(frontmatter.content)?)
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
