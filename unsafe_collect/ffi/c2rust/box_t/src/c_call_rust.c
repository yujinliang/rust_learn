#include <stddef.h>

// Returns ownership to the caller.
struct Foo* foo_new(void);

// Takes ownership from the caller; no-op when invoked with NULL.
void foo_delete(struct Foo*);

int main() {
    foo_delete(foo_new());
     foo_delete(NULL);
}