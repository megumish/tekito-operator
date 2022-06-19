FROM rust:1.61 as builder

WORKDIR /workspace

COPY Cargo.toml Cargo.toml
COPY .cargo .cargo

COPY Cargo.lock Cargo.lock
COPY src src
COPY target/release target/release

RUN apt update && apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --bin manager

FROM gcr.io/distroless/static:nonroot
WORKDIR /
COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/manager .
USER 65532:65532

ENTRYPOINT ["/manager"]