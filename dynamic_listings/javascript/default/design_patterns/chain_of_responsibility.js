class Handler{
    // This is the handler abstract/class interface that the sender connects to
    constructor(){
        // In javascript there are no real facilities for abstract classes
        // So we need to play around
        if (new.target == Handler){
            // The new.target changes in derivative classes
            throw new TypeError("Cannot construct abstract objects")
        }
        this.next = null;  // The next handler in the chain
    }

    handle_request(){
        if (condition){
            // In case I can handle this request
            return this.real_handler();
        }

        if (this.next != null){
            return next.handle_request();
        }
    }

    real_handler(){
        // This function gets implemented in the concrete classes
    }

    add_handler(new_handler){
        this.next = new_handler;
        return this.next;  // Allows for chaining .add_handler().add_handler()...
    }
}


