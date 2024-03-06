use std::io;
use std::time::Duration;
use tracing::info;

use rdkafka::{
    client::Client,
    producer::{self, FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};

use rdkafka::message::{Header, OwnedHeaders};

fn kafka_producer() -> FutureProducer {
    let prod = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "clean-urchin-14880-eu2-kafka.upstash.io:9092",
        )
        .set("sasl.mechanism", "SCRAM-SHA-256")
        // .set("security.protocol", "SASL_SSL")
        .set(
            "sasl.username",
            "Y2xlYW4tdXJjaGluLTE0ODgwJOR5pLMKQJuqzz3h6s9lGZpaeUr40jGeAD7y4CQ",
        )
        .set(
            "sasl.password",
            "MTczNTA1YTMtZmU4YS00YmMyLWE2NTktN2FiNTg4OWI3ZmYy",
        )
        .create()
        .expect("PRODUCER CREATION ERROR ❌ℹ️😭");

    prod
}

async fn produce_topic(producer: FutureProducer, req_payload: String) -> String {
    println!("EXECUTING THE PRODUCER");
    let data: FutureRecord<String, String> = FutureRecord::to("DEMO")
        .payload(&req_payload)
        // .key(&format!("12345"))
        // .timestamp(234567) // random number
    ;

    let ans = producer
        .send(data, None)
        .await
        .expect("Unable to send req payload to kafka queue !");

    println!("RESPONSE:");

    "sent".to_string()
}

#[tokio::main]
async fn main() {
    let producer = kafka_producer();

    let i = 9997;
    
    let delivery_status = producer
                .send(
                    FutureRecord::to("DEMO")
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    Duration::from_secs(0),
                )
                .await;

    // loop {
    // let mut input = String::new();
    // println!("Enter messege to added to kafka queue: ");
    // io::stdin().read_line(&mut input).expect("Enable to read input message");
    // input = input.trim().to_string();
    // if input != "." {
    //     tokio::join!(produce_topic(producer.clone(), input));
    //     println!("Ok processing !!");
    // }
    // else {
    //     break;
    // }
    // }

    // tokio::join!(produce_topic(producer.clone(), String::from("THIS IS TEST MESSAGE SENT FROM THE PRODUCER.RS !!") ));
    // println!("GOING FOR IT !!");
    // let res = produce_topic(
    //     producer.clone(),
    //     String::from("THIS IS TEST MESSAGE SENT FROM THE PRODUCER.RS !!")
    // ).await;
    // println!("Message sent to broker !! {}", res);
}
