# Build linter
FROM rust:1-alpine3.15 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
WORKDIR /clomonitor/clomonitor-linter
RUN cargo build --release

# Final stage
FROM alpine:3.15
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=builder /clomonitor/clomonitor-linter/target/release/clomonitor-linter /usr/local/bin
