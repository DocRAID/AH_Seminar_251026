#include <memory>
#include <iostream>
using namespace std;

struct Test {
    int value;
    Test() { cout << "malloc\n"; }
    ~Test() { cout << "free\n"; }
};

int main() {
    cout<<"start\n";
    {
        Test* p = new Test();
        // delete p;
    }
    cout<<"end\n";
}

