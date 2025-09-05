# Build tern
FROM golang:1.25.0-alpine3.22 AS tern
RUN apk --no-cache add git
RUN go install github.com/jackc/tern@latest

# Build final image
FROM alpine:3.22.1
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=tern /go/bin/tern /usr/local/bin
COPY database/migrations .
