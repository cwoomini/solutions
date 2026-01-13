#include <iostream>
#include <string>

#define STACK_SIZE 10000

class Stack {
public:
    void push(int x) {
        if (head < STACK_SIZE - 1)
            stk[++head] = x;
    }

    int pop() {
        if (empty()) return -1;
        return stk[head--];
    }

    int size() {
        return head + 1;
    }

    int empty() {
        if (head == -1) return 1;
        return 0;
    }

    int top() {
        if (empty()) return -1;
        return stk[head];
    }

private:
    int stk[STACK_SIZE] = {0};
    int head = -1;
};

int main() {
    int n;
    std::cin >> n;

    Stack stk;

    for (int i = 0; i < n; i++) {
        std::string cmd;
        std::cin >> cmd;

        if (cmd == "push") {
            int x;
            std::cin >> x;
            stk.push(x);
        } else if (cmd == "pop") {
            std::cout << stk.pop() << std::endl;
        } else if (cmd == "size") {
            std::cout << stk.size() << std::endl;
        } else if (cmd == "empty") {
            std::cout << stk.empty() << std::endl;
        } else if (cmd == "top") {
            std::cout << stk.top() << std::endl;
        }
    }
    return 0;
}
