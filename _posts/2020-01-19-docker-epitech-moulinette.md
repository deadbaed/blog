---
layout: post
title:  "how to use the docker of the epitech moulinette"
date:   2020-01-19
---

# how to use the docker of the epitech moulinette

this guide will show you how to install docker, download the epitech moulinette container and learn how to use it for your projects.

## install

- ubuntu: `sudo apt install docker.io`
- arch: `sudo pacman -S docker`

## setup docker before first use

use docker without root privileges with `sudo usermod -aG docker $USER` and **REBOOT** your computer afterwards for changes to take effect.

- `sudo systemctl start docker` and `sudo systemctl stop docker` will start and stop docker when you need it
- `sudo systemctl enable docker` to start docker on every boot

## make sure docker runs correctly

run `docker run hello-world` to make sure that docker pulls the `hello-world` container to run on your computer.

if you get a message like "Hello from Docker!", that means it works. time to get the epitech container.

## get the epitech container

run `docker pull epitechcontent/epitest-docker` to download the epitech moulinette environement. make sure to have fast internet, because the container is about 5 gigabytes.

## start the container and get a shell

go into the directory you want to get a shell in the epitech container.

run `docker run -it --rm -v $(pwd):/home/project -w /home/project epitechcontent/epitest-docker /bin/bash` and you will get a bash prompt: you are now in the container. run the commands you want, and exit the shell when you are done (either with `exit` or by entering `ctrl + d`).

if you are using docker on windows (inside powershell), run `docker run -it --rm -v ${pwd}:/home/project -w /home/project epitechcontent/epitest-docker /bin/bash` 
