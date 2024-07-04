//? Boot using EFI chainloading.

#include <menix/boot.h>

#include <efi.h>
#include <efilib.h>

EFI_STATUS EFIAPI kernel_boot(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
	InitializeLib(ImageHandle, SystemTable);
	Print(L"Booting menix via EFI chainload!\n");
	kernel_main();
	return EFI_SUCCESS;
}
