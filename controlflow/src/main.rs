fn main() {
    
    // let age = 19;

    // if age>60 || age<18 {
    //     print!("you cant drive")
    // }else {
    //     print!("drive on")
    // }


    // let mut count = 0;
// 
    // 'counting_up: loop {
        // println!("count = {count}");
        // let mut remaining = 10;
// 
        // loop {
            // println!("remaining = {remaining}");
            // if remaining == 9 {
                // break;
            // }
            // if count == 2 {
                // break 'counting_up;
            // }
            // remaining -= 1;
        // }
        // count+=1;
    // }

    let arr : [u32;10] = [1,2,5,4,6,8,8,4,55,4];
    for i in 0..arr.len()  {
        print!("{:?}",arr[i]);
    }
    println!();
    for i in arr{
        print!("{i}")
    }
    println!();
    for i in 1..4  {
        print!("{i}")
    }
    println!();
    for i in (1..4).rev()  {
        print!("{i}")
    }
   Res

}
