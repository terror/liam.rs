#! /usr/bin/env bash

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Utilities                                                              ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

title_wrapper() {
    echo "$1" | sed -E -e "s/\..+$//g"  -e "s/_(.)/ \1/g" -e "s/^(.)/\1/g"
}

read_time() {
    echo "$(eva -f 1 $1/150 | xargs)"
}

height() {
    echo "$(eva -f 2 $1*18*0.0222 | xargs)"
}

link_wrapper() {
    echo -ne "
    <tr>
        <td class="table-post">
            <div class=\"date\">
                $3
            </div>
            <a href=\"/posts/$1\" class=\"post-link\">
                <span class=\"post-link\">$2</span>
            </a>
        </td>
        <td class="table-stats">
            <span class=\"stats-number\">
                $4
            </span>
            <span class="stats-unit">min</span>
        </td>
    </tr>
    "
}

intro() {
    echo -ne "
    <div class="intro">
        Hi.
        <div class="hot-links">
            <a href="https://liam.rs/index.xml" class="feed-button">Subscribe</a>
            <a href="https://www.buymeacoffee.com/terror" class="donate-button" target="_blank">Donate</a>
        </div>
        <p>I'm Liam, I go by terror on the internet.</p>
        <p>
        I am a second year Computer Science Student @ John Abbott College.
        When I'm not doing coursework, I enjoy bodybuilding, reading and solving programming problems.
        </p>
        <p>Send me a message by email at liam@scalzulli.com or discord at liam#0005</p>
    </div>
    "
}

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Meta & Intro                                                           ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

cat > ./docs/index.html << EOF
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
<link rel="stylesheet" href="./style.css">
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
<body>
    <div>
    <img src="https://files.catbox.moe/2jvig1.jpeg" width="100" height="100">
    <h4 class="subheading">liam's musings</h4>
    </div>
    <div class="posts">
    <div class="post">
EOF

echo -ne "$(intro)<table>" >> ./docs/index.html

# ───────────────────────────────────────────────────────────────────────────│─╗
# │ Posts                                                                  ─╬─│┼
# ╚────────────────────────────────────────────────────────────────────────────│

posts=$(ls -t ./posts)
mkdir -p docs/posts
rm -rf "./docs/posts/"

for f in $posts; do
    file="./posts/$f"
    id="${file##*/}"

    stats=$(wc "$file")
    words="$(echo $stats | awk '{ print $2 }')"
    lines="$(echo $stats | awk '{ print $1 }')"
    r_time="$(read_time $words)"
    height="$(height $lines)"
    post_title=$(title_wrapper "$id")

    echo "[~] $post_title"
    post_date=$(date -r "$file" "+%d/%m — %Y")
    post_link=$(link_wrapper "${id%.*}" "$post_title" "$post_date" "$r_time" "$height")
    echo -ne "$post_link" >> docs/index.html

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

cat >> ./docs/index.html << EOF
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
