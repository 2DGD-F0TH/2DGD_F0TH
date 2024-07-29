# -------------------------------------
#arr is an array of integers
n = len(arr)
# Traverse through all elements
for i in range(n):
    # Last i elements are in place due to the nature of the sort
    for j in range(0, n - i - 1):
        # Swap if the element found is greater than the next element
        if arr[j] > arr[j+1] :
            # Use python's swapping and unpacking
            arr[j], arr[j+1] = arr[j+1], arr[j]

# -------------------------------------

for item in arr:
    print(item)
