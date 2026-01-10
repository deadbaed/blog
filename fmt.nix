{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  treefmt-nix ? import sources.treefmt-nix,
}:

treefmt-nix.mkWrapper pkgs {
  projectRootFile = ".git/config";

  programs.nixfmt = {
    enable = true;
    excludes = [ "Cargo.nix" ];
  };

  programs.rustfmt.enable = true;
}
