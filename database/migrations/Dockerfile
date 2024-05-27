# Build tern
FROM golang:1.22.3-alpine3.19 AS tern
RUN apk --no-cache add git
RUN go install github.com/jackc/tern@latest

# Build final image
FROM alpine:3.20.0
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=tern /go/bin/tern /usr/local/bin
COPY database/migrations .
