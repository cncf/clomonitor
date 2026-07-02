# Build notifier
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
WORKDIR /clomonitor/clomonitor-notifier
RUN cargo build --release --locked

# Final stage
FROM alpine:3.24.1
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-notifier /usr/local/bin
