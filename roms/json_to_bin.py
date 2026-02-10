import binascii
import re

FP = r"./INVADERS-E.json5"


def main():
    with open(FP, "r") as f:
        lines = [line for line in f]

    lines = lines[1:-1]
    hex_strings = []
    for line in lines:
        split = [string.strip() for string in line.split(",", 15)]
        hex_strings += split[:-1]
        last = split[-1]
        last = re.split(",|]", last)
        hex_strings += [last[0]]

    with open("./INVADERS.e", "wb") as f:
        for i in range(0, len(hex_strings), 2):
            slice = hex_strings[i : i + 2]
            for hex_string in reversed(slice):
                hex_string_strip = hex_string.lstrip("0").lstrip("x")
                print(f"{hex_string = }, {hex_string_strip = }")

                f.write(binascii.unhexlify(hex_string_strip))


if __name__ == "__main__":
    main()
