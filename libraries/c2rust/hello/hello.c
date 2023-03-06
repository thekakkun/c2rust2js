#include "hello.h"

int hello()
{
    return 42;
}

int add(int a, int b)
{
    return a + b;
}

void square_elems(System sys)
{
    for (int i = 0; i <= sys.elem_count; ++i)
    {
        int *elem_val = &sys.elem[i].val;
        *elem_val = (*elem_val) * (*elem_val);
    };
}