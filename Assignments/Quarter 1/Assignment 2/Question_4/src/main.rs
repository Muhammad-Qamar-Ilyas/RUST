/* Q # 4. Write a program to know the result of the test (Pass/Fail) by using a user defined function, and perform the following operations: 
(Note: Consider marks of only 2 subjects for the sake of simplicity. Maximum marks are 100 for each subject.) 

a. Add marks and print the total. 
b. Calculate the percentage and print it, return percentage to main function 
c. If percentage >= 70, Print Pass, Else, print Fail.  */

fn result(sub_1 : u32, sub_2 : u32) -> (f32){
    let obt_marks = sub_1 + sub_2;
    let total_marks = 200;
    let per : f32 = (obt_marks*100/total_marks) as f32;
    println!("Subject 1 marks are {}\nSubject 2 marks are {}",sub_1,sub_2);
    println!("Obtained marks are {} out of 200 & the percentage is {} %",obt_marks,per);
    return (per)
}

fn main() {
    let sub1 = 79;
    let sub2 = 50;
    let per = result(sub1,sub2);
    if per >= 70.0{
        println!("Pass");
    }
    else{
        println!("Fail");
    }
}
