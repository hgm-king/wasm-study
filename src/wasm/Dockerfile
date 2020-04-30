FROM rust:latest

WORKDIR /src

RUN cargo install wasm-pack
RUN cargo install cargo-web

EXPOSE 8000

COPY . /src

CMD [ "ls" ]
