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
{pkgs}: let
  inherit (pkgs) mkShell lib darwin stdenv;

  common = import ./common.nix;
  rustflags = common.rustflags stdenv;

  darwinNativeBuildInputs = with darwin.apple_sdk.frameworks; [
    SystemConfiguration
    CoreFoundation
    Security
  ];

  linuxNativeBuildInputs = with pkgs; [mold];

  # rpathInputs = with pkgs; [
  #   openssl
  # ];

  nativeBuildInputs = with pkgs;
    [pkg-config]
    ++ (lib.optional stdenv.isLinux linuxNativeBuildInputs)
    ++ (lib.optional stdenv.isDarwin darwinNativeBuildInputs);

  buildInputs = with pkgs;
    [
      cargo-upgrades
      cargo-machete
      cargo-nextest
      cargo-expand
      cargo-about
      cargo-bloat
      cargo-deny

      (common.mkRustPlatform pkgs.rust-bin)
      #openssl
      git
    ]
    ++ (lib.optional stdenv.isLinux [glibc]);
in
  mkShell {
    inherit buildInputs nativeBuildInputs;

    #LD_LIBRARY_PATH = lib.makeLibraryPath rpathInputs;

    name = "charted-helm-plugin-dev";
    shellHook = ''
      export RUSTFLAGS="${rustflags}"
    '';
  }
