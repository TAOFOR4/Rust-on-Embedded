### `memory.x`
The `memory.x` file is a linker script that defines the memory layout for the embedded system. It specifies the locations and sizes of the Flash and RAM memory regions, ensuring proper placement of program code and data within the device's memory. For STM32F103T8C6, this includes defining 64 KB of Flash and 20 KB of SRAM.

### `.cargo/config`
The `.cargo/config` file configures the Rust build system for embedded targets. It specifies the target architecture (e.g., ARM Cortex-M) and additional linker arguments. This setup ensures that the Rust program is compiled correctly for the embedded system.
