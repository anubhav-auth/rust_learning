// primitive data types
// int; float, bool, char

// Integer
// rust has signer(+ and -) and unsigned inteer(only+) types of diff sizes
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: UnSigned integers

fn main(){
    //arrays
    // let numbers: [i32; 5] = [1,2,3,4,5];
    // println!("Number Array {:?}", numbers);

    // // let mix = [1,2,"adbejd", true]; not allowed
    // let fruits:[&str; 4] = ["apple", "orange", "melon", "grapes"];

    // println!("all fruits {:?}", fruits);
    // println!("first fruit {}", fruits[4]);


    // tuples
    // let  human1: (String, i32, bool) = ("Anubhav".to_string(), 22, true);
    // let human2: (&str, i32, bool) = ("Anubhav", 22, true);
    // println!("Human tuple: {:?}", human1);

    // mix tuple

    // let mix_tup:(&str, i32, bool, [&str; 4], [i32; 3], [bool; 2], f64, f64)   = ("Anubhav", 22 ,true, ["heyy", "how", "are","you"], [2,4,5,], [true, false], 0.25, 54.2);
    // println!("Mix tuple: {:?}", mix_tup);

    // let number_slices: &[i32] = &[1,2,3,4,5];
    // println!("number slices: {:?}", number_slices)

    // let mut str: String = "Hell".to_string();
    // str.push_str("yeah!!");

    // let mut str: String = String::from("hell, ");
    // str.push_str("yeah!!");
    
    // let string: String = String::from("Hello world!!");

    // let slice: &str = &string[0..4];
    // print(slice);
    print!("{}",X);
    let h = add(2, 4);
}

const X: i32 = {
    let price = 43;
    let quant = 56;
    price*quant
};

fn add(a:i32, b:i32) -> i32{
    return a+b;
}
