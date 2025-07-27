# ifndef SDL_INIT_UTILS
# define SDL_INIT_UTILS

#include <SDL2/SDL_pixels.h>
# include <stdbool.h>
# include "SDL2/SDL.h"

bool init_sdl(void);
void final_cleanup(void);
void sdl_window(void);
bool handle_input(void);

extern SDL_Window *window;
extern SDL_Renderer *renderer;
extern SDL_Color COLOR_WHITE;
extern SDL_Color COLOR_BLACK;

# endif
