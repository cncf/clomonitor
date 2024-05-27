# Build tracker
FROM rust:1-alpine3.19 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver/Cargo.* clomonitor-apiserver
COPY clomonitor-archiver/Cargo.* clomonitor-archiver
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-notifier/Cargo.* clomonitor-notifier
COPY clomonitor-registrar/Cargo.* clomonitor-registrar
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-tracker
RUN cargo build --release

# Build OpenSSF scorecard
FROM golang:1.22.3-alpine3.19 AS scorecard
WORKDIR /tmp
RUN apk --no-cache add git make bash gcc musl-dev binutils-gold
RUN git clone https://github.com/ossf/scorecard
WORKDIR /tmp/scorecard
RUN git checkout v4.13.0
RUN make install
RUN make build

# Final stage
FROM alpine:3.20.0
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-tracker /usr/local/bin
COPY --from=scorecard /tmp/scorecard/scorecard /usr/local/bin/scorecard
