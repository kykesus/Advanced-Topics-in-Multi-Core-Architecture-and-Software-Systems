FROM rust:1.50-slim

COPY . /parsort/
RUN cargo install --path /parsort
WORKDIR /parsort
CMD cargo build --target-dir /parsort/artifacts