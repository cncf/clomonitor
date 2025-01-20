# Build archiver
FROM rust:1-alpine3.21 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver/Cargo.* clomonitor-apiserver
COPY clomonitor-archiver clomonitor-archiver
COPY clomonitor-core/Cargo.* clomonitor-core
COPY clomonitor-linter/Cargo.* clomonitor-linter
COPY clomonitor-notifier/Cargo.* clomonitor-notifier
COPY clomonitor-registrar/Cargo.* clomonitor-registrar
COPY clomonitor-tracker/Cargo.* clomonitor-tracker
WORKDIR /clomonitor/clomonitor-archiver
RUN cargo build --release

# Final stage
FROM alpine:3.21.2
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-archiver /usr/local/bin
