# r34-dlp

## Disclaimer

> ⚠️ **100% AI-generated slop:** This code was entirely architected, written, and refactored by AI agents under the supervision of a Supreme Femboy. Use at your own risk.

Rule34.xxx image and video downloader with resume capability.

## Features

- Parallel file downloads
- Automatic sorting of negative tags in the configuration
- Skip GIF files option
- Duplicate detection by hash
- Download history to prevent duplicates
- Progress bars for each file
- Non-blocking async disk I/O using Tokio
- Atomic file reservation to prevent file overwrites

## Installation

### Via `cargo install`

```bash
cargo install --path .
```

### Manual Build

```bash
cargo build --release
```

The executable will be located at `target/release/r34-dlp`.

### Arch Linux (PKGBUILD)

```bash
makepkg -si
```

## Usage

```bash
r34-dlp -t "tag1 tag2 -exclude_tag" -n 30
```

### CLI Arguments

| Argument | Description |
|----------|-------------|
| `-t, --tags <TAGS>` | Tags separated by space (required). Negative tags start with `-` |
| `-n, --limit <LIMIT>` | Number of posts to download (default: 20) |
| `-o, --output <DIR>` | Output directory (default: tags as folder name) |
| `-c, --config <FILE>` | Path to config file |
| `--dry-run` | Show what would be downloaded without downloading |
| `--no-history` | Do not save download history |
| `--check-hash` | Check SHA256 hashes to detect duplicates |
| `--skip-gif` | Skip GIF files entirely (not counted in limit) |

## Configuration

On first run, `config.toml` is created in `~/.config/r34-dlp/config.toml` (or `$XDG_CONFIG_HOME/r34-dlp/config.toml`):

```toml
api_key = "YOUR_API_HERE"
user_id = 1234567
global_negative_tags = [
    "3d",
    "low_res",
]
sort_negative_tags = true
parallel_downloads = 4
```

### Configuration Fields

| Field | Description |
|-------|-------------|
| `api_key` | API key from rule34.xxx |
| `user_id` | User ID |
| `global_negative_tags` | Tags to exclude from all downloads |
| `sort_negative_tags` | Automatically sort negative tags alphabetically |
| `parallel_downloads` | Number of parallel downloads |

## Examples

Download 50 images by tags, excluding 3d:

```bash
r34-dlp -t "furry -3d" -n 50
```

Download 30 files without GIFs to the specified folder:

```bash
r34-dlp -t "cat_girl" -n 30 --skip-gif -o ./my_collection
```

Preview what would be downloaded:

```bash
r34-dlp -t "dog" -n 10 --dry-run
```

## Supported Formats

- jpg, jpeg, png, gif, webm, mp4

## Download History

History is saved to `.downloaded.toml` in the output directory and contains:
- IDs of downloaded posts
- File hashes (if `--check-hash` is used)

This allows resuming interrupted downloads without duplicates.

---

# r34-dlp

Загрузчик изображений и видео с Rule34.xxx с поддержкой возобновления загрузки.

## Возможности

- Параллельная загрузка файлов
- Автоматическая сортировка негативных тегов в конфиге
- Пропуск GIF файлов
- Проверка на дубликаты по хешу
- История загрузок для предотвращения повторов
- Прогресс-бары для каждого файла
- Неблокирующий асинхронный ввод-вывод Tokio
- Атомарное резервирование имен файлов без риска перезаписи

## Установка

### Через `cargo install`

```bash
cargo install --path .
```

### Ручная сборка

```bash
cargo build --release
```

Исполняемый файл будет в `target/release/r34-dlp`.

### Arch Linux (PKGBUILD)

```bash
makepkg -si
```

## Использование

```bash
r34-dlp -t "tag1 tag2 -exclude_tag" -n 30
```

### Аргументы CLI

| Аргумент | Описание |
|----------|----------|
| `-t, --tags <TAGS>` | Теги через пробел (обязательно). Отрицательные теги начинаются с `-` |
| `-n, --limit <LIMIT>` | Количество постов для загрузки (по умолчанию 20) |
| `-o, --output <DIR>` | Выходная директория (по умолчанию — имя тегов) |
| `-c, --config <FILE>` | Путь к файлу конфигурации |
| `--dry-run` | Показать что будет скачано без фактической загрузки |
| `--no-history` | Не сохранять историю загрузок |
| `--check-hash` | Проверять SHA256 хеши для обнаружения дубликатов |
| `--skip-gif` | Полностью пропустить GIF файлы (не учитываются в лимите) |

## Конфигурация

При первом запуске создается `config.toml` в `~/.config/r34-dlp/config.toml` (или `$XDG_CONFIG_HOME/r34-dlp/config.toml`):

```toml
api_key = "YOUR_API_HERE"
user_id = 1234567
global_negative_tags = [
    "3d",
    "low_res",
]
sort_negative_tags = true
parallel_downloads = 4
```

### Поля конфигурации

| Поле | Описание |
|------|----------|
| `api_key` | API ключ от rule34.xxx |
| `user_id` | ID пользователя |
| `global_negative_tags` | Теги для исключения из всех загрузок |
| `sort_negative_tags` | Автоматически сортировать негативные теги по алфавиту |
| `parallel_downloads` | Количество параллельных загрузок |

## Примеры

Скачать 50 изображений по тегам, исключая 3d:

```bash
r34-dlp -t "furry -3d" -n 50
```

Скачать 30 файлов без GIF в указанную папку:

```bash
r34-dlp -t "cat_girl" -n 30 --skip-gif -o ./my_collection
```

Посмотреть что будет скачано:

```bash
r34-dlp -t "dog" -n 10 --dry-run
```

## Поддерживаемые форматы

- jpg, jpeg, png, gif, webm, mp4

## История загрузок

История сохраняется в файл `.downloaded.toml` в выходной директории и содержит:
- ID скачанных постов
- Хеши файлов (если используется `--check-hash`)

Это позволяет возобновлять прерванные загрузки без повторов.
