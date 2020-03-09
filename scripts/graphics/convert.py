#!/usr/bin/env python3
"""Helper script to convert images to c source code"""
import argparse


def convert(content, width, height):
    """Convert function"""
    i = 0
    char = 0
    bit_count = 0
    for row in range(height):
        res = []
        for _col in range(width):
            while content[i] != "0" and content[i] != "1":
                i += 1
            if content[i] == "0":
                char |= 1
            if bit_count % 8 == 7:
                res.append(f"0x{char:02x}")
                char = 0
            char <<= 1
            i += 1
            bit_count += 1

        if row == height - 1 and bit_count % 8 != 0:
            char <<= 7 - (bit_count % 8)
            res.append(f"0x{char:02x}")

        yield res


def main():
    """Main function"""
    parser = argparse.ArgumentParser()
    parser.add_argument("pbmfile")
    parser.add_argument(
        "--name", help="Name to give to the resulting variable", nargs="?"
    )
    args = parser.parse_args()

    with open(args.pbmfile) as file:
        content = file.readlines()

    # Filter out comments
    content = [x for x in content if not x.startswith("#") and not x.startswith(" ")]

    (width, height) = [int(x) for x in content[1].strip().split(" ")]

    # First row is magic number (P1), second row is width/height
    content = [x for x in content[2:]]
    content_str = "".join(x for x in content)

    print(f"// width = {width}, height = {height}")
    name = args.name if args.name is not None else "data"
    print(f"static const uint8_t {name}_{width}_{height}[] = {{")
    print(",\n".join([", ".join(x) for x in convert(content_str, width, height)]))
    print("};")


if __name__ == "__main__":
    main()
