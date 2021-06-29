
//  use std::env;
//  use ::teststructopt ;
//mod test;
mod my_mods;
mod test;

fn main() {
    let opt = test::getstructopt();
    println!("test {:#?}", opt);
    let opts = my_mods::opts_mod::getstructopt();
    println!("mymods {:#?}", opts);
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);

    /* let x = 5;

    let y = {
        let x = 3;
        x + 1
    }; */

    // println!("x 的值为 : {}", x);
    // println!("y 的值为 : {}", y)
    // fn five() -> i32 {
    //     5
    // }
    // println!("five() 的值为: {}", five());
    // println!("{:?}",format!("{:?}", (3, 4)));
}
