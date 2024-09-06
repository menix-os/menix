# Include architecture specific drivers
if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/${MENIX_ARCH})
	add_subdirectory(${MENIX_ARCH})
endif()

# USB
add_subdirectory(usb)
