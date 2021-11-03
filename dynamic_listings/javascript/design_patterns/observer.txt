class Subject{
    /* This is the observed class that contains the list of observers and
     * the notifyObservers method */

    constructor(){
        this.observers = [];
    }

    register_observer(observer){
        observers.push(observer);
    }

    notifyObservers(){
        for (const observer in this.observers){
            observer.update();
        }
    }
}


class Observer{
    /* This is the class that contains the update method, used to force
     * an update in the observer */

    update(){
        console.log("I have been updated!");
    }
}


let subject = new Subject();
let observer = new Observer();
subject.register_observer(observer);
subject.notifyObservers();
