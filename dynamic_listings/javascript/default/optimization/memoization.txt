class MyMemoizedObject{
    let memory = {};

    memoizedfunction(parameter){
        if (parameter in this.memory){
            // If the result was calculated earlier, we can just return it
            return this.memory[parameter];
        }
        // If the result has never been calculated we do so.
        // ...
        // Very complex and heavy calculations here
        // ...
        result = something_complex;
        // Now we save the result in our memory, so other calls with the same parameter will be faster
        this.memory[parameter] = result;
        return result;
    }
}
