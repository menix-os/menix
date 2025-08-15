#ifndef _SERVERS_BOOTSTRAP_MESSAGES_H
#define _SERVERS_BOOTSTRAP_MESSAGES_H

#include <stddef.h>

#define MESSAGE(name, request, response) \
    struct name##_req { \
        struct request; \
    }; \
    struct name##_res { \
        struct response; \
    };

// Associate the sending process with a well known name.
MESSAGE(
    bootstrap_set_name,
    {
        size_t name_len;
        const char name[];
    },
    { bool ok; }
)

MESSAGE(
    bootstrap_find,
    {
        size_t name_len;
        const char name[];
    },
    { bool ok; }
)

#endif
