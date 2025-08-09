# include <raylib.h>
# include <stdlib.h>
# include "window_context.h"

int main(void) {

    window();

    while (!WindowShouldClose()) {

        BeginDrawing();
        ClearBackground(BLACK);
        EndDrawing();
    }
    
    return EXIT_SUCCESS;
}
