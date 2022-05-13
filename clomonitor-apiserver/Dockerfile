# Build apiserver
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-core clomonitor-core
COPY clomonitor-apiserver clomonitor-apiserver
WORKDIR /clomonitor/clomonitor-apiserver
RUN cargo build --release

# Build frontend
FROM node:14-alpine3.15 AS frontend-builder
WORKDIR /web
COPY web .
ENV NODE_OPTIONS=--max_old_space_size=4096
RUN yarn install
RUN yarn build

# Build docs
FROM klakegg/hugo:0.78.2 AS docs-builder
WORKDIR /
COPY scripts scripts
COPY docs docs
RUN scripts/prepare-docs.sh
WORKDIR /docs/www
RUN hugo

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates terminus-font ttf-inconsolata ttf-dejavu font-bitstream-type1 font-bitstream-speedo font-bitstream-75dpi font-bitstream-100dpi font-noto ttf-font-awesome font-noto-extra && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-apiserver/target/release/clomonitor-apiserver /usr/local/bin
COPY --from=frontend-builder /web/build ./web/build
COPY --from=frontend-builder /web/package.json ./web
COPY --from=docs-builder /web/build/docs ./web/build/docs
