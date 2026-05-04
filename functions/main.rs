fn main (){
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    function_labeled_measurement(10, 'm');
    let x = five();
    println!("The value of x is: {x}");
    let y = plus_one(5);
    println!("The value of y is: {y}");


}


fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn function_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
