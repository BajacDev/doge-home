# inspired by https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/

PI_IP=localhost # Be sure to change this!
#TARGET=armv7-unknown-linux-gnueabihf # Pi 2/3/4
TARGET=arm-unknown-linux-gnueabihf # Pi 0/1
PORT=5022
EXEC=doge-home

echo "build binary"
cross build --target $TARGET

echo "upload binary"
sshpass -p 'raspberry' scp -P $PORT -r ./target/$TARGET/debug/$EXEC pi@$PI_IP:/home/pi

echo "execute binary"
sshpass -p 'raspberry' ssh -p $PORT pi@$PI_IP "./$EXEC"