def count_backwards(n: int):
    # Stop condition
    if n == 0:
        # If we don't do this, we won't print 0
        print(n)
        return
    # Procedure
    print(n)
    # Recursive call
    count_backwards(n-1)
