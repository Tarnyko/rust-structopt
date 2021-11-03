# syntax=docker/dockerfile:1
FROM ubuntu:latest

RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . / /code
WORKDIR /code
RUN cargo build --release

EXPOSE 8080
CMD ["/code/target/release/rust-structopt", "counter.txt"]
