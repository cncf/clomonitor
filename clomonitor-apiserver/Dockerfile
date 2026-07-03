# Build apiserver
FROM rust:1.96.1-alpine3.24 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver clomonitor-apiserver
COPY clomonitor-archiver clomonitor-archiver
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-notifier clomonitor-notifier
COPY clomonitor-registrar clomonitor-registrar
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-apiserver
RUN cargo build --release --locked

# Build frontend
FROM node:24-alpine3.23 AS frontend-builder
RUN apk --no-cache add git
WORKDIR /web
COPY web .
ENV NODE_OPTIONS=--max_old_space_size=4096
RUN yarn install --network-concurrency 1
RUN yarn build

# Build docs
FROM klakegg/hugo:0.111.3 AS docs-builder
WORKDIR /
COPY scripts scripts
COPY docs docs
RUN scripts/prepare-docs.sh
WORKDIR /docs/www
RUN hugo

# Final stage
FROM alpine:3.24.1
RUN apk --no-cache add ca-certificates ttf-opensans && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-apiserver /usr/local/bin
COPY --from=frontend-builder /web/build ./web/build
COPY --from=frontend-builder /web/package.json ./web
COPY --from=docs-builder /web/build/docs ./web/build/docs
