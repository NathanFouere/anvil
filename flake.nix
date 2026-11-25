{
  description = "Anvil: formally verified Kubernetes controllers (dev environment)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system:
        f (import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; }));

      # Prebuilt Verus release (https://github.com/verus-lang/verus/releases).
      # To upgrade: bump `version`, `commit`, the hashes (set hash = "" and copy the one
      # Nix reports) and `rustVersion` (the "toolchain" field of version.json in the archive).
      verus = {
        version = "0.2026.11.02.8bf68ee";
        # Full commit of the release: vstd (a git dependency in Cargo.toml) must match it.
        commit = "8bf68ee741df833ea5d0fe6a2255126fdea87e47";
        rustVersion = "1.99.1";
        assets = {
          x86_64-linux = { arch = "x86"; hash = "sha256-rRdcaRMbXq1Q5SN/JJ0vUzguLmF8i52PBYXgBSFUgtY="; };
          aarch64-linux = { arch = "arm64"; hash = "sha256-aA5+hBRQGJmWiBh1wI/9v3u9ARehgxSL4TeJXz7T3LQ="; };
        };
      };
    in
    {
      packages = forAllSystems (pkgs:
        let
          asset = verus.assets.${pkgs.stdenv.hostPlatform.system};

          # The exact toolchain Verus was built against (rust_verify links librustc_driver).
          rustToolchain = pkgs.rust-bin.stable.${verus.rustVersion}.default.override {
            extensions = [ "rustc-dev" "llvm-tools" "rust-src" "rust-analyzer" ];
          };
        in
        {
          inherit rustToolchain;

          verus = pkgs.stdenv.mkDerivation {
            pname = "verus";
            inherit (verus) version;
            src = pkgs.fetchzip {
              url = "https://github.com/verus-lang/verus/releases/download/release%2F${verus.version}/verus-${verus.version}-${asset.arch}-linux.zip";
              inherit (asset) hash;
              stripRoot = false;
            };
            nativeBuildInputs = [ pkgs.autoPatchelfHook ];
            # libstdc++ for the bundled z3, librustc_driver for rust_verify
            buildInputs = [ pkgs.stdenv.cc.cc.lib rustToolchain ];
            dontStrip = true;
            installPhase = ''
              runHook preInstall
              # Verus looks for its libs, vstd and z3 next to its own executable.
              mkdir -p $out/lib/verus $out/bin
              cp -r ./verus-${asset.arch}-linux/* $out/lib/verus/
              for b in verus cargo-verus; do
                ln -s $out/lib/verus/$b $out/bin/$b
              done
              runHook postInstall
            '';
          };
        });

      devShells = forAllSystems (pkgs:
        let own = self.packages.${pkgs.stdenv.hostPlatform.system}; in
        {
          default = pkgs.mkShell {
            packages = [
              own.rustToolchain
              own.verus

              # Native deps of the controllers (kube with openssl-tls)
              pkgs.pkg-config
              pkgs.openssl

              # Tooling used by tools/ and e2e/
              pkgs.go
              pkgs.kind
              pkgs.kubectl
            ];

            # Use the Nix toolchain instead of asking rustup for one.
            VERUS_USE_RUSTUP = "0";

            # Cargo.lock is not tracked, so cargo would pick vstd from verus `main`,
            # which is usually incompatible with the pinned Verus binary.
            shellHook = ''
              if ! grep -q "verus-lang/verus.git#${verus.commit}" Cargo.lock 2>/dev/null; then
                echo "anvil: pinning vstd to Verus ${verus.version}"
                cargo update -p vstd --precise ${verus.commit} --quiet || true
              fi
            '';
          };
        });
    };
}
