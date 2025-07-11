//! # Relayer Kafka Sink
//!
//! A high-throughput Kafka sink service for processing, debugging and storing relayer data
//! from Kafka streams to PostgreSQL.
//!
//! This service is part of the Twilight Relayer ecosystem and stores RPC requests, failed requests,
//! and event logs Kafka message backup in PostgreSQL DB. This package is created for debugging
//! purposes and does not affect the relayer performance.
//!
//! ## Features
//!
//! - Multi-threaded processing with 4 separate threads for different data types
//! - PostgreSQL integration with connection pooling for efficient database operations  
//! - Kafka consumer that handles multiple topics with advanced offset management
//! - Thread pools for concurrent processing of database operations
//! - Automatic initialization of required database tables
//! - Docker support for containerized deployment
//!
//! ## Usage
//!
//! Configure the service using environment variables:
//!
//! ```env
//! # PostgreSQL Configuration
//! POSTGRESQL_URL=postgresql://username:password@localhost:5432/database_name
//!
//! # Kafka Configuration  
//! KAFKA_BROKER=localhost:9092
//! ```
//!
//! The service will:
//!
//! 1. Initialize PostgreSQL tables
//! 2. Start consumer threads for:
//!    - CLIENT-REQUEST topic
//!    - CoreEventLogTopic
//!    - CLIENT-FAILED-REQUEST topic
//!    - RelayerStateQueue topic
//! 3. Run continuously, consuming and storing messages
//!
//! See the [README](https://github.com/twilight-project/relayer-kafka-sink) for complete documentation.

pub mod config;
pub mod kafkalib;
pub mod query;
pub mod threadpool;

#[macro_use]
extern crate lazy_static;
