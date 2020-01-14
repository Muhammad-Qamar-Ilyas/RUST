/* Q # 2. Write a rust program and define a user defined function that receives 4 arguments of different data types (integer, float, boolean, string) and print them on the console.   */

fn data_type_print (a:i32,b:f32,c: &str,d:bool){
    println!("{} is an integer\n{} is a float\n{} is a string\n{} is a boolean",a,b,c,d);
}

fn main() {
    data_type_print(-1234, 0.127381274234,"Happy Birth Day", false);
}
