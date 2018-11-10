# lolcommits

I just came across a small tool named
[__lolcommits__](https://github.com/mroth/lolcommits) and i was like
__lol what a name__, so i started to look at the github page, to see that
this tool takes a pic of your webcam everytime you make a git commit!

This is the best idea ever! The first thing i thought when i knew what it does
was __ohmy god i can make timelapses of me coding now__ (_cause i love making
timelapses of everything i do_)!  
...

But it turns out that [__people have been doing that already__](https://www.youtube.com/watch?v=mdzPI7Id840)
so i gotta catch up haha

## how it works

_very quickly_: The way it works is you install ruby and some other programs
then you install the __gem__ for the program, you run `lolcommits --enable`
which is going to create a file `.git/hooks/post-commit`.  
From there you can edit options like to make a gif or to delay the process
(if your webcam takes time to warm up) or some other options, everything goes
through the `post-commit` hook.

By default, when you make a commit, the picture will be stored in
`~/.lolcommits`, under the folder of the git project.

That tool is so well-made that there is even plugins to the program!
That's awesome

## example

im pretty sure you want an example, so [__here it is__](/img/lolcommits/0adb5c36e3a.jpg)!  
fyi, it was when i was working on the blog, here's the [__actual commit__](https://github.com/ploctaux/deadbaed/commit/0adb5c36e3acf448c52c3f5cdedef608fc881290) on github.

