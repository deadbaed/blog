---
layout: post
title:  "error on zsh when using vim and autocomplete"
date:   2018-03-13
---

# error on zsh when using vim and autocomplete

do you use __zsh__ and __ohmyzsh__ ?

do you run into an issue when you are about to edit a file with vim, and you use the __Tab__ key to autocomplete the filename, but instead you get something like this:
```
$ vim ~/filena<TAB>
_arguments:448: _vim_files: function definition file not found
```

annoying af, right ?

## how to fix it

here's how to fix it: delete the zcompdump directory off your personal directory, and reload your zsh config file (or close and open a new shell).

`rm -rf ~/.zcompdump*; source ~/.zshrc;`

it took me at least 15 mins to find that .... hopefully i save some time for you .

### sources

[stackoverflow](https://unix.stackexchange.com/questions/280622/zsh-fails-at-path-completition-when-command-is-vim#280626) and [github](https://github.com/robbyrussell/oh-my-zsh/issues/518)
