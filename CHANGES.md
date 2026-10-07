# Progress Report

**Date:** 2026-10-07

## Completed Tasks (Latest)

### 1. Code Modularization
- Split `src/main.rs` into specialized modules:
  - `src/cli.rs`: CLI argument parsing via `clap`.
  - `src/config.rs`: configuration loading, default config creation, tag sorting.
  - `src/history.rs`: download history tracking and duplicate detection persistence.
  - `src/api.rs`: Rule34 API communication, post parsing, tag preparation.
  - `src/downloader.rs`: asynchronous multi-file downloader, progress bars, atomic file reservation.
  - `src/main.rs`: entry point orchestrating all components.

### 2. Standard Config Location (`~/.config/r34-dlp/config.toml`)
- Relocated default config from next to binary to `~/.config/r34-dlp/config.toml` (respects `$XDG_CONFIG_HOME`).
- Automatically creates parent directory if it does not exist.
- Simplified tag sorting: default tags are pre-sorted in `Config::default()`, and loaded configs are sorted in-memory via `config.global_negative_tags.sort()` without file splicing.

### 3. Non-blocking Asynchronous File I/O
- Replaced blocking `std::fs::File` and `std::io::copy` with `tokio::fs::File` and `tokio::io::AsyncWriteExt`.
- Streaming large files (MP4/WebM) no longer blocks Tokio runtime worker threads.

### 4. Safe Filename Reservation and Atomic Indexing
- Files are downloaded to temporary `.part` files while computing SHA256 hashes on the fly.
- Duplicate hash check is performed before reserving final file names.
- File naming is reserved only upon successful download and validation using `AtomicUsize`.
- The index is never decremented on errors or skips, preventing race conditions and file overwrite bugs.
- Renames temporary files to final target files atomically.

### 5. `cargo install` and `PKGBUILD`
- Added Arch Linux `PKGBUILD` for building and packaging `r34-dlp`.
- Added `cargo install --path .` instructions to documentation and updated `Cargo.toml` package metadata.

---

**Date:** 2026-05-28

## Completed Tasks (Previous)

### 1. Auto-sorting of Negative Tags

- Added `sort_negative_tags: Option<bool>` field to config (default: `true`)
- When loading config, tags in `global_negative_tags` are sorted alphabetically
- File structure is preserved

### 2. Parallel Downloads

- Added `parallel_downloads: Option<usize>` field to config (default: `4`)
- Migrated to tokio async runtime
- Uses `tokio::task::JoinSet` for parallel tasks
- `Semaphore` limits the number of concurrent downloads
- Switched from `reqwest::blocking` to async `reqwest`

### 3. `--skip-gif` Flag

- Added `--skip-gif` CLI argument
- When enabled, GIF files are completely ignored
- Not counted towards download limit
- Program requests additional API pages to retrieve the required number of non-GIF files

### 4. Progress Bar Fix

- Progress bar updated chunk-by-chunk using `file_res.chunk().await`
- MultiProgress support for parallel bars

---

# Отчет о проделанной работе

**Дата:** 2026-10-07

## Выполненные задачи (Свежие)

### 1. Модуляризация кодовой базы
- `src/main.rs` разбит на логические модули:
  - `src/cli.rs`: парсинг аргументов командной строки через `clap`.
  - `src/config.rs`: загрузка конфига, создание дефолтного конфига, сортировка тегов.
  - `src/history.rs`: история загрузок и сохранение хешей.
  - `src/api.rs`: запросы к API Rule34, разбор постов, кодирование и подготовка тегов.
  - `src/downloader.rs`: асинхронный загрузчик файлов, прогресс-бары, атомарное резервирование имен.
  - `src/main.rs`: точка входа, координирующая работу компонентов.

### 2. Расположение конфига в `~/.config/r34-dlp/config.toml`
- Конфиг перенесен из директории с бинарником в `~/.config/r34-dlp/config.toml` (с поддержкой `$XDG_CONFIG_HOME`).
- Автоматически создаются родительские каталоги.
- Упрощена сортировка: дефолтные теги отсортированы изначально в `Config::default()`, а при загрузке вызывается `config.global_negative_tags.sort()`.

### 3. Неблокирующий асинхронный ввод-вывод Tokio
- Блокирующие вызовы `std::fs::File` и `std::io::copy` заменены на `tokio::fs::File` и `tokio::io::AsyncWriteExt`.
- Загрузка тяжелых видеофайлов (MP4/WebM) больше не блокирует рабочие потоки рантайма Tokio.

### 4. Безопасное резервирование имен и индекс на `AtomicUsize`
- Файлы загружаются во временные `.part` файлы с потоковым расчетом SHA256 хеша.
- Проверка дубликатов по хешу выполняется до резервирования итогового имени.
- Имя файла и индекс резервируются только после успешной загрузки и проверки с помощью `AtomicUsize`.
- Индекс никогда не откатывается назад при ошибках или пропуске дубликатов, что исключает коллизии и перезапись файлов.
- Атомарное переименование временного файла в целевой через `tokio::fs::rename`.

### 5. `cargo install` и `PKGBUILD`
- Написан Arch Linux `PKGBUILD` для сборки и упаковки.
- Добавлен вариант установки через `cargo install --path .` в документацию, дополнены метаданные в `Cargo.toml`.

---

**Дата:** 2026-05-28

## Выполненные задачи (Предыдущие)

### 1. Автосортировка негативных тегов
- Добавлено поле `sort_negative_tags: Option<bool>` в конфиг (по умолчанию `true`)
- При загрузке конфига теги в `global_negative_tags` сортируются по алфавиту

### 2. Параллельная загрузка
- Добавлено поле `parallel_downloads: Option<usize>` в конфиг (по умолчанию `4`)
- Переведен на async runtime tokio
- Использован `Semaphore` для ограничения одновременных загрузок

### 3. Флаг `--skip-gif`
- Добавлен аргумент `--skip-gif` для пропуска GIF файлов

### 4. Исправление прогресс-бара
- Заменено чтение на chunk-by-chunk
- Использован `MultiProgress` из indicatif для отображения нескольких баров одновременно
