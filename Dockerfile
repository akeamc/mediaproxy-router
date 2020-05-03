FROM ekidd/rust-musl-builder:nightly-2020-04-10 AS builder
RUN USER=rust cargo init
COPY --chown=rust:rust Cargo.* ./
RUN cargo build --release
RUN rm -r target/x86_64-unknown-linux-musl/release/deps/mediaproxy_router*
COPY --chown=rust:rust . .
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/mediaproxy-router /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/mediaproxy-router", "--listen", "0.0.0.0:80"]