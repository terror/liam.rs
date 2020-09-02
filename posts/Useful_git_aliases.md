I really enjoy using `aliases` for git commands because a) I'm lazy
and b) it makes me feel more productive.

Some basic ones I use

```
alias ga="git add"
alias gc="git commit"
alias gp="git pull"
alias gd="git diff"
alias gch="git checkout"
alias gst="git status"
alias gp="git push -u origin master"
alias gpo="git push origin"
alias gb="git branch"
alias gcb="git checkout -b"

```

These ones are set up in my `.bashrc` file so that I can just execute them as
is, i.e `gc` -> `git commit`.

Some more complex ones (except for fetch) in my `.gitconfig` I enjoy:

```
f = fetch
fu = fetch upstream master
track = branch --set-upstream-to=upstream/master master
parent = "!git show-branch | grep '*' | grep -v \"$(git rev-parse --abbrev-ref HEAD)\" | head -n1 | sed 's/.*\\[\\(.*\\)\\].*/\\1/' | sed 's/[\\^~].*//' #"
lg = log --oneline
last = log -1 HEAD
```  

`fu` and `track` are useful for doing things when I'm working on a forked repository

`parent` shows the parent branch of the current working branch

`lg` shows commit history one line at a time

`last` shows information for the last commit 

Whether they make me more productive or not, I just enjoy using them and see no
reason not to.
