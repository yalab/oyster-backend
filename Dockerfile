FROM rust:1.50-slim-buster
RUN apt update && apt upgrade -y && apt install -y libpq-dev && cargo install diesel_cli --no-default-features --features postgres
