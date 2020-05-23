// src/simple.c

typedef void (*AddCallback)(int result);

void simple_add_two_numbers(int a, int b, AddCallback cb)
{
    int result = a + b;
    cb(result);
}