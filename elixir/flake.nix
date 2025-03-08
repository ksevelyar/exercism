{
  description = "exercism-elixir";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/b58df7fc9d5f02c269091f2b0b81a6e06fc859bb";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      beamPackages = pkgs.beam.packagesWith pkgs.beam.interpreters.erlang_27;
      elixir = beamPackages.elixir_1_18;
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          elixir
          pkgs.elixir_ls
          pkgs.inotify-tools
        ];

        shellHook = ''
          mkdir -p .nix-mix .nix-hex
          export MIX_HOME=$PWD/.nix-mix
          export HEX_HOME=$PWD/.nix-mix
          export PATH=$MIX_HOME/bin:$HEX_HOME/bin:$PATH
          mix local.hex --force

          export ERL_AFLAGS="-kernel shell_history enabled -kernel shell_history_path '\"$PWD/.erlang-history\"'"
        '';
      };
    });
}
