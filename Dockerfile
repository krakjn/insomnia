FROM rust:alpine
RUN apk add --no-cache dbus dbus-dev gcc musl-dev
CMD [ "/bin/ash" ]