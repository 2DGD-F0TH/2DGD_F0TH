#!/usr/bin/env python3
import random


def main() -> None:
    happened: int = 0
    # Monte Carlo Method we do 10000 "extractions"
    for _ in range(10000):
        # Get a random number between 1 and 5
        n: int = random.randint(1, 5)
        if n == 1:
            # If it's 1, we have a match!
            happened += 1
    # We print the result
    print(happened / 10000)


if __name__ == '__main__':
    # This is some boilerplate to execute directly from shell
    main()
