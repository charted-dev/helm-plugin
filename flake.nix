# 🐻‍❄️🌺 charted-helm-plugin: Helm plugin to help faciliate operations with charted-server
# Copyright 2023-2025 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
{
  description = "🐻‍❄️🌺 Helm plugin to help faciliate operations with charted-server";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs = {
        follows = "nixpkgs";
      };
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    systems,
    ...
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    overlays = [
      (import rust-overlay)
    ];

    nixpkgsFor = system:
      import nixpkgs {
        inherit system overlays;
      };
  in {
    formatter = eachSystem (system: (nixpkgsFor system).alejandra);
    packages = eachSystem (system: let
      pkgs = nixpkgsFor system;
      helm-plugin = pkgs.callPackage ./nix/package.nix {};
    in {
      inherit helm-plugin;

      default = helm-plugin;
    });

    devShells = eachSystem (system: let
      pkgs = nixpkgsFor system;
    in {
      default = import ./nix/devshell.nix {inherit pkgs;};
    });
  };
}
