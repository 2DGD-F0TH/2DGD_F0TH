#!/usr/bin/env python
"""
A small script to show the status of listings translations
"""
import shutil
from os.path import join as pjoin
from os.path import dirname, normpath, relpath
from os import walk, listdir, sep

ROOT_DIR = dirname(dirname(__file__))
DIRECTORY = pjoin(ROOT_DIR, "dynamic_listings")

SYMBOLS = {
    "block": "▇",
    "dbl_border": "═",
    "heavy_border": "━",
    "light_border": "─",
    "tee": "├",
    "vdots": "⁞"
}


class LVAPath:
    """
    Language and Variant-agnostic path: a class that is hashable
    by its path, without considering its language and language variant
    """
    internal_path = None
    path_components: list[str] = []

    def __init__(self, path, filename):
        """
        Initialization of internal variables, by splitting and removing
        the first 2 components, and then joining the remaining
        """
        normalized_path = relpath(normpath(path), DIRECTORY)
        split_path = normalized_path.split(sep)
        self.path_components = split_path[2:]
        self.internal_path = pjoin(*self.path_components, filename)

    def __eq__(self, o):
        """
        Equality operator
        """
        return self.internal_path == o.internal_path

    def __hash__(self):
        """
        Hash operator, for set usage
        """
        return hash(self.internal_path)

    def __str__(self):
        """
        Simple toString operator
        """
        return str(self.internal_path)


def print_bar(language, max_lang_length, length, reference_max, terminal_size):
    """
    Prints a progress bar to show how many listings are complete
    """
    # Calculate the amount of blocks needed to get 100%
    bar_size = terminal_size.columns - (6 + max_lang_length)
    # Calculate the ratio between the length and the reference_max
    ratio = length / reference_max
    # The integer part of the bar, we don't really need the fractionary part
    int_part = int(ratio * bar_size)
    # Add 3 to max_lang_length to account for the square brackets [] + padding
    print(
        "[{}]".format(language).rjust(max_lang_length + 3, " "),
        "{}".format(SYMBOLS["block"] * int_part,)
    )


def main() -> None:
    """
    Simple script to visually check the status of listings porting
    """
    term_size = shutil.get_terminal_size((80, 20))
    languages: list = listdir(DIRECTORY)
    files: dict = {}
    for language in languages:
        variants = listdir(pjoin(DIRECTORY, language))
        for variant in variants:
            if variant == "default":
                langvar = language
            else:
                langvar = f"{language} + {variant}"
            files[langvar] = {
                LVAPath(d, item)
                for d, _, fn in walk(pjoin(DIRECTORY, language, variant))
                for item in fn
                if item.endswith("txt")
            }
    reference = len(files["pseudocode"])
    max_lang_length: int = len(max(files.keys(), key=len))
    print(term_size.columns * SYMBOLS["dbl_border"])
    print("Current Listings Status")
    print(term_size.columns * SYMBOLS["heavy_border"])
    for language, file_set in files.items():
        print_bar(
            language,
            max_lang_length,
            len(file_set),
            reference,
            term_size
        )

    pseudocode_files: set = files["pseudocode"]
    # ----
    for langvar, file_list in files.items():
        if langvar != "pseudocode":
            files_set = pseudocode_files - file_list
            if files_set:
                print(term_size.columns * SYMBOLS["heavy_border"])
                print("Missing Listings: {}".format(langvar))
                print(term_size.columns * SYMBOLS["light_border"])
                for fil in files_set:
                    print(fil)
    print(term_size.columns * SYMBOLS["dbl_border"])


if __name__ == "__main__":
    main()
