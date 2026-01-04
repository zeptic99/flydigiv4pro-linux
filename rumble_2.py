#!/usr/bin/env python3
import os
import time
import errno

PATH="/dev/hidraw5"
RID=0x05
BASE_HEX = ("0458000000000800"*7) + "04580000000008"
base = bytes.fromhex(BASE_HEX)
assert len(base)==63

def open_fd():
    while True:
        try:
            return os.open(PATH, os.O_WRONLY)
        except FileNotFoundError:
            time.sleep(0.2)

def write_payload(fd, p63):
    os.write(fd, bytes([RID]) + p63)

def safe_write(fd, p63):
    try:
        write_payload(fd, p63)
        return fd
    except OSError as e:
        if e.errno == errno.ENODEV:
            # controller reset/disconnected: reopen
            try:
                os.close(fd)
            except Exception:
                pass
            print("ENODEV: controller reset; waiting for hidraw to return...")
            time.sleep(0.5)
            fd = open_fd()
            return fd
        raise

def pulse_opcode(fd, opcode, a=0xFF, b=0xFF, ms=250):
    p = bytearray(base)
    p[0] = opcode      # opcode only
    p[2] = a
    p[3] = b
    fd = safe_write(fd, bytes(p))
    time.sleep(ms/1000)
    fd = safe_write(fd, base)
    time.sleep(0.25)
    return fd

def main():
    fd = open_fd()
    print("Opcode probe (robust). Ctrl+C to stop.")
    # Avoid 0x02 (reset). Probe a tiny set around it.
    for opcode in (0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08):
        print(f"opcode 0x{opcode:02x}")
        fd = pulse_opcode(fd, opcode)
    os.close(fd)

if __name__ == "__main__":
    main()
