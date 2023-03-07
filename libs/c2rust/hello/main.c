#include <stdio.h>
#include <stdlib.h>

#include "hello.h"

int main()
{
    printf("%d\n", hello());
    printf("37 + 42 is %d\n", add(37, 42));

    System sys;
    void *r = calloc(50, sizeof(sys.elem[0]));
    sys.elem = r;

    Element ele0 = {2};
    Element ele1 = {3};

    sys.elem[sys.elem_count++] = ele0;
    sys.elem[sys.elem_count++] = ele1;

    for (int i = 0; i < sys.elem_count; ++i)
    {
        printf("%d element value: %d\n", i, sys.elem[i].val);
    };

    square_elems(sys);

    for (int i = 0; i < sys.elem_count; ++i)
    {
        printf("%d element value: %d\n", i, sys.elem[i].val);
    };

    return 0;
}
