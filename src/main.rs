
extern crate iron;
extern crate router;
extern crate urlencoded;

extern crate slack_indexer;

use urlencoded::UrlEncodedBody;


use iron::prelude::*;
use iron::status;
use router::Router;

use slack_indexer::bughunter;
use slack_indexer::db::{Db, IndexedMessage};

fn main() {

    let db: Db = Db::new();

    db.create_tables();

    let text = "No, I'm talking about ABCN-123, not ABCN-145";

    let bugs = bughunter::get_bugs(text.to_string());

    for cap in &bugs {
        println!("Match {}", cap);
        let message: IndexedMessage = IndexedMessage::new(cap, text);

        //conn.execute(message.save());

    }

    fn hello_world( request: &mut Request) -> IronResult<Response> {

        match request.get_ref::<UrlEncodedBody>() {
            Ok(ref hashmap) => println!("Parsed body: {:?}", hashmap),
            Err(ref e) => println!("Error: {:?}", e),
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

