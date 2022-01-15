FROM rust:1.24.0 as builder

FROM debian:buster

RUN apt-get update && apt-get install -y curl cargo

RUN mkdir -p /var/www/app
WORKDIR /var/www/app
COPY . /var/www/app
RUN ls -la /var/www/app
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo build
CMD trunk serve
EXPOSE 8080
