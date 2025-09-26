use uacpi_sys::{
    UACPI_STATUS_OK, uacpi_handle, uacpi_io_addr, uacpi_size, uacpi_status, uacpi_u8, uacpi_u16,
    uacpi_u32,
};

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read8(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _out_value: *mut uacpi_u8,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read16(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _out_value: *mut uacpi_u16,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_read32(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _out_value: *mut uacpi_u32,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write8(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _in_value: uacpi_u8,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write16(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _in_value: uacpi_u16,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_write32(
    _arg1: uacpi_handle,
    _offset: uacpi_size,
    _in_value: uacpi_u32,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_map(
    _base: uacpi_io_addr,
    _len: uacpi_size,
    _out_handle: *mut uacpi_handle,
) -> uacpi_status {
    return UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_handle) {
    _ = handle;
}
