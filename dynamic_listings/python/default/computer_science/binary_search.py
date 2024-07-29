def binary_search(lst: list[int], item: int) -> int | None:
    first = 0
    last = len(lst) - 1
    while first <= last:
        # Find the middle element
        midpoint = (first + last) // 2
        if lst[midpoint] == item:
            # We found it!
            return midpoint
        else:
            if item < lst[midpoint]:
                # Continue on the "first half"
                last = midpoint - 1
            else:
                # Continue on the "second half"
                first = midpoint + 1
    # We return None to tell we didn't find anything
    return None