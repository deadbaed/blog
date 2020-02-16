---
layout: post
title:  "generate a quick password"
date:   2019-01-12
---

# generate a quick password


> protip: if you have an encrypted partition it's even better

you need **sudo** or **root privileges** for this to work

`sudo cat /dev/sda1 | date +%s | head -c 32 | sha512sum | base64 | head -c 32`

-phil
