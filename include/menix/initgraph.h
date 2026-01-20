#pragma once

#include <menix/common.h>
#include <menix/compiler.h>
#include <menix/list.h>
#include <stddef.h>

struct initgraph_node {
    const char* label;
    size_t unsatisfied_deps;
    bool wanted;
    bool done;

    SLIST_HEAD(struct initgraph_edge*) in_edges;
    SLIST_HEAD(struct initgraph_edge*) out_edges;
    SLIST_LINK(struct initgraph_node*) pending_link;

    void (*action)();
};

struct initgraph_edge {
    struct initgraph_node* source;
    struct initgraph_node* target;
    SLIST_LINK(struct initgraph_edge*) in_link;
    SLIST_LINK(struct initgraph_edge*) out_link;
};

void initgraph_execute(void (*on_reached)(struct initgraph_node* node));

void initgraph_edge_register(struct initgraph_edge* edge);

#define INITGRAPH_DEPS(name, ...) static struct initgraph_node* name[] = {__VA_ARGS__}

#define INITGRAPH_STAGE(name, display_name, act, depends, entails) \
    [[__section(".initgraph.nodes")]] \
    struct initgraph_node name = { \
        .label = display_name, \
        .action = act, \
    }; \
    [[__section(".initgraph.ctors")]] \
    static void UNIQUE_IDENT(initgraph)() { \
        static struct initgraph_edge* __initgraph_edge_depends[ARRAY_SIZE(depends)]; \
        for (size_t i = 0; i < ARRAY_SIZE(depends); i++) { \
            __initgraph_edge_depends[i]->source = (depends)[i]; \
            __initgraph_edge_depends[i]->target = &name; \
            initgraph_edge_register(__initgraph_edge_depends[i]); \
        } \
        static struct initgraph_edge* __initgraph_edge_entails[ARRAY_SIZE(entails)]; \
        for (size_t i = 0; i < ARRAY_SIZE(entails); i++) { \
            __initgraph_edge_entails[i]->source = &name; \
            __initgraph_edge_entails[i]->target = (entails)[i]; \
            initgraph_edge_register(__initgraph_edge_entails[i]); \
        } \
    }
