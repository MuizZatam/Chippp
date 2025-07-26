# include <stdio.h>
# include <stdlib.h>
# include "utils/sdl_utils.h"


int main() {

    if (!init_sdl()) {

        exit(EXIT_FAILURE);
    }
    
    exit(EXIT_SUCCESS);
}
