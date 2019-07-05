FROM ekidd/rust-musl-builder:1.35.0-openssl11 as builder

ADD . ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder \
    /home/rust/src/static \
    /app/static
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/augie-auth \
    /usr/local/bin/
CMD /usr/local/bin/augie-auth
