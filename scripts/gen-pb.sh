#!/bin/bash

set -xe

export GENERATE_PB=1
rm -rf src/generated/pb
mkdir -p src/generated/pb
cargo clean -p cloudray-agent
cargo build || true

mv src/generated/pb/cloudray.agent.a2o.rs src/generated/pb/a2o.rs
mv src/generated/pb/cloudray.agent.o2a.rs src/generated/pb/o2a.rs
