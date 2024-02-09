#include <unordered_map>

class MyMemoizedObject{
    private:
        std::unordered_map<std::string, std::string>* memory = new std::unordered_map<std::string, std::string>();

    public:
        std::string memoizedfunction(string parameter){
            if (memory->contains(parameter)){
                // If the result was calculated earlier, we can just return it
                return memory[parameter];
            }
            // If the result has never been calculated we do so.
            // ...
            // Very complex and heavy calculations here
            // ...
            std::string result = something_complex;
            // Now we save the result in our memory, so other calls with the same parameter will be faster
            memory[parameter] = result;
            return result;
        }
}
