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
