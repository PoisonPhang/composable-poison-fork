{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      composable-bench-node = self'.packages.composable-bench-node;

      benchmarks-run-once = chainspec:
        pkgs.writeShellScriptBin "run-benchmarks-once" ''
          ${composable-bench-node}/bin/composable benchmark pallet \
          --chain="${chainspec}" \
          --execution=wasm \
          --wasm-execution=compiled \
          --pallet="*" \
          --extrinsic="*" \
          --steps=2 \
          --repeat=2
        '';

      generate-benchmarks = { chain, steps, repeat }:
        pkgs.writeShellScriptBin "generate-benchmarks" ''
          ${composable-bench-node}/bin/composable benchmark pallet \
          --chain="${chain}-dev" \
          --execution=wasm \
          --wasm-execution=compiled \
          --pallet="*" \
          --extrinsic="*" \
          --steps=${builtins.toString steps} \
          --repeat=${builtins.toString repeat} \
          --output=code/parachain/runtime/${chain}/src/weights
        '';

      mkBenchmarksCiPackage = name: package:
        pkgs.stdenv.mkDerivation {
          inherit name;
          src = ./.;
          installPhase = ''
            mkdir -p $out
            cd $out
            ${pkgs.lib.meta.getExe package} > benchmark-logs.txt
          '';
        };

    in {
      packages = rec {
        check-dali-dev-benchmarks = benchmarks-run-once "dali-dev";
        check-picasso-dev-benchmarks = benchmarks-run-once "picasso-dev";
        check-composable-dev-benchmarks = benchmarks-run-once "composable-dev";

        check-dali-benchmarks-ci =
          mkBenchmarksCiPackage "check-dali-benchmarks-ci"
          check-dali-dev-benchmarks;
        check-picasso-benchmarks-ci =
          mkBenchmarksCiPackage "check-picasso-benchmarks-ci"
          check-picasso-dev-benchmarks;
        check-composable-benchmarks-ci =
          mkBenchmarksCiPackage "check-composable-benchmarks-ci"
          check-composable-dev-benchmarks;
      };
      apps = let flake-utils = self.inputs.flake-utils;
      in {
        # TODO: move list of chains out of here and do map
        benchmarks-once-composable =
          flake-utils.lib.mkApp { drv = benchmarks-run-once "composable-dev"; };
        benchmarks-once-dali =
          flake-utils.lib.mkApp { drv = benchmarks-run-once "dali-dev"; };
        benchmarks-once-picasso =
          flake-utils.lib.mkApp { drv = benchmarks-run-once "picasso-dev"; };
        benchmarks-generate-dali = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "dali";
            steps = 50;
            repeat = 10;
          };
        };
        benchmarks-generate-picasso = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "picasso";
            steps = 50;
            repeat = 10;
          };
        };
        benchmarks-generate-composable = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "composable";
            steps = 50;
            repeat = 10;
          };
        };
        benchmarks-generate-quick-dali = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "dali";
            steps = 2;
            repeat = 2;
          };
        };
        benchmarks-generate-quick-picasso = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "picasso";
            steps = 2;
            repeat = 2;
          };
        };
        benchmarks-generate-quick-composable = flake-utils.lib.mkApp {
          drv = generate-benchmarks {
            chain = "composable";
            steps = 2;
            repeat = 2;
          };
        };
      };
    };
}
