use super::*;

pub(crate) struct Environment {
  environment: minijinja::Environment<'static>,
}

impl Environment {
  pub(crate) fn new() -> Result<Self> {
    let mut environment = minijinja::Environment::new();

    environment.set_keep_trailing_newline(true);

    for (name, path) in [
      ("index.html", "templates/index.html.j2"),
      ("post.html", "templates/post.html.j2"),
      ("posts.html", "templates/posts.html.j2"),
      ("projects.html", "templates/projects.html.j2"),
      ("rss.xml", "templates/rss.xml.j2"),
    ] {
      environment.add_template_owned(name, fs::read_to_string(path)?)?;
    }

    Ok(Self { environment })
  }

  pub(crate) fn render<T: Serialize>(
    &self,
    name: &str,
    context: T,
  ) -> Result<String> {
    Ok(self.environment.get_template(name)?.render(context)?)
  }

  pub(crate) fn write<T: Serialize>(
    &self,
    path: impl AsRef<Path>,
    template: &str,
    context: T,
  ) -> Result {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    fs::write(path, self.render(template, context)?)?;

    Ok(())
  }

  pub(crate) fn write_post(&self, post: &Post) -> Result {
    println!("[~] {}", post.title);

    let path = PathBuf::from("docs/posts")
      .join(post.slug.to_string())
      .join("index.html");

    self.write(path, "post.html", minijinja::context! { post => post })
  }
}
