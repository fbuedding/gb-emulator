include "hardware.inc"
SECTION "Header", ROM0[$100]

    jp EntryPoint
    nop

    ds $150 - @, 0 ; Make room for the header

EntryPoint:
    ld a, 3
    add a, 5

