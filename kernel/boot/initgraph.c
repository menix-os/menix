#include <menix/assert.h>
#include <menix/common.h>
#include <menix/initgraph.h>
#include <menix/list.h>
#include <menix/print.h>

extern void (*__ld_init_ctors_start[])();
extern void (*__ld_init_ctors_end[])();
extern struct initgraph_node __ld_init_start[];
extern struct initgraph_node __ld_init_end[];

void initgraph_execute(void (*on_reached)(struct initgraph_node*)) {
    // Initialize all edges.
    void (**current)() = __ld_init_ctors_start;
    while (current < __ld_init_ctors_end) {
        (*current)();
        current++;
    }

    const size_t num_nodes = (__ld_init_end - __ld_init_start);
    struct initgraph_node* nodes = __ld_init_start;

    // Mark all nodes as wanted.
    for (size_t i = 0; i < num_nodes; i++) {
        nodes[i].wanted = true;
    }

    // Get pending nodes.
    SLIST_HEAD(struct initgraph_node*) pending = {0};
    for (size_t i = 0; i < num_nodes; i++) {
        struct initgraph_node* node = &nodes[i];
        if (!node->wanted || node->done || node->unsatisfied_deps != 0)
            continue;

        SLIST_INSERT_HEAD(&pending, node, pending_link);
    }

    // Run all nodes.
    struct initgraph_node* node = SLIST_FIRST(&pending);
    while (node) {
        on_reached(node);

        ASSERT(node->wanted, "Node should be wanted!");
        ASSERT(!node->done, "Node should not be done!");
        ASSERT(node->unsatisfied_deps == 0, "Node has unsatisfied dependencies!");
        node->action();
        node->done = true;

        struct initgraph_edge* edge;
        SLIST_FOREACH(edge, &node->out_edges, out_link) {
            struct initgraph_node* successor = edge->target;
        }

        node = SLIST_NEXT(node, pending_link);
    }
}

void initgraph_edge_register(struct initgraph_edge* edge) {
    SLIST_INSERT_HEAD(&edge->source->out_edges, edge, out_link);
    SLIST_INSERT_HEAD(&edge->source->in_edges, edge, in_link);
    edge->source->unsatisfied_deps += 1;
}

static void my_foo() {
    kprintf("Hello!\n");
}
INITGRAPH_DEPS(empty);
INITGRAPH_STAGE(foo, "foo", my_foo, empty, empty);

static void my_foo2() {
    kprintf("I ran afterwards!\n");
}
INITGRAPH_DEPS(depends, &foo);
INITGRAPH_DEPS(entails);
INITGRAPH_STAGE(foo2, "foo", my_foo2, depends, empty);
