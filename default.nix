{
  lib,
  wasm-bindgen-cli_0_2_100, # likely, I'll have to switch to buildWasmBindgenCli in the not-too-far future
  rustc,
  rustPlatform,
  stdenv,
  just,
  cargo,
  zip,
}:
stdenv.mkDerivation {
  pname = "rowserext";
  version = "0.1.0";

  src = ./.;
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    just
    rustc.llvmPackages.lld
    cargo
    wasm-bindgen-cli_0_2_100
    rustPlatform.cargoSetupHook
    zip
  ];

  postPatch = ''
    runHook cargoSetupHook
    substituteInPlace */justfile --replace-fail 'cargo build' 'cargo build --frozen'
  '';
  env.CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  buildPhase = ''
    mkdir $out
    for ex in join-on-time lionel; do
    # for ex in join-on-time; do
      pushd $ex
      just release
      zip -9X $out/$ex.xpi $(
        find . -type f ! -name Cargo.\* ! -name \*.rs ! -name justfile \
          -exec touch -t 198001010000 {} + -print
      )
      popd
    done
  '';
  installPhase = "true";

  meta = with lib; {
    description = "Rust browser extensions";
    license = licenses.mit;
    platforms = platforms.linux;
    homepage = "https://github.com/jcaesar/rowserext";
  };
}
