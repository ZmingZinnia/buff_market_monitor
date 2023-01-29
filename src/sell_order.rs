use chrono::{Local};
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use rusqlite::{params, Connection};
const BUFF_MARKET_URL: &'static str = "https://buff.163.com/api/market/goods/sell_order";

#[derive(Serialize, Deserialize)]
struct GoodsInfo {
    goods_id: i64,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct SellItem {
    price: String,
}

#[derive(Serialize, Deserialize)]
struct SellOrderPage {
    goods_info: GoodsInfo,
    items: [SellItem; 1]
}

fn get_query_string(goods_id: &str, tag_id: &str) -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis().to_string();

    let mut param_map = HashMap::new();
    param_map.insert("game", "csgo");
    param_map.insert("sort_by", "default");
    param_map.insert("allow_tradable_cooldown", "1");
    param_map.insert("_", &in_ms);
    param_map.insert("tag_ids", &tag_id);
    param_map.insert("page_num", "1");
    param_map.insert("goods_id", goods_id);
    let mut param_str_list: Vec<String> = vec![];
    for (k, v) in param_map {
        param_str_list.push([k, v].join("="));
    }
    return param_str_list.join("&")
}

fn get_query_link(goods_id: &str, tag_id: &str) -> String {
    let query_string = get_query_string(goods_id, tag_id);
    let link_items = [BUFF_MARKET_URL, &query_string];
    let url = link_items.join("?");
    return url
}

pub fn get_and_save_sell_orders(goods_id: &str, tag_id: &str) {
    let url = get_query_link(goods_id, tag_id);
    let body= &*reqwest::blocking::get(url).unwrap().text().unwrap();
    //println!("{}", body);
    let resp: Value = serde_json::from_str(body).unwrap();
    println!("{}", resp);
    println!("{}", resp["data"]);
    //let sell_order_page =  trans_to_sell_order_dto(goods_id, resp);
    //save_data(sell_order_page)
    // println!("{}", sell_order_page.goods_info.name);
}

//fn trans_to_sell_order_dto(goods_id: &str, resp: Value) -> SellOrderPage {
//    let sell_item = SellItem { price: resp["data"]["items"][0]["price"].to_string()};
//    let goods_info = GoodsInfo { goods_id: goods_id.parse::<i64>().unwrap(), name: resp["data"]["goods_infos"].get(goods_id).unwrap()["name"].to_string()};
//    return SellOrderPage {goods_info: goods_info, items: [sell_item]};
//}
//
//fn save_data(sell_order_page: SellOrderPage) {
//    let path = "./buff_market_monitor.db3";
//    let conn = Connection::open(path).unwrap();
//    conn.execute(
//        "INSERT INTO buff_sell_info (goods_id, name, price, create_time) VALUES (?, ?, ?, ?)",
//        params![&sell_order_page.goods_info.goods_id, &sell_order_page.goods_info.name, &sell_order_page.items[0].price, Local::now().to_string()],
//    ).unwrap();
//}