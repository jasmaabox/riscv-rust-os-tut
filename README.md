# RISC-V Rust Tutorial

Following [tutorial](http://osblog.stephenmarz.com/index.html) to make OS

## Setup
  - Build cross compiler using [riscv-gnu-toolchain](https://github.com/riscv/riscv-gnu-toolchain)
  - QEMU
    - `apt-get install build-essential zlib1g-dev pkg-config libglib2.0-dev binutils-dev libboost-all-dev autoconf libtool libssl-dev libpixman-1-dev libpython-dev python-pip python-capstone virtualenv`
    - `./configure --target-list=riscv64-softmmu && make`
    - `cp ./riscv64-softmmu/qemu-system-riscv64 /opt/riscv/bin`