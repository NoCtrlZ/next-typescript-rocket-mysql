FROM rust:1.40 as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update -y &&\
    apt-get install musl-tools -y &&\
    rustup default nightly &&\
    rustup target add x86_64-unknown-linux-musl &&\
    cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.10

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app .

CMD ["./target/x86_64-unknown-linux-musl/release/backend"]

EXPOSE 8000
