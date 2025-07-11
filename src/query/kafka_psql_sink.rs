// Import required dependencies
use crate::config::*;
use crate::kafkalib::offset_manager::OffsetManager;
use crate::threadpool::ThreadPool;
use crossbeam_channel::{unbounded, Receiver, Sender};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::{thread, time};

// Type alias for offset completion tracking
pub type OffsetCompletion = (i32, i64);

extern crate postgres_types;

/// Uploads RPC commands from Kafka to PostgreSQL
/// Uses a threadpool to process messages concurrently
pub fn upload_rpc_command_to_psql() {
    let (recever, tx_consumed) = receive_event_from_kafka_queue(
        // RPC_CLIENT_REQUEST.clone().to_string(),
        RPC_CLIENT_REQUEST.clone(),
        RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST.clone(),
        0,
    )
    .unwrap();
    let threadpool = ThreadPool::new(5, String::from("upload_rpc_command_to_psql"));
    let recever1 = recever.lock().unwrap();
    loop {
        let (data, offset_complition) = recever1.recv().unwrap();
        let tx_consumed_clone = tx_consumed.clone();
        threadpool.execute(move || {
            psql_rpc_command(data, tx_consumed_clone, offset_complition);
        });
    }
}

/// Uploads failed RPC commands from Kafka to PostgreSQL
/// Uses a threadpool to process messages concurrently
pub fn upload_rpc_failed_command_to_psql() {
    let (recever, tx_consumed) = receive_event_from_kafka_queue(
        // RPC_CLIENT_REQUEST.clone().to_string(),
        RPC_CLIENT_FAILED_REQUEST.clone(),
        RELAYER_KAFKA_SINK_GROUP_CLIENT_REQUEST_FAILED.clone(),
        0,
    )
    .unwrap();
    let threadpool = ThreadPool::new(2, String::from("upload_rpc_failed_command_to_psql"));
    let recever1 = recever.lock().unwrap();
    loop {
        let (data, offset_complition) = recever1.recv().unwrap();
        let tx_consumed_clone = tx_consumed.clone();
        threadpool.execute(move || {
            psql_rpc_failed_command(data, tx_consumed_clone, offset_complition);
        });
    }
}

/// Uploads event logs from Kafka to PostgreSQL
/// Uses a threadpool to process messages concurrently
pub fn upload_event_log_to_psql() {
    let (recever, tx_consumed) = receive_event_from_kafka_queue(
        // CORE_EVENT_LOG.clone().to_string(),
        CORE_EVENT_LOG.clone(),
        RELAYER_KAFKA_SINK_GROUP_CORE_EVENT_LOG.clone(),
        0,
    )
    .unwrap();
    let threadpool = ThreadPool::new(5, String::from("upload_event_log_to_psql"));
    let recever1 = recever.lock().unwrap();
    loop {
        let (data, offset_complition) = recever1.recv().unwrap();
        let tx_consumed_clone = tx_consumed.clone();
        threadpool.execute(move || {
            psql_event_logs(data, tx_consumed_clone, offset_complition);
        });
    }
}

/// Uploads relayer state queue messages from Kafka to PostgreSQL
/// Uses a threadpool to process messages concurrently
pub fn upload_relayer_state_queue_to_psql() {
    let (recever, tx_consumed) = receive_event_from_kafka_queue(
        RELAYER_STATE_QUEUE.clone(),
        RELAYER_KAFKA_SINK_GROUP_RELAYER_STATE_QUEUE.clone(),
        0,
    )
    .unwrap();
    let threadpool = ThreadPool::new(2, String::from("upload_relayer_state_queue_to_psql"));
    let recever1 = recever.lock().unwrap();
    loop {
        let (data, offset_complition) = recever1.recv().unwrap();
        let tx_consumed_clone = tx_consumed.clone();
        threadpool.execute(move || {
            psql_relayer_state_queue(data, tx_consumed_clone, offset_complition);
        });
    }
}

/// Receives events from a Kafka queue and returns channels for processing
///
/// # Arguments
/// * `topic` - Kafka topic to consume from
/// * `group` - Consumer group ID
/// * `_partition` - Kafka partition number (currently unused)
///
/// # Returns
/// * Result containing:
///   - Receiver for event data and offset completion
///   - Sender for offset completion acknowledgements
pub fn receive_event_from_kafka_queue(
    topic: String,
    group: String,
    _partition: i32,
) -> Result<
    (
        Arc<Mutex<Receiver<(EventLogRPCQuery, OffsetCompletion)>>>,
        Sender<OffsetCompletion>,
    ),
    KafkaError,
> {
    let (sender, receiver) = unbounded();
    let (tx_consumed, rx_consumed) = unbounded::<OffsetCompletion>();
    let _topic_clone = topic.clone();
    thread::spawn(move || {
        // get the last offset from kafka
        let last_offset = get_offset_from_kafka(topic.clone(), group.clone());
        println!("last_offset: {:#?}", last_offset);

        // create an offset tracker
        let offset_tracker = Arc::new(OffsetManager::new(last_offset - 1));

        let worker_tracker = offset_tracker.clone();
        let commit_tracker = offset_tracker.clone();

        let broker = vec![KAFKA_BROKER.clone()];
        let mut con = Consumer::from_hosts(broker.clone())
            // .with_topic(topic)
            .with_group(group.clone())
            .with_topic_partitions(topic.clone(), &[0])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();
        let mut connection_status = true;
        let mut topic_partition: i32 = 0;
        while connection_status {
            let sender_clone = sender.clone();
            let mss = con.poll().unwrap();
            if mss.is_empty() {
            } else {
                for ms in mss.iter() {
                    for m in ms.messages() {
                        match serde_json::from_str(&String::from_utf8_lossy(&m.value)) {
                            Ok(value) => {
                                let message = EventLogRPCQuery {
                                    offset: m.offset,
                                    key: String::from_utf8_lossy(&m.key).to_string(),
                                    value: value,
                                };
                                match sender_clone.send((message, (ms.partition(), m.offset))) {
                                    Ok(_) => {}
                                    Err(arg) => {
                                        eprintln!("Closing Kafka Consumer Connection : {:#?}", arg);
                                        connection_status = false;
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error parsing message on kafka client: {:?}", e);
                                continue;
                            }
                        }
                    }
                    if connection_status == false {
                        break;
                    }
                }
            }

            // Process consumed offsets
            while !rx_consumed.is_empty() {
                match rx_consumed.recv() {
                    Ok((partition, offset)) => {
                        worker_tracker.mark_done(offset);
                        topic_partition = partition;
                    }
                    Err(_e) => {
                        connection_status = false;
                        eprintln!(
                            "The consumed channel is closed: {:?}",
                            thread::current().name()
                        );
                        break;
                    }
                }
            }

            // Commit offsets if available
            if let Some(offset) = commit_tracker.next_commit_offset() {
                let e = con.consume_message(&topic, topic_partition, offset);

                if e.is_err() {
                    eprintln!("Kafka connection failed {:?}", e);
                    break;
                }

                let e = con.commit_consumed();
                if e.is_err() {
                    eprintln!("Kafka connection failed {:?}", e);
                    break;
                }
            }
            if connection_status == false {
                break;
            }
        }
    });
    Ok((Arc::new(Mutex::new(receiver)), tx_consumed))
}

/// Struct representing a message from Kafka containing event log or RPC query data
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventLogRPCQuery {
    pub offset: i64,
    pub key: String,
    pub value: Value,
}

/// Inserts RPC command data into PostgreSQL
pub fn psql_rpc_command(
    data: EventLogRPCQuery,
    tx_consumed: crossbeam_channel::Sender<OffsetCompletion>,
    offset_complition: OffsetCompletion,
) {
    //creating static connection
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    let query = format!(
        "INSERT INTO public.rpc_query(\"offset\", key, payload) VALUES ({},'{}',$1);",
        data.offset, data.key
    );
    match client.execute(&query, &[&data.value]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error inserting into psql: {:?}", e);
        }
    }
    match tx_consumed.send(offset_complition) {
        Ok(_) => {}
        Err(_) => {}
    }
}

/// Inserts failed RPC command data into PostgreSQL
pub fn psql_rpc_failed_command(
    data: EventLogRPCQuery,
    tx_consumed: crossbeam_channel::Sender<OffsetCompletion>,
    offset_complition: OffsetCompletion,
) {
    //creating static connection
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    let query = format!(
        "INSERT INTO public.rpc_query_failed(\"offset\", key, payload) VALUES ({},'{}',$1);",
        data.offset, data.key
    );
    match client.execute(&query, &[&data.value]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error inserting into psql: {:?}", e);
        }
    }
    match tx_consumed.send(offset_complition) {
        Ok(_) => {}
        Err(_) => {}
    }
}

/// Inserts event log data into PostgreSQL
pub fn psql_event_logs(
    data: EventLogRPCQuery,
    tx_consumed: crossbeam_channel::Sender<OffsetCompletion>,
    offset_complition: OffsetCompletion,
) {
    //creating static connection
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    let query = format!(
        "INSERT INTO public.event_logs(\"offset\", key, payload) VALUES ({},'{}',$1);",
        data.offset, data.key
    );
    match client.execute(&query, &[&data.value]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error inserting into psql: {:?}", e);
        }
    }
    match tx_consumed.send(offset_complition) {
        Ok(_) => {}
        Err(_) => {}
    }
}

/// Inserts relayer state queue data into PostgreSQL
pub fn psql_relayer_state_queue(
    data: EventLogRPCQuery,
    tx_consumed: crossbeam_channel::Sender<OffsetCompletion>,
    offset_complition: OffsetCompletion,
) {
    //creating static connection
    let mut client = POSTGRESQL_POOL_CONNECTION.get().unwrap();

    let query = format!(
        "INSERT INTO public.relayer_state_queue(\"offset\", key, payload) VALUES ({},'{}',$1);",
        data.offset, data.key
    );
    match client.execute(&query, &[&data.value]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error inserting into psql: {:?}", e);
        }
    }
    match tx_consumed.send(offset_complition) {
        Ok(_) => {}
        Err(_) => {}
    }
}

/// Gets the last committed offset from Kafka for a topic/group
pub fn get_offset_from_kafka(topic: String, group: String) -> i64 {
    let broker = vec![std::env::var("BROKER")
        .expect("missing environment variable BROKER")
        .to_owned()];
    let mut con = Consumer::from_hosts(broker)
        // .with_topic(topic)
        .with_group(group)
        .with_topic_partitions(topic.clone(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let connection_status = true;
    while connection_status {
        let mss = con.poll().unwrap();
        if mss.is_empty() {
            // Sleep briefly when no messages available
            thread::sleep(time::Duration::from_millis(100));
        } else {
            for ms in mss.iter() {
                for m in ms.messages() {
                    return m.offset;
                }
            }
        }
    }
    return 0;
}
