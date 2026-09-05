#[derive(Default)]
struct Subject {
    /* This is the observed class that contains the list of observers and
     * the notifyObservers method */
    observers: Vec<Observer>,
}

impl Subject {
    fn register_observer(&mut self, observer: Observer) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update();
        }
    }
}

#[derive(Default)]
struct Observer {
    /* This is the class that contains the update method, used to force
     * an update in the observer */
}

impl Observer {
    fn update(&self) {
        println!("I have been updated!");
    }
}

fn main() {
    let mut subject = Subject::default();
    let observer = Observer::default();
    subject.register_observer(observer);
    subject.notify_observers();
}
