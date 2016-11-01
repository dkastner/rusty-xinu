FROM scorpil/rust:nightly

RUN apt-get update && \
    apt-get install -y \
      build-essential \
      git \
      grub \
      grub-pc-bin \
      nasm \
      xorriso \
      && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /build

ADD Cargo.toml /build/

RUN cargo update

ADD . /build/
