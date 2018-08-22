use num::Integer;

#[test]
fn test_timer() {

    print_result(0, 2);
    print_result(5, 5);
    print_result(5, 2);
    print_result(13, 4);
    print_result(100, 10);
    print_result(101, 10);

}

fn print_result(a: usize, b: usize) {

    println!("{}/{} = {}, ceil = {}", a,b, a as f32 / b as f32, (a as f32 / b as f32).ceil());
    println!("{}/{}.div_floor() =  {}", a, b, a.div_floor(&b));
    println!("{}/{}.div_ceil() = {:?}", a, b, div_ceil(a, b)) ;
    println!("");
}


fn div_ceil(a:usize, b: usize) -> usize {
    println!("d, r {:?}", a.div_rem(&b));

    match a.div_rem(&b) {
        (d, r) if (r == 0) => d,
        (d, _) => d + 1,
    }

}
