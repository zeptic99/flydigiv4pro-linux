import os
import time
print("hi, im working")
fd = os.open("/dev/hidraw5", os.O_WRONLY)
print("opened hidraw5")
buf = bytes([0x05] + [0x00]*63)
os.write(fd, buf)
print("written")
os.close(fd)
print("closed")

path = "/dev/hidraw5"
buf = bytes([0x05] + [0x00]*63)
fd = os.open(path, os.O_WRONLY)
try:
    n = os.write(fd, buf)
    print("wrote", n, "bytes")
finally:
    os.close(fd)

path = "/dev/hidraw5"
fd = os.open(path, os.O_WRONLY)


def send(b):

    buf = bytearray([0x05] + [0x00]*63)
    for i, v in b.items():
        buf[i] = v
    os.write(fd, bytes(buf))


# Try motor guesses
tests = [


    {1: 0x01, 2: 0xff},
    {1: 0x01, 3: 0xff},
    {2: 0xff, 3: 0xff},
    {4: 0xff},
]

for t in tests:
    print("sending", t)
    send(t)
    time.sleep(1)
    send({})     # stop
    time.sleep(0.5)

os.close(fd)
