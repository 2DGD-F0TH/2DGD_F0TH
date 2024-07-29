#!/usr/bin/env python3
import random


def main() -> None:
    happened: int = 0
    # Monte Carlo Method we do 10000 "extractions"
    for _ in range(10000):
        # Get a random number between 1 and 100
        n: int = random.randint(1, 100)
        if n <= 13:
            # If it's less or equal than 13, we got a match
            happened += 1
    # We print the result
    print(happened / 10000)


if __name__ == '__main__':
    # This is some boilerplate to execute directly from shell
    main()
