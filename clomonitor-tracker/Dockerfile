# Build tracker
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-tracker
RUN cargo build --release

# Build OpenSSF scorecard
FROM golang:1.17-alpine3.15 AS scorecard
RUN apk --no-cache add git
RUN go get -u github.com/ossf/scorecard/v4

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-tracker/target/release/clomonitor-tracker /usr/local/bin
COPY --from=scorecard /go/bin/scorecard /usr/local/bin
