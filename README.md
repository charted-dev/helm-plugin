### 🐻‍❄️🌺 `charted-helm-plugin`
#### *[Helm] plugin to faciliate operations with [charted-server]*

**charted-helm-plugin** is a [Helm] plugin developed by the **charted** team to help faciliate operations with [charted-server]. This was originally in the [old source tree in `charted-dev/charted`] but has moved to its old place, **charted-dev/helm-plugin**!

The move was needed so that the binary size can be even smaller than what is required. Since we use [`azalia-remi`] in the server's source tree, it was pulling crates that aren't being used by the Helm plugin, which is probably our fault but it has been moved back here.

The project is maintained alongside of the main repository.

## Installation
You can use the **helm plugin install** command:

```shell
# [staging] From the Git repository
$ helm plugin install https://github.com/charted-dev/helm-plugin

# From official sources (recommended)
$ helm plugin install https://charts.noelware.org/x/helm-plugin
```

and the **helm charted** command is now avaliable:

```shell
🐻‍❄️🌺 Helm plugin to help faciliate operations with charted-server

Usage: helm charted <COMMAND> [...ARGS]

Commands:
  completions
  auth         Subcommand to perform auth-related actions
  help         Print this message or the help of the given subcommand(s)

Options:
  -l, --log-level <LEVEL>  Configures the log level for the logs that are transmitted [env: CHARTED_HELM_LOG_LEVEL=] [default: INFO]
  -h, --help               Print help
```

### Nix/NixOS
Under the [`Noelware/nixpkgs-noelware`] overlay, **charted-helm-plugin** is avaliable that can be wrapped with `wrapHelm kubernetes-helm`:

```nix
{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        noelware = {
            url = "github:Noelware/nixpkgs-noelware";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { nixpkgs, noelware, ... }: let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
            inherit system;

            overlays = [(import noelware)];
        };
    in
    {
        devShells.${system}.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                # this will bundle `helm charted` in your devshell
                (wrapHelm kubernetes-helm {
                    plugins = [charted-helm-plugin];
                })
            ];
        };
    };
}
```

## Contributions
Thanks for considering contributing to **charted-helm-plugin**! Before you boop your heart out on your keyboard ✧ ─=≡Σ((( つ•̀ω•́)つ, we recommend you to do the following:

-   Read the [Code of Conduct](./.github/CODE_OF_CONDUCT.md)
-   Read the [Contributing Guide](./.github/CONTRIBUTING.md)

If you read both if you're a new time contributor, now you can do the following:

-   [Fork me! ＊\*♡( ⁎ᵕᴗᵕ⁎ ）](https://github.com/charted-dev/helm-plugin/fork)
-   Clone your fork on your machine: `git clone https://github.com/your-username/helm-plugin`
-   Create a new branch: `git checkout -b some-branch-name`
-   BOOP THAT KEYBOARD!!!! ♡┉ˏ͛ (❛ 〰 ❛)ˊˎ┉♡
-   Commit your changes onto your branch: `git commit -am "add features （｡>‿‿<｡ ）"`
-   Push it to the fork you created: `git push -u origin some-branch-name`
-   Submit a Pull Request and then cry! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡

## License
**charted-helm-plugin** is released under the **Apache 2.0** License with love and care by the team. Also, **Dr. Pepper** is also to blame. #notsponsored

[old source tree in `charted-dev/charted`]: https://github.com/charted-dev/charted/tree/56fe557b186ce8ca1743f5d783dec335d2b39175/crates/helm/plugin
[`Noelware/nixpkgs-noelware`]: https://github.com/Noelware/nixpkgs-noelware
[charted-server]: https://charts.noelware.org
[`azalia-remi`]: https://github.com/Noelware/azalia/tree/master/crates/remi
[Helm]: https://helm.sh
