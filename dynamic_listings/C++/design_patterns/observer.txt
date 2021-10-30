#include <vector>
#include <iostream>

class Subject{
    /* This is the observed class that contains the list of observers and
     * the notifyObservers method */

    private:
        std::vector<Observer*> observers* = new std::vector<Observer*>();

    public:
        void register_observer(Observer* observer){
            observers->push_back(observer);
        }

        void notifyObservers(){
            for (Observer* observer: observers){
                observer->update();
            }
        }
};


class Observer{
    /* This is the class that contains the update method, used to force
     * an update in the observer */

    public:
        void update(){
            std::cout << "I have been updated!" << std::endl;
        }
};


int main(){
    Subject subject = Subject();
    Observer observer = Observer();
    subject.register_observer(&observer);
    subject.notifyObservers();
}
