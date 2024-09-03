#include <stdio.h>

#define SDL_MAIN_HANDLED

#include "libmadeline.h"
#include "SDL2/SDL.h"

//Screen dimension constants
const int SCREEN_SCALE = 32;
const int SCREEN_WIDTH = 40 * 32;
const int SCREEN_HEIGHT = 23 * 32;
const int WORLD_WIDTH = 320;
const int WORLD_HEIGHT = 180;

const uint32_t SOLID = 0xFFFFFFFF;
const uint32_t SPAWN = 0xFF0000FF;
const uint32_t WATER = 0x5B6EE1FF;

typedef struct Color {
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
} Color;

#define TRY_SDL(func, ...) if (func(__VA_ARGS__) != 0) { \
        fprintf(stderr, "SDL function " #func " raised an error! %s\n", SDL_GetError() ); \
        returnCode = 1; \
        goto end; \
    }

#define TRY_SDL_NULL(type, param, func, ...) type param = func(__VA_ARGS__); \
    if (param == NULL) { \
        fprintf(stderr, "SDL function " #func " raised an error! %s\n", SDL_GetError() ); \
        return 1; \
    }

SDL_Surface * collisionImage;


bool isColor(struct CLST_Vector2 position, uint32_t RGBA) {
    //return position.y > 130;
    
    int posX = floorf(position.x / WORLD_WIDTH * collisionImage->w);
    int posY = floorf(position.y / WORLD_HEIGHT * collisionImage->h);

#if SDL_BYTEORDER != SDL_BIG_ENDIAN
    RGBA = 
        ((RGBA & 0xFF000000) >> 24) |
        ((RGBA & 0x00FF0000) >> 8) |
        ((RGBA & 0x0000FF00) << 8) |
        ((RGBA & 0x000000FF) << 24);
#endif

    return ((uint32_t *) collisionImage->pixels)[posY * collisionImage->w + posX] == RGBA;
}

bool collideColor(struct CLST_Vector2 position, uint32_t RGBA) {
    return isColor(position, RGBA);
}

bool collisionCheck(const struct CLST_Madeline * maddy , struct CLST_Vector2 position) {
    return collideColor(position, SOLID);
}

bool waterCheck(const struct CLST_Madeline * maddy , struct CLST_Vector2 position) {
    return collideColor(position, WATER);
}

float signum(float value) {
    return (value > 0) - (value < 0);
}

int main(int argc, char *argv[]) {
    SDL_Window* window = NULL;   

    SDL_Event e;
    bool quit = false;
    int returnCode = 0;

    TRY_SDL(SDL_Init, SDL_INIT_EVERYTHING);

    TRY_SDL_NULL(,
        window,
        SDL_CreateWindow,
        "libmadeline demo", 
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
        SCREEN_WIDTH, SCREEN_HEIGHT,
        SDL_WINDOW_SHOWN
    );

    TRY_SDL_NULL(SDL_Surface *, collision, SDL_LoadBMP, "collision.bmp");
    TRY_SDL_NULL(SDL_Surface *, visual, SDL_LoadBMP, "visual.bmp");
    TRY_SDL_NULL(SDL_Renderer *, renderer, SDL_CreateRenderer, window, -1, SDL_RENDERER_ACCELERATED );
    TRY_SDL_NULL(SDL_Texture *, collisionTexture, SDL_CreateTextureFromSurface, renderer, collision); 
    TRY_SDL_NULL(SDL_Texture *, visualTexture, SDL_CreateTextureFromSurface, renderer, visual); 
    TRY_SDL_NULL(, collisionImage, SDL_ConvertSurfaceFormat, collision, SDL_PIXELFORMAT_RGBA32, 0);
    SDL_FreeSurface(collision);

    double xScale = SCREEN_WIDTH / collisionImage->w;
    double yScale = SCREEN_HEIGHT / collisionImage->h;

    if (collisionImage->pixels == NULL) {
        fprintf(stderr, "Failed to load map pixels!");
        return 1;
    }

    uint32_t * pixels = (uint32_t*) collisionImage->pixels;

    CLST_Vector2 spawnLocation = (CLST_Vector2) { .x = 0, .y = 0 };

    for (int y = 0; y < collisionImage->h; y++) {
        for (int x = 0; x < collisionImage->w; x++) {
            if (pixels[y * collisionImage->w + x] == SPAWN) {
                spawnLocation.x = ((float) x) / collisionImage->w * WORLD_WIDTH;
                spawnLocation.y = ((float) x) / collisionImage->w * WORLD_HEIGHT;
                goto exitloop;
            }
        }
    }
    exitloop:
    
    CLST_Madeline * maddy = CLST_New(spawnLocation);

    maddy->collision_callback = &collisionCheck;
    maddy->water_callback = &waterCheck;

    uint64_t now = SDL_GetPerformanceCounter();
    uint64_t last = 0;

    double deltaTime = 0;
    bool printPressedLastFrame = false;
    
    while (!quit) {
        // delta-time calculation
        last = now;
        now = SDL_GetPerformanceCounter();
        deltaTime = (now - last);
        deltaTime /= SDL_GetPerformanceFrequency();

        // event handling (controls)
        while (SDL_PollEvent(&e))
            if (e.type == SDL_QUIT)
                quit = true;

        const uint8_t * state = SDL_GetKeyboardState(NULL);

        maddy->input.aim.x = state[SDL_SCANCODE_RIGHT] - state[SDL_SCANCODE_LEFT];
        maddy->input.aim.y = state[SDL_SCANCODE_DOWN] - state[SDL_SCANCODE_UP];
        maddy->input.jump = state[SDL_SCANCODE_C];
        maddy->input.grab = state[SDL_SCANCODE_Z];
        maddy->input.dash = state[SDL_SCANCODE_X];
        if (maddy->input.aim.x != 0 && maddy->input.aim.y != 0) {
            maddy->input.aim.x *= M_SQRT1_2;
            maddy->input.aim.y *= M_SQRT1_2;
        }

        if (state[SDL_SCANCODE_P] && !printPressedLastFrame) {
            char* debugString = CLST_DebugString(maddy);
            fprintf(stderr, debugString);
            CLST_DropDebugString(debugString);
        }
        printPressedLastFrame = state[SDL_SCANCODE_P];

        // ticking madeline
        CLST_Tick(maddy, deltaTime);

        // graphics
        TRY_SDL(SDL_RenderCopy, renderer, visualTexture, NULL, NULL);
        CLST_Color rectColor;
        if (maddy->dash_attack_timer > 0.) {
            rectColor = CLST_Color_FLASH_HAIR;
        } else if (maddy->flash && maddy->stamina < CLST_CLIMB_TIRED_THRESHOLD) {
            rectColor = CLST_Color_RED;
        } else if (maddy->dashes == 1) {
            rectColor = CLST_Color_NORMAL_HAIR;
        } else {
            rectColor = CLST_Color_USED_HAIR;
        }

        TRY_SDL(SDL_SetRenderDrawColor, renderer, rectColor.r, rectColor.g, rectColor.b, rectColor.a);
        CLST_Vector2 center = (CLST_Vector2) {
            .x = maddy->position.x + maddy->collider.position.x + maddy->collider.size.x / 2,
            .y = maddy->position.y + maddy->collider.position.y + maddy->collider.size.y / 2,
        };
        CLST_Vector2 size = (CLST_Vector2) {
            .x = maddy->collider.size.x * maddy->sprite_scale.x,
            .y = maddy->collider.size.y * maddy->sprite_scale.y,
        };
        TRY_SDL(SDL_RenderFillRect, renderer, &(SDL_Rect) {
            (center.x - size.x / 2) * (SCREEN_WIDTH / WORLD_WIDTH),
            (center.y - size.y / 2) * (SCREEN_HEIGHT / WORLD_HEIGHT),
            size.x * (SCREEN_WIDTH / WORLD_WIDTH),
            size.y * (SCREEN_HEIGHT / WORLD_HEIGHT),
        });
        printf("\r%f    ", deltaTime);
        
        /*fprintf(
            stderr,
            "\x1b[0;0H\x1b[2Kpos: %f, %f (%f, %f)\n\x1b[2Kspd: %f, %f\n\x1b[2Kon_ground: %d\n\x1b[2Kstate: %d\n\x1b[2Kinput: %f, %f",
            maddy->position.x, maddy->position.y,
            maddy->rem_position.x, maddy->rem_position.y,
            maddy->speed.x, maddy->speed.y,
            maddy->on_ground,
            maddy->state,
            maddy->input.aim.x, maddy->input.aim.y
        );*/

        SDL_RenderPresent(renderer);

        double elapsed = (SDL_GetPerformanceCounter() - now) / SDL_GetPerformanceFrequency();
        SDL_Delay(1000 * fmaxf(1. / 60. - elapsed, 0));
    }

    end:

    SDL_DestroyTexture( visualTexture );
    SDL_DestroyTexture( collisionTexture );
    SDL_DestroyWindow( window );
    SDL_Quit();

    CLST_Drop(maddy);

    return returnCode;
}