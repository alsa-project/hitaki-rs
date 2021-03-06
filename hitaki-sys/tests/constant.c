// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_FAILED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_DISCONNECTED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_LOCKED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_NOT_OPENED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_OPENED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_UNLOCKED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_IS_USED);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_ERROR_WRONG_CLASS);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_BEBOB);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_DICE);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_DIGI00X);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_FIREFACE);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_FIREWORKS);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_MOTU);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_OXFW);
    PRINT_CONSTANT((gint) HITAKI_ALSA_FIREWIRE_TYPE_TASCAM);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_CHANNEL);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_CLOCK);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_COMMAND);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_LED);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_MIRROR);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_PAN);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_PARAMETER);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_QUAD_COUNT);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_BAD_RATE);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_COMM_ERR);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_DSP_TIMEOUT);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_FLASH_BUSY);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_INCOMPLETE);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_INVALID);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_OK);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_TIMEOUT);
    PRINT_CONSTANT((gint) HITAKI_EFW_PROTOCOL_ERROR_UNSUPPORTED);
    return 0;
}
