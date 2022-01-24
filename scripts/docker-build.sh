#!/bin/sh

# db-migrator
docker build \
    -f database/migrations/Dockerfile \
    -t remonitor/db-migrator \
.

# apiserver
docker build \
    -f remonitor-apiserver/Dockerfile \
    -t remonitor/apiserver \
.

# tracker
docker build \
    -f remonitor-tracker/Dockerfile \
    -t remonitor/tracker \
.
