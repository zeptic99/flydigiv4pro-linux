import os, fcntl

HIDIOCGFEATURE = 0xC0404807  # x86_64 Linux; worked for you


def get(rid):
    fd = os.open("/dev/hidraw5", os.O_RDONLY)
    buf = bytearray([rid] + [0x00]*63)
    try:
        fcntl.ioctl(fd, HIDIOCGFEATURE, buf, True)
        return bytes(buf)
    finally:
        os.close(fd)


for rid in (0x00, 0x04, 0x05, 0x06):
    try:
        data = get(rid)
        print(f"rid req {rid:02x} -> {data[:8].hex()}  (len={len(data)})")
    except OSError as e:
        print(f"rid req {rid:02x} -> ERROR: {e}")
