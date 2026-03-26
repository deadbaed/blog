+++
title = "Run Nix on Forgejo Actions on NixOS"
date = 2026-03-26T18:47:13+01:00[Europe/Paris]
uuid = "4df7718e-c073-4513-9176-2b359593c0e1"
description = "Build your favorite Nix code with Forgejo Actions on a NixOS runner"
+++

I will start by assuming that you have a Forgejo instance running, and you know what is Forgejo Actions. If that is not the case, I invite you to read the [Forgejo User documentation](https://forgejo.org/docs/latest/user/actions/overview/) and the [Forgejo Admin documentation](https://forgejo.org/docs/latest/admin/actions/). They are a good read!

Please note that NixOS is not required to have a Forgejo Actions runner building nix code. I deploy my runner with NixOS because it is so damn simple to run and maintain.

## Setup a runner

### Registration token

Let's say that your forgejo instance is available at https://forgejo.example.net.

With an administrator account, go to https://forgejo.example.net/admin/actions/runners and get a registration token.

Get the token, and be ready to put it when needed.

If you put the registration token directly in your configuration file, it might end up in the nix store, which is publicly readable by any user on the machine. To prevent this you can encrypt it, I recommend using [agenix](https://github.com/ryantm/agenix) for that. It uses SSH keys for the encryption and it works great.

### OCI containers and Forgejo labels

The runner will need to have a label to know which jobs it can pick up. There are many labels [from which you can choose from](https://forgejo.org/docs/latest/admin/actions/#choosing-labels) but for our usecase the `docker` label is enough.

I prefer to use [Podman](https://podman.io) over Docker, but using Docker is just fine.

In the past I used the excellent guide from [Robin Appelman](https://icewind.nl/entry/gitea-actions-nix/) which creates a container image with Nix pre-installed. But it required flakes, and every once in a while I had to update the flake's inputs, rebuild the image, and load it to the container tool.

I want something that is simpler, and requires less maintenance. Fortunately there are pre-built container images (which already work with existing GitHub workflows) for [act](https://github.com/nektos/act) (the Forgejo actions runner is based on act). A popular option is <https://github.com/catthehacker/docker_images>, and an alternative could be <https://github.com/tcpipuk/act-runner>.

I will name the images like they are GitHub Actions to keep things similar.

```nix
{ config, lib, pkgs, ... }:

{
  virtualisation.containers.enable = true;
  virtualisation.oci-containers.backend = "podman";
  virtualisation = {
    podman = {
      enable = true;
      dockerCompat = true;
    };
  };

  services.gitea-actions-runner = {
    package = pkgs.forgejo-runner;
    instances.forgejo-runner = {
      enable = true;
      name = "my-forgejo-runner"; # TODO: put a cute name for your runner
      token = "put-token-here"; # TODO: use agenix
      url = "https://forgejo.example.net/"; # TODO: put the link to your instance
      labels = [
        "node-24:docker://node:24-trixie"
        "ubuntu-24.04:docker://ghcr.io/catthehacker/ubuntu:runner-24.04"
        "ubuntu-latest:docker://ghcr.io/catthehacker/ubuntu:runner-latest"
      ];
      settings = {
        container.network = "host";
      };
    };
  };
}
```

You are done configuring the runner! Rebuild the NixOS machine and you should see the runner appear in https://forgejo.example.net/admin/actions/runners.

## Write your workflow file

To make sure everything works, create a test git repo on Forgejo with a simple `default.nix` or `shell.nix`, and create a file in `.forgejo/workflows/ci.yml`.

To install Nix, simply use the [install-nix-action](https://github.com/cachix/install-nix-action) from [cachix](https://www.cachix.org). If you use [npins](https://github.com/andir/npins), you can even set the path to nixpkgs directly to save time:

```yaml
name: CI
on:
  push:
jobs:
  my_nix_job:
    runs-on: ubuntu-latest
    steps:
      - uses: https://data.forgejo.org/actions/checkout@v6
      - uses: https://github.com/cachix/install-nix-action@v31
      - name: Get Nixpkgs path
        run: |
          nixpkgs_path=$(nix eval --raw -f npins/default.nix nixpkgs)
          echo "NIX_PATH=nixpkgs=$nixpkgs_path" >> "$FORGEJO_ENV"
      - run: nix-build ./default.nix
```

Commit and push the file, and you should have the job being picked on https://forgejo.example.net/user/repo/actions!
