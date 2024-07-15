def count_forwards(n: int) -> None:
    # Stop condition
    if n == 0:
        # If we don't do this, we won't print 0
        print(n)
        return
    # Recursive call
    count_forwards(n-1)
    # Procedure
    print(n)
