FROM composablefi/ci-linux:2022-04-18 as builder

COPY . /build
WORKDIR /build

RUN cargo build --release

# ===== SECOND STAGE ======

# FROM composablefi/mmr-polkadot:latest as mmr-polkadot

FROM ubuntu:21.10
LABEL description="Docker image with Composable"

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN groupadd -g 1000 service && useradd -m -s /bin/sh -g 1000 -G service service && \
	mkdir -p /apps/composable/scripts /apps/composable/target/release /apps/basilisk-node/target/release /apps/polkadot/target/release && \
	apt-get update && apt-get install -y --no-install-recommends apt-utils ca-certificates curl make gcc openssl libssl-dev pkg-config cmake git && \
	curl -fsSL https://deb.nodesource.com/setup_17.x | bash - && \
	apt-get update && apt-get install -y --no-install-recommends nodejs && \
	npm install --global npm yarn && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete;

RUN mkdir -p /apps/polkadot/target/release && \
	curl https://github.com/paritytech/polkadot/releases/download/v0.9.18/polkadot -Lo  /apps/polkadot/target/release/polkadot && \
	chmod +x /apps/polkadot/target/release/polkadot

COPY --from=builder /build/target/release/composable /apps/composable/target/release/
# COPY --from=mmr-polkadot /polkadot /apps/polkadot/target/release/
COPY ./scripts/polkadot-launch /apps/composable/scripts/polkadot-launch

WORKDIR /apps/composable/scripts/polkadot-launch

RUN chown -R service /apps/composable/scripts/polkadot-launch && yarn

USER service
EXPOSE 9945 9988 9998
ENTRYPOINT ["yarn", "composable"]
