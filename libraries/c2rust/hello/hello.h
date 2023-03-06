#ifndef HELLO_H
#define HELLO_H

int hello();

int add(int a, int b);

typedef struct
{
    int val;
} Element;

typedef struct
{
    Element *elem;
    int elem_count;
} System;

void square_elems(System sys);

#endif
