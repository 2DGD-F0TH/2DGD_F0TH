def bubbleSort(lst):
    n = len(lst)
    # Traverse through all elements
    for i in range(n):
        # Last i elements are in place due to the nature of the sort
        for j in range(0, n - i - 1):
            # Swap if the element found is greater than the next element
            if lst[j] > lst[j+1]:
                # Use python's swapping and unpacking
                lst[j], lst[j+1] = lst[j+1], lst[j]
