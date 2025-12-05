# Exchange gRPC

Пример реализации биржи с торговыми операциями на gRPC.

## Структура проекта

```
exchange-grpc/
├── Cargo.toml          # Зависимости и конфигурация
├── build.rs            # Генерация кода из proto
├── proto/
│   └── exchange.proto  # Protobuf схема сервиса
├── src/
│   ├── main.rs         # Точка входа сервера
│   ├── server.rs        # Реализация gRPC сервера
│   ├── client.rs        # Клиент для работы с сервером
│   └── lib.rs           # Публичный API библиотеки
└── examples/
    └── client_example.rs # Пример использования клиента
```

## Установка зависимостей

### macOS
```bash
brew install protobuf
```

### Linux
```bash
# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# Или скачайте с https://github.com/protocolbuffers/protobuf/releases
```

## Сборка

```bash
cargo build
```

## Запуск

### 1. Запустите сервер

В первом терминале:
```bash
RUST_LOG=info cargo run --bin exchange_server
```

Сервер запустится на `127.0.0.1:50051`.

### 2. Запустите клиент

Во втором терминале:
```bash
RUST_LOG=info cargo run --example client_example
```

## Тестирование с grpcurl

Установите grpcurl:
```bash
brew install grpcurl  # macOS
```

Проверьте список сервисов:
```bash
grpcurl -plaintext localhost:50051 list
```

Создайте ордер:
```bash
grpcurl -plaintext -d '{
  "user_id": "user_456",
  "symbol": "ETH",
  "type": "ORDER_TYPE_BUY",
  "price": 3000.0,
  "quantity": 50
}' localhost:50051 exchange.ExchangeService/CreateOrder
```

Получите баланс:
```bash
grpcurl -plaintext -d '{
  "user_id": "user_456",
  "currency": "USD"
}' localhost:50051 exchange.ExchangeService/GetBalance
```

## API

Сервис поддерживает следующие методы:

- `CreateOrder` - создание ордера на покупку/продажу
- `GetBalance` - получение баланса пользователя
- `GetActiveOrders` - получение списка активных ордеров
- `StreamQuotes` - поток котировок в реальном времени (server streaming)
- `CancelOrder` - отмена ордера

## Зависимости

- `tonic` - gRPC фреймворк для Rust
- `tokio` - асинхронный runtime
- `prost` - сериализация protobuf
- `rand` - генерация случайных чисел
- `chrono` - работа с датами и временем

