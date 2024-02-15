class Strategy{
    // This class defines the strategy interface the client will refer to

    public:
        // This algorithm will be implemented by the subclasses
        virtual void algorithm() = 0;
};

class ConcreteStrategy1: public Strategy{
    public:
        void algorithm() override{
            // Real implementation of the algorithm
            // DO STUFF
        }
};

class ConcreteStrategy2: public Strategy{
    public:
        void algorithm() override{
            // Real implementation of the algorithm
            // DO STUFF SLIGHTLY DIFFERENTLY
        }
};

// Example Usage
int main(){
    Strategy* to_execute;
    if (condition){
        to_execute = new ConcreteStrategy1();
    }else{
        to_execute = new ConcreteStrategy2();
    }
    to_execute->algorithm();  // This will execute 1 or 2 depending on "condition"
}
