#! /usr/bin/env bash

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Utilities                                                              ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

INDEX_BLOG_LIMIT=3
INDEX_BLOG_COUNT=0
INDEX_REVIEW_LIMIT=3
INDEX_REVIEW_COUNT=0

title_wrapper() {
  echo "$1" | sed -E -e "s/\..+$//g"  -e "s/_(.)/ \1/g" -e "s/^(.)/\1/g"
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
  # 5 - type
  echo -ne "
  <tr>
    <td class=\"table-post\">
      <div class=\"date\">
        $3
      </div>
      <a href=\"/$5/$1\" class=\"post-link\">
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
      I am a Computer Science Student at John Abbott College currently interning as a Software Engineer at Matrox.
      When I'm not doing coursework, I enjoy bodybuilding, reading and programming.
    </p>
    <p>Send me a message by email at liam@scalzulli.com or discord at liam#0005</p>
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
    <img src="https://files.catbox.moe/s77mzm.jpeg" width="100" height="100">
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

reviews=$(ls -t ./reviews)
rm -rf "./docs/reviews/"
mkdir -p docs/reviews

cat << EOF | tee ./docs/posts/index.html ./docs/reviews/index.html ./docs/index.html > /dev/null
<!DOCTYPE html>
<html lang="en">
<head>
<!-- Global site tag (gtag.js) - Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=UA-151954055-3"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'UA-151954055-3');
</script>
<link rel="stylesheet" href="/style.css">
<link rel="alternate" type="application/atom+xml" title="liam's musings" href="./index.xml">
<meta charset="UTF-8">
<meta name="viewport" content="initial-scale=1">
<meta content="#ffffff" name="theme-color">
<meta name="HandheldFriendly" content="true">
<meta property="og:title" content="liam">
<meta property="og:type" content="website">
<meta property="og:description" content="liam's musings">
<meta property="og:url" content="https://liams.me">
<title>liam.rs</title>
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

paths=("Posts" "Reviews")
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
  post_link=$(link_wrapper "${id%.*}" "$post_title" "$post_date" "$r_time" "posts")

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
# │ Reviews                                                                ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

echo "$(recent_link "Reviews")" >> ./docs/index.html

echo -ne "
  <h1>Reviews</h1>
  <div class=\"separator\"></div>
  <table>
    " >> ./docs/reviews/index.html

for f in $reviews; do
  file="./reviews/$f"
  id="${file##*/}"

  stats=$(wc "$file")
  words="$(echo "$stats" | awk '{ print $2 }')"
  lines="$(echo "$stats" | awk '{ print $1 }')"
  r_time="$(read_time "$words")"
  height="$(height "$lines")"
  review_title=$(title_wrapper "$id")

  echo "[~] $review_title"

  review_date=$(date -r "$file" "+%d/%m — %Y")
  review_link=$(link_wrapper "${id%.*}" "$review_title" "$review_date" "$r_time" "reviews")

  echo -ne "$review_link" >> ./docs/reviews/index.html

  if [[ $INDEX_REVIEW_COUNT -lt $INDEX_REVIEW_LIMIT ]]; then
    echo -ne "$review_link" >> docs/index.html
  fi

  ((INDEX_REVIEW_COUNT+=1))

  id="${id%.*}"
  mkdir -p "docs/reviews/$id"
  esh -s /bin/bash \
    -o "docs/reviews/$id/index.html" \
    "./templates/post.esh" \
    file="$file" \
    date="$review_date" \
    title="$review_title" \
    read_time="$r_time" \
    height="$height" \
    intro="$(intro)"
done

echo "$(more_link reviews)" >> ./docs/index.html

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

cat << EOF | tee -a ./docs/posts/index.html ./docs/reviews/index.html ./docs/index.html > /dev/null
  </table>
  <div class="separator"></div>
  <div class="footer">
    <a href="https://www.goodreads.com/mail" target="_blank">Books</a> ·
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
