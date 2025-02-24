// UStar File system

#include <menix/fs/fs.h>
#include <menix/fs/ustar.h>
#include <menix/fs/vfs.h>
#include <menix/util/log.h>

#include <string.h>

typedef struct [[gnu::packed]]
{
	char name[100];
	char mode[8];
	char uid[8];
	char gid[8];
	char size[12];
	char mtime[12];
	char checksum[8];
	char type;
	char linkname[100];
	char signature[6];
	char version[2];
	char owner[32];
	char group[32];
	char devmajor[8];
	char devminor[8];
	char prefix[155];
} UStarFsHeader;

typedef enum
{
	UStarFileType_Regular = 0,
	UStarFileType_Normal = '0',
	UStarFileType_HardLink = '1',
	UStarFileType_SymLink = '2',
	UStarFileType_CharDev = '3',
	UStarFileType_BlockDev = '4',
	UStarFileType_Directory = '5',
	UStarFileType_FIFO = '6',
	UStarFileType_Contigous = '7',
	UStarFileType_GNULongPath = 'L'
} UStarFileType;

static usize oct2bin(const char* str)
{
	int n = 0;
	while (*str)
	{
		n *= 8;
		n += *str - '0';
		str++;
	}
	return n;
}

i32 ustarfs_init(VfsNode* mount, void* data, usize size)
{
	UStarFsHeader* current_file = (void*)data;
	char* name_override = NULL;

	usize files_loaded = 0;
	while (strncmp(current_file->signature, "ustar", 5) == 0)
	{
		char* name = current_file->name;
		if (name_override != NULL)
		{
			name = name_override;
			name_override = NULL;
		}

		usize file_mode = oct2bin(current_file->mode);
		usize file_size = oct2bin(current_file->size);

		VfsNode* node = NULL;
		switch (current_file->type)
		{
			case UStarFileType_Regular:
			case UStarFileType_Normal:
			case UStarFileType_Contigous:
			{
				node = vfs_node_add(mount, name, file_mode | S_IFREG);
				if (node)
					node->handle->write(node->handle, NULL, (void*)current_file + 512, file_size, 0);
				files_loaded++;
				break;
			}
			case UStarFileType_SymLink:
			{
				vfs_sym_link(mount, name, current_file->linkname);
				files_loaded++;
				break;
			}
			case UStarFileType_Directory:
			{
				vfs_node_add(mount, name, file_mode | S_IFDIR);
				break;
			}
			case UStarFileType_GNULongPath:
			{
				name_override = (void*)current_file + 512;
				name_override[file_size] = 0;
				break;
			}
		}

		current_file = (void*)current_file + 512 + ALIGN_UP(file_size, 512);
	}

	print_log("vfs: Loaded %zu files from UStar archive at 0x%p (Size = %zu)\n", files_loaded, data, size);
	return 0;
}
