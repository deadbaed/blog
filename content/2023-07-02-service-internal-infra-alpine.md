+++
title = "Setup a service on our internal infrastructure on Alpine Linux"
date = 2023-07-02
+++

Now we have a basic internal infrastructure with:
- Everything hidden and encrypted through the network (WireGuard)
- Pretty internal domain names instead of raw ip addresses (CoreDNS)
- A basic http server just in case (Caddy)
- Our own TLS certificates that are easy to get (Step CA)

But everything is on the same machine. While it could be okay, I will host the services I want on other machines.

I will run them on the same Proxmox cluster, but the possibilities are endless (as long you can get WireGuard running).

# Get started

Install Alpine. Setup ssh and repositories.

# WireGuard

We will set up WireGuard, but not a server, a regular peer that will connect to the WireGuard server.

Create a new peer on the WireGuard server, and copy the config file to the new peer.

## Install

Install WireGuard:
```sh
apk add wireguard-tools
```

To load the WireGuard module on startup, edit
```
/etc/modules
```

and simply add
```
wireguard
```
and reboot.

## Configure

Copy the config file to
```
/etc/wireguard/wg0.conf
```

## Start

Copy the `init.d` script for WireGuard like we did for the original server.

And ask it to start on boot.

Reboot and make sure everything works, you should see WireGuard logs when the machine is starting.

And the DNS should be working! Try to ping an internal DNS name.

Sometimes the DNS will go back to the system's default (probably your DHCP server's), so force the DNS as seen in the post about CoreDNS.

# DNS entry

In the main server, edit CoreDNS to add a new DNS entry for the newly added peer.

Save and restart CoreDNS.

# MOTD

Add the dynamic MOTD if you feel like it. I did.

# Reverse proxy

Before installing and starting services, let's add a reverse proxy for security + some sweet TLS certs.

I'll be using caddy. You will need to enable the `community` repo first.
```sh
apk add caddy
```

Let's get a hello world:
```sh
/etc/caddy/Caddyfile
```
```
# global
{
        # step-ca ACME server
        acme_ca https://10.131.111.1:444/acme/acme/directory
}

docker.philt3r docker.philt3r:80 {
    respond "Hello, world!"
}
```

I start the service on ports `80` and `443` to get the initial TLS certificate, I will remove access on port `80` afterward.

Don't start caddy yet.

# TLS certificates

On our new server, we need to trust the root ca. Download the root ca, and ask the system to trust it:
```sh
wget --no-check-certificate https://10.131.111.1:444/roots.pem -O /usr/local/share/ca-certificates/philt3r.crt
update-ca-certificates 
```

Now we can start caddy and enable it on boot:
```sh
rc-service caddy start
rc-update add caddy
```

You should get a Hello World on port 443. If you do, you can disable access from port `80` in the Caddyfile and restart caddy.

# Install the service

Now we can install the service we want to host, start it, and configure caddy to be a reverse proxy for it.

Repeat the process for the other services you want to host.

Protip: serve the services on `127.0.0.1` and use caddy to restrict access only from the WireGuard peers (since there is the DNS restriction).

Sample `Caddyfile`:
```
# global
{
        # step-ca ACME server
        acme_ca https://10.131.111.1:444/acme/acme/directory
}

docker.philt3r {
        reverse_proxy 127.0.0.1:3000
}
```

# Docker

Since I'll be using Docker to host most services, I'll install it:
```sh
apk add docker
rc-update add docker
rc-service docker start
```

Then spin up your docker containers and route them with caddy.