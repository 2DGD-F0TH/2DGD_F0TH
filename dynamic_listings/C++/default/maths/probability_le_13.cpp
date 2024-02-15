#include <cstdlib>
#include <ctime>
#include <iostream>

int main(){
    int happened = 0;
    // We seed the randomizer with out system time
    std::srand(std::time(nullptr));
    // Monte Carlo Method we do 10000 "extractions"
    for (int i = 0; i < 10000; i++){
        // Get a random number between 1 and 100
        int n = std::rand() % 100 + 1;
        if (n <= 1){
            // If it's less or equal than 13, we have a match!
            happened++;
        }
    }
    // We print the result
    std::cout << happened << std::endl;
}
