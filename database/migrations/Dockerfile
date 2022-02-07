FROM ghcr.io/rust-db/refinery:main
RUN addgroup --system clomonitor && adduser --system clomonitor --ingroup clomonitor
USER clomonitor
WORKDIR /home/clomonitor
COPY database/migrations/schema database/migrations/schema
