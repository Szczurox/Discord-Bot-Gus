FROM rust:1.59.0
RUN rustup default stable
RUN cargo install cargo-shuttle