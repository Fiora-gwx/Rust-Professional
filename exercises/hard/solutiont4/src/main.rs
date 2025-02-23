
//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    let time = "2025-12-31";
    let result = calc_time::time_info(time);
    println!("{}", result); //1,3,365,0,48,1
}