#include "utils/sdl_utils.h"
#include <stdbool.h>
#include <stdlib.h>


int main() {

  if (!init_sdl())
    exit(EXIT_FAILURE);

  SDL_SetRenderDrawColor(renderer, COLOR_BLACK.r, COLOR_BLACK.g, COLOR_BLACK.b,
                         COLOR_BLACK.a);
  SDL_RenderClear(renderer);
  
  bool running = true;

  while (running) {

    running = handle_input();

    SDL_Delay(16);
    SDL_RenderPresent(renderer);
  }

  final_cleanup();

  exit(EXIT_SUCCESS);
}
