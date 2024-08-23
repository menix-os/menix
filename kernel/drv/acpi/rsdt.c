// RSDP/XSDT functions.

#include <menix/drv/acpi/acpi.h>
#include <menix/drv/acpi/types.h>
#include <menix/drv/pci/pci.h>
#include <menix/drv/pci/pci_acpi.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/video/bmp.h>
#include <menix/video/fb.h>

#include <string.h>

static AcpiRsdt* rsdt;

// Performs a sanity check on a block of data.
static u8 acpi_checksum(void* ptr, usize size)
{
	// ACPI standard mandates that all data must add up to 0.
	u8 sum = 0;
	u8* cur = ptr;
	for (usize i = 0; i < size; i++)
		sum += cur[i];
	return sum;
}

void acpi_init(AcpiRsdp* rsdp)
{
	kassert(rsdp != NULL, "Failed to set RSDP: None given!\n");
	rsdt = ACPI_ADDR(rsdp->xsdt_address);

	kmesg("Initialized ACPI (Rev. %u)\n", rsdp->revision);

#ifdef CONFIG_boot_logo
	// Draw Boot logo.
	FrameBuffer* fb = fb_get_early();
	AcpiBgrt* bgrt = acpi_find_table("BGRT", 0);
	BmpHeader* bmp = ACPI_ADDR(bgrt->image_addr);
	u8* data = ((u8*)bmp) + bmp->offset;
	if (bmp->dib.bpp / 8 != fb->mode.cpp)
	{
		// Convert image to ARGB.
		data = kmalloc(bmp->dib.width * bmp->dib.height * fb->mode.cpp);
		bmp_unpack24_to_32(data, bmp);
	}

	FbDrawRegion region = {
		.x_src = bgrt->image_xoff,
		.y_src = bgrt->image_yoff,
		.data = data,
		.width = bmp->dib.width,
		.height = bmp->dib.height,
	};

	fb->funcs.draw_region(fb, &region);

	// Free the buffer.
	if (bmp->dib.bpp / 8 != fb->mode.cpp)
	{
		kfree(data);
	}
#endif

	// The PCI subsystem depends on ACPI. Now we can enable it.
	pci_init_acpi();
}

void* acpi_find_table(const char* signature, usize index)
{
	usize num = 0;

	// Iterate over all tables.
	const usize num_entries = (rsdt->header.length - sizeof(AcpiDescHeader)) / 8;
	for (usize i = 0; i < num_entries; i++)
	{
		// Get the address to the next table.
		AcpiDescHeader* ptr = (void*)ACPI_ADDR(((u64*)rsdt->entries)[i]);
		// Check the signature.
		if (!acpi_checksum(ptr, ptr->length) && !memcmp(ptr->signature, signature, 4) && num++ == index)
			return (void*)ptr;
	}

	return NULL;
}
