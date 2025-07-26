# include <stdbool.h>
# include "sdl_utils.h"
# include "SDL2/SDL.h"


bool init_sdl(void) {

    // returns the status of SDL Initialization
    // if a required interface fails to Initialize,
    // return false otherwise return true
    if (SDL_Init(SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_TIMER) != 0) {

        SDL_Log("COULD NOT INITIALIZE SDL SUBSYSTEMS! - %s\n", SDL_GetError());
        return false;
    }

    return true;
}


void final_cleanup(void) {

    SDL_Quit();
} 
