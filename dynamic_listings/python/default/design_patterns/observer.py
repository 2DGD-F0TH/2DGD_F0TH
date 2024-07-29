class Subject:
    """
    This is the observed class that contains the list of observers
    and the notifyObservers method
    """

    def __init__(self):
        self.observers = []

    def register_observer(self, observer: Observer):
        self.observers.append(observer)

    def notify_observers(self):
        for observer in self.observers:
            observer.update()

class Observer:
    """
    This is the class that contains the update method, used to force an
    update in the observer
    """

    def update(self):
        print("I have been updated!");


subject = Subject()
observer = Observer()
subject.register_observer(observer)
subject.notify_observers()
