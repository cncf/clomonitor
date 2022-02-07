# Build apiserver
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
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

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-apiserver/target/release/clomonitor-apiserver /usr/local/bin
COPY --from=frontend-builder /web/build ./web/build
COPY --from=frontend-builder /web/package.json ./web
