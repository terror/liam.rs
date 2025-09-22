import os
import subprocess as sp
import typing as t
from dataclasses import dataclass
from datetime import datetime
from glob import glob

import frontmatter

HEADER = """
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="HandheldFriendly" content="true">
<meta name="msapplication-TileColor" content="#da532c">
<meta name="theme-color" content="#ffffff">
<meta name="viewport" content="initial-scale=1">
<meta property="og:title" content="liam">
<meta property="og:description" content="liam's musings">
<meta property="og:type" content="website">
<meta property="og:url" content="https://liam.rs">
<link rel="alternate" type="application/atom+xml" title="liam's musings" href="./index.xml">
<link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
<link rel="icon" type="image/png" href="/favicon/favicon-96x96.png" sizes="96x96" />
<link rel="icon" type="image/svg+xml" href="/favicon/favicon.svg" />
<link rel="manifest" href="/favicon/site.webmanifest" />
<link rel="shortcut icon" href="/favicon/favicon.ico" />
<link rel="stylesheet" href="/css/style.css">
<title>Projects · liam.rs</title>
<!-- Global site tag (gtag.js) - Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=UA-151954055-3"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){ dataLayer.push(arguments); }
  gtag('js', new Date());
  gtag('config', 'UA-151954055-3');
</script>
</head>
  <body>
    <div class="posts">
    <div class="post">
    <a href="/" class="post-end-link">Home</a>
    <span>/</span>
    <a class="post-end-link">Projects</a>
    <h1>Projects</h1>
    <div class="separator"></div>
    <table>
"""

FOOTER = """
</table>
<div class="separator"></div>
<div class="footer">
  <a href="/projects">Projects</a> ·
  <a href="https://stream.liam.rs" target="_blank">Stream</a> ·
  <a href="https://linkedin.com/in/liam-scalzulli" target="_blank"
  >LinkedIn</a
  >
  · <a href="https://github.com/terror" target="_blank">Github</a> ·
  <a href="mailto:liam@scalzulli.com">Mail</a> ·
  <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">
  <img
  class="footimgs"
  src="https://d33wubrfki0l68.cloudfront.net/94387e9d77fbc8b4360db81e72603ecba3df94a7/632bc/static/cc.svg"
  />
  </a>
</div>
</div>
</div>
</body>
</html>
"""


@dataclass
class Project:
  title: str
  date: str
  repo: str
  topics: t.List[str]
  lead: str
  image: str
  content: str

  pandoc = ['pandoc', '--mathjax', '--quiet', '-t', 'html', '--syntax-highlighting', 'monochrome']

  def markdown(self) -> str:
    return sp.run(
      self.pandoc, input=self.content, capture_output=True, text=True, check=True
    ).stdout

  def template(self) -> str:
    return f"""
    <tr class="grid-container" id="{self.title.lower().replace(' ', '-')}">
      <td>
        <div style="margin-bottom: 1em">
          <strong>{project.title}</strong> {datetime.strptime(str(project.date), '%Y-%m-%d').strftime('%B %Y')}
        </div>
        <div style="margin-bottom: 1em; display: flex; gap: 0.4em">
          {''.join(map(lambda topic: f'<span style="padding: 2px 4px 2px 4px; border: 2px solid var(--dark-white); border-radius: 0.5em">{topic}</span>', project.topics))}
        </div>
        <em>
          {project.lead}
        </em>
        <div>
          {self.markdown()}
        </div>
      </td>
      <td style="display: flex; align-items: center">
        <img src='{'/images/' + project.image}'>
      </td>
    </tr>
    """


if __name__ == '__main__':
  print('[+] PROJECTS')

  output = 'docs/projects/index.html'

  if os.path.exists(output):
    os.remove(output)

  os.makedirs(os.path.dirname(output), exist_ok=True)

  projects = sorted(
    map(
      lambda x: (lambda y: Project(content=y.content, **y))(frontmatter.load(x)),
      glob('projects/*.md'),
    ),
    key=lambda project: datetime.strptime(str(project.date), '%Y-%m-%d'),
    reverse=True,
  )

  with open(output, 'w') as out:
    out.write(HEADER)

    for project in projects:
      print(f'[~] {project.title}')
      out.write(project.template())

    out.write(FOOTER)
