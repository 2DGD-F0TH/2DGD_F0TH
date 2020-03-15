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

    pseudocode_files:set = {item
        for _, _, fn in walk("../listings/pseudocode/")
        for item in fn
    }
    for language in languages:
        if language != "pseudocode":
            file_list: set = {item
                for _, _, fn in walk(pjoin("../listings", language))
                for item in fn
            }
            files: set ={item
                for item in pseudocode_files
                if item not in file_list
            }
            print("Missing Listings: {}".format(language))
            for f in files:
                print(f)


if __name__ == "__main__":
    main()
