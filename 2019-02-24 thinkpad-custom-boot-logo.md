# how to put a custom boot logo on a thinkpad

> *WARNING*
>
> i need to warn you that you are on your own. even though it works fine, i'm not responsible of what you do.

you have a thinkpad. it's beautiful, it's fast, it's perfect, it doesn't run windows; there's just one thing that could be perfect: the boot logo.

by default on new models, your boot logo look like this:

![](/img/thinkpad-custom-boot-logo/default-logo.jpg)

to get a custom boot logo, you need:

* an internet connection
* a compatible model
* a usb flash drive
* a gif image (you can also use a *bmp* or a *jpg* image, but i've found that with *gif* images it works better)

## get the BIOS update

to install your custom boot logo, you need to flash a BIOS update.

to download the BIOS update, go on [lenovo's support website](https://pcsupport.lenovo.com/us/en), choose **drivers and updates** and find your model.

next, go in the section **BIOS/UEFI** and download the **BIOS Update (Bootable CD)**. it will download a *iso* image.

if you can't find the update image, that means that you're out of luck, and you can't get a custom boot logo. sorry.

## convert the iso image

now that you have the iso image, you need to convert it to a *img* file. to do so, run the following command in a terminal:

`geteltorito -o bios-image.img bios-image-downloaded.iso`

if you don't have **geteltorito**, look online to install it.

## flash the image on your usb drive

now it's time to flash the image on your usb drive. get the name of your flash drive using `lsblk`, plug your usb drive, and run `lsblk` again to see your drive.

go into the folder where the *img* file is located, and run in a terminal:

`sudo dd if=bios-image.img of=/dev/sdX bs=1M status=progress oflag=sync`

where **X** is your drive letter that you know thanks to `lsblk`.

## get the *gif* file to use

if you want, i already have a [selection of images](/img/thinkpad-custom-boot-logo/boot-logo/) ready to be used, or you can make your own!

you can see the requirements, go in your usb drive, open the **readme.txt** in the *flash* folder.

put the *gif* image in the *flash* folder, and name it **LOGO1.JPG**. copy that image, and name that one **LOGO2.JPG**.

check the **readme.txt** file to see the filenames, they might differ on different models.

## flash the BIOS update

reboot your computer, and boot on the usb flash drive. if you don't know how, the internet should help you with that.

now that the flash utility has booted, choose the second option, and follow the instructions.

the computer will reboot, flash the update, and when it will reboot, you should get your custom boot logo!

![](/img/thinkpad-custom-boot-logo/custom-logo.jpg)

if you want to go back to the default logo, simply reflash the bios update, when when asked if you want to use your custom logo, say no, and the default logo will be put back.

let me know if it worked!

-phil
