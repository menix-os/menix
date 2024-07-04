# ? PCI(e) Drivers

set(MENIX_PARENT_CAT menix_drv_pci)
add_library(${MENIX_PARENT_CAT} STATIC pci.c)
target_link_libraries(${MENIX_PARENT_CAT} PUBLIC menix_arch_${MENIX_ARCH_DIR})
include(${MENIX_UTIL_PATH})
add_option(pci ON)
