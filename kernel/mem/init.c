#include <kernel/assert.h>
#include <kernel/mem.h>
#include <kernel/panic.h>
#include <kernel/print.h>
#include <menix/status.h>
#include <string.h>

extern uint8_t __ld_text_start[];
extern uint8_t __ld_text_end[];
extern uint8_t __ld_rodata_start[];
extern uint8_t __ld_rodata_end[];
extern uint8_t __ld_data_start[];
extern uint8_t __ld_data_end[];
extern uint8_t __ld_kernel_start[];

static virt_t hhdm_base = 0;
struct page_table mem_kernel_table = {0};

static menix_status_t (*alloc_fn)(size_t num_pages, enum alloc_flags flags, phys_t* out);
static menix_status_t (*free_fn)(phys_t start, size_t num_pages);

void mem_set_page_allocator(
    menix_status_t (*alloc)(size_t, enum alloc_flags, phys_t*),
    menix_status_t (*free)(phys_t, size_t)
) {
    alloc_fn = alloc;
    free_fn = free;
}

menix_status_t mem_page_alloc(size_t num_pages, enum alloc_flags flags, phys_t* out) {
    return alloc_fn(num_pages, flags, out);
}

menix_status_t mem_page_free(phys_t start, size_t num_pages) {
    return free_fn(start, num_pages);
}

void* mem_hhdm(phys_t phys) {
    return (void*)(phys + hhdm_base);
}

static phys_t bump_cursor = 0;
static size_t bump_len = 0;

static menix_status_t bootstrap_alloc(size_t num_pages, enum alloc_flags flags, phys_t* out) {
    const size_t bytes = num_pages * mem_page_size();
    bump_cursor += bytes;

    if (((intptr_t)bump_len - bytes) <= 0)
        return MENIX_ERR_NO_MEMORY;
    bump_len -= bytes;

    if (!(flags & ALLOC_NOZERO)) {
        memset(mem_hhdm(bump_cursor), 0, bytes);
    }

    *out = bump_cursor;
    return MENIX_OK;
}

static menix_status_t bootstrap_free(phys_t addr, size_t num_pages) {
    // We don't free bootstrap memory.
    ASSERT(false, "Attempted to free bootstrap memory. Fix this!");
}

void mem_init(struct phys_mem* map, size_t length, virt_t kernel_virt, phys_t kernel_phys, virt_t tmp_hhdm) {
    // This function creates a kernel page table and initializes all memory managers.

    kprintf("Usable memory map entries:\n");
    for (size_t i = 0; i < length; i++) {
        if (map[i].length > 0 && map[i].usage == PHYS_USABLE)
            kprintf("[%4zu] [%p - %p]\n", i, (void*)map[i].address, (void*)(map[i].address + map[i].length - 1));
    }

    // Set up the bootstrap allocator.
    struct phys_mem* largest = map;
    for (size_t i = 0; i < length; i++) {
        if (map[i].length > largest->length && map[i].usage == PHYS_USABLE)
            largest = map + i;
    }
    kprintf(
        "Using region [%p - %p] for bootstrap allocator\n",
        (void*)largest->address,
        (void*)(largest->address + largest->length - 1)
    );
    bump_cursor = largest->address;
    bump_len = largest->length;
    size_t bump_old = bump_len;
    mem_set_page_allocator(bootstrap_alloc, bootstrap_free);

    // Set the HHDM base address to the address given by the loader.
    // We must not keep any virtual addresses to this region,
    // since we're likely going to map it at a different base address.
    hhdm_base = tmp_hhdm;

    ASSERT(mem_pt_new_kernel(&mem_kernel_table, 0) == MENIX_OK, "Unable to allocate the kernel page table");

    kprintf("Mapping text segment at %p\n", __ld_text_start);
    for (uint8_t* p = __ld_text_start; p <= __ld_text_end; p += mem_page_size()) {
        menix_status_t status = mem_pt_map(
            &mem_kernel_table,
            (virt_t)p,
            (phys_t)(p - __ld_kernel_start + kernel_phys),
            PTE_READ | PTE_EXEC,
            CACHE_NONE
        );
        ASSERT(status == MENIX_OK, "Failed to map %p with error %i", p, status);
    }

    kprintf("Mapping rodata segment at %p\n", __ld_rodata_start);
    for (uint8_t* p = __ld_rodata_start; p < __ld_rodata_end; p += mem_page_size()) {
        menix_status_t status = mem_pt_map(
            &mem_kernel_table,
            (virt_t)p,
            (phys_t)(p - __ld_kernel_start + kernel_phys),
            PTE_READ,
            CACHE_NONE
        );
        ASSERT(status == MENIX_OK, "Failed to map %p with error %i", p, status);
    }

    kprintf("Mapping data segment at %p\n", __ld_data_start);
    for (uint8_t* p = __ld_data_start; p < __ld_data_end; p += mem_page_size()) {
        menix_status_t status = mem_pt_map(
            &mem_kernel_table,
            (virt_t)p,
            (phys_t)(p - __ld_kernel_start + kernel_phys),
            PTE_READ | PTE_WRITE,
            CACHE_NONE
        );
        ASSERT(status == MENIX_OK, "Failed to map %p with error %i", p, status);
    }

    // Map HHDM.
    tmp_hhdm = 0xFFFF'C000'0000'0000;
    for (size_t i = 0; i < length; i++) {
        for (size_t p = 0; p < map[i].length; p += mem_page_size()) {
            menix_status_t status =
                mem_pt_map(&mem_kernel_table, (virt_t)(tmp_hhdm + p), (phys_t)p, PTE_READ | PTE_WRITE, CACHE_NONE);
            ASSERT(status == MENIX_OK, "Failed to map %p with error %i", (void*)(tmp_hhdm + p), status);
        }
    }
    // Update the HHDM base.
    hhdm_base = tmp_hhdm;

    // Switch to our own page table.
    mem_pt_set(&mem_kernel_table);

    // We don't need the bootstrap allocator from this point on.
    kprintf("Early memory init finished, using %zu KiB of memory\n", (bump_old - bump_len) / 1024);

    // Initialize the buddy page allocator.
}
