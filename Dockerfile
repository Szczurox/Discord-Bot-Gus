FROM rust:latest
RUN rustup default stable
RUN cargo install cargo-shuttle