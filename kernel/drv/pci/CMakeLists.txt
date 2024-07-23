# PCI(e) Drivers

include(${MENIX_UTIL_PATH})
set(MENIX_PARENT_CAT menix_drv_pci)
add_library(${MENIX_PARENT_CAT} INTERFACE)
add_option(pci ON)
