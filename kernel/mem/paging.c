#include <kernel/assert.h>
#include <kernel/mem/mm.h>
#include <kernel/mem/paging.h>
#include <kernel/mem/types.h>
#include <kernel/sys/spin.h>

static struct page_table kernel_table = {};

// Gets a reference to the PTE at the given virtual address.
// If `check_only` is set, only checks if the PTE exists,
// and doesn't allocate new levels if they don't already exist.
// If it can't allocate a page if it has to, returns `nullptr`.
static pte_t* get_pte(struct page_table* pt, virt_t vaddr, bool check_only) {
    ASSERT(spin_locked(&pt->lock), "page table was not locked");

    pte_t* current_head = mem_hhdm(pt->root);
    size_t index = 0;

    for (int8_t level = mem_num_levels() - 1; level > 0; level--) {
        const size_t addr_mask = (1 << mem_level_bits()) - 1;
        const size_t addr_shift = mem_page_bits() + (mem_level_bits() * level);

        index = (vaddr >> addr_shift) & addr_mask;
        pte_t* pte = &current_head[index];

        if (pte_is_present(pte)) {
            // Get the next level.
            current_head = mem_hhdm(pte_address(pte));
        } else {
            // If the current level isn't present, we can skip the rest.
            if (check_only)
                return nullptr;

            // TODO
        }

        current_head = mem_hhdm(pte_address(pte));
    }

    return &current_head[index];
}

bool mem_pt_map(struct page_table* pt, virt_t vaddr, phys_t paddr, enum pte_flags flags) {
    spin_lock(&pt->lock);

    pte_t* pte = get_pte(pt, vaddr, false);
    if (!pte)
        return false;
    // TODO

    spin_unlock(&pt->lock);
    return true;
}
