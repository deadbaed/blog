+++
title = "archlinux how old is your installation"
date = 2019-04-18
+++

on archlinux, to see when you installed arch on your computer, run this command

```
sed -n "/ installed $1/{s/].*/]/p;q}" /var/log/pacman.log
```

it will display the date and the time when you ran `pacstrap` on the live cd to install your system.

on my laptop, i get **[2018-10-21 21:05]**, which is when i switched from fedora back to arch because my school required fedora.
