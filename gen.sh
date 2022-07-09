#! /usr/bin/env bash

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Utilities                                                              ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

INDEX_BLOG_LIMIT=3
INDEX_BLOG_COUNT=0

title_wrapper() {
  echo "$1" | sed -E -e "s/\..+$//g"  -e "s/-(.)/ \1/g" -e "s/^(.)/\1/g"
}

read_time() {
  echo "$(eva -f 1 "$1"/150 | xargs)"
}

height() {
  echo "$(eva -f 2 "$1"*18*0.0222 | xargs)"
}

link_wrapper() {
  # 1 - id
  # 2 - title
  # 3 - date
  # 4 - read time
  echo -ne "
  <tr>
    <td class=\"table-post\">
      <div class=\"date\">
        $3
      </div>
      <a href=\"/posts/$1\" class=\"post-link\">
        <span class=\"post-link\">$2</span>
      </a>
    </td>
    <td class=\"table-stats\">
      <span class=\"stats-number\">
        $4
      </span>
      <span class=\"stats-unit\">min</span>
    </td>
  </tr>
  "
}

intro() {
  echo -ne "
    <div class=\"intro\">
      Hi.
    <div class=\"hot-links\">
      <a href=\"https://liam.rs/index.xml\" class=\"feed-button\">Subscribe</a>
      <a href=\"https://www.buymeacoffee.com/terror\" class=\"donate-button\" target=\"_blank\">Donate</a>
    </div>
    <p>I'm Liam.</p>
    <p>
      I am compsci undergrad with a minor in mathematics @ mcgill university, programmer and
      rationalist. For fun I enjoy working on open-source projects, reading, and lifting weights.
    </p>
    <p>Send me a message by email: liam [at] scalzulli.com or matrix: worse:matrix.org</p>
  </div>
  "
}

recent_link() {
  # 1 - sub page
  cat << EOF
  <tr><td class="recent-heading"><span class="recent-heading">Recent $1</span></td></tr>
EOF
}

more_link() {
  # 1 - sub page
  cat << EOF
  <tr><td><a href="/$1" class="post-end-link">More ⟶ </a></td></tr>
EOF
}

breadcrumbs() {
  # 1 - path
  cat << EOF
  <a href="/" class="post-end-link">Home</a>
  <span>/</span>
  <a class="post-end-link">$1</a>
EOF
}

logo() {
  cat << EOF
  <div>
  <img class='avatar' src="https://files.catbox.moe/2ldb2v.jpeg" width="100" height="100">
  <h4 class="subheading">liam's musings</h4>
  </div>
EOF
}

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Meta & Intro                                                           ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

posts=$(ls -t ./posts)
rm -rf "./docs/posts/"
mkdir -p docs/posts

cat << EOF | tee ./docs/posts/index.html ./docs/index.html > /dev/null
<!DOCTYPE html>
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
<link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png"/>
<link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png"/>
<link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png"/>
<link rel="manifest" href="/favicon/site.webmanifest"/>
<link rel="mask-icon" href="/favicon/safari-pinned-tab.svg" color="#5bbad5">
<link rel="stylesheet" href="/css/style.css">
<title>liam.rs</title>
<!-- Global site tag (gtag.js) - Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=UA-151954055-3"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){ dataLayer.push(arguments); }
  gtag('js', new Date());
  gtag('config', 'UA-151954055-3');
</script>
</head>
EOF

echo "[+] INDEX"

cat << EOF >> ./docs/index.html
  <body>
  $(logo)
  <div class="posts">
  <div class="post">
  $(intro)
  <table>
  $(recent_link "Posts")
EOF

paths=("Posts")
for p in "${paths[@]}"; do
  cat << EOF | tee -a ./docs/"${p,,}"/index.html > /dev/null
  <body>
    <div class="posts">
    <div class="post">
    $(breadcrumbs "$p")
EOF
done

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Posts                                                                  ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

echo -ne "
  <h1>Posts</h1>
  <div class=\"separator\"></div>
  <table>
    " >> ./docs/posts/index.html

for f in $posts; do
  file="./posts/$f"
  id="${file##*/}"

  stats=$(wc "$file")
  words="$(echo "$stats" | awk '{ print $2 }')"
  lines="$(echo "$stats" | awk '{ print $1 }')"
  r_time="$(read_time "$words")"
  height="$(height "$lines")"
  post_title=$(title_wrapper "$id")

  echo "[~] $post_title"

  post_date=$(date -r "$file" "+%d/%m — %Y")
  post_link=$(link_wrapper "${id%.*}" "$post_title" "$post_date" "$r_time")

  echo -ne "$post_link" >> ./docs/posts/index.html

  if [[ $INDEX_BLOG_COUNT -lt $INDEX_BLOG_LIMIT ]]; then
    echo -ne "$post_link" >> docs/index.html
  fi

  ((INDEX_BLOG_COUNT+=1))

  id="${id%.*}"
  mkdir -p "docs/posts/$id"
  esh -s /bin/bash \
    -o "docs/posts/$id/index.html" \
    "./templates/post.esh" \
    file="$file" \
    date="$post_date" \
    title="$post_title" \
    read_time="$r_time" \
    height="$height" \
    intro="$(intro)"
done

echo "$(more_link posts)" >> ./docs/index.html

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ RSS                                                                    ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

echo "generating RSS feeds ..."
esh -s /bin/bash \
  -o "./docs/index.xml" \
  "./templates/rss.esh"

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Footer                                                                 ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

cat << EOF | tee -a ./docs/posts/index.html ./docs/index.html > /dev/null
  </table>
  <div class="separator"></div>
  <div class="footer">
    <a href="https://www.goodreads.com/fear" target="_blank">Books</a> ·
    <a href="https://notebook.liam.rs" target="_blank">Notes</a> ·
    <a href="https://linkedin.com/in/liam-scalzulli" target="_blank">LinkedIn</a> ·
    <a href="https://github.com/terror" target="_blank">Github</a> ·
    <a href="mailto:liam@scalzulli.com">Mail</a> ·
    <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">
        <img class="footimgs" src="https://d33wubrfki0l68.cloudfront.net/94387e9d77fbc8b4360db81e72603ecba3df94a7/632bc/static/cc.svg">
    </a>
  </div>
  </div>
</div>
</body>
</html>
EOF
