# ? PCI/PCIe Drivers

set(MENIX_PARENT_CAT menix_drv_pci)
add_library(${MENIX_PARENT_CAT} STATIC
	pci.c
)
include(${MENIX_UTIL_PATH})
