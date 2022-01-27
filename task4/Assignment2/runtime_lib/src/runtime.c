///////////////////////////////////////////////
//////          runtime.c       ///////////////
///////////////////////////////////////////////

#ifdef __cplusplus
extern "C" {
#endif

///////////////////////////////////////////////

#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stddef.h>
//#include <stdbool.h>
#include <assert.h>
#include <math.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <unistd.h>
#include <fcntl.h>
#include <math.h>
#include <string.h>
#include <assert.h>
#include <stdbool.h>
#include <signal.h>
#include "./runtime.h" ///

#define PAD_SIZE 64
int __runtime_fd;
int errno;

#define __runtime_start_addr_userspace 0x000001000000 
#define __runtime_end_addr_userspace   0x7F0000000000 

size_t __runtime_metadata_file_size = __runtime_end_addr_userspace - __runtime_start_addr_userspace;
void * __runtime_shadow_base;
void * __runtime_shadow_upper;

void * __runtime_heap_lowest;
void * __runtime_heap_highest;

__attribute__((__used__))
__attribute__((__optnone__))
void *
__runtime_get_shadow_addr (void * ptr)
{
    return ptr +  (uintptr_t)__runtime_shadow_base;
}

__attribute__((__used__))
__attribute__((__optnone__))
void * 
__runtime_checkbound (void * ptr)
{
    
    bool * shadow_addr= __runtime_get_shadow_addr(ptr);
    
    if ((size_t)shadow_addr > (size_t)__runtime_shadow_upper ||
        (size_t)shadow_addr < (size_t)__runtime_shadow_base) {
        return ptr; 
    }
     
    if (!*shadow_addr) {
        printf(">>>>>>>>>> Out-of-bounds error: %p \n", shadow_addr); 
    }
    return ptr; 
}

__attribute__((__used__))
__attribute__((__optnone__))
void  
__runtime_init_heap_obj (void * ptr, size_t sz, size_t fat_sz) 
{
    assert(fat_sz-sz== FIXED_SIZE);
    void * shadow_addr= __runtime_get_shadow_addr(ptr); 
    memset (shadow_addr, IS_VALID, sz); 
    memset (shadow_addr+sz, IS_INVALID, fat_sz-sz); 
}

__attribute__((__used__))
__attribute__((__optnone__))
void * 
__runtime_mymalloc (size_t sz) 
{
    
    /*  we give restriction: the obj size <= 64 */
    //assert(sz <= 64); 
    size_t padded_size= sz+ FIXED_SIZE; 
    void * ptr= malloc(padded_size);
    assert(ptr);

    /*  zero-setting an object  */
    
    __runtime_init_heap_obj (ptr, sz, padded_size);
    
    
    return ptr;
}

__attribute__((__used__))
__attribute__((__optnone__))
void  
__runtime_main_prologue () 
{
    __runtime_shadow_base = mmap (0, 
            (__runtime_metadata_file_size),
            PROT_READ | PROT_WRITE | PROT_NONE, 
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE,
            -1,//__runtime_fd, 
            (off_t)0);
    
    assert(__runtime_shadow_base != MAP_FAILED);
    __runtime_shadow_upper= __runtime_shadow_base+(size_t)+__runtime_metadata_file_size;

}

 __attribute__((__used__))
__attribute__((__optnone__))
void  
__runtime_main_epilogue () 
{
    munmap (__runtime_shadow_base, 
            __runtime_metadata_file_size); 
}

#ifdef __cplusplus
}
#endif

