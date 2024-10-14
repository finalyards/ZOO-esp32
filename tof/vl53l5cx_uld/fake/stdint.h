#pragma once
// tbd. Get proper RISCV32 includes, some day

typedef signed char int8_t;
typedef short int16_t;

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;

_Static_assert(sizeof(int8_t) == 1, "int8_t wrong size");
_Static_assert(sizeof(int16_t) == 2, "int16_t wrong size");

_Static_assert(sizeof(uint8_t) == 1, "uint8_t wrong size");
_Static_assert(sizeof(uint16_t) == 2, "uint16_t wrong size");
_Static_assert(sizeof(uint32_t) == 4, "uint32_t wrong size");
