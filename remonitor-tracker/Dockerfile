# Build tracker
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev openssl-dev
WORKDIR /remonitor
COPY remonitor-linter remonitor-linter
COPY remonitor-tracker remonitor-tracker
WORKDIR /remonitor/remonitor-tracker
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates git openssl && addgroup -S remonitor && adduser -S remonitor -G remonitor
USER remonitor
WORKDIR /home/remonitor
COPY --from=builder /remonitor/remonitor-tracker/target/release/remonitor-tracker /usr/local/bin
