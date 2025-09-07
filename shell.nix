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
  }
