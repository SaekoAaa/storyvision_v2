# Мигратор базы данных для storyvision
Использует mysql crate и чтение из файла для удобной миграции туда и обратно
Переменная окружения IS_REVERT для обратной миграции базы данных
Записывает сборку в DOCKER_VERSION.yaml файл

## compose.yaml исключительно для тестирования работоспособности

## Типы миграций (MIGRATION_TYPE=):
1 - applies migration
2 - reverts migration
3 - applies and fills data
4 - applies but clears data
