FROM hub.docker.palantir.build/alpine:3.1

RUN apk add --update python && rm -rf /var/cache/apk/*

COPY run.py /usr/bin
ENTRYPOINT ["/usr/bin/run.py"]

EXPOSE 1234
