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

add_option(debug BOOL OFF)
