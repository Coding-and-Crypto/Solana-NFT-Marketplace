FROM ubuntu

RUN apt update && apt upgrade -y
RUN apt install build-essential bzip2 curl libssl-dev libudev-dev pkg-config wget -y

# Install node & npm using nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . $NVM_DIR/nvm.sh && nvm install 16.15.1
ENV PATH=$NVM_DIR/versions/node/v16.15.1/bin:$PATH
RUN npm i -g npm@8.12.2 yarn

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -q
ENV PATH=/root/.cargo/bin:$PATH

# Install Solana
RUN wget -o solana-release.tar.bz2 https://github.com/solana-labs/solana/releases/download/v1.10.25/solana-release-x86_64-unknown-linux-gnu.tar.bz2
RUN tar jxf solana-release-x86_64-unknown-linux-gnu.tar.bz2
ENV PATH=$PWD/solana-release/bin:$PATH
RUN mkdir -p /root/.config/solana

# Install Anchor CLI
RUN cargo install --git https://github.com/project-serum/anchor anchor-cli

# ## Copy your local Solana keypair into the "wallet" folder
COPY wallet/keypair.json /root/.config/solana/id.json

RUN solana config set --url https://api.devnet.solana.com
RUN solana config set --keypair /root/.config/solana/id.json

WORKDIR anchor

# TODO: libssl.so.1.1 problem