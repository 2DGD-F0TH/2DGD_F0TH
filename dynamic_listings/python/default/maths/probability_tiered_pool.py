import random


def get_tiered_drop() -> int:
    # 1 = Common, 2 = Uncommon, 3 = Rare, 4 = Epic
    n: int = random.randint(1, 100)
    if n <= 50:
        # Common Tier
        return 1
    if n <= 80:
        # Uncommon Tier
        # Since n <=50 has already returned false, we know this
        # branch will only happen if 50<n<=80
        return 2
    if n <= 95:
        # Rare Tier
        # Since both n<=50 and n<=80 both returned false, we know
        # this branch will only happen if 80<n<=95
        return 3
    # Epic Tier
    # All other branches failed, so we'll get here only if
    # 95<n<=100
    return 4
