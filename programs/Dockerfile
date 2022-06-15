FROM frolvlad/alpine-glibc:alpine-3.16_glibc-2.35

RUN apk update && apk upgrade -a &&\
    apk add --update bash build-base wget curl nodejs npm eudev-dev &&\
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -q &&\
    mv root/.cargo $PWD/.cargo &&\
    wget -o solana-release.tar.bz2 https://github.com/solana-labs/solana/releases/download/v1.10.25/solana-release-x86_64-unknown-linux-gnu.tar.bz2 &&\
    tar jxf solana-release-x86_64-unknown-linux-gnu.tar.bz2 &&\
    mkdir -p /root/.config/solana &&\
    npm install -g @project-serum/anchor-cli

ENV PATH=$PWD/.cargo/bin:$PATH
# ENV PATH=$PWD/.cargo/bin:$PWD/solana-release/bin:$PATH

# ## Copy your local Solana keypair into the "wallet" folder
# COPY wallet/keypair.json /root/.config/solana/id.json

# RUN solana config set --url https://api.devnet.solana.com
# RUN solana config set --keypair /root/.config/solana/id.json
