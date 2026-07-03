# Build tracker
FROM rust:1.96.1-alpine3.24 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver clomonitor-apiserver
COPY clomonitor-archiver clomonitor-archiver
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-notifier clomonitor-notifier
COPY clomonitor-registrar clomonitor-registrar
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-tracker
RUN cargo build --release --locked

# Build OpenSSF scorecard
FROM golang:1.24.7-alpine3.22 AS scorecard
WORKDIR /tmp
RUN apk --no-cache add git make bash gcc musl-dev binutils-gold
RUN git clone https://github.com/ossf/scorecard
WORKDIR /tmp/scorecard
RUN git checkout v4.13.0
RUN make install
RUN make build

# Final stage
FROM alpine:3.24.1
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-tracker /usr/local/bin
COPY --from=scorecard /tmp/scorecard/scorecard /usr/local/bin/scorecard
