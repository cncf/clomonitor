# Build linter
FROM rust:1-alpine3.16 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver/Cargo.* clomonitor-apiserver
COPY clomonitor-registrar/Cargo.* clomonitor-registrar
COPY clomonitor-tracker/Cargo.* clomonitor-tracker
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
WORKDIR /clomonitor/clomonitor-linter
RUN cargo build --release

# Get OpenSSF scorecard binary
FROM alpine:3.16 AS scorecard
WORKDIR /tmp
RUN wget -O scorecard.tar.gz https://github.com/ossf/scorecard/releases/download/v4.6.0/scorecard_4.6.0_linux_amd64.tar.gz
RUN tar xvfz scorecard.tar.gz
RUN chown root:root scorecard-linux-amd64
RUN chmod 755 scorecard-linux-amd64

# Final stage
FROM alpine:3.16.2
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-linter/target/release/clomonitor-linter /usr/local/bin
COPY --from=scorecard /tmp/scorecard-linux-amd64 /usr/local/bin/scorecard
