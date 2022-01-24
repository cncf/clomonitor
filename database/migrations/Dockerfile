# Build refinery
FROM rust:1-alpine3.15 as refinery
RUN apk --no-cache add musl-dev openssl-dev
RUN cargo install refinery_cli

# Build final image
FROM alpine:3.15
RUN addgroup -S db-migrator && adduser -S db-migrator -G db-migrator
USER db-migrator
WORKDIR /home/remonitor
COPY --from=refinery /usr/local/cargo/bin/refinery /usr/local/bin
COPY database database
