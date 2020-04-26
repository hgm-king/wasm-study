FROM rust:latest

WORKDIR /src

RUN cargo install wasm-pack
RUN cargo install cargo-generate

COPY . /src

CMD [ "ls" ]
