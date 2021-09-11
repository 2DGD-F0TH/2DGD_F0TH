#!/usr/bin/env python
"""
A small script to show the status of listings translations
"""
from os.path import join as pjoin
from os.path import dirname
from os import walk, listdir

DIRECTORY = pjoin(dirname(__file__), "../dynamic_listings")


def main() -> None:
    """
    Simple script to visually check the status of listings porting
    """
    languages: list = listdir(DIRECTORY)
    files: set = set()
    for language in languages:
        files = {
            item
            for _, _, fn in walk(pjoin(DIRECTORY, language))
            for item in fn
            if item.endswith("txt")
        }
        print("[{}]".format(language).rjust(15, " "),
              "{}".format("|" * len(files)))

    pseudocode_files: set = {
        item
        for _, _, fn in walk(pjoin(DIRECTORY, "pseudocode"))
        for item in fn
    }
    for language in languages:
        if language != "pseudocode":
            file_list: set = {
                item
                for _, _, fn in walk(pjoin(DIRECTORY, language))
                for item in fn
            }
            files = {
                item
                for item in pseudocode_files
                if item not in file_list
            }
            if files:
                print("Missing Listings: {}".format(language))
                print(30 * "-")
                for fil in files:
                    print(fil)
                print(30 * "=")


if __name__ == "__main__":
    main()
