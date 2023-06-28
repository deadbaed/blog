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
        allow net 10.131.110.0/24 10.131.111.0/24
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

Since we want to have our DNS only inside WireGuard, we want WireGuard to start only after CoreDNS has started. To do so, edit
```
/etc/init.d/wg
```
and update this section:
```patch
--- /etc/init.d/wg
+++ /etc/init.d/wg
@@ -4,7 +4,7 @@
 description="WireGuard"

 depend() {
-    need localmount net sysctl
+    need localmount net sysctl coredns
     after bootmisc
 }
```

And reboot! Or restart the WireGuard service.

The logs of CoreDNS should be available at
```
/var/log/coredns/coredns.log
```
