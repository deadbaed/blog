+++
title = "Have unfree packages inside a nix shell"
date = 2026-11-06T17:54:17+01:00[Europe/Paris]
uuid = "cdc3ad80-ba68-4ff1-a027-1fe19e4fc0a7"
description = "Fine-grained nix shells with unfree packages"
+++

In your `configuration.nix` for nix-darwin or NixOS you can do this to allow [unfree software](https://wiki.nixos.org/wiki/Unfree_software):

```nix
{
  config,
  pkgs,
  lib,
  ...
}:

{
  nixpkgs.config.allowUnfreePredicate =
    pkg:
    builtins.elem (lib.getName pkg) [
      "steam"
      "steam-original"
      "steam-unwrapped"
      "steam-run"
      "steamcmd"
    ];
}
```

But what about in a `shell.nix` used for an individual project?

You will need to use [`lib.getName`](https://noogle.dev/f/lib/getName) to define the list of packages to allow. Since that requires an instance of `nixpkgs`, instanciate one just to use that function from the lib.

Store the predicate in an attrset, and instanciate a `nixpkgs` with the attrset as its configuration. Use that instance as usual:

```nix
{
  pkgs ? import &lt;nixpkgs&gt; { },
}:

let
  config = {
    allowUnfreePredicate = 
      pkg:
      builtins.elem (pkgs.lib.getName pkg) [
        "packer"
        "tart"
      ];
  };
  pkgs = import &lt;nixpkgs&gt; { inherit config; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    packer
    tart
  ];
}
```
