use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use serde::{Serialize, Deserialize};
const BUFF_MARKET_URL: &'static str = "https://buff.163.com/api/market/goods/sell_order";

#[derive(Debug, Serialize, Deserialize)]
struct GoodsInfo {
    goods_id: i64,
    icon_url: String,
    name: String,
    steam_price_cny: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SellItem {
    price: String,
    img_src: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SellOrder {
    goods_info: GoodsInfo,
    items: [SellItem; 3]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOrderResp {
    code:  String,
    data: SellOrder,
    msg: String
}

fn get_query_string(goods_id: &str) -> String {
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
    param_map.insert("page_num", "1");
    param_map.insert("goods_id", goods_id);
    let mut param_str_list: Vec<String> = vec![];
    for (k, v) in param_map {
        param_str_list.push([k, v].join("="));
    }
    return param_str_list.join("&")
}

fn get_query_link(goods_id: &str) -> String {
    let query_string = get_query_string(goods_id);
    let link_items = [BUFF_MARKET_URL, &query_string];
    let url = link_items.join("?");
    return url
}

pub fn get_sell_orders(goods_id: &str) {
    let url = get_query_link(goods_id);
    let body= reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("{}", body);
    //let resp: SellOrderResp = serde_json::from_str(&body);
    //return resp;
}