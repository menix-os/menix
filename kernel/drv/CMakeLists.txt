# Driver subsystems (Not drivers themselves, see `/mod/`!)

include(${MENIX_UTIL_PATH})
add_library(menix_drv INTERFACE)

add_subdirectory(pci)

target_link_libraries(menix_drv INTERFACE menix_drv_pci)
