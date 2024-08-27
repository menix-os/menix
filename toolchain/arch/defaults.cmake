# Common options that are selected independently from the architecture.

add_option(arch STRING "${MENIX_ARCH}")
add_option(version STRING "${MENIX_VERSION}")
add_option(bits NUMBER ${MENIX_BITS})

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

# Boot options
add_option(boot_logo BOOL OFF)

# Memory allocation
add_option(allocator_slab BOOL ON)
add_option(stack_size NUMBER 0x200000)
add_option(page_size NUMBER 0x1000)
add_option(vm_mmap_min_addr NUMBER 0x10000)
