# Copyright (C) 2022 NexToken Tech LLC.
# SPDX-License-Identifier: Apache-2.0

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# ===== START FIRST STAGE ======
FROM phusion/baseimage:jammy-1.0.0 as builder
LABEL maintainer="team@trex.ink"
LABEL description="trex builder."

ARG FEATURES
ARG PROFILE=release
ARG STABLE=nightly
WORKDIR /rustbuilder
COPY . /rustbuilder/trex

# PREPARE OPERATING SYSTEM & BUILDING ENVIRONMENT
RUN apt-get update && \
	apt-get install -y pkg-config libssl-dev git clang libclang-dev diffutils gcc make m4 build-essential curl file cmake protobuf-compiler libprotobuf-dev

# UPDATE RUST DEPENDENCIES
ENV RUSTUP_HOME "/rustbuilder/.rustup"
ENV CARGO_HOME "/rustbuilder/.cargo"

RUN curl -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
ENV PATH "$PATH:/rustbuilder/.cargo/bin"
RUN rustup show
RUN rustup update $STABLE --no-self-update

# BUILD RUNTIME AND BINARY
RUN rustup target add wasm32-unknown-unknown --toolchain $STABLE
RUN if [[ -z "$FEATURES" ]] ;  \
        then cd /rustbuilder/trex && RUSTC_BOOTSTRAP=1 cargo +nightly build --$PROFILE --locked;  \
        else cd /rustbuilder/trex && RUSTC_BOOTSTRAP=1 cargo +nightly build --$PROFILE --features $FEATURES --locked;  \
    fi

# ===== START SECOND STAGE ======
FROM phusion/baseimage:jammy-1.0.0
LABEL maintainer="team@trex.ink"
LABEL description="trex binary."
ARG PROFILE=release
COPY --from=builder /rustbuilder/trex/target/$PROFILE/trex /usr/local/bin

# REMOVE & CLEANUP
RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share/trex && \
	ln -s /root/.local/share/trex /data
RUN	rm -rf /usr/bin /usr/sbin

# FINAL PREPARATIONS
EXPOSE 30333 9933 9944
VOLUME ["/data"]
WORKDIR /usr/local/bin
ENTRYPOINT ["trex"]
#===== END SECOND STAGE ======
