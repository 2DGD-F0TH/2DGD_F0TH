def binarySearch(lst, item):
    first = 0
    last = len(lst) - 1
    found = False
    while first <= last and not found:
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
    if not found:
        # We return None to tell we didn't find anything
        return None
