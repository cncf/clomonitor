# Build apiserver
FROM rust:1-alpine3.17 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver clomonitor-apiserver
COPY clomonitor-archiver/Cargo.* clomonitor-archiver
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter/Cargo.* clomonitor-linter
COPY clomonitor-registrar/Cargo.* clomonitor-registrar
COPY clomonitor-tracker/Cargo.* clomonitor-tracker
WORKDIR /clomonitor/clomonitor-apiserver
RUN cargo build --release

# Build frontend
FROM node:19-alpine3.17 AS frontend-builder
WORKDIR /web
COPY web .
ENV NODE_OPTIONS=--max_old_space_size=4096
RUN yarn install
RUN yarn build

# Build docs
FROM klakegg/hugo:0.107.0 AS docs-builder
WORKDIR /
COPY scripts scripts
COPY docs docs
RUN scripts/prepare-docs.sh
WORKDIR /docs/www
RUN hugo

# Final stage
FROM alpine:3.17.1
RUN apk --no-cache add ca-certificates ttf-opensans && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-apiserver /usr/local/bin
COPY --from=frontend-builder /web/build ./web/build
COPY --from=frontend-builder /web/package.json ./web
COPY --from=docs-builder /web/build/docs ./web/build/docs
