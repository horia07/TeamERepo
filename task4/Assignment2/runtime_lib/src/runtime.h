///////////////////////////////////////////////
//////          runtime.h           ///////////////
///////////////////////////////////////////////

#ifndef RUNTIME_HEADER_H_
#define RUNTIME_HEADER_H_

#ifdef __cplusplus
extern "C" {
#endif

///////////////////////////////////////////////

#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stddef.h>
#include <assert.h>
#include <math.h>
#include <stdbool.h>

#define IS_VALID    true 
#define IS_INVALID  false 

#define FIXED_SIZE  64

//extern void * __runtime_shadow_base;

extern void * __runtime_checkbound (void * ptr);
extern void * __runtime_mymalloc (size_t sz);
extern void __runtime_main_prologue ();
extern void __runtime_main_epilogue ();

///////////////////////////////////////////////


#ifdef __cplusplus
}
#endif

#endif // RUNTIME_HEADER_H_
