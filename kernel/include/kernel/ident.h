#pragma once

#include <uapi/types.h>
#include <kernel/errno.h>

struct ident {
    uid_t uid;
    uid_t euid;
    uid_t suid;

    gid_t gid;
    gid_t egid;
    gid_t sgid;
};

menix_errno_t ident_set_uids(struct ident* self, uid_t uid, uid_t euid, uid_t suid);
menix_errno_t ident_set_gids(struct ident* self, uid_t gid, uid_t egid, uid_t sgid);
