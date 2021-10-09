class LazyObject:
    list_reference = None

    def __init__(self, lst):
        # Saves the reference to the original list
        self.list_reference = lst

    def __getitem__(self, index):
        # Calculates the halved number on-demand
        return self.list_reference[index] / 2
