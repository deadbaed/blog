{pkgs}: let
  blog-dev-server = pkgs.writeShellScriptBin "blog-dev-server" ''
    python3 -m http.server --directory ./target/blog 4343
  '';
  blog-dev-compile = pkgs.writeShellScriptBin "blog-dev-compile" ''
    cargo watch -s "cargo run"
  '';
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs;
      [
        # rust toolchain
        rustc
        cargo
        rustfmt
        clippy
        rust-analyzer

        python3Full
        tailwindcss_4
        cargo-watch # rebuild when files change

        # generate opengraph images
        firefox
        geckodriver

        # local scripts
        blog-dev-server
        blog-dev-compile
      ]
      ++ (
        if stdenv.isLinux
        then [
          xvfb-run # simulate an xorg display in CI
        ]
        else []
      );

    shellHook = ''
      echo "Use 'blog-dev-compile' to build the blog"
      echo "Use 'blog-dev-server' to serve built files"
    '';

    # Certain Rust tools won't work without this
    # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
    # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }
