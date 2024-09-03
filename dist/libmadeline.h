/*
This code is licensed under the MPL, authored by baltdev, in the year 2024. Read here: https://www.mozilla.org/en-US/MPL/2.0/.


Celeste's Player.cs, which this is based on, is under the MIT License, included below:

--------
MIT License

Copyright (c) 2018 Noel Berry

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
--------

Generated with cbindgen. Probably don't edit this file.

Have fun you dork <3

*/

#ifndef AYO_MAD_LEAN_IN_OTHER_GAME_REAL
#define AYO_MAD_LEAN_IN_OTHER_GAME_REAL

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace celeste {
#endif  // __cplusplus

#define CLST_MAX_RUN 90.

#define CLST_RUN_ACCEL 1000.

#define CLST_RUN_REDUCE 400.

#define CLST_AIR_MULT 0.65

#define CLST_JUMP_GRACE_TIME 0.1

#define CLST_JUMP_SPEED -105.

#define CLST_JUMP_H_BOOST 40.

#define CLST_VAR_JUMP_TIME 0.2

#define CLST_CEILING_VAR_JUMP_GRACE 0.05

#define CLST_UPWARD_CORNER_CORRECTION 4

#define CLST_WALL_SPEED_RETENTION_TIME 0.06

#define CLST_WALL_JUMP_CHECK_DIST 3.

#define CLST_WALL_JUMP_FORCE_TIME 0.16

#define CLST_WALL_JUMP_H_SPEED (CLST_MAX_RUN + CLST_JUMP_H_BOOST)

#define CLST_WALL_SLIDE_TIME 1.2

#define CLST_WALL_SLIDE_START_MAX 20.

#define CLST_MAX_DASHES 1

#define CLST_CLIMB_MAX_STAMINA 110.

#define CLST_CLIMB_UP_COST (100. / 2.2)

#define CLST_CLIMB_STILL_COST 10.

#define CLST_CLIMB_JUMP_COST (CLST_CLIMB_MAX_STAMINA / 4.)

#define CLST_CLIMB_TIRED_THRESHOLD 20.

#define CLST_DASH_V_FLOOR_SNAP_DIST 3.

#define CLST_LIFT_X_CAP 250.

#define CLST_LIFT_Y_CAP -130.

#define CLST_SUPER_JUMP_H 260.

#define CLST_DUCK_SUPER_JUMP_X_MULT 1.25

#define CLST_DUCK_SUPER_JUMP_Y_MULT 0.5

#define CLST_SUPER_WALL_JUMP_VAR_TIME 0.25

#define CLST_SUPER_WALL_JUMP_H (CLST_MAX_RUN + (CLST_JUMP_H_BOOST * 2.))

#define CLST_SUPER_WALL_JUMP_SPEED -160.

#define CLST_CLIMB_JUMP_BOOST_TIME 0.2

#define CLST_REBOUND_SPEED_X 140.

#define CLST_REBOUND_SPEED_Y -120.

#define CLST_REBOUND_VAR_JUMP_TIME 0.15

#define CLST_REFLECT_BOUND_SPEED 220.

#define CLST_DASH_CORNER_CORRECTION 4

#define CLST_DODGE_SLIDE_SPEED_MULT 1.2

#define CLST_FAST_MAX_FALL 240.

#define CLST_MAX_FALL 160.

#define CLST_FAST_MAX_ACCEL 300.

#define CLST_CLIMB_CHECK_DIST 2.

#define CLST_CLIMB_UP_CHECK_DIST 2

#define CLST_CLIMB_GRAB_Y_MULT 0.2

#define CLST_CLIMB_NO_MOVE_TIME 0.1

#define CLST_CLIMB_UP_SPEED -45.

#define CLST_CLIMB_DOWN_SPEED 80.

#define CLST_CLIMB_SLIP_SPEED 30.

#define CLST_CLIMB_ACCEL 900.

#define CLST_CLIMB_HOP_X 100.

#define CLST_DUCK_CORRECT_CHECK 4

#define CLST_DUCK_CORRECT_SLIDE 50.

#define CLST_DUCK_FRICTION 500.

#define CLST_HALF_GRAV_THRESHOLD 40.

#define CLST_GRAVITY 900.

#define CLST_DASH_COOLDOWN 0.2

#define CLST_DASH_REFILL_COOLDOWN 0.1

#define CLST_DASH_ATTACK_TIME 0.3

#define CLST_DASH_SPEED 240.

#define CLST_SWIM_DASH_SPEED_MULT 0.75

#define CLST_DASH_TIME 0.15

#define CLST_END_DASH_SPEED 160.

#define CLST_END_DASH_UP_MULT 0.75

#define CLST_SWIM_Y_SPEED_MULT 0.5

#define CLST_SWIM_MAX_RISE -60.

#define CLST_SWIM_MAX 80.

#define CLST_SWIM_UNDERWATER_MAX 60.

#define CLST_SWIM_ACCEL 600.

#define CLST_SWIM_REDUCE 400.

typedef enum CLST_DashCollisionResults {
    ResIgnore,
    ResRebound,
    ResBounce,
} CLST_DashCollisionResults;

typedef enum CLST_DashCoroutineBreakpoint {
    BreakpointStart,
    Breakpoint1,
    Breakpoint2,
} CLST_DashCoroutineBreakpoint;

enum CLST_Facings
#ifdef __cplusplus
  : int8_t
#endif // __cplusplus
 {
    FacingLeft = -1,
    FacingRight = 1,
};
#ifndef __cplusplus
typedef int8_t CLST_Facings;
#endif // __cplusplus

typedef enum CLST_RumbleLength {
    LenShort,
    LenMedium,
} CLST_RumbleLength;

typedef enum CLST_RumbleStrength {
    StrLight,
    StrMedium,
    StrStrong,
} CLST_RumbleStrength;

typedef enum CLST_State {
    StNormal,
    StClimb,
    StDash,
    StSwim,
} CLST_State;

typedef struct CLST_Vector2 {
    float x;
    float y;
} CLST_Vector2;

/**
 * A struct detailing the current player input.
 *
 * Note that you will have to buffer input yourself.
 */
typedef struct CLST_Input {
    /**
     * The aim direction of the movement.
     */
    struct CLST_Vector2 aim;
    /**
     * The leniency per axis for having the stick point in an exact direction.
     */
    struct CLST_Vector2 deadzone;
    bool jump;
    bool grab;
    bool dash;
    bool talk;
    bool jump_consumed;
    bool grab_consumed;
    bool dash_consumed;
    bool talk_consumed;
} CLST_Input;

typedef struct CLST_Inventory {
    uint8_t max_dashes;
    bool no_refills;
    float max_stamina;
    float gravity_mult;
} CLST_Inventory;

typedef struct CLST_Hitbox {
    struct CLST_Vector2 position;
    struct CLST_Vector2 size;
} CLST_Hitbox;

typedef struct CLST_Color {
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
} CLST_Color;
#define CLST_Color_WHITE (CLST_Color){ .r = 255, .g = 255, .b = 255, .a = 255 }
#define CLST_Color_RED (CLST_Color){ .r = 255, .g = 0, .b = 0, .a = 255 }
#define CLST_Color_NORMAL_HAIR (CLST_Color){ .r = 172, .g = 50, .b = 50, .a = 255 }
#define CLST_Color_FLY_POWER_HAIR (CLST_Color){ .r = 242, .g = 235, .b = 109, .a = 255 }
#define CLST_Color_USED_HAIR (CLST_Color){ .r = 0, .g = 68, .b = 183, .a = 255 }
#define CLST_Color_FLASH_HAIR CLST_Color_WHITE
#define CLST_Color_TWO_DASHES_HAIR (CLST_Color){ .r = 255, .g = 109, .b = 239, .a = 255 }
#define CLST_Color_TRANSPARENT (CLST_Color){ .r = 0, .g = 0, .b = 0, .a = 0 }

/**
 * An instance of Madeline's movement controller.
 *
 * Don't set any fields that don't have documentation
 * unless you know what you're doing
 * or it's clear what that will do.
 */
typedef struct CLST_Madeline {
    /**
     * Madeline's state. DO NOT manually set this.
     * Instead, use CLST_SetState.
     */
    enum CLST_State state;
    /**
     * The player's input.
     */
    struct CLST_Input input;
    /**
     * A callback when sound is supposed to be played.
     * These sounds can be mapped to whatever you want.
     *
     * Note that these sound names are not allocated, and are in fact pointers to static values!
     * You do not need to, and in fact should not, delete, free, or use CLST_DropDebugString on them.
     */
    void (*sound_callback)(const char*);
    /**
     * A callback for controller rumble.
     */
    void (*rumble_callback)(enum CLST_RumbleStrength, enum CLST_RumbleLength);
    /**
     * A callback to run to see if a position is solid.
     * If unset, this will default to false.
     */
    bool (*collision_callback)(const struct CLST_Madeline*, struct CLST_Vector2);
    /**
     * A callback to run to see if a position has swimmable water in it.
     * If unset, this will default to false.
     */
    bool (*water_callback)(const struct CLST_Madeline*, struct CLST_Vector2);
    /**
     * A callback to run to see if Madeline can climb a wall at a position.
     * If unset, this will default to true.
     */
    bool (*can_climb_callback)(const struct CLST_Madeline*, struct CLST_Vector2);
    /**
     * A callback that's run when Madeline collides with a solid object while dashing.
     * If unset, this will default to Ignore.
     */
    enum CLST_DashCollisionResults (*dash_collision_callback)(const struct CLST_Madeline*,
                                                              struct CLST_Vector2);
    /**
     * A callback to call to determine a friction factor.
     * Returning 0 means no friction, and returning 1 means full friction.
     * If unset, this will default to 1.
     */
    float (*friction_callback)(const struct CLST_Madeline*);
    /**
     * Madeline's current inventory.
     */
    struct CLST_Inventory inventory;
    /**
     * The speed of the surface that Madeline is on.
     * Make sure you set this when Madeline is on top of a moving object!
     */
    struct CLST_Vector2 lift_speed;
    float time_active;
    float coroutine_timer;
    enum CLST_DashCoroutineBreakpoint dash_coroutine_breakpoint;
    struct CLST_Vector2 position;
    struct CLST_Vector2 rem_position;
    struct CLST_Vector2 speed;
    struct CLST_Hitbox collider;
    struct CLST_Hitbox hurtbox;
    CLST_Facings facing;
    bool just_respawned;
    float stamina;
    uint8_t last_dashes;
    uint8_t dashes;
    uint8_t max_dashes;
    float dash_attack_timer;
    float dash_cooldown_timer;
    float dash_refill_cooldown_timer;
    struct CLST_Vector2 dash_dir;
    struct CLST_Vector2 before_dash_speed;
    bool started_dashing;
    bool dash_started_on_ground;
    struct CLST_Vector2 last_aim;
    float idle_timer;
    bool on_ground;
    bool was_on_ground;
    int8_t wall_slide_dir;
    float wall_slide_timer;
    int8_t wall_boost_dir;
    float wall_boost_timer;
    bool auto_jump;
    float jump_grace_timer;
    float var_jump_timer;
    float var_jump_speed;
    float auto_jump_timer;
    int8_t move_x;
    int8_t force_move_x;
    float force_move_x_timer;
    float wall_speed_retention_timer;
    float wall_speed_retained;
    struct CLST_Color hair_color;
    float hair_flash_timer;
    struct CLST_Color override_hair_color;
    bool flash;
    float play_footstep_on_land;
    float highest_air_y;
    int8_t hop_wait_x;
    float hop_wait_x_speed;
    bool was_tired;
    bool was_ducking;
    float max_fall;
    float climb_no_move_timer;
    int8_t last_climb_move;
} CLST_Madeline;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Allocates a new instance of Madeline on the heap.
 *
 * **DO NOT DELETE OR FREE THIS.**
 * Instead, use CLST_Drop to safely deallocate the object.
 */
struct CLST_Madeline *CLST_New(struct CLST_Vector2 position);

/**
 * Deallocates the object allocated from CLST_New.
 *
 * The pointer passed in **must come from CLST_New.**
 */
void CLST_Drop(struct CLST_Madeline *this_);

/**
 * Gets a string representaiton of this object.
 * This should probably only be used for debugging.
 *
 * **DO NOT DELETE OR FREE THE STRING.**
 * Instead, pass it to CLST_DropDebugString to safely deallocate it.
 *
 * Along with this, do not alter the string before deallocating it.
 */
char *CLST_DebugString(const struct CLST_Madeline *self);

/**
 * Safely deallocates a string obtained via CLST_DebugString.
 *
 * Do not pass a string from sound_callback to this function!
 */
void CLST_DropDebugString(char *char_);

/**
 * Ticks Madeline's internal state, using the delta-time between the last tick.
 */
void CLST_Tick(struct CLST_Madeline *self, float delta_time);

bool CLST_MoveHExact(struct CLST_Madeline *self, int32_t amount, bool callback);

bool CLST_MoveVExact(struct CLST_Madeline *self, int32_t amount, bool callback);

/**
 * Moves Madeline on the X axis.
 */
bool CLST_MoveH(struct CLST_Madeline *self, float amount, bool callback);

/**
 * Moves Madeline on the Y axis.
 */
bool CLST_MoveV(struct CLST_Madeline *self, float amount, bool callback);

/**
 * Sets the state of Madeline's state machine,
 * respecting the state's methods for beginning and ending.
 *
 * You should probably always use this instead of setting state directly.
 */
void CLST_SetState(struct CLST_Madeline *self, enum CLST_State state);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace celeste
#endif  // __cplusplus

#endif  /* AYO_MAD_LEAN_IN_OTHER_GAME_REAL */
