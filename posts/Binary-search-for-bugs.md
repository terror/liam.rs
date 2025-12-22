I recently encountered a build failure in a dependency of a Rust crate I was trying to install which led me to
be introduced to a git command called `bisect`.

From the official docs found over at [git-scm.com](https://git-scm.com/docs/git-bisect){target="_blank"}, the small description under the name heading
reads:

> Use binary search to find the commit that introduced a bug

Sounds super cool right? Using this feature I was able to track down the commit that introduced the bug in less than 6 steps.

Here's a general overview of how this command works:

1. Run `git bisect start` to start the bisect session

2. Specify a bad commit (usually HEAD) using `git bisect bad <commit-id>`

3. Specify a good commit using `git bisect good <commit-id>`

This will kick off the binary search and pick a new commit in which you can evaluate using `git bisect good` or `git bisect bad`, in between the two endpoints.

It is clear that using this method, over the linear scan approach, can save a tremendous amount of time in the quest
for finding where a breaking change was introduced.
