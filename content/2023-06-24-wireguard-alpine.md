+++
title = "Setup WireGuard server on Alpine Linux"
date = 2023-06-24
+++

Let's do this baremetal, no Docker!

I will do this inside a [Proxmox](https://www.proxmox.com/en/) virtual machine.

# Get started

Start by installing [Alpine Linux](https://www.alpinelinux.org/): Run the installer, next, next, next, and boot the os once it's done.

# Setup ssh

Copy ssh key (run this on your local machine):
```sh
ssh-copy-id -i ~/.ssh/id_rsa.pub user@ip
```

Login via ssh, and install your favorite editor:
```sh
doas apk add vim
```

Edit ssh config to force ssh key use:
```sh
doas vim /etc/ssh/sshd_config
```

Find and update these statements:
```
PermitRootLogin no
PubkeyAuthentication yes
```

Restart ssh service, logout, and log back in
```sh
doas rc-service sshd restart
```

# Setup alpine package manager

I use `mirrors.ircam.fr` as my mirror

Open 
```sh
/etc/apk/repositories
```

add the community repo, and run updates:
```sh
doas apk -U upgrade
```

# WireGuard basics

Install WireGuard:
```sh
doas apk add wireguard-tools
```

## Kernel module

Load the module
```sh
doas modprobe wireguard
```

To launch the module on startup, edit
```
/etc/modules
```

and simply add
```
wireguard
```

at the bottom, and save the file.

## IP forwarding

Edit
```
/etc/sysctl.conf
```

and add
```sh
net.ipv4.ip_forward = 1
```
at the bottom of the file, and save

Launch sysctl on startup with
```sh
doas rc-update add sysctl
```
and reboot.

# IP Addresses

Pick a range if ip addresses to use: [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918)

I'll pick `10.131.111.x` for the WireGuard peers.

Calculate your CIDR: [https://www.ipaddressguide.com/cidr](https://www.ipaddressguide.com/cidr)

Here's my network layout:

- CIDR: `10.131.110.0/23`
- Start: `10.131.110.0`
- End: `10.131.111.255`

Network services:
- Start: `10.131.110.0/24`
- End: `10.131.110.255/24`

WireGuard:
- Start: `10.131.111.0/24`
- End: `10.131.111.255/24`

# Generate keys for WireGuard

Do everything as root (doas is the equivalent of sudo):
```sh
doas su
```

Move to the wireguard configuration, I'll store everything there for easy access:
```sh
cd /etc/wireguard/
```

Generate the private and public key, store them in files (we'll use them later):
```sh
wg genkey | tee philt3r-privatekey | wg pubkey > philt3r-publickey
```

# Configure server interface

All the server configuration will happen in
```
/etc/wireguard/wg0.conf
```

Protip for vim users: To add content of a file in current buffer directly: [StackOverflow answer](https://stackoverflow.com/a/19087947/4809297)

```ini
[Interface]
# Name = wg0
Address = 10.131.111.1/24
ListenPort = 51820
PrivateKey = <server-private-key>
PostUp = iptables -t nat -A POSTROUTING -s 10.131.111.0/24 -o %i -j MASQUERADE;
PostUp = iptables -t nat -A POSTROUTING -s 10.131.110.0/24 -o %i -j MASQUERADE;
PostUp = iptables -A INPUT -p udp -m udp --dport 51820 -j ACCEPT;
PostUp = iptables -A FORWARD -i %i -j ACCEPT;
PostUp = iptables -A FORWARD -o %i -j ACCEPT;
PostDown = iptables -t nat -D POSTROUTING -s 10.131.111.0/24 -o %i -j MASQUERADE;
PostDown = iptables -t nat -D POSTROUTING -s 10.131.110.0/24 -o %i -j MASQUERADE;
PostDown = iptables -D INPUT -p udp -m udp --dport 51820 -j ACCEPT;
PostDown = iptables -D FORWARD -i %i -j ACCEPT;
PostDown = iptables -D FORWARD -o %i -j ACCEPT;
```

Once it's good, make sure only root can read and write to the files:
```sh
chmod 600 /etc/wireguard/*
```

# Add new peer

You will need to repeat this for each new peer

```sh
cd /etc/wireguard/
```

## Generate keys

Starting now, `name` is a placeholder for the name of the peer.

I typically use the format **name-of-person** followed by **device-name**. For example, the peer for my phone will be **phil-iphone**.

Create folder to store keys for the peer:
```sh
mkdir -p peers/name
```

Generate preshared key (not required):
```sh
wg genpsk | tee peers/name/preshared.psk
```

Generate private and public keys for the peer:
```sh
wg genkey | tee peers/name/private.key | wg pubkey > peers/name/public.key
```

## Update server configuration

Edit your `wg0.conf`, add at the bottom:

```ini
[Peer]
# Name = name
PublicKey = <peers/name/public.key>
PresharedKey = <peers/name/preshared.psk>
AllowedIPs = 10.131.111.2/32
AllowedIPs = 10.131.110.0/24
AllowedIPs = 10.131.111.0/24
```

## Peer configuration

Now let's create the configuration to give to the peer:

Create the file
```
peers/name/philt3r-name.wg.conf
```

And put the following
```ini
[Interface]
PrivateKey = <peers/name/private.key>
Address = 10.131.111.2/24
#DNS = 10.131.111.1

[Peer]
PublicKey = <server-public-key>
PresharedKey = <peers/name/preshared.psk>
Endpoint = <server-ip>:51820
AllowedIPs = 10.131.110.0/24
AllowedIPS = 10.131.111.0/24
PersistentKeepalive = 25
```

DNS info is not used yet, it's normal, I will enable it once my DNS server will be created (not in this blog post though).

## Distribute config

Either give the configuration file we just created, or make a qr code:

```sh
apk add libqrencode
```

And run 
```sh
qrencode -t ansiutf8 < peers/name/philt3r-name.wg.conf
```

## Restart WireGuard

If you already have WireGuard running, simply run
```
rc-service wg restart
```
to restart the server with your new peer.

# Start WireGuard manually

Make sure to open the port on your router in **UDP** mode! I spent a lot of time debugging to realize that my port was in TCP, double check!

Make sure to be root before, don't use `doas` or `sudo`!
```sh
wg-quick up wg0
```

On the peer, start the tunnel.

On the server, run
```sh
wg
```
to check the status of WireGuard. You should see the peer and some stats it is connected.

If you do not see info about the peer even if it is not connected, that means you did something wrong in the configuration!

From your peer, you should be able to ping the WireGuard internal IP:
```
10.131.111.1
```

- iOS: [Ping](https://apps.apple.com/fr/app/ping-network-utility/id576773404)
- OSX / Linux: `ping`

If you can ping the ip, you're good!

You may not be able to go on the internet, or even make DNS requests, it's normal.

We are just testing if the tunnel works. You can stop the tunnel.

# Stop WireGuard manually

```sh
wg-quick down wg0
```

# Script to launch on server startup

To start WireGuard on startup, we will write an OpenRC script. It will be located in
```
/etc/init.d/wg
```

Put the following:
```sh
#!/sbin/openrc-run
#

description="WireGuard"

depend() {
    need localmount net sysctl
    after bootmisc
}

start() {
    ebegin "Starting WireGuard"
    wg-quick up wg0
    eend $?
}

stop() {
    ebegin "Stopping WireGuard"
    wg-quick down wg0
    eend $?
}

status() {
    wg show wg0
}
```

Give it executable access
```sh
chmod +x /etc/init.d/wg
```

## Manual

- Start: `rc-service wg start`
- Stop: `rc-service wg stop`
- Restart: `rc-service wg restart`
- Status: `rc-service wg status`

## Startup
- Add at startup: `rc-update add wg`
- Remove from startup: `rc-update del wg`
- Show services at startup: `rc-status`

Reboot and make sure everything works, you should see WireGuard logs when your server is starting.

# Resources

These resources helped me when setting up my WireGuard server. Thanks!

- [https://github.com/pirate/wireguard-docs](https://github.com/pirate/wireguard-docs)
- [https://blog.ruanbekker.com/blog/2020/01/11/setup-a-wireguard-vpn-server-on-linux/](https://blog.ruanbekker.com/blog/2020/01/11/setup-a-wireguard-vpn-server-on-linux/)
- [https://try.popho.be/wg.html](https://try.popho.be/wg.html)

