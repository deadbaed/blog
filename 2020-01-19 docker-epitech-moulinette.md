# how to use the docker of the epitech moulinette

this guide will show you how to install docker, download the epitech moulinette container and learn how to use it for your projects.

## install

ubuntu: __`sudo apt install docker.io`__

## setup docker before first use

__`sudo usermod -aG docker $USER`__ and reboot your computer afterwards for changes to take effect.

__`sudo systemctl start docker`__ will start docker when you need it
__`sudo systemctl enable docker`__ to start docker on every boot

## make sure docker runs correctly

run __`docker run hello-world`__ to make sure that docker pulls the `hello-world` container to run on your computer.

if you get a message like "Hello from Docker!", that means it works. time to get the epitech container.

## get the epitech container

run __`docker pull epitechcontent/epitest-docker`__ to download the epitech moulinette environement. make sure to have fast internet, because the container is about 5 gigabytes.

## start the container and get a shell

go into the directory you want to get a shell in the epitech container.

run __`docker run -it --rm -v $(pwd):/home/project -w /home/project epitechcontent/epitest-docker /bin/bash`__ and you will get a bash prompt: you are now in the container. run the commands you want, and exit the shell when you are done (either with `exit` or by entering `ctrl + d`).
