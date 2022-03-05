#!/usr/bin/env python
"""
A small script to show the status of listings translations
"""
import shutil
from os.path import join as pjoin
from os.path import dirname
from os import walk, listdir

DIRECTORY = pjoin(dirname(__file__), "../dynamic_listings")


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
        "{}".format("▇" * int_part,)
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
                item
                for _, _, fn in walk(pjoin(DIRECTORY, language, variant))
                for item in fn
                if item.endswith("txt")
            }
    reference = len(files["pseudocode"])
    max_lang_length: int = len(max(files.keys(), key=len))
    print(term_size.columns * "═")
    print("Current Listings Status")
    print(term_size.columns * "━")
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
                print(term_size.columns * "━")
                print("Missing Listings: {}".format(langvar))
                print(term_size.columns * "─")
                for fil in files_set:
                    print(fil)
    print(term_size.columns * "═")


if __name__ == "__main__":
    main()
