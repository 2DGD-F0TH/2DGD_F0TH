#!/usr/bin/env python
from os.path import join as pjoin
from os import walk, listdir


def main() -> None:
    """
    Simple script to visually check the status of listings porting
    """
    languages: list = listdir("../listings")
    for language in languages:
        files: set = {item
                      for _, _, fn in walk(pjoin("../listings", language))
                      for item in fn}
        print("[{}]".format(language).rjust(15, " "),
              "[{}]".format("*" * len(files)))


if __name__ == "__main__":
    main()
