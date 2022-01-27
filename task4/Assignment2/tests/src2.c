#include <stdio.h>

void assign_malloc (void * ptr, unsigned count)
{
    int * tmp= (int*)ptr;

    /* Buffer overflow when i is equals to 5 */
    for (unsigned i=0; i<count; i++) {
        *tmp= i*10;
        printf("> assign_malloc. %u: %d\n", i, *tmp);
        tmp++;
    }
}
