# Direct Rendering Manager

add_module(udrm "Marvin Friedrich" "Direct Rendering Manager" MAIN OFF ON
	udrm_bridge.c
	udrm/src/udrm_core.c
	udrm/src/udrm_bochs.c
)
target_include_directories(${MENIX_CURRENT_MOD} PRIVATE udrm/include)
