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
  makeRustPlatform,
  pkg-config,
  installShellFiles,
  openssl,
  rust-bin,
  lib,
}: let
  common = import ./common.nix;
  rust-toolchain = common.mkRustPlatform rust-bin;
  rustPlatform = common.mkNixpkgsRustPlatform {inherit makeRustPlatform;} rust-toolchain;
  version = common.cargoTOML.package.version;
in
  rustPlatform.buildRustPackage {
    inherit version;

    pname = "charted-helm-plugin";
    src = ../.;

    cargoBuildFlags = ["--bin" "charted-helm-plugin"];
    cargoLock.lockFile = ../Cargo.lock;

    nativeBuildInputs = [pkg-config installShellFiles];
    buildInputs = [openssl];

    env.CHARTED_DISTRIBUTION_KIND = "nix";
    postPatch = ''
      sed -i '/^platformHooks:/,+2 d' plugin.yaml
    '';

    postInstall = ''
      install -Dm644 plugin.yaml $out/charted-helm-plugin/plugin.yaml
      mv $out/bin $out/charted-helm-plugin
    '';

    meta = with lib; {
      description = "🐻‍❄️📦 Free, open-source way to distribute Helm charts across the world";
      maintainers = with maintainers; [auguwu spotlightishere];
      mainProgram = "helm charted";
      changelog = "https://charts.noelware.org/changelogs/helm-plugin#v${version}";
      homepage = "https://charts.noelware.org";
      license = with licenses; [asl20];
    };
  }
