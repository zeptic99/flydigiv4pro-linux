import os, time

path = "/dev/hidraw6"  # MUST map to ep 0x05
fd = os.open(path, os.O_WRONLY)

def rumble(strong, weak):
    pkt = bytes([0x05, 0x0f, strong, weak] + [0x00]*28)
    os.write(fd, pkt)

# rumble on
rumble(0xf7, 0xf7)
time.sleep(1.5)

# rumble off
rumble(0x00, 0x00)

os.close(fd)
