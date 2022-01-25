FROM ghcr.io/rust-db/refinery:main
RUN addgroup --system remonitor && adduser --system remonitor --ingroup remonitor
USER remonitor
WORKDIR /home/remonitor
COPY database/migrations/schema database/migrations/schema
