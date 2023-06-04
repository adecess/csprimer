import struct
import math


def zero_extend_to_seven(bin_num: str) -> str:
    # zero extend up to a multiple of 7 bits
    return bin_num.zfill(math.ceil(len(bin_num) / 7) * 7)


def add_one_bit(byte: str, index: int) -> str:
    bit = '0' if index == 0 else '1'
    return bit + byte


def split_into_groups(extended_binary: str) -> list:
    # split into 7-bit groups
    return [extended_binary[i:i + 7] for i in range(0, len(extended_binary), 7)]


def seven_to_eight(seven_bit_list: list) -> list:
    # turn each in octets (set most significant bit on each byte except last byte)
    return [add_one_bit(seven_bit_list[i], i) for i in range(0, len(seven_bit_list))]


def binary_list_to_byte_list(byte_list: list) -> list:
    # put in little endian order and turn into a list of byte objects
    return [struct.pack('B', int(bit_string, 2)) for bit_string in reversed(byte_list)]


def hex_to_bits(byte_string: bytes) -> list:
    # turn each byte objects into binary strings and put them into a list
    return [format(byte_string[i], '08b') for i in range(0, len(byte_string))]


def byte_list_to_seven_bit_list(bytes_list: list) -> reversed:
    # drop continuation bits and put in little-endian order
    seven_bit_list = [byte[1:] for byte in bytes_list]
    return reversed(seven_bit_list)


def decode_int(bits_list: list) -> int:
    # concatenate and interpret as integer
    bit_string = ''.join(bits_list)
    return int(bit_string, 2)


def encode(num: int) -> bytes:
    bin_num = format(num, 'b')
    zero_extended_bin_num = zero_extend_to_seven(bin_num)
    seven_bit_groups = split_into_groups(zero_extended_bin_num)
    octets = seven_to_eight(seven_bit_groups)
    byte_objects_list = binary_list_to_byte_list(octets)
    return b''.join(byte_objects_list)


def decode(byte_string: bytes):
    bytes_list = hex_to_bits(byte_string)
    bits_list = byte_list_to_seven_bit_list(bytes_list)
    return decode_int(bits_list)


assert encode(1) == b'\x01'
assert encode(150) == b'\x96\x01'
assert encode(18446744073709551615) == b'\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01'

assert decode(b'\x01') == 1
assert decode(b'\x96\x01') == 150
assert decode(b'\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01') == 18446744073709551615


if __name__ == '__main__':
    print('ok')
