import re
import sys


def hex_to_int(hex_str):
    hex_numbers = "0123456789abcdef"
    integer = 0
    for i in hex_str:
        integer = integer * 16 + hex_numbers.index(i)
    return integer


def hex_to_rgb(match_obj):
    hex_color = match_obj.group()[1:]
    return "rgb(" + " ".join([str(hex_to_int(hex_color[i:i+2])) for i in range(0, len(hex_color), 2)]) + ")"


if __name__ == "__main__":
    input_text = sys.stdin.read()
    output_text = re.sub(r'#\w+', hex_to_rgb, input_text)
    sys.stdout.write(output_text)
