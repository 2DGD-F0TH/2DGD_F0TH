class Observer:
    """
    This is the class that contains the update method, used to force an
    update in the observer
    """

    def update(self) -> None:
        print("I have been updated!")


class Subject:
    """
    This is the observed class that contains the list of observers
    and the notifyObservers method
    """

    def __init__(self) -> None:
        self.observers: list[Observer] = []

    def register_observer(self, observer: Observer) -> None:
        self.observers.append(observer)

    def notify_observers(self) -> None:
        for observer in self.observers:
            observer.update()


subject: Subject = Subject()
observer: Observer = Observer()
subject.register_observer(observer)
subject.notify_observers()
