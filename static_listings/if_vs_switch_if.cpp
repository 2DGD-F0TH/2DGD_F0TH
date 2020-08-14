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
