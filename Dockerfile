##
##  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
##
##  Licensed with GNU Affero General Public License v3.0 or later
##

FROM node:alpine as tailwind

COPY tailwind.config.js tailwind.config.js
RUN npx tailwindcss-cli@latest build -o tailwind.css


FROM rust:slim as builder

# Install required dependencies
#RUN apk add --no-cache musl-dev openssl-dev mariadb-dev npm
RUN DEBIAN_FRONTEND=noninteractive apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get -y upgrade
RUN DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends default-libmysqlclient-dev
RUN DEBIAN_FRONTEND=noninteractive apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# Create new project
RUN USER=root cargo new --bin nekosu_web

# Create user for nekosu-web and chown directory
RUN addgroup --system --gid 1000 nekosu && adduser --system --uid 1000 --gid 1000 nekosu
RUN chown -R nekosu:nekosu /nekosu_web

# Switch to the project directory and user
WORKDIR /nekosu_web
USER nekosu

# Expose port and set entrypoint
EXPOSE 8080
CMD ["./target/release/nekosu-web"]

# Precompile dependencies
COPY --chown=nekosu:nekosu Cargo.toml Cargo.toml
RUN cargo build --release

# Remove precompile garbage
RUN rm src/*.rs
RUN rm target/release/deps/nekosu_web*

# Copy over assets
COPY --chown=nekosu:nekosu assets assets/
RUN mkdir -p ./assets/css/
COPY --chown=nekosu:nekosu --from=tailwind tailwind.css ./assets/css/tailwind.css

# Add the source code and build the final project
COPY --chown=nekosu:nekosu src src
COPY --chown=nekosu:nekosu templates templates
RUN cargo build --release