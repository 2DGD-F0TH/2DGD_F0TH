class EagerObject:
    halved_numbers = None

    def __init__(self, lst):
        # Calculates all the halved numbers immediately
        self.halved_numbers = [x/2 for x in lst]

    def __getitem__(self, index):
        # Calculates the halved number on-demand
        return self.halved_numbers[index]
