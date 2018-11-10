# grub, how to fix nvidia issue for epitech student 2023

to everyone who got a computer with an nvidia card, ___YOU SUCK___!  
just kidding, here's how to fix a bug with grub to disable _nouveau_ (the open source graphics driver) and use the proprietary nvidia driver.

# howto

first, turn on your computer and boot to fedora.  
next, open a terminal and run the command: `sudo nano /etc/default/grub`, and enter your root password.  

you should get something similar to [this image](/img/grub/nano.png).

go to the end of the line starting with `GRUB_CMDLINE_LINUX`, and just before the last double quote `"`, add this: __`nouveau.modeset=0`__.

at the end, you should get something like [this image](/img/grub/nano-after.png).

if you do (_and you should_) get the result like the image, hit __ctrl + x__ to exit and save the file, confirm with `Y` and hit __enter__ when you see the filename.  
nano should exit, and you should get your prompt back.

next, we need to regenerate grub's configuration file. to do that, run the command `sudo grub2-mkconfig -o /boot/efi/EFI/fedora/grub.cfg`.  
__grub2-mkconfig__ will do its job and will generate your configuration files for your different kernels. at the end you _should_ see `done`, something like in [this image](/img/grub/mkconfig.png).

if you do, __congrats!__, your grub is now fixed, and you can reboot your computer and you will be able to boot without having to change anything before starting up your computer.

if it still not working, well i can't do much for you, you're on your own. but it should work, if you followed all the instructions correctly.

hit me up if you need help.

-phil
