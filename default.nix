{
  headless ? false,
  release ? true,

  sources ? import ./npins,
  pkgs ? import sources.nixpkgs { },
  fmt ? import ./fmt.nix { inherit sources pkgs; },
  supervisord ? import sources.nix-supervisord { inherit pkgs; },
  leptos_ssg ? import sources.leptos_ssg { inherit sources pkgs headless; },
  cargo_nix ? pkgs.callPackage ./Cargo.nix { inherit release; },
}:
let
  supervisordProject = supervisord.mkSupervisor {
    project_name = "deadbaed-blog";
    paths = supervisord.mkPaths { };
    programs = [
      {
        name = "watchexec";
        command = "${pkgs.watchexec}/bin/watchexec -N -w content/ cargo run";
      }
      {
        name = "webserver";
        command = "${pkgs.python3}/bin/python -m http.server --directory ./target/blog 4343";
      }
      (
        let
          path = "./target/blog";
        in
        {
          name = "tailwind";
          command = "cat ${leptos_ssg.tailwind.tailwindLeptosSsg}/style.css > ${path}/style.css";
          pre_commands = [ "mkdir -p ${path}" ];
          start_secs = 0;
        }
      )
    ];
  };

  createSymlink = pkgs.writeShellScriptBin "createSymlink" ''
    ln -s $(npins get-path leptos_ssg) "$@"
  '';

  updateLockfiles = pkgs.writeShellScriptBin "updateLockfiles" ''
    set -xe
    symlink="leptos_ssg"
    test -L $symlink && rm $symlink && ${createSymlink}/bin/createSymlink $symlink

    cargo build
    crate2nix generate
  '';

  blogCrate = cargo_nix.rootCrate.build.override {
    features = [
      "prod"
    ];
  };

  buildWithOpengraph = pkgs.writeShellScriptBin "buildWithOpengraph" ''
    set -x
    opengraph_css=$(mktemp)
    ${leptos_ssg.tailwind.copyTailwindOpengraph}/bin/cp-tailwind-opengraph $opengraph_css
    ${leptos_ssg.opengraph.runWithGeckodriver}/bin/runWithGeckodriver ${blogCrate}/bin/blog $opengraph_css
    ${leptos_ssg.tailwind.copyTailwindLeptosSsg}/bin/cp-tailwind-leptos_ssg target/blog/www/style.css
    rm $opengraph_css
  '';

in
{
  productionShell = pkgs.mkShellNoCC {
    packages = [ buildWithOpengraph ];
    shellHook = leptos_ssg.opengraph.supervisordShellHook;
  };

  shell = pkgs.mkShellNoCC {
    inherit (leptos_ssg) env;

    packages =
      with pkgs;
      [
        # formatter
        fmt

        # nix
        npins
        nil
        nixfmt-rfc-style
        crate2nix

        supervisordProject.supervisord-wrapper
        supervisordProject.supervisorctl-wrapper
        supervisordProject.supervisord-kill
        lnav

        createSymlink
        updateLockfiles
      ]
      ++ leptos_ssg.rustTools;

    shellHook = supervisordProject.shellHook;
  };
}
