# Build linter
FROM rust:1.98.0-alpine3.24 as builder
RUN apk --no-cache add musl-dev perl make
WORKDIR /clomonitor
COPY Cargo.* ./
COPY clomonitor-apiserver clomonitor-apiserver
COPY clomonitor-archiver clomonitor-archiver
COPY clomonitor-core clomonitor-core
COPY clomonitor-linter clomonitor-linter
COPY clomonitor-notifier clomonitor-notifier
COPY clomonitor-registrar clomonitor-registrar
COPY clomonitor-runner clomonitor-runner
COPY clomonitor-tracker clomonitor-tracker
WORKDIR /clomonitor/clomonitor-linter
RUN cargo build --release --locked

# Build OpenSSF scorecard
FROM golang:1.26.4-alpine3.22 AS scorecard
WORKDIR /tmp
RUN apk --no-cache add git make bash gcc musl-dev binutils-gold
RUN git clone https://github.com/ossf/scorecard
WORKDIR /tmp/scorecard
RUN git checkout v4.13.0
RUN make install
RUN make build

# Install AFDocs using the locked dependency set
FROM alpine:3.24.1 AS afdocs
RUN apk --no-cache add nodejs npm
WORKDIR /opt/afdocs
COPY clomonitor-runner/afdocs/package*.json ./
RUN npm ci --omit=dev --ignore-scripts

# Final stage
FROM alpine:3.24.1
RUN apk --no-cache add nodejs \
    && addgroup -S clomonitor \
    && adduser -S clomonitor -G clomonitor
COPY --from=builder /clomonitor/target/release/clomonitor-linter /usr/local/bin
COPY --from=scorecard /tmp/scorecard/scorecard /usr/local/bin/scorecard
COPY --from=afdocs /opt/afdocs /opt/afdocs
RUN ln -s /opt/afdocs/node_modules/.bin/afdocs /usr/local/bin/afdocs
USER clomonitor
WORKDIR /home/clomonitor
