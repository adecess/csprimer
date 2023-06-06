import re
import sys


def hex_to_int(hex_str):
    hex_numbers = "0123456789abcdef"
    integer = 0
    for i in hex_str.lower():
        integer = (integer << 4) | hex_numbers.index(i)
    return integer


def digit_pair_list(hex_color):
    return [str(hex_to_int(hex_color[i:i + 2])) for i in range(0, len(hex_color), 2)]


def single_digit_list(hex_color):
    return [str(hex_to_int(hex_color[i]*2)) for i in range(0, len(hex_color))]


def percent_trunc(percent):
    return round(hex_to_int(percent) / 255, 5)


def hex_to_rgb(match_obj):
    hex_color = match_obj.group()[1:-1]
    rgb_colors = []
    percentage = ''
    if len(hex_color) == 6:
        rgb_colors.extend(digit_pair_list(hex_color))
    elif len(hex_color) == 3:
        rgb_colors.extend(single_digit_list(hex_color))
    elif len(hex_color) == 8:
        percentage = 'a'
        colors = hex_color[:6]
        percent = hex_color[6:]
        rgb_colors.extend(digit_pair_list(colors))
        rgb_colors.append(f"/ {percent_trunc(percent)}")
    elif len(hex_color) == 4:
        percentage = 'a'
        colors = hex_color[:3]
        percent = hex_color[3:]
        rgb_colors.extend(single_digit_list(colors))
        rgb_colors.append(f"/ {percent_trunc(percent * 2)}")

    return f"rgb{percentage}(" + " ".join(rgb_colors) + ");"


if __name__ == "__main__":
    input_text = sys.stdin.read()
    output_text = re.sub(r'#\w+;', hex_to_rgb, input_text)
    sys.stdout.write(output_text)
