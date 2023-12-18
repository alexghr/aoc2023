{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # The version of wasm-bindgen-cli needs to match the version in Cargo.lock
    # Update this to include the version you need
    nixpkgs-for-wasm-bindgen.url = "github:NixOS/nixpkgs/4e6868b1aa3766ab1de169922bb3826143941973";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, nixpkgs-for-wasm-bindgen, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
        craneLib = ((crane.mkLib pkgs).overrideToolchain rustToolchain);

        # When filtering sources, we want to allow assets other than .rs files
        src = lib.cleanSourceWith {
          src = ./.; # The original, unfiltered source
          filter = path: type:
            (lib.hasSuffix "\.txt" path) ||
            (craneLib.filterCargoSources path type);
        };


        # When building a workspace with crane, it's a good idea
        # to set "pname" and "version".
        commonArgs = {
          inherit src;
          pname = "trunk-workspace";
          version = "0.1.0";
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        # Native packages

        nativeArgs = commonArgs // {
          pname = "trunk-workspace-native";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

        day1 = craneLib.buildPackage (nativeArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "--package day1";
        });
        day2 = craneLib.buildPackage (nativeArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "--package day2";
        });
        day3 = craneLib.buildPackage (nativeArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "--package day3";
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit day1;
          inherit day2;
          inherit day3;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-app-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Check formatting
          my-app-fmt = craneLib.cargoFmt commonArgs;
        };

        apps.day1-part1 = flake-utils.lib.mkApp {
          name = "part1";
          drv = day1;
        };
        apps.day1-part2 = flake-utils.lib.mkApp {
          name = "part2";
          drv = day1;
        };

        apps.day2-part1 = flake-utils.lib.mkApp {
          name = "part1";
          drv = day2;
        };
        apps.day2-part2 = flake-utils.lib.mkApp {
          name = "part2";
          drv = day2;
        };

        apps.day3-part1 = flake-utils.lib.mkApp {
          name = "part1";
          drv = day3;
        };
        apps.day3-part2 = flake-utils.lib.mkApp {
          name = "part2";
          drv = day3;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};
          buildInputs = [
            pkgs.rust-analyzer-unwrapped
            rustToolchain
          ];

          shellHook = ''
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            pkgs.trunk
          ];
        };
      });
}
