# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

FROM ubuntu:22.04
LABEL maintainer="team@trex.ink"
LABEL description="trex builder."

ARG FEATURES
ARG PROFILE=release
ARG STABLE=nightly
WORKDIR /rustbuilder

# UPDATE RUST DEPENDENCIES
ENV RUSTUP_HOME "/rustbuilder/.rustup"
ENV CARGO_HOME "/rustbuilder/.cargo"
ENV RUST_TOOLCHAIN nightly-2022-08-18

# PREPARE OPERATING SYSTEM & BUILDING ENVIRONMENT
RUN apt-get update && \
	apt-get install -y pkg-config libssl-dev git clang libclang-dev diffutils gcc make m4 build-essential curl file cmake protobuf-compiler libprotobuf-dev

RUN curl -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
RUN . $CARGO_HOME/env
ENV PATH "$PATH:/rustbuilder/.cargo/bin"
RUN rustup default $RUST_TOOLCHAIN
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-gc
RUN rm -rf /root/.cargo/registry && rm -rf /root/.cargo/git

COPY . /rustbuilder/trex
VOLUME ["/src"]