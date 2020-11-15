I have recently gotten into the habit of reading more blogs, in particular, ones featured
on [HackerNews](https://news.ycombinator.com/news). 

Reading books, personally, has been a difficult habit to cultivate
over time, which I mainly attribute to the periodic feelings of being "stuck" or bored on a certain set of books. Blogs, however, offer a condensed read making them efficient and often amusing sources of information.
 
My typical sourcing routine would usually just consist of scrolling through HackerNews, potentially
seeing something that peaked my interest and reading it, or realizing I didn't have the time and putting the url off in a `.txt` file somewhere.

Realizing the severe lack of efficiency present in this routine, I thought about how to make it better. First, by asking the question: what were the things I didn't like?

1. Having to scroll through articles myself
2. Reading on mobile or desktop
3. Having no effortless `like` or `archive` feature

To address these problems I chose to try out [`Instapaper`](https://www.instapaper.com/) and build a simple tool called [`hackerpaper`](https://github.com/terror/hackerpaper).

Instapaper has a feature that let's you send articles to your Kindle, which makes reading blogs much more comfortable. Moreover, the built in `like` and `archive` features carryover to the Kindle as well, making it a seamless process of `read` -> `archive or like` -> `repeat`.

To solve the issue of having to scroll through articles myself and add them to Instapaper, as mentioned above, I built `hackerpaper`. This tool sends HackerNews articles to Instapaper with certain filters such as type, subdomains or a specific article id.

I run a `cron job` that sends the articles to Instapaper at 5:00AM and have Instapaper send those articles to my Kindle at 7:00AM.

The script is as simple as:
```bash
$ ~/path/to/python3 ~/path/to/hackerpaper -a <email> <password> -t topstories
```

To recap the routine:  
1. Have `hackerpaper` send articles to Instapaper  
2. Have `Instapaper` send those articles to Kindle   
3. Read and archive those articles  
4. Repeat

