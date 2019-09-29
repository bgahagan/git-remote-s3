#!/bin/bash

exec docker run -p 9001:9000 -i --rm \
  -e MINIO_ACCESS_KEY=test \
  -e MINIO_SECRET_KEY=test1234 \
  -e MINIO_DOMAIN=localhost \
  --name git_remote_s3_minio \
  minio/minio server /home/shared
