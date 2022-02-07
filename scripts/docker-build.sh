#!/bin/sh

# dbmigrator
docker build \
    -f database/migrations/Dockerfile \
    -t clomonitor/dbmigrator \
.

# apiserver
docker build \
    -f clomonitor-apiserver/Dockerfile \
    -t clomonitor/apiserver \
.

# tracker
docker build \
    -f clomonitor-tracker/Dockerfile \
    -t clomonitor/tracker \
.
