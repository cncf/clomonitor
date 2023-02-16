# Build registrar
FROM rust:1-alpine3.17 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver/Cargo.* clomonitor-apiserver
COPY clomonitor-archiver/Cargo.* clomonitor-archiver
COPY clomonitor-core/Cargo.* clomonitor-core
COPY clomonitor-linter/Cargo.* clomonitor-linter
COPY clomonitor-tracker/Cargo.* clomonitor-tracker
COPY clomonitor-registrar clomonitor-registrar
WORKDIR /clomonitor/clomonitor-registrar
RUN cargo build --release

# Final stage
FROM alpine:3.17.2
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-registrar /usr/local/bin
