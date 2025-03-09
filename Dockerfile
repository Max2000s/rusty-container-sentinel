# Build stage with musl-tools
FROM rust:latest AS builder

# Install necessary MUSL build tools
RUN apt-get update && \
	apt-get install -y musl-tools && \
	rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

# Set the linker explicitly for MUSL target
ENV RUSTFLAGS="-C linker=musl-gcc"

RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage (Alpine)
FROM alpine:latest
RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rusty-container-sentinel /usr/local/bin/rusty-container-sentinel

CMD ["/usr/local/bin/rusty-container-sentinel"]
