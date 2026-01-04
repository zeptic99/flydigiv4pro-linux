#!/usr/bin/env python3
import os
import time

PATH = "/dev/hidraw5"
RID = 0x05


def send(fd: int, payload63: bytes) -> None:
    if len(payload63) != 63:
        raise ValueError(f"payload must be 63 bytes, got {len(payload63)}")
    os.write(fd, bytes([RID]) + payload63)


def stop(fd: int) -> None:
    send(fd, bytes([0] * 63))


def pulse(fd: int, payload63: bytes, on_ms: int = 250) -> None:
    send(fd, payload63)
    time.sleep(on_ms / 1000.0)
    stop(fd)
    time.sleep(0.15)


def mk_bytearray(kv: dict[int, int]) -> bytes:
    p = bytearray([0] * 63)
    for idx, val in kv.items():
        if not (0 <= idx <= 62):
            raise ValueError("payload index out of range 0..62")
        p[idx] = val & 0xFF
    return bytes(p)


def main() -> None:
    fd = os.open(PATH, os.O_WRONLY)
    try:
        print("Sweeping commands; hold the controller. Ctrl+C to stop.")
        cmd_range = range(0x00, 0x21)  # 0..0x20

        patterns = [
            # cmd at payload[0], motors at [1],[2]
            lambda c: mk_bytearray({0: c, 1: 0xFF, 2: 0xFF}),
            lambda c: mk_bytearray({0: c, 1: 0xFF, 2: 0x00}),
            lambda c: mk_bytearray({0: c, 1: 0x00, 2: 0xFF}),

            # cmd at payload[0], motors at [2],[3]
            lambda c: mk_bytearray({0: c, 2: 0xFF, 3: 0xFF}),
            lambda c: mk_bytearray({0: c, 2: 0xFF, 3: 0x00}),
            lambda c: mk_bytearray({0: c, 2: 0x00, 3: 0xFF}),

            # cmd at payload[1] instead
            lambda c: mk_bytearray({1: c, 2: 0xFF, 3: 0xFF}),
            lambda c: mk_bytearray({1: c, 2: 0xFF, 3: 0x00}),
            lambda c: mk_bytearray({1: c, 2: 0x00, 3: 0xFF}),
        ]

        for c in cmd_range:
            for pat in patterns:
                pulse(fd, pat(c), on_ms=250)

        stop(fd)
        print("Done.")
    finally:
        os.close(fd)


if __name__ == "__main__":
    main()
