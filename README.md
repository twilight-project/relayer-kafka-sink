# Relayer Kafka Sink

A Relayer Kafka Sink service that consumes messages from specific Kafka topics and stores them in PostgreSQL tables. This service is part of the Twilight Relayer ecosystem and stores RPC requests, failed requests, and event logs Kafka message backup in PostgreSQL DB. This package is created for debugging purposes and does not affect the relayer performance in any manner.

## Overview

The Relayer Kafka Sink (`relayer-kafka-sink`) is a multi-threaded service that:

- Consumes messages from 4 different Kafka topics
- Processes them concurrently using thread pools
- Stores them in corresponding PostgreSQL tables
- Provides reliable data persistence for the Twilight Relayer system

## Features

- **Multi-threaded processing**: Runs 4 separate threads for different data types
- **PostgreSQL integration**: Connection pooling for efficient database operations
- **Kafka consumer**: Consumes from multiple topics with advanced offset management
- **Thread pools**: Concurrent processing of database operations with configurable worker counts
- **Auto-initialization**: Automatically creates required database tables
- **Docker support**: Ready for containerized deployment
- **Offset tracking**: Advanced offset management for reliable message processing

## Architecture

The service processes 4 types of data streams:

1. **RPC Commands** (`CLIENT-REQUEST` topic → `rpc_query` table)
2. **Failed RPC Commands** (`CLIENT-FAILED-REQUEST` topic → `rpc_query_failed` table)
3. **Event Logs** (`CoreEventLogTopic` topic → `event_logs` table)
4. **Relayer State Queue** (`RelayerStateQueue` topic → `relayer_state_queue` table)

## Prerequisites

- Rust 1.87.0+ (2021 edition)
- PostgreSQL 12+
- Kafka cluster
- Docker (optional, for containerized deployment)

## Configuration

Configure the service using environment variables in a `.env` file. A comprehensive `.env.example` file is provided with detailed comments explaining each variable:

```env
# PostgreSQL Configuration
POSTGRESQL_URL=postgresql://username:password@localhost:5432/database_name

# Kafka Configuration
KAFKA_BROKER=localhost:9092

# Kafka Topics (optional, will use defaults if not specified)
RPC_CLIENT_REQUEST=CLIENT-REQUEST
RPC_CLIENT_FAILED_REQUEST=CLIENT-FAILED-REQUEST
CORE_EVENT_LOG=CoreEventLogTopic
RELAYER_STATE_QUEUE=RelayerStateQueue

# Kafka Consumer Groups (optional, will use defaults if not specified)
RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST=RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST
RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED=RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED
RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG=RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG
RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE=RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE
```

## Database Schema

The service automatically creates the following PostgreSQL tables:

```sql
-- RPC Query requests
CREATE TABLE IF NOT EXISTS public.rpc_query (
    "offset" bigint NOT NULL,
    key VARCHAR(1024) NOT NULL,
    payload json NOT NULL
);

-- Failed RPC requests
CREATE TABLE IF NOT EXISTS public.rpc_query_failed (
    "offset" bigint NOT NULL,
    key VARCHAR(1024) NOT NULL,
    payload json NOT NULL
);

-- Event logs
CREATE TABLE IF NOT EXISTS public.event_logs (
    "offset" bigint NOT NULL,
    key VARCHAR(1024) NOT NULL,
    payload json NOT NULL
);

-- Relayer state queue
CREATE TABLE IF NOT EXISTS public.relayer_state_queue (
    "offset" bigint NOT NULL,
    key VARCHAR(1024) NOT NULL,
    payload json NOT NULL
);

```

## Installation

### From Source

1. Clone the repository:

```bash
git clone https://github.com/twilight-project/relayer-kafka-sink.git
cd relayer-kafka-sink
```

2. Copy the example configuration and customize it:

```bash
cp .env.example .env
# Edit .env with your specific configuration values
```

3. Build:

```bash
cargo build --release
```

### Using Docker

1. Build the Docker image:

```bash
docker build -t relayer-kafka-sink .
```

2. Create your configuration file:

```bash
cp .env.example .env
# Edit .env with your specific configuration values
```

3. Run with your `.env` file:

```bash
docker run -d --env-file .env relayer-kafka-sink
```

## Usage

Once started, the service will:

1. Wait 10 seconds for initialization
2. Initialize PostgreSQL tables
3. Start 4 consumer threads:
   - `upload_rpc_command_to_psql` - Processes CLIENT-REQUEST topic
   - `upload_event_log_to_psql` - Processes CoreEventLogTopic
   - `upload_rpc_failed_command_to_psql` - Processes CLIENT-FAILED-REQUEST topic
   - `upload_relayer_state_queue_to_psql` - Processes RelayerStateQueue topic
4. Run continuously, consuming and storing messages

The service outputs:

```
relayer-kafka-sink running successfully...
```

## Thread Pool Configuration

Each consumer thread uses a thread pool with different worker counts:

- **RPC Client Pool**: 5 workers - Handles successful RPC requests
- **PSQL Client Failed Pool**: 2 workers - Handles failed RPC requests
- **PSQL Pool**: 5 workers - Handles event logs
- **Relayer State Queue Pool**: 2 workers - Handles relayer state queue messages

## Kafka Topics

The service consumes from these Kafka topics:

| Topic                   | Consumer Group                                   | Purpose                  |
| ----------------------- | ------------------------------------------------ | ------------------------ |
| `CLIENT-REQUEST`        | `RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST`        | RPC command requests     |
| `CLIENT-FAILED-REQUEST` | `RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED` | Failed RPC requests      |
| `CoreEventLogTopic`     | `RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG`        | System event logs        |
| `RelayerStateQueue`     | `RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE`   | Relayer state queue data |

## Troubleshooting

### Common Issues

1. **Database Connection Issues**

   - Verify `POSTGRESQL_URL` is correct
   - Check PostgreSQL is running and accessible
   - Ensure database exists and user has proper permissions

2. **Kafka Connection Issues**

   - Verify `KAFKA_BROKER` configuration
   - Check Kafka cluster is running
   - Ensure topics exist: `CLIENT-REQUEST`, `CLIENT-FAILED-REQUEST`, `CoreEventLogTopic`, `RelayerStateQueue`

3. **Service Startup Issues**
   - Check all required environment variables are set
   - Verify `.env` file is in the correct location and properly formatted
   - Reference `.env.example` for correct variable names and format
   - Check logs for initialization errors

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Support

For support and questions:

- Create an issue in the [GitHub repository](https://github.com/twilight-project/relayer-kafka-sink/issues)
- Contact the Twilight Project team

## Related Projects

- [Relayer Order API](https://github.com/twilight-project/relayer-order-api)
- [Twilight Relayer Core](https://github.com/twilight-project/relayer-core)
- [Twilight Project](https://github.com/twilight-project)
