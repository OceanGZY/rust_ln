/*
 * @Author: OCEAN.GZY
 * @Date: 2022-10-25 14:12:18
 * @LastEditors: OCEAN.GZY
 * @LastEditTime: 2022-10-25 14:17:46
 * @FilePath: /hello-rust/src/main.rs
 * @Description: 注释信息
 */
use ferris_says::say;
use std::io::{stdout,BufWriter};
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
