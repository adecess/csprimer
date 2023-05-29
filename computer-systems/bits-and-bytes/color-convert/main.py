import re
import sys


def hex_to_rgb(match_obj):
    return match_obj.group()[1:]


if __name__ == "__main__":
    input_text = sys.stdin.read()
    output_text = re.sub(r'#\w+', hex_to_rgb, input_text)
    sys.stdout.write(output_text)
