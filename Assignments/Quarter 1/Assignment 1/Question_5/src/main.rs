/* Q # 5​. Write a Rust program by initializing an array of Cars, and another array with their prices.Print the data as below:  
Output be like: 
Car Name: Passo - Price: 800000 
Car Name: Vitz - Price: 900000 
Car Name: Swift - Price: 950000 
(Note: For the sake of simplicity, limit only 5 cars.)  */


fn main() {                                                                     // Main function
    let cars_name = ["Passo","Vitz","Swift"];                                   // Array data
    let car_price = ["800,000","900,000","950,000"];                            // Array data
    println!("Car Name : {} - Price : {}",cars_name[0],car_price[0]);           // Array data print
    println!("Car Name : {} - Price : {}",cars_name[1],car_price[1]);           // Array data print
    println!("Car Name : {} - Price : {}",cars_name[2],car_price[2]);           // Array data print
}
