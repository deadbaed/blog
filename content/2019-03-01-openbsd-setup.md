+++
title = "openbsd first setup after install"
date = 2019-03-01
+++

just installed openbsd on my chromebook, seems to be working fine!

since it's my first time using openbsd, here are some stuff that i will start to do on my machines after installing openbsd:

## enable `doas`

`doas` is kinda the equivalent of `sudo`. to enable it, run `cp /etc/examples/doas.conf /etc` to copy the doas config file.

## disable root account

now that `doas` is ready, we dont need to root account anymore. to disable it, run `usermod -p'*' root` to set the root password to `*`. this will prevent root from log on directly to the machine (with `su` as an example), but with `doas` we can run `doas sh` to get a shell.

## install missing firmware for your hardware

maybe your wifi card isn't working? or maybe you can't display any graphical interface? maybe that's because you don't have the firmware for it: here's how to install it:

run `doas fw_update -i` to see the missing firmwares.

so grab a flash drive, format it in *fat* filesystem format, go to [firmware.openbsd.org](http://firmware.openbsd.org/firmware/), download the missing firmwares, along with the **SHA256.sig** and **index.txt** files, and put them on the usb key.

mount the flash drive on openbsd, and run `doas fw_update -p *path of flash drive*` to install the firmwares from the flash drive.

your missing firmware should not be anymore.
