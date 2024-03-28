{
  lib,
  wasm-bindgen-cli,
  rustc,
  rustPlatform,
  stdenv,
  just,
  cargo,
  web-ext,
}:
stdenv.mkDerivation {
  pname = "rowserext";
  version = "0.1.0";

  src = ./.;
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = let
    wasm-bindgen = wasm-bindgen-cli.override {
      version = "0.2.87";
      hash = "sha256-0u9bl+FkXEK2b54n7/l9JOCtKo+pb42GF9E1EnAUQa0=";
      cargoHash = "sha256-AsZBtE2qHJqQtuCt/wCAgOoxYMfvDh8IzBPAOkYSYko=";
    };
  in [
    just
    rustc.llvmPackages.lld
    cargo
    wasm-bindgen
    rustPlatform.cargoSetupHook
    web-ext
  ];

  postPatch = ''
    runHook cargoSetupHook
    substituteInPlace */justfile --replace-fail 'cargo build' 'cargo build --frozen'
  '';
  env.CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  buildPhase = ''
    for ex in lionel join-on-time; do
      pushd $ex
      just release
      rm pkg/*.ts
      popd
      web-ext build -s $ex -a art -n $ex.xpi
    done
  '';
  checkPhase = '''';
  installPhase = ''
    mkdir -p $out
    cp -art $out art/*
  '';

  meta = with lib; {
    description = "Rust browser extensions";
    license = licenses.mit;
    platforms = platforms.linux;
    homepage = "https://github.com/jcaesar/rowserext";
  };
}
