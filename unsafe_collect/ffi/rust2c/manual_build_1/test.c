// gcc -c -Wall -Werror -fpic test.c  //for dynamic library
//gcc -c test.c //for static library.
//ar rcs libtest.a test.o  //for static library

int add(int a, int b) {
    return a +b;
}