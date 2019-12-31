# RISC-V Rust Tutorial

Following [tutorial](http://osblog.stephenmarz.com/index.html) to make OS

## Setup (Linux)
  - Build using [riscv-gnu-toolchain](https://github.com/riscv/riscv-gnu-toolchain)
  - Add `/opt/riscv/bin` to PATH

### Cross compiler
    # Pre-req
    sudo apt-get install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev
      
    make linux # Takes a long time...

### QEMU
    # Pre-req
    sudo apt-get install build-essential zlib1g-dev pkg-config libglib2.0-dev binutils-dev libboost-all-dev autoconf libtool libssl-dev libpixman-1-dev libpython-dev python-pip python-capstone virtualenv

    ./configure --target-list=riscv64-softmmu && make
    sudo cp ./riscv64-softmmu/qemu-system-riscv64 /opt/riscv/bin