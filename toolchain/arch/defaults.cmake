# Common options that are selected independently from the architecture.

config_option(arch STRING "${MENIX_ARCH_NAME}")
config_option(version STRING "${MENIX_VERSION}")
config_option(release STRING "${MENIX_RELEASE}")
config_option(bits NUMBER ${MENIX_BITS})

# Licenses
add_option(license_LGPL BOOL ON)
add_option(license_MIT BOOL ON)
add_option(license_BSD3 BOOL ON)

# Kernel logging
add_option(ktrace BOOL ON)
add_option(ktrace_registers BOOL ON)
add_option(ktrace_max NUMBER 32)

# ACPI
add_option(acpi BOOL ON)

# Symmetric Multi-processing
add_option(smp BOOL ON)

# PCI(e)
add_option(pci BOOL ON)

# Memory allocation
add_option(vm_map_min_addr NUMBER 0x10000)
add_option(vm_map_base NUMBER 0xFFFF900000000000)
add_option(vm_map_foreign_base NUMBER 0xFFFFD00000000000)

# User-space constants
add_option(user_stack_size NUMBER 0x200000)
add_option(user_stack_addr NUMBER 0x70000000000)
add_option(user_interp_base NUMBER 0x60000000000)

add_option(debug BOOL OFF)
