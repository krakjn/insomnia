FROM rust:alpine
# for static binaries
ENV CGO_ENABLED=0
RUN apk add --no-cache go 
# dbus dbus-dev gcc musl-dev
CMD [ "/bin/ash" ]
