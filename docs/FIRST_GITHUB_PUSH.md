# Первый push на GitHub

Этот документ описывает ручную подготовку первого GitHub push для JSentinel.

Не вставляйте приватные токены в репозиторий, docs, issues или CI logs.

## 1. Создать пустой репозиторий на GitHub

На GitHub создайте новый пустой репозиторий без автогенерации README, license и `.gitignore`, потому что эти файлы уже есть локально.

Remote URL будет такого вида:

```text
https://github.com/<OWNER>/<REPO>.git
```

## 2. Инициализировать git, если ещё не сделано

Из корня проекта:

```powershell
cd G:\JSentinel
git init
git branch -M main
```

## 3. Добавить remote origin

```powershell
git remote add origin https://github.com/<OWNER>/<REPO>.git
```

Если remote уже существует:

```powershell
git remote -v
git remote set-url origin https://github.com/<OWNER>/<REPO>.git
```

## 4. Сделать первый commit

```powershell
git status
git add .
git commit -m "Initial JSentinel scaffold"
```

Перед commit проверьте, что в staged files нет `.env`, локальных SQLite DB, `target/`, `node_modules/` или других dev artifacts.

## 5. Push в main

```powershell
git push -u origin main
```

## 6. Где смотреть GitHub Actions

Откройте репозиторий на GitHub и перейдите во вкладку `Actions`.

Первый CI должен запустить:

- `cargo check --workspace`
- `cargo test --workspace`
- frontend dependency install
- `npm run build`

## 7. Что прислать, если CI упал

Пришлите:

- название failed job;
- название failed step;
- error block из лога;
- 30-80 строк вокруг первой реальной ошибки;
- если ошибка повторяется ниже, достаточно первого meaningful блока.

Не присылайте токены, cookies, private keys или secrets.
