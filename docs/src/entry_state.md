# Expected machine state

Upon entry in `main()`, Menix expects the following machine state:

## System state
- Interrupts should be turned off.
- All AP cores should be active and parked.

## Memory layout

In virtual memory, the following mapping should exist:

| Segment | Virtual address         | Physical address       | Length in bytes                     | Permissions |
| ------- | ----------------------- | ---------------------- | ----------------------------------- | ----------- |
| hhdm    | `0xffff_8000_0000_0000` | `0x0`                  | `0x1000_0000_0000`                  | `rw-`       |
| text    | `LD_TEXT_START`         | Any consecutive memory | `LD_TEXT_END` - `LD_TEXT_START`     | `r-x`       |
| rodata  | `LD_RODATA_START`       | Any consecutive memory | `LD_RODATA_END` - `LD_RODATA_START` | `r--`       |
| data    | `LD_DATA_START`         | Any consecutive memory | `LD_DATA_END` - `LD_DATA_START`     | `rw-`       |
| percpu  | `LD_PERCPU_START`       | Any consecutive memory | `LD_PERCPU_END` - `LD_PERCPU_START` | `rw-`       |

The `percpu` segment needs at least `0x1000_0000` bytes of distance from the end of the higher half region.
This means that for KASLR, the highest address for `LD_KERNEL_END` can be `0xffff_ffff_efff_ffff`.
