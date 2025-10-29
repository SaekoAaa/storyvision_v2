
# StoryMotion Backend Server

## В данный момент полностью переношу архитектуру на Feature-based по причине сложности деплоя.
### Было: 
Layered Architecture

<img width="327" height="588" alt="image" src="https://github.com/user-attachments/assets/76cbde6c-f174-42a1-8579-895e9106666a" />

### Стало:
Feature based architecture

<img width="402" height="574" alt="image" src="https://github.com/user-attachments/assets/3fd03e28-d38f-4f8a-87a5-8b5c9a0407b7" />

## Запуск

```bash
# Локальный запуск
cargo run

# Запуск контейнера
docker compose up -d --build
```

## Архитектура
~~ В процессе ~~

## Основные возможности

* **SOLID-паттерны** — чистая и расширяемая структура
* **Валидация запросов** — строгая проверка входных данных
* **Тестирование отдельных слоёв** (*в процессе покрытия Service Layer*)
* **API-документация** — сгенерирована через [utoipa](https://docs.rs/utoipa/latest/utoipa/)
* **Запуск в контейнере** — Docker и Docker Compose
* **Логирование и метрики** — централизованный сбор логов, метрики в процессе интеграции

## План спринтов

| # | Этап                                              | Статус         |
| - | ------------------------------------------------- | -------------- |
| 1 | Создание шаблона Figma и схемы БД              | ✅ Завершено    |
| 2 | Авторизация и заполнение БД                    | ✅ Завершено    |
| 3 | API проектов, страница GitHub                  | ✅ Завершено    |
| 4 | Документирование и тестирование API            | ✅ Завершено    |
| 5 | API героев и событий, метрики                  | 🚧 В работе    |
| 6 | Подключение Neo4j                             | 🚧 Планируется |
| 7 | Тестирование Service layer, Postman collection | 🚧 В процессе  |

## API

Документация доступна по эндпоинту:

```
/rapidoc
```

(генерируется автоматически через `utoipa`)


## Идеи для развития

* [ ] Добавить OpenTelemetry для метрик
* [ ] Реализовать кэширование на уровне сервиса
* [ ] Автоматизировать миграции и CI/CD
* [ ] Добавить интеграционные тесты


## Технологии

* **Rust**, `axum`, `tokio`, `sqlx`, `utoipa`
* **MySQL / Neo4j**
* **Docker Compose**
* **Postman**, **Figma**

## Галерея
### Обработка ошибок и документация

<img width="892" height="1042" alt="image" src="https://github.com/user-attachments/assets/ffda4862-f638-4dfc-a814-1e7581b7dc82" />
<img width="1667" height="524" alt="image" src="https://github.com/user-attachments/assets/41b3fca0-13ba-4b68-a1ad-6ab066da5587" />

### Логирование в консоль

<img width="1051" height="92" alt="image" src="https://github.com/user-attachments/assets/0cfcee19-18b6-4f49-b7ec-db7654cda21a" />

### Мигратор базы данных 

<img width="875" height="171" alt="image" src="https://github.com/user-attachments/assets/53233289-96cc-45c6-a2d0-9c63da38d340" />

---

**Storyvision** — часть проекта *StoryMotion*
Создано с ❤️ на Rust
