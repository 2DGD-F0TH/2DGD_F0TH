class Handler{
    // This is the handler abstract/class interface that the sender connects to
    private:
        Handler* next = nullptr;  // The next handler in the chain

        // This function gets implemented in the concrete classes
        virtual void real_handler() = 0;

    public:
        void handle_request(){
            if (condition){
                // In case I can handle this request
                return real_handler();
            }

            if (next){
                return next->handle_request();
            }
        }

        Handler& add_handler(Handler* new_handler){
            next = new_handler;
            return *next;  // Allows for chaining .add_handler().add_handler()...
        }
};
