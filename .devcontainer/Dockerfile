# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
RUN apt-get update && apt-get -y install postgresql-client gcc-multilib npm nodejs
RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN cargo install diesel_cli
RUN curl -L git.io/nodebrew | perl - setup
# CMD export PATH=$HOME/.nodebrew/current/bin:$PATH
# RUN nodebrew install v16.14.2
EXPOSE [3000, 8080]
