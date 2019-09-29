FROM alpine

RUN apk add --no-cache git gnupg py-pip && \
    pip install awscli

WORKDIR /test

COPY run-tests.sh .

RUN ./run-tests.sh
