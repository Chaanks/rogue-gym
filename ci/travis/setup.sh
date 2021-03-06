#!/bin/bash

set -e

# Find the installed version of a binary, if any
_installed() {
    VERSION=$($@ --version 2>/dev/null || echo "$@ none")
    echo $VERSION | rev | cut -d' ' -f1 | rev
}

# Find the latest available version of a binary on `crates.io`
_latest() {
    VERSION=$(cargo search -q "$@" | grep "$@" | cut -f2 -d"\"")
    echo $VERSION
}

### Setup Rust toolchain #######################################################
curl -SsL "https://sh.rustup.rs/" | sh -s -- -y --default-toolchain=$RUST_VERSION
export PATH=$PATH:$HOME/.cargo/bin

### Setup python linker flags ##################################################

python -c """
import sysconfig
cfg = sorted(sysconfig.get_config_vars().items())
print('\n'.join(['{}={}'.format(*x) for x in cfg]))
"""

export PYTHON_LIB=$(python -c "import sysconfig as s; print(s.get_config_var('LIBDIR'))")

# find $PYTHON_LIB
export LIBRARY_PATH="$LIBRARY_PATH:$PYTHON_LIB"

# delete any possible empty components
# https://github.com/google/pulldown-cmark/issues/122#issuecomment-364948741
LIBRARY_PATH=$(echo $LIBRARY_PATH | sed -E -e 's/^:*//' -e 's/:*$//' -e 's/:+/:/g')

export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB:$HOME/rust/lib"
