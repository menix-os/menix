// VirtIO Network Card

#include <menix/common.h>
#include <menix/system/module.h>

#include <string.h>

typedef struct
{
	u8 mac[6];
	u16 status;
	u16 max_virtqueue_pairs;
	u16 mtu;
	u32 speed;
	u8 duplex;
	u8 rss_max_key_size;
	u16 rss_max_indirection_table_length;
	u32 supported_hash_types;
	u32 supported_tunnel_types;
} VirtioNetConfig;

MODULE_FN i32 init_fn()
{
	return 0;
}

MODULE_DEFAULT(init_fn, NULL);
