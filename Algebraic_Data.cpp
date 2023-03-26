#include <iostream>
using std::string;

// template<class A>
// class List {
//   bool _isNil;
//   A    value;
// public: 
//   List() : _isNil(true) {};
//   List(A x): _isNil(false), value(x) {};
// };


template<class A>
class Node {
  A value;
public:
  // Node(A x, List<A> l): value(x) {};
};

template<class A>
class List {
  Node<A> * _head;
public: 
  List() : _head(nullptr) {};
  // List(A a, List<A> l): _head(new List<A>(a, l)) {}
};

int main() {
  // auto ls = List<string>("H", List<string>("e", List<string>()));
}
