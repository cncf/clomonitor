# Build linter
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
WORKDIR /clomonitor/clomonitor-linter
RUN cargo build --release

# Get OpenSSF scorecard binary
FROM alpine:3.15 AS scorecard
WORKDIR /tmp
RUN wget -O scorecard.tar.gz https://github.com/ossf/scorecard/releases/download/v4.4.0/scorecard_4.4.0_linux_amd64.tar.gz
RUN tar xvfz scorecard.tar.gz

# Final stage
FROM alpine:3.15
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-linter/target/release/clomonitor-linter /usr/local/bin
COPY --from=scorecard /tmp/scorecard-linux-amd64 /usr/local/bin/scorecard
