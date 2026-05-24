# CI Triage

Не чините CI вслепую без логов. Сначала определите, какой step упал, и сохраните первый meaningful error block.

## Если упал cargo check

Пришлите:

- failed step: `Cargo check`;
- первую Rust compiler error-секцию;
- строки с `error[E...]`;
- file path и line numbers;
- 30-80 строк вокруг первой ошибки.

## Если упал cargo test

Пришлите:

- failed step: `Cargo test`;
- название failed test;
- panic/assertion output;
- stdout/stderr конкретного теста, если GitHub показывает его отдельно.

## Если упал npm install или npm ci

Пришлите:

- failed step: frontend dependency install;
- npm error block;
- package name/version, если ошибка связана с dependency resolution;
- npm version из лога, если он есть.

## Если упал npm run build

Пришлите:

- failed step: `Build frontend`;
- TypeScript или Vite error;
- file path и line number;
- 30-80 строк вокруг первой ошибки.

## Чего не делать

- Не удалять тесты только ради зелёного CI.
- Не выкидывать event/db/core функциональность ради сборки.
- Не добавлять telemetry, analytics, ad SDK, release publishing или secrets.
- Не начинать OS backend, пока Package 1 build не зелёный.
