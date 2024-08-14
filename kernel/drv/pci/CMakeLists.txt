# PCI(e) driver subsystem

include(${MENIX_UTIL_PATH})
add_library(menix_drv_pci STATIC
	pci_register.c
)
