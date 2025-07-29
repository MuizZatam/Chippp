#include "sdl_utils.h"
#include "SDL2/SDL.h"
#include <stdbool.h>

SDL_Window *window = NULL;
SDL_Renderer *renderer = NULL;
// 64x32 pixels - scaled by 10
int window_width = 640;
int window_height = 320;
SDL_Color COLOR_WHITE = {255, 255, 255, 255};
SDL_Color COLOR_BLACK = {0, 0, 0, 255};

bool init_sdl(void) {

  // returns the status of SDL Initialization
  // if a required interface fails to Initialize,
  // return false otherwise, attempt to create window
  // if not initialized return false, otherwise true
  if (SDL_Init(SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_TIMER) != 0) {

    SDL_Log("COULD NOT INITIALIZE SDL SUBSYSTEMS! - %s\n", SDL_GetError());
    return false;
  }

  window =
      SDL_CreateWindow("Chippp!", SDL_WINDOWPOS_CENTERED,
                       SDL_WINDOWPOS_CENTERED, window_width, window_height, 0);

  if (!window) {

    SDL_Log("FAILED TO INITIALIZE WINDOW! - %s\n", SDL_GetError());
    return false;
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

  if (!renderer) {

    SDL_Log("FAILED TO CREATE RENDERER - %s\n", SDL_GetError());
    return false;
  }

  return true;
}

void final_cleanup() {

  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}

bool handle_input() {

  SDL_Event event;

  while (SDL_PollEvent(&event)) {

    switch (event.type) {
    case SDL_QUIT:
      return false;
    }
  }

  return true;
}
