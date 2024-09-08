add_module(xhci_pci "Marvin Friedrich" "USB 3.0 support via PCI" MAIN ON ON
	main.c
)
require_option(usb_core)
require_option(pci)
