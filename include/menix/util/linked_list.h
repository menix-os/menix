// Linked list data structure.

#pragma once
#include <menix/common.h>

typedef struct LinkedList
{
	struct LinkedList* next;
	struct LinkedList* prev;
} LinkedList;
