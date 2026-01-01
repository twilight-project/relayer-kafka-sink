#![allow(dead_code)]
#![allow(unused_imports)]
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

lazy_static! {
    /// Static Global PostgreSQL Pool connection
    /// Initializes a connection pool to PostgreSQL using environment variables
    pub static ref POSTGRESQL_POOL_CONNECTION: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        dotenv::dotenv().ok();
        // Get PostgreSQL URL from environment variable
        let postgresql_url =
            std::env::var("POSTGRESQL_URL").expect("missing environment variable POSTGRESQL_URL");
        let manager = PostgresConnectionManager::new(
            postgresql_url.parse().unwrap(),
            NoTls,
        );
        r2d2::Pool::new(manager).unwrap()
    };

    /// Kafka topic name for client RPC requests
    pub static ref RPC_CLIENT_REQUEST: String =
        std::env::var("RPC_CLIENT_REQUEST").unwrap_or(String::from("CLIENT-REQUEST"));

    /// Kafka topic name for failed client RPC requests
    pub static ref RPC_CLIENT_FAILED_REQUEST: String =
        std::env::var("RPC_CLIENT_FAILED_REQUEST").unwrap_or(String::from("CLIENT-FAILED-REQUEST"));

    /// Kafka topic name for core event logs
    pub static ref CORE_EVENT_LOG: String =
        std::env::var("CORE_EVENT_LOG").unwrap_or(String::from("CoreEventLogTopic"));

    /// Kafka consumer group for client RPC requests
    pub static ref RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST: String =
        std::env::var("RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST")
            .unwrap_or(String::from("RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST"));

    /// Kafka consumer group for failed client RPC requests
    pub static ref RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED: String =
        std::env::var("RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED")
            .unwrap_or(String::from("RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED"));

    /// Kafka consumer group for core event logs
    pub static ref RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG: String =
        std::env::var("RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG")
            .unwrap_or(String::from("RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG"));

    /// Kafka broker address
        pub static ref BROKERS: Vec<String> = {
        dotenv::dotenv().ok();

        std::env::var("KAFKA_BROKER")
            .unwrap_or_else(|_| "localhost:9092".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    /// Kafka topic name for relayer state queue
    pub static ref RELAYER_STATE_QUEUE: String =
        std::env::var("RELAYER_STATE_QUEUE").unwrap_or(String::from("RelayerStateQueue"));

    /// Kafka consumer group for relayer state queue
    pub static ref RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE: String =
        std::env::var("RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE")
            .unwrap_or(String::from("RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE"));
}
