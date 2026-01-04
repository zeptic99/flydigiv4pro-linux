import os
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
