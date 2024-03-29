FROM rust:latest

COPY src/* ./src/
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo install --path .

ENTRYPOINT ["unifi-protect-bulk-download"]