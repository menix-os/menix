# Direct Rendering Manager

add_module(udrm "Marvin Friedrich" "Direct Rendering Manager" OFF ON
	udrm_bridge.c
	udrm/src/udrm.c
	udrm/src/udrm_core.c
	udrm/src/udrm_virtio.c
)

if(TARGET ${MENIX_CURRENT_MOD})
	target_include_directories(${MENIX_CURRENT_MOD} PRIVATE udrm/include)
endif()
