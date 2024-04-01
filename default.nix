{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
     boost
     cmake
     gflags
     git
     glog
     libevent
     libtool
     lz4
     xz
     openssl
     snappy
     zlib
     cargo
     rustc
    #  python 3.11
     python3
    (pkgs.python3.withPackages (python-pkgs: [
      python-pkgs.flit
      python-pkgs.turnt
    ]))
    ];

    # Set any environment variables if needed
    # shellHook = ''
    #   export SOME_ENV_VAR="value"
    # '';
}