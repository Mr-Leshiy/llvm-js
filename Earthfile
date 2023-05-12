VERSION 0.7
FROM rust:latest

install-deps:
   RUN apt update -y && apt upgrade -y
   RUN apt install cmake -y
   RUN cmake --version

install-llvm:
   FROM +install-deps
   RUN git clone --branch release/14.x https://github.com/llvm/llvm-project.git
   RUN cd llvm-project && cmake -S llvm -B build && cmake --build build && cmake --install build
   SAVE ARTIFACT build

install-chef:
   RUN cargo install --debug cargo-chef

prepare-cache:
    FROM +install-chef
    COPY --dir crates testing Cargo.lock Cargo.toml .
    RUN cargo chef prepare
    SAVE ARTIFACT recipe.json

# Using cutoff-optimization to ensure cache hit (see examples/cutoff-optimization)
build-cache:
    FROM +install-chef
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home