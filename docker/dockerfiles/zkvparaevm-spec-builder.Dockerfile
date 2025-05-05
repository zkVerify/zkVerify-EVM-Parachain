FROM rust:1-bookworm AS builder

RUN apt-get update -qq \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      clang \
      cmake \
      lld \
      protobuf-compiler \
    && apt-get -y clean \
    && apt-get -y autoclean \
    && apt-get -y autoremove \
    && rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*.deb

# Add Rust targets and components
RUN rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src

ARG PROFILE="release"
ARG FEATURES=""

WORKDIR /usr/src/node
COPY . .

RUN cargo build --profile "${PROFILE}" --features "${FEATURES}"

FROM ubuntu:24.04 AS node

SHELL ["/bin/bash", "-c"]

ARG BINARY="zkverify-evm-para-spec-builder"
ARG DESCRIPTION="zkVerify EVM Parachain Spec Builder"
ARG AUTHORS="infrastructure@zkverify.io"
ARG VENDOR="zkVerify"
ARG PROFILE="release"
ARG FEATURES=""

ENV BINARY="${BINARY}" \
    RUN_USER="user"

LABEL io.image.authors="${AUTHORS}" \
      io.image.vendor="${VENDOR}" \
      io.image.description="${DESCRIPTION}" \
      io.image.profile="${PROFILE}" \
      io.image.features="${FEATURES}"

USER root
WORKDIR /app

COPY --from=builder "/usr/src/node/target/${PROFILE}/zkverify-evm-para-node" "/usr/local/bin/zkverify-evm-para-spec-builder"
RUN chmod -R a+rx "/usr/local/bin"

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      aria2 \
      ca-certificates \
      curl \
      jq \
    && useradd -m -U -s /bin/bash -d "/${RUN_USER}" "${RUN_USER}" \
    && mkdir -p /data "/${RUN_USER}/.local/share" \
    && chown -R "${RUN_USER}:${RUN_USER}" /data "/${RUN_USER}" \
    && ln -s /data "/${RUN_USER}/.local/share" \
    && apt-get -y clean \
    && apt-get -y autoclean \
    && apt-get -y autoremove \
    && rm -rf /var/{lib/apt/lists/*,cache/apt/archives/*.deb} /tmp/*

COPY docker/scripts/entrypoint.sh .
RUN chmod +x entrypoint.sh

USER "${RUN_USER}"

ENTRYPOINT ["/app/entrypoint.sh"]
