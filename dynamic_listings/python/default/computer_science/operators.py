# Operators can be treated as functions, that means you can
# Assign them to a variable.

# This...

if a == b and c == d:
    # Do something...
    ...

# Is equivalent to this

complex_condition: bool = (a == b and c == d)

if complex_condition:
    # ...
    ...

# ---------------8<---------------

# Also this...

def thing(a: int, b: int) -> bool:
    if a == b:
        return True
    else:
        return False

# Is equivalent to...

def thing(a: int, b: int) -> bool:
    return a == b
