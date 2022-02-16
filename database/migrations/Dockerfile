# Build tern
FROM golang:1.17-alpine3.15 AS tern
RUN apk --no-cache add git
RUN go get -u github.com/jackc/tern

# Build final image
FROM alpine:3.15
RUN addgroup -S clomonitor && adduser -S clomonitor -G clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY --from=tern /go/bin/tern /usr/local/bin
COPY database/migrations .
