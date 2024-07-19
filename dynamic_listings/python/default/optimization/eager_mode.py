class LazyObject:
    halved_numbers: list[int] = None

    def __init__(self, lst: list[int]):
        # Calculates all the halved numbers immediately
        self.halved_numbers = [x/2 for x in lst]

    def __getitem__(self, index: int):
        # Calculates the halved number on-demand
        return self.halved_numbers[index]
