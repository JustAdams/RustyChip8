# Writing a basic Chip 8 interpreter in Rust

Chip8 is a simple virtual machine.

![chip8_R1IQKNaLWe](https://github.com/user-attachments/assets/8e02e0f3-a556-4019-a21d-dd727df4f913)
GIF ROM: outlaw.ch8

## Controls
The top left of a QWERTY keyboard is used for this implementation. I make no guarantee that the key ordering will match the orderings that the author of any given ROM has intended.

| Keyboard | Chip8 Mapping |
| -------- | ------------- |
|     1    |       1       |
|     2    |       2       |
|     3    |       3       |
|     4    |       C       |
|     Q    |       4       |
|     W    |       5       |
|     E    |       6       |
|     R    |       D       |
|     A    |       7       |
|     S    |       8       |
|     D    |       9       |
|     F    |       E       |
|     Z    |       A       |
|     X    |       0       |
|     C    |       B       |
|     V    |       F       |

## References
* https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
* https://austinmorlan.com/posts/chip8_emulator/
