

## Запуск

```bash
cargo run
```

## Управление

| Клавиша | Действие |
|---------|----------|
| WASD    | Движение |
| E       | Действие |
| Esc     | Меню (Save / Load / Keybinds / Exit) |

Кейбинды меняются в меню → Управление. Сохраняются вместе с игрой.

## Сборка под Windows из Arch

```bash
# Один раз:
sudo pacman -S mingw-w64-gcc
rustup target add x86_64-pc-windows-gnu

# Собрать .exe:
cargo build --release --target x86_64-pc-windows-gnu
# Файл: target/x86_64-pc-windows-gnu/release/kk-game.exe
```

## Save-файлы

- Windows: `%APPDATA%\KoryavayaIgra\KoryavayaIgra\`
- Linux:   `~/.config/KoryavayaIgra/KoryavayaIgra/`
# kk-game
# kk-game
