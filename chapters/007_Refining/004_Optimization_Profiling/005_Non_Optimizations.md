Non-Optimizations
-----------------

In this small section we take a look at some alleged "optimizations" that actually do nothing (or close to nothing) for our game's performance.

### "Switches are faster than IFs"

Some people allege that using "switch" statements instead of "if" statements is bound to optimize the game. This an overstatement, and we can prove it with a simple test.

Let's create two C++ listings, like follows:

```{.cpp caption="IFs vs Switch - IF Statements"}
#include <iostream>
using namespace std;
int main(){
  for (int i = 0; i < 10000000; i++){
    int x = rand() % 5;
    if (x==1){
      cout << "One" << endl;
    }else if (x==2){
      cout << "Two" << endl;
    }else if(x==3){
      cout << "Three" << endl;
    }else if(x==4){
      cout << "Four" << endl;
    }else if(x==5){
      cout << "Five" << endl;
    }
  }
  return 0;
}
```

```{.cpp caption="IFs vs Switch - Switch Statements"}
#include <iostream>
using namespace std;
int main(){
  for (int i = 0; i < 10000000; i++){
    int x = rand() % 5;
    switch(x){
      case 1:
        cout << "One" << endl;
        break;
      case 2:
        cout << "Two" << endl;
        break;
      case 3:
        cout << "Three" << endl;
        break;
      case 4:
        cout << "Four" << endl;
        break;
      case 5:
        cout << "Five" << endl;
        break;
    }
  }
  return 0;
}
```

These pieces of code will be compiled without any optimization, using G++, using the following command:

```{.sh}
g++ -Wall -Wextra -Werror -O0 filename.cpp -o filename.bin
```

Where "filename" is replaced by the source name, then each file will be executed using the "time" linux command, like follows:

```{.sh}
time ./filename.bin
```

Below we can see the results for both the codes:

![Time taken by the IF code](./images/profiling_optimization/if_time.png){width=60%}

![Time taken by the Switch code](./images/profiling_optimization/switch_time.png){width=60%}

We can see a difference of just around 0.25 seconds, over 10 Million iterations. If you changed an equivalent IF statement for a Switch statement, you would earn a quarter of a second every 46 hours of gameplay at 60fps.

The right choice is the simply choose the structure that lets you have the most readable code: the more your code is readable, the easier it is to understand; the easier to understand, the lower the probability that there is a bug in there (or a performance hog of some sort).

### Blindly Applying Optimizations

There rarely is something more wrong you can do when optimizing than blindly applying optimizations without considering your application context.

Using resource pooling in an environment with limited memory (but plenty of CPU power) can prove a disaster: it's better to instantiate and destroy objects in such case.

Sometimes animators can be faster than LERPing/Tweening, mostly when you have to tween objects with multiple children: tweening would create a lot of CPU-bound calculations for the new position and size that will make the whole thing bog down and get choppy.

The only thing you can do is think first and try later: this book can give you some suggestions, but nothing should be taken at face value. Remember the context your game is working in and **do not treat all platforms like they're the same**: WebGL is different than Console which is different than Mobile.
