/*
 * @Author: OCEAN.GZY
 * @Date: 2024-05-23 09:42:41
 * @LastEditors: OCEAN.GZY
 * @LastEditTime: 2024-05-23 11:12:20
 * @FilePath: /rust_ln/basic-knowledge/src/variables_ln/basic.rs
 * @Description: 注释信息
 */

struct Struct {
    g: i32,
}

pub fn learn() {
    // let x = 5; // 不可修改
    let _x = 5; // 忽略未使用的变量
    let mut x = 5; // 可修改

    println!("the value of x is :{}", x);

    x = 6;
    println!("the new  value of x is :{}", x);

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    b = true;
    assert_eq!(a, b);

    // 解构式赋值
    let (c, d, e, f, g);
    (c, d) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [e, .., f, _] = [1, 2, 3, 4, 5];
    Struct { g, .. } = Struct { g: 5 };
    assert_eq!([1, 2, 1, 4, 5], [c, d, e, f, g]);
}
