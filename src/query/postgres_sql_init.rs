use crate::config::POSTGRESQL_POOL_CONNECTION;

/// Initialize all required PostgreSQL tables for the application
/// Creates tables for:
/// - Binance BTC ticker data
/// - Event logs
/// - RPC queries
/// - Failed RPC queries
/// - Relayer state queue
pub fn init_psql() {
    match create_binance_ticker_table() {
        Ok(_) => println!("binancebtctickernew table inserted successfully"),
        Err(arg) => println!("binancebtctickernew table creation failed, {:#?}", arg),
    }
    match create_event_logs_table() {
        Ok(_) => println!("event_logs table inserted successfully"),
        Err(arg) => println!("event_logs table creation failed, {:#?}", arg),
    }
    match create_rpc_query_table() {
        Ok(_) => println!("rpc_query table inserted successfully"),
        Err(arg) => println!("rpc_query table creation failed, {:#?}", arg),
    }
    match create_rpc_query_failed_req_table() {
        Ok(_) => println!("rpc_query failed table inserted successfully"),
        Err(arg) => println!("rpc_query failed table creation failed, {:#?}", arg),
    }
    match create_relayer_state_queue_table() {
        Ok(_) => println!("relayer_state_queue table inserted successfully"),
        Err(arg) => println!("relayer_state_queue table creation failed, {:#?}", arg),
    }
}

/// Creates the Binance BTC ticker table if it doesn't exist
///
/// Table schema:
/// - id: Auto-incrementing serial primary key
/// - e: Event type (VARCHAR)
/// - TimeStamp_E: Event timestamp (BIGINT)
/// - s: Symbol (VARCHAR)
/// - c,o,h,l: Close, Open, High, Low prices (NUMERIC)
/// - v,q: Volume metrics (NUMERIC)
/// - topic: Kafka topic (VARCHAR)
/// - partition_msg: Kafka partition (BIGINT)
/// - offset_msg: Kafka offset (BIGINT)
fn create_binance_ticker_table() -> Result<(), r2d2_postgres::postgres::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS binancebtctickernew(
        id SERIAL 
       ,e VARCHAR(14) NOT NULL
      ,TimeStamp_E BIGINT  NOT NULL
      ,s VARCHAR(7) NOT NULL
      ,c NUMERIC(50,8) NOT NULL
      ,o NUMERIC(50,8) NOT NULL
      ,h NUMERIC(50,8) NOT NULL
      ,l NUMERIC(50,8) NOT NULL
      ,v NUMERIC(150,8) NOT NULL
      ,q NUMERIC(150,8) NOT NULL
    ,topic VARCHAR(50) NOT NULL
    ,partition_msg BIGINT NOT NULL
    ,offset_msg BIGINT NOT NULL
    );
    "
    );
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    match client.execute(&query, &[]) {
        Ok(_) => Ok(()),
        Err(arg) => Err(arg),
    }
}

/// Creates the event logs table if it doesn't exist
///
/// Table schema:
/// - offset: Message offset (BIGINT)
/// - key: Message key (VARCHAR)
/// - payload: Message payload (JSON)
fn create_event_logs_table() -> Result<(), r2d2_postgres::postgres::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS public.event_logs (
            \"offset\" bigint NOT NULL,
            key VARCHAR(1024) NOT NULL,
            payload json NOT NULL
          );"
    );
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    match client.execute(&query, &[]) {
        Ok(_) => Ok(()),
        Err(arg) => Err(arg),
    }
}

/// Creates the RPC query table if it doesn't exist
///
/// Table schema:
/// - offset: Message offset (BIGINT)
/// - key: Message key (VARCHAR)
/// - payload: Message payload (JSON)
fn create_rpc_query_table() -> Result<(), r2d2_postgres::postgres::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS public.rpc_query (
            \"offset\" bigint NOT NULL,
            key VARCHAR(1024) NOT NULL,
            payload json NOT NULL
          );"
    );
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    match client.execute(&query, &[]) {
        Ok(_) => Ok(()),
        Err(arg) => Err(arg),
    }
}

/// Creates the failed RPC query table if it doesn't exist
///
/// Table schema:
/// - offset: Message offset (BIGINT)
/// - key: Message key (VARCHAR)
/// - payload: Message payload (JSON)
fn create_rpc_query_failed_req_table() -> Result<(), r2d2_postgres::postgres::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS public.rpc_query_failed (
            \"offset\" bigint NOT NULL,
            key VARCHAR(1024) NOT NULL,
            payload json NOT NULL
          );"
    );
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    match client.execute(&query, &[]) {
        Ok(_) => Ok(()),
        Err(arg) => Err(arg),
    }
}

/// Creates the relayer state queue table if it doesn't exist
///
/// Table schema:
/// - offset: Message offset (BIGINT)
/// - key: Message key (VARCHAR)
/// - payload: Message payload (JSON)
fn create_relayer_state_queue_table() -> Result<(), r2d2_postgres::postgres::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS public.relayer_state_queue (
            \"offset\" bigint NOT NULL,
            key VARCHAR(1024) NOT NULL,
            payload json NOT NULL
          );"
    );
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    match client.execute(&query, &[]) {
        Ok(_) => Ok(()),
        Err(arg) => Err(arg),
    }
}
