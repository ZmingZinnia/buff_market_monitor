mod sell_order;


fn main() {
    let goods_infos = [("769438", "2186185"), ("42486", "446798"), ("43091", "446992")];
    for (goods_id, tag_id) in goods_infos {
        sell_order::get_and_save_sell_orders(goods_id, tag_id);
    }
}