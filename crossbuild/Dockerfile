# from https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/
# not used now, will be useful when external library will be needed in cross compilation

# Use the container that comes with `cross` as a base. It's already got
# a cross-compile toolchain installed, so that's less work for us.
FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libssl-dev:armhf libasound2-dev:armhf