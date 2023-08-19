#include <cstdint>
struct PageDirectoryEntry {
	uint64_t* value;
};

struct PageTable {
    PageDirectoryEntry* entries[512];
};

extern "C" void Mov2CR3(PageTable* PML4) {
	asm ("mov %0, %%cr3" : : "r" (PML4));
}