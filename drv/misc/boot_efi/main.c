#include <efi.h>
#include <efilib.h>

extern void kernel_main(void);

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
	InitializeLib(ImageHandle, SystemTable);
	Print(L"Hello, world!\n");
	kernel_main();
	return EFI_SUCCESS;
}
