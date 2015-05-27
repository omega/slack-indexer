
extern crate iron;
extern crate router;
extern crate regex;
extern crate postgres;
use postgres::{Connection, SslMode};

struct IndexedMessage {
    matchedstring: String,
    teamid: String,
    teamdomain: String,
    channelid: String,
    channelname: String,
    timestamp: f64,
    userid: String,
    username: String,
    text: String,
    triggerword: Option<String>
}


use iron::prelude::*;
use iron::status;
use router::Router;
use regex::Regex;
use std::io::Read;
use std::collections::HashMap;


fn main() {
    let conn = Connection::connect("postgresql://postgres@localhost", &SslMode::None)
            .unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS indexedmessage (
            matchedstring varchar(32) not null,
            teamid VARCHAR(64) not null,
            teamdomain varchar(255) not null,
            channelid varchar(64) not null,
            channelname varchar(64) not null,
            timestamp float(53) not null,
            userid varchar(64) not null,
            username varchar(64) not null,
            text text not null,
            triggerword varchar(64)
        )", &[]).unwrap();


    let bug_hunter = Regex::new(r"\b([A-Z]+-\d+)\b").unwrap();

    let text = "No, I'm talking about ABCN-123, not ABCN-145";


    println!("Text {}", text);

    for cap in bug_hunter.captures_iter(text) {
        println!("Match {}", cap.at(1).unwrap_or(""));
        let message = IndexedMessage {
            matchedstring: cap.at(1).unwrap().to_string(),
            teamid: "T0001".to_string(),
            teamdomain: "thailanddevs".to_string(),
            channelid: "C2147483705".to_string(),
            channelname: "jira-testing".to_string(),
            timestamp: 1355517523.000005,
            userid: "U2147483697".to_string(),
            username: "omega".to_string(),
            text: text.to_string(),
            triggerword: None
        };

        conn.execute("INSERT INTO indexedmessage (
                matchedstring,
                teamid, teamdomain,
                channelid, channelname,
                timestamp,
                userid, username,
                text,
                triggerword
            ) VALUES(
                $1,
                $2, $3,
                $4, $5,
                $6,
                $7, $8,
                $9,
                $10
            )",
            &[
                &message.matchedstring,
                &message.teamid, &message.teamdomain,
                &message.channelid, &message.channelname,
                &message.timestamp,
                &message.userid, &message.username,
                &message.text,
                &message.triggerword
            ]
        ).unwrap();
    }

    fn hello_world( request: &mut Request) -> IronResult<Response> {

        // Getting POST body
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();

        // Spliting it with `&`
        let params: Vec<&str> = payload.split('&').collect();

        // Assiging each param with HASHMAP
        let mut data = HashMap::new();
        for pair in params.iter() {
            let key_pair: Vec<&str> = pair.split('=').collect();
            data.insert( key_pair[0], key_pair[1] );
            println!("{:?} -> {}", key_pair[0], key_pair[1]);
        }

        Ok(Response::with((status::Ok, "OK" )))

    }

    fn apejens(_: &mut Request) -> IronResult<Response> {
        println!("Got a apejens request");
        Ok(Response::with((status::Ok, "Hello, World!")))
    }

    let mut router = Router::new();
    router.post("/", hello_world);
    router.get("/apejens", apejens);
    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
