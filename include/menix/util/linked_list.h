// Linked list data structure.

#pragma once
#include <menix/common.h>

typedef struct LinkedList LinkedList;

struct LinkedList
{
	LinkedList* next;
	LinkedList* prev;
};
