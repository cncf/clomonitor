#!/bin/sh

# apiserver
docker build \
    -f clomonitor-apiserver/Dockerfile \
    -t clomonitor/apiserver \
.

# archiver
docker build \
    -f clomonitor-archiver/Dockerfile \
    -t clomonitor/archiver \
.

# dbmigrator
docker build \
    -f database/migrations/Dockerfile \
    -t clomonitor/dbmigrator \
.

# linter
docker build \
    -f clomonitor-linter/Dockerfile \
    -t clomonitor/linter \
.

# registrar
docker build \
    -f clomonitor-registrar/Dockerfile \
    -t clomonitor/registrar \
.

# tracker
docker build \
    -f clomonitor-tracker/Dockerfile \
    -t clomonitor/tracker \
.

