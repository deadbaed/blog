+++
title = "Automount a Hetzner Storage Box with sshfs on NixOS"
date = 2024-08-10
+++

I had my eyes on some nice arm64 servers from Hetzner, and I finally pulled the trigger, I got the `CAX21`.

It is also the opportunity to reduce time of maintaining my infrastructure, I will use NixOS to setup my server.
By having a couple of configuration files, it will be easier to review, edit and update the system.

But time will tell if it is the good decision, and not sticking to a imperative distribution such as Debian.

# Not a lot of storage

The only downside with these servers is the storage -- I only have 80 gigabytes of storage on mine.
Fortunately, Hetzner has their **Storage Box** offerings, I picked up a `BX11` which has 1 terabyte of storage!

The plan is to mount the storage box as a regular drive and have applications use it normally.
The main applications will be documents, media, backups -- not speed critical data such as databases or logs.

# Storage Box ordering and setup

Start by ordering your Storage Box, I think mine took less than an hour to be provisioned and delivered to me.

On the configuration panel, I only ticked `SSH support` and the disabled the rest.
If you will use the storage box outside of the Hetzner network, enable `External reachability`.

Finally, you cannot set the password yourself, you will have to reset it.

# SSH keys

On the server, generate a new ssh key with `ssh-keygen` which will be used to connect to the storage box.

Since I want the storage box to be mounted automatically on startup, so I did not set a passphrase on the key.

Copy the ssh to the storage box with:
```shell
ssh-copy-id -p 23 -s user@storagebox.example.org
```

More documentation on ssh keys with storage box: <https://docs.hetzner.com/robot/storage-box/backup-space-ssh-keys>

# NixOS configuration

The easy part, and the reason why I think I will like to use NixOS on my server:

You can put it inside your `configuration.nix` directly, I placed it inside its own file.

```nix
{ ... }:

{
  fileSystems."/mnt/storagebox" = {
    device = "user@storagebox.example.org:/some/path";
    fsType = "fuse.sshfs";
    options = [
      "identityfile=/place/to/ssh/key/somewhere"
      "idmap=user"
      "x-systemd.automount" # mount the filesystem automatically on first access
      "allow_other" # don't restrict access to only the user which `mount`s it (because that's probably systemd who mounts it, not you)
      "user" # allow manual `mount`ing, as ordinary user.
      "_netdev"
    ];
  };
  boot.supportedFilesystems."fuse.sshfs" = true;
}
```

Thank you so much to [this Discourse post](https://discourse.nixos.org/t/how-to-auto-mount-with-sshfs-as-a-normal-user/48276/3?u=philt3r) for the configuration snippet!

Run
```shell
nixos-rebuild switch && cd /mnt/storagebox
```
and you are able to read and write files!
