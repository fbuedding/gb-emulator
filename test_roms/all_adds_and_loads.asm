;include "hardware.inc"
SECTION "Header", ROM0[$100]

    jp EntryPoint
    nop

    ds $150 - @, 0 ; Make room for the header

EntryPoint:
    ; Initialize global variables
    ld a, 0
    ld [test], a
    ld a, 5
    ld b, 5
    ld c, 5
    ld d, 5
    ld e, 5
    ld h, 5
    ld l, 5
    add a, b
    add a, c
    add a, d
    add a, e
    add a, h
    add a, l
    add a, a
    ld hl, $0f0f

SECTION "Test", WRAM0
    test: db


