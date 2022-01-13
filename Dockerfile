FROM rust:latest

RUN RUSTFLAGS="-C target-cpu=native" cargo install keepass-diff

WORKDIR /app

ENTRYPOINT ["keepass-diff"]
