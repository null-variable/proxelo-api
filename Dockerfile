# syntax=docker/dockerfile:1
# Toolchain for this repository: buf, Go and Rust at the versions CI uses.
# The source is mounted at /src (see compose.yaml), so generated code lands
# in the checkout:
#   docker build -t proxelo-api-tools .
#   docker run --rm -v "$PWD:/src" proxelo-api-tools buf generate
FROM bufbuild/buf:1.75.0 AS buf

FROM golang:1.26-trixie AS go

FROM rust:1.98-slim-trixie
COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY --from=go /usr/local/go /usr/local/go
ENV PATH=/usr/local/go/bin:$PATH \
    GOPATH=/go \
    GOFLAGS=-buildvcs=false
RUN apt-get update \
 && apt-get install -y --no-install-recommends git \
 && rm -rf /var/lib/apt/lists/* \
 && rustup component add clippy rustfmt \
 && git config --system --add safe.directory /src
WORKDIR /src
