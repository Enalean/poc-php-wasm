{
pkgs ? import (
  fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/1cc8a7ba8844f68a646da509a3976b52f406a28c.tar.gz";
    sha256 = "sha256:1dxsv600lcdaj8dl87i0nxj7k057ddpi299ba8zvqc7knh8b7642";
  }
) {}}:

pkgs.mkShell {
  buildInputs = [
    pkgs.bash
    pkgs.rustup
    pkgs.hyperfine
    pkgs.wasmtime
    ( # See https://github.com/NixOS/nixpkgs/pull/189600
      pkgs.rustPlatform.buildRustPackage rec {
        pname = "wasmer";
        version = "2.3.0";

        src = pkgs.fetchFromGitHub {
          owner = "wasmerio";
          repo = pname;
          rev = version;
          sha256 = "sha256-25wWgMNybbsEf/1xmm+8BPcjx8CSW9ZBzxGKT/DbBXw=";
          fetchSubmodules = true;
        };

        cargoSha256 = "sha256-tswsbijNN5UcSZovVmy66yehcEOpQDGMdRgR/1mkuE8=";

        buildInputs = [ pkgs.libffi pkgs.libxml2 pkgs.ncurses pkgs.zlib ];
        nativeBuildInputs = [ pkgs.cmake pkgs.pkg-config pkgs.llvmPackages_12.llvm ];

        buildFeatures = [ "cranelift" "jit" "llvm" "singlepass" ];
        cargoBuildFlags = [
          # must target manifest and desired output bin, otherwise output is empty
          "--manifest-path" "lib/cli/Cargo.toml"
          "--bin" "wasmer"
        ];

        # Can't use test-jit:
        # error: Package `wasmer-workspace v2.3.0 (/build/source)` does not have the feature `test-jit`
        checkFeatures = [ "test-cranelift" ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      }
    )
    pkgs.wasmedge
    pkgs.binaryen
    (pkgs.php81.buildEnv {
        extensions = { all, ... }: with all; [ ffi ];
        extraConfig = "ffi.enable=true";
    })
  ];
}
