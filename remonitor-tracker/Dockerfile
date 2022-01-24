# Build tracker
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev
WORKDIR /remonitor
COPY remonitor-linter remonitor-linter
COPY remonitor-tracker remonitor-tracker
WORKDIR /remonitor/remonitor-tracker
RUN cargo build --release

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates git && addgroup -S tracker && adduser -S tracker -G tracker
USER tracker
WORKDIR /home/remonitor
COPY --from=builder /remonitor/remonitor-tracker/target/release/remonitor-tracker ./tracker
CMD ["./tracker"]
