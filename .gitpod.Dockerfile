FROM gitpod/workspace-full

RUN sudo apt-get update && sudo apt-get install -y \
    pkg-config \
    build-essential \
    libssl-dev \
    curl \
    libudev-dev \
    && sudo rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup component add rustfmt \
    && rustup component add clippy \
    && rustup target add wasm32-unknown-unknown 