# Build apiserver
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev
WORKDIR /remonitor
COPY remonitor-apiserver remonitor-apiserver
WORKDIR /remonitor/remonitor-apiserver
RUN cargo build --release

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates && addgroup -S remonitor && adduser -S remonitor -G remonitor
USER remonitor
WORKDIR /home/remonitor
COPY --from=builder /remonitor/remonitor-apiserver/target/release/remonitor-apiserver ./apiserver
CMD ["./apiserver"]
