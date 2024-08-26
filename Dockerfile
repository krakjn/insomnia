FROM rust:alpine
RUN apk add --no-cache go 
# dbus dbus-dev gcc musl-dev
CMD [ "/bin/ash" ]
