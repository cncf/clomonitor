# Build tracker
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-tracker
RUN cargo build --release

# Get OpenSSF scorecard binary
FROM alpine:3.15 AS scorecard
WORKDIR /tmp
RUN wget -O scorecard-linux-amd64 https://github.com/ossf/scorecard/releases/download/v4.4.0/scorecard-linux-amd64
RUN chmod +x scorecard-linux-amd64

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-tracker/target/release/clomonitor-tracker /usr/local/bin
COPY --from=scorecard /tmp/scorecard-linux-amd64 /usr/local/bin/scorecard
