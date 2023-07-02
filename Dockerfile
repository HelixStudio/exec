FROM alpine:3.18

RUN apk add --no-cache rust cargo clang ghc

WORKDIR /exec

COPY . .

RUN cargo build --release

EXPOSE 8080/tcp

CMD ["./target/release/exec"]
