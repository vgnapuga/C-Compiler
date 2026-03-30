int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 5;
    int y = 10;

    if (x < y) {
        x = add(x, y);
    }

    return 0;
}
