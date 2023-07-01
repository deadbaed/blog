+++
title = "Setup CoreDNS on Alpine Linux"
date = 2023-06-25
+++

Now that we have a WireGuard VPN, let's add a DNS server, to type letters instead of numbers!

# Install CoreDNS

You will need to enable the `community` repo first.
```sh
doas apk add coredns
```

# Configuration

Create the config in
```sh
/etc/coredns/Corefile
```

```
# snippets
(common) {
    cache 60
    acl {
        allow net 127.0.0.1 10.131.110.0/24 10.131.111.0/24
        block
    }
}

# intranet
philt3r {
    import common
    log . {combined} {
        class denial error success
    }

    hosts {
        10.131.111.1 intra.philt3r
        falltrough
    }
}

# extranet
. {
    import common

    # Free DNS
    forward . 212.27.40.240 212.27.40.241
}
```

My DNS service of choice comes from [free.fr](https://free.fr). Feel free to put your own favorite DNS service!

# Script to launch on server startup

CoreDNS already has a service!

- Add at startup: `rc-update add coredns`
- Remove from startup: `rc-update del coredns`
- Show services at startup: `rc-status`

The logs of CoreDNS should be available at
```
/var/log/coredns/coredns.log
```

# Use CoreDNS on the system

Now that we have our DNS server, let's use it on our server!

If you use DHCP to get the ip address of your server, the DNS will always be used from the DHCP.

We want to use our own DHCP server.

Create the file (and the folder associated with it)
```
/etc/udhcpc/udhcpc.conf
```
and put
```
RESOLV_CONF="NO"
```

Then, edit the
```
/etc/resolv.conf
```
and put
```
nameserver 127.0.0.1
```

Restart the server.
