# Writing a basic Chip 8 interpreter in Rust

Chip8 is a simple virtual machine.

![Discord_9RP3HKI9Y2](https://github.com/user-attachments/assets/3e597c36-e94f-40e6-81c4-02e5a36dd5e5)
GIF ROM: octojam1title.ch8

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
