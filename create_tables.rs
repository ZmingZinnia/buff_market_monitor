extern crate rusqlite;
use rusqlite::Connection;

fn main() {
    let path = "./buff_market_monitor.db3";
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE TABLE GoodsSellItems (
            id   INTEGER PRIMARY KEY,
            goods_id INTEGER NOT NULL,
            name varchar(100) NOT NULL,
            price varchar(20) NOT NULL,
            create_time datetime NOT NULL
        )",
        (), // empty list of parameters.
    ).unwrap();
}