#include <cstdint>
#include <cstddef>

struct EFI_MEMORY_DESCRIPTOR {
    uint32_t type;
    void* physAddr;
    void* virtAddr; 
    size_t numPages;
    uint64_t attribs;
};


extern "C" EFI_MEMORY_DESCRIPTOR* GetMemoryDesc(EFI_MEMORY_DESCRIPTOR* mMap, size_t i, size_t mMapDescSize) {
	EFI_MEMORY_DESCRIPTOR* desc = (EFI_MEMORY_DESCRIPTOR*)((uint64_t)mMap + (i * mMapDescSize));
	return desc;
}