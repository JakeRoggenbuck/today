#!/usr/bin/python3
"""Create a file with the current date"""

from datetime import datetime


def main():
    d = datetime.now().date()

    with open(f"{d}.md", "w") as file:
        file.write(f"{d}")


if __name__ == "__main__":
    main()
