#include "SDL_render.h"
#include "utils/sdl_utils.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>


int main() {

  if (!init_sdl())
    exit(EXIT_FAILURE);

  SDL_SetRenderDrawColor(renderer, COLOR_BLACK.r, COLOR_BLACK.g, COLOR_BLACK.b,
                         COLOR_BLACK.a);
  SDL_RenderClear(renderer);
  
  bool running = true;

  while (running) {
    // record initial time
    // emulate instructions
    running = handle_input();
    // record time after emulation
    // delay should be 16 - (after - initial)

    // approx 60 Hz/fps delay
    SDL_Delay(16);
    SDL_RenderPresent(renderer);
  }

  final_cleanup();

  exit(EXIT_SUCCESS);
}
