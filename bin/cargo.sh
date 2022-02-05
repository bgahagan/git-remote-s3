#!/bin/bash
docker run --rm -v "$(dirname $0)/..":/usr/src/myapp:Z -w /usr/src/myapp rust:1 cargo "$@"
