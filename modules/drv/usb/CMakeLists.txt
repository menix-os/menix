add_module(usb_core "Marvin Friedrich" "USB subsystem" MAIN ON ON
	main.c
)

add_subdirectory(xhci_pci)
