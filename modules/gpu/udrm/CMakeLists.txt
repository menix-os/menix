# uDRM

add_module(udrm "Marvin Friedrich" "uDRM Bridge" OFF ON
	udrm_bridge.c
	udrm/src/udrm.c
	udrm/src/udrm_core.c
	udrm/src/udrm_virtio.c
)

if(TARGET udrm)
	target_include_directories(udrm PRIVATE udrm/include)
endif()
