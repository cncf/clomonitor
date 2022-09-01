# Build registrar
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-registrar clomonitor-registrar
WORKDIR /clomonitor/clomonitor-registrar
RUN cargo build --release

# Final stage
FROM alpine:3.15
RUN apk --no-cache add ca-certificates git && addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-registrar/target/release/clomonitor-registrar /usr/local/bin
