class LazyObject:
    list_reference: list[int] = None

    def __init__(self, lst: list[int]):
        # Saves the reference to the original list
        self.list_reference = lst

    def __getitem__(self, index: int):
        # Calculates the halved number on-demand
        return self.list_reference[index] / 2
