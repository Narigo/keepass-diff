FROM rust:latest
WORKDIR /app
COPY . /app
RUN cargo install --path .
CMD ["keepass-diff"]

