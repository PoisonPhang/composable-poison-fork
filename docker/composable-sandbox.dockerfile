FROM composablefi/ci-linux:2022-04-18 as builder

# ISSUE: we already copied context when started build, what the heck we are roing here? it slows down builds
COPY . /build
WORKDIR /build

# NOTE: decide prio and responsible for migration to nix after https://github.com/ComposableFi/composable/issues/1426
RUN cargo +nightly build --release -p wasm-optimizer
RUN cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm

RUN export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm) && \
	export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm) && \
	export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm) && \
	cargo build --release --features=builtin-wasm

# ===== SECOND STAGE ======

FROM composablefi/mmr-polkadot:latest as mmr-polkadot

FROM ubuntu:21.10
LABEL description="Docker image with Composable"

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
# ISSUE: basilisc is obsolete
RUN groupadd -g 1000 service && useradd -m -s /bin/sh -g 1000 -G service service && \
	mkdir -p /apps/composable/scripts /apps/composable/target/release /apps/polkadot/target/release && \
	apt-get update && apt-get install -y --no-install-recommends apt-utils ca-certificates curl git && \
	curl -fsSL https://deb.nodesource.com/setup_17.x | bash - && \
	apt-get update && apt-get install -y --no-install-recommends nodejs && \
	npm install --global npm yarn && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete;

COPY --from=builder /build/target/release/composable /apps/composable/target/release/
COPY --from=mmr-polkadot /polkadot /apps/polkadot/target/release/
COPY ./scripts/polkadot-launch /apps/composable/scripts/polkadot-launch

WORKDIR /apps/composable/scripts/polkadot-launch

RUN chown -R service /apps/composable/scripts/polkadot-launch && \
	yarn && \
	sed -i 's/"--rpc-cors=all"/"--rpc-cors=all", "--ws-external", "--unsafe-rpc-external", "--rpc-methods=unsafe"/' composable.json

USER service
EXPOSE 9945 9988
ENTRYPOINT ["yarn", "composable"]
