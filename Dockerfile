FROM rust:alpine
RUN apk add --no-cache dbus dbus-dev musl-dev
CMD [ "/bin/ash" ]
