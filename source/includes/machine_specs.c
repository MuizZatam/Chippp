# include <stdint.h>
# include <stdbool.h>


# define MEMORY_SIZE 4096
# define DISPLAY_WIDTH 64 * 100 // scaled by a factor of 100
# define DISPLAY_HEIGHT 32 * 100
# define STACK_SIZE 16
# define REGISTERS 16


typedef struct {
    
    // 4096 elements x 8 bits = 32,768 bits i.e 4kb or 4,096 bytes
    uint8_t memory[MEMORY_SIZE];

    // Represents a True value for an active display cell,
    // False for 64 x 32 = 2,048 cells
    // Each cell is scaled up 100 times
    bool display[DISPLAY_WIDTH * DISPLAY_HEIGHT];

    // 16 elements x 16 bits = 256 bits i.e. 32 bytes
    uint16_t stack[STACK_SIZE];

    // Program counter
    uint16_t pc;

    // Index register
    uint16_t I;

    // Timers
    uint8_t delay_timer;
    uint8_t sound_timer;

    // General-purpose registers V0-VF
    uint8_t V[CHIP8_REGISTER_COUNT];

} Machine;



