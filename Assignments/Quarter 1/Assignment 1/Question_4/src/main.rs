/* Q # 4. Write a Rust program and declare a tuple for data “Fruit_Name, Weight and Price”. Destructure the tuple in different variables and print them on your screen/terminal.  */

fn main() {                                                             // Main function
    let tuple_data = ("Mango","10 Kg",800);                             // Tuple initialization with values
    let fruit_name = tuple_data.0;                                      // Accessing tuple value at 0 position an assigning to a variable
    let weight = tuple_data.1;                                          // Accessing tuple value at 1 position an assigning to a variable
    let price = tuple_data.2;                                           // Accessing tuple value at 2 position an assigning to a variable
    println!("The fruit name is {}.\nIts weight is {} & its price is {} per {}.",fruit_name,weight,price,weight);      // Printing values of variable
}
