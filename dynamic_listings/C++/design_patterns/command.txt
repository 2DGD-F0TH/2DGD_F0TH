class Command{
    // This is the abstract class that will be used as interface
    public:
        virtual void execute() = 0;
};

class JumpCommand: public Command{
    // This will implement the execute method
    public:
        void execute(){
            jump();
        }

    private:
        void jump(){
            // DO STUFF
        }
};
