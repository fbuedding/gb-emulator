# gb-emulator (WIP)

Ein in **Rust** geschriebener Game Boy (DMG-01) Emulator.

> **Status:** In aktiver Entwicklung (Work In Progress).
> Aktuell liegt der Fokus auf der Implementierung der CPU-Instruktionen und der Architektur. Es ist noch keine grafische Ausgabe von Spielen möglich.

Dieses Projekt dient primär zu Lernzwecken, um die interne Funktionsweise des Game Boys, die Z80-ähnliche Prozessorarchitektur und hardwarenahe Programmierung mit Rust zu verstehen.

## Features (Aktueller Stand)

* **CPU Core:**
    * Nachbildung der Register (8-Bit und 16-Bit Paare wie `AF`, `BC`, `HL`).
    * Implementierung des Flag-Registers (Zero, Subtract, Half-Carry, Carry).
    * Fetch-Decode-Execute Zyklus implementiert.
* **Instruction Set:**
    * Unterstützung für reguläre und `CB`-prefixed Opcodes.
    * Teilweise Implementierung von Arithmetic, Load und Jump Instruktionen.
* **Architektur:**
    * Modulare Struktur (`Cpu`, `MemoryBus`, `Instruction` Enums).
    * Eigene Datentypen für typsicheres Decoding (z.B. `R8`, `R16` Enums).
* **Testing & Qualitätssicherung:**
    * **Integration von RGBDS:** Das Projekt nutzt `build.rs`, um Assembler-Test-ROMs (`test_roms/*.asm`) automatisch zu kompilieren.
    * **Unit-Tests:** Die CPU wird gegen echte, kompilierte Hardware-Instruktionen getestet, um bit-genaue Ergebnisse sicherzustellen (z.B. `all_adds_and_loads`).
* **Frontend:**
    * Initiale Einbindung von **Raylib** für zukünftiges Rendering und Input-Handling.

## Voraussetzungen

Um das Projekt zu bauen und die Tests auszuführen, werden folgende Tools benötigt:

1.  **Rust & Cargo** (neueste stable Version)
2.  **RGBDS** (`rgbasm`, `rgblink`, `rgbfix`) - Wird benötigt, um die Test-ROMs im `test_roms/` Ordner zu kompilieren.
3.  **Raylib** Development Libraries (abhängig vom Betriebssystem).

## Build & Run

Das Projekt nutzt ein Build-Skript, das automatisch die Assembly-Dateien kompiliert, wenn Änderungen erkannt werden.

```bash
# Projekt bauen
cargo build

# Tests ausführen (führt die CPU-Tests gegen die kompilierten ROMs aus)
cargo test

# Emulator starten (Aktuell Debug-Fenster)
cargo run

```

## Dokumentation & Referenzen

Die Entwicklung stützt sich auf diverse technische Dokumentationen, die im `docs/` Ordner referenziert werden.
Die CPU-Implementierung orientiert sich unter anderem an:

* [Decoding Gameboy Z80 Opcodes](https://archive.gbdev.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html) - Algorithmus für das Instruction Decoding.
* RGBDS Community & Pan Docs.

## Roadmap

* [x] Grundlegendes CPU-Gerüst & Memory Bus
* [x] Automatische Kompilierung von Test-ROMs via `build.rs`
* [x] Instruction decoding
* [ ] Vervollständigung des Instruction Sets (ALU, Stack, Misc)
* [ ] Timer Implementierung
* [ ] PPU (Pixel Processing Unit) & Rendering
* [ ] Interrupt Handling
* [ ] Memory Banking Controllers (MBC)

---

*Disclaimer: Dieses Projekt ist nicht mit Nintendo verbunden.*

