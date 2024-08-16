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