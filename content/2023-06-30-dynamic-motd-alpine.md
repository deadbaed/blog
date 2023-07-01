+++
title = "Dynamic MOTD on Alpine Linux"
date = 2023-06-30
+++

When we sign in to our server, the message of the day (MOTD) is pretty lame. Let's get something better!

This is the default MOTD of alpine:
```
Welcome to Alpine!

The Alpine Wiki contains a large amount of how-to guides and general
information about administrating Alpine systems.
See <http://wiki.alpinelinux.org>.

You can setup the system with the command: setup-alpine

You may change this message by editing /etc/motd.
```

And here's my new MOTD. I even show the WireGuard ip address:

```


  Name: intra.philt3r
  Kernel: 6.1.35-0-lts
  Distro: Alpine Linux v3.18
  Version 3.18.2

  Uptime: 0 days, 0 hours, 22 minutes
  CPU Load: 0.00, 0.00, 0.00

  Memory: 468M
  Free Memory: 217M

  Disk: 6.6G
  Free Disk: 6.6G

  eth0 Address: 192.168.1.71
  wg0 Address: 10.131.111.1


```

Start and enable cron at startup (it should be installed by default):
```sh
rc-service crond start
rc-update add crond
```

Let's run a script every 15 minutes to update the `/etc/motd` file:
```
/etc/periodic/15min/motd
```

Here's the content of my MOTD:
```sh
#!/bin/sh
#. /etc/os-release
PRETTY_NAME=`awk -F= '$1=="PRETTY_NAME" { print $2 ;}' /etc/os-release | tr -d '"'`
VERSION_ID=`awk -F= '$1=="VERSION_ID" { print $2 ;}' /etc/os-release`
UPTIME_DAYS=$(expr `cat /proc/uptime | cut -d '.' -f1` % 31556926 / 86400)
UPTIME_HOURS=$(expr `cat /proc/uptime | cut -d '.' -f1` % 31556926 % 86400 / 3600)
UPTIME_MINUTES=$(expr `cat /proc/uptime | cut -d '.' -f1` % 31556926 % 86400 % 3600 / 60)
cat > /etc/motd << EOF


  Name: `hostname`
  Kernel: `uname -r`
  Distro: $PRETTY_NAME
  Version $VERSION_ID

  Uptime: $UPTIME_DAYS days, $UPTIME_HOURS hours, $UPTIME_MINUTES minutes
  CPU Load: `cat /proc/loadavg | awk '{print $1 ", " $2 ", " $3}'`

  Memory: `free -m | head -n 2 | tail -n 1 | awk {'print  $2'}`M
  Free Memory: `free -m | head -n 2 | tail -n 1 | awk {'print $4'}`M

  Disk: `df -h / | awk  '{ a = $2 } END { print a }'`
  Free Disk: `df -h / | awk '{ a =  $2 } END { print a }'`

  eth0 Address: `ifconfig eth0 | grep "inet addr" |  awk -F: '{print $2}' | awk '{print $1}'`
  wg0 Address: `ifconfig wg0 | grep "inet addr" |  awk -F: '{print $2}' | awk '{print $1}'`


EOF
```

Make the script executable, and check if it's good:
```sh
chmod a+x /etc/periodic/15min/motd
run-parts --test /etc/periodic/15min
```

If you're lazy and don't want to wait 15 minutes, run the script directly:
```sh
/etc/periodic/15min/motd
```

Log out and log back in, you should see the new MOTD!

# Resources

[https://kingtam.win/archives/apline-custom.html](https://kingtam.win/archives/apline-custom.html)

I just copy/pasted and changed the MOTD.
