extern crate postgres;
extern crate rustc_serialize;

use self::postgres::{Connection, SslMode};
//use self::rustc_serialize::json;

//#[derive(RustcDecodable)]
#[allow(dead_code)]
pub struct IndexedMessage {
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

impl IndexedMessage {
    pub fn new(ms: &str, text: &str) -> IndexedMessage {
        return self::IndexedMessage {
            matchedstring: ms.to_string(),
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
    }
    /*
    pub fn save(&self) -> () {
        return ("INSERT INTO indexedmessage (
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
                &self.matchedstring,
                &self.teamid, &self.teamdomain,
                &self.channelid, &self.channelname,
                &self.timestamp,
                &self.userid, &self.username,
                &self.text,
                &self.triggerword
            ]
        );
    }
    */
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Db {

        return self::Db {
            conn: Connection::connect("postgresql://postgres@localhost", &SslMode::None)
        .unwrap(),
        }
    }
    pub fn create_tables(&self) {
        &self.conn.execute("CREATE TABLE IF NOT EXISTS indexedmessage (
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
    }
}


