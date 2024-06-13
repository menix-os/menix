# --------------
# USB HID driver
# --------------

add_module(usb_hid "marv7000" "USB driver for input devices" LGPL TRUE OFF
	main.c
)
