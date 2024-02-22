#include <iostream>
using std::string;

template<class A, class B>class Either{
    bool    _isRight;
    A       left;
    B       right;

public:

    Either(A x) { left = x;  _isRight = false;}
    Either(B x) { right = x; _isRight = true;}

    void Left(A x) { left = x; _isRight = false;}
    void Right(B x) { right = x; _isRight = true;}

    bool isRight(){ return _isRight; } 
    bool isLeft(){ return !_isRight; } 

    A    getLeft() { return left;}
    B    getRight() { return right;}
};

int i(int n) {return n;}
int j(bool b) {return b ? 0:1;}

int m(Either<string, int> const & e) {

}


int main(int argc, char** argv) {

    auto eith = Either<string, int>("haha");

    if (argc >= 2) {
        eith.Left("haha");
    } else {
        eith.Right(42);
    }

    if (eith.isRight()) {
        std::cout << eith.getRight() << std::endl;
    } else {
        std::cout << eith.getLeft() << std::endl;
    }

}
