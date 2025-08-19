+++
title = "how to use the docker of the epitech moulinette"
date = 2020-01-19T00:00:00-00[UTC]
uuid = "f26b5c26-f736-4650-85c0-4aed139abbe4"
description = "Basic tutorial to use Docker for epitech students"
+++

this guide will show you how to install docker, download the epitech moulinette container and learn how to use it for your projects.

## install

ubuntu:
```
sudo apt install docker.io
```

arch:
```
sudo pacman -S docker
```

## setup docker before first use

to use docker without root privileges run
```
sudo usermod -aG docker $USER
```
and **REBOOT** your computer afterwards for changes to take effect.

to start docker on every boot
```
sudo systemctl enable docker
```

## get the epitech container

```
docker pull epitechcontent/epitest-docker
```
will download the epitech moulinette environement. make sure to have fast internet, because the container is about 5 gigabytes.

## start the container and get a shell

go into the directory you want to get a shell in the epitech container.

```
docker run -it --rm -v $(pwd):/home/project -w /home/project epitechcontent/epitest-docker /bin/bash
```
will get you a bash prompt: you are now in the container. run the commands you want, and exit the shell when you are done.

if you are using docker on windows (inside powershell), run
```
docker run -it --rm -v ${pwd}:/home/project -w /home/project epitechcontent/epitest-docker /bin/bash
```

