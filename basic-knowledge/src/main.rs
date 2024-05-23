/*
 * @Author: OCEAN.GZY
 * @Date: 2024-05-23 09:41:05
 * @LastEditors: OCEAN.GZY
 * @LastEditTime: 2024-05-23 11:16:07
 * @FilePath: /rust_ln/basic-knowledge/src/main.rs
 * @Description: 注释信息
 */

mod variables_ln;
use variables_ln::basic::learn;

mod basictype;
use basictype::basic::ptbasictype;

fn main() {
    println!("Hello, world!");
    learn();
    ptbasictype();
}
