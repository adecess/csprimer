import sys
import re
import time


def beep(match_obj):
    if match_obj:
        for _ in range(int(match_obj.group())):
            sys.stdout.buffer.write(b'\x07')
            sys.stdout.flush()
            time.sleep(0.5)
        return 'beep n times'


if __name__ == "__main__":
    for line in sys.stdin:
        output_text = beep(re.fullmatch(r'^\d+$', line.rstrip('\n')))
