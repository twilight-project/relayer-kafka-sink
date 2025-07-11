// Allow unused code and imports during development
#![allow(dead_code)]
#![allow(unused_imports)]

// Import required modules
mod config;
mod kafkalib;
mod query;
mod threadpool;

// Import dependencies
use config::*;
use query::*;
use std::{thread, time};

// Enable lazy_static macro
#[macro_use]
extern crate lazy_static;

fn main() {
    // Initial delay to allow services to start up
    thread::sleep(time::Duration::from_millis(10000));

    // Load environment variables from .env file
    dotenv::dotenv().expect("Failed loading dotenv");

    // Initialize PostgreSQL tables
    init_psql();
    thread::sleep(time::Duration::from_millis(100));
    // Spawn thread for processing RPC commands
    let upload_rpc_command_to_psql_thread = thread::Builder::new()
        .name(String::from("upload_rpc_command_to_psql"))
        .spawn(move || {
            crate::query::upload_rpc_command_to_psql();
        })
        .unwrap();

    // Spawn thread for processing event logs
    let upload_event_log_to_psql_thread = thread::Builder::new()
        .name(String::from("upload_event_log_to_psql"))
        .spawn(move || {
            crate::query::upload_event_log_to_psql();
        })
        .unwrap();

    // Spawn thread for processing failed RPC commands
    let upload_rpc_failed_command_to_psql_thread = thread::Builder::new()
        .name(String::from("upload_rpc_failed_command_to_psql"))
        .spawn(move || {
            crate::query::upload_rpc_failed_command_to_psql();
        })
        .unwrap();

    // Spawn thread for processing relayer state queue
    let upload_relayer_state_queue_to_psql_thread = thread::Builder::new()
        .name(String::from("upload_relayer_state_queue_to_psql"))
        .spawn(move || {
            crate::query::upload_relayer_state_queue_to_psql();
        })
        .unwrap();

    println!("relayer-kafka-sink running successfully...");
    upload_rpc_command_to_psql_thread.join().unwrap();
    upload_event_log_to_psql_thread.join().unwrap();
    upload_rpc_failed_command_to_psql_thread.join().unwrap();
    upload_relayer_state_queue_to_psql_thread.join().unwrap();
}
