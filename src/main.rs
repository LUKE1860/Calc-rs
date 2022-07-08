use std::io;
use ansi_term::Color;
fn first(){
let mut input1=String::new();
let mut input2=String::new();
println!("Enter a first number");
io::stdin().read_line(&mut input1).expect("Wrong format");
let x1:f64=input1.trim().parse().expect("Wrong format");
println!("Enter a second number");
io::stdin().read_line(&mut input2).expect("Wrong format");
let x2:f64=input2.trim().parse().expect("Wrong format");
let add=x1+x2;
println!("Equals {}",add);
}
fn second(){
    let mut input1=String::new();
    let mut input2=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Enter a second number");
    io::stdin().read_line(&mut input2).expect("Wrong format");
    let x2:f64=input2.trim().parse().expect("Wrong format");
    let add=x1-x2;
    println!("Equals {}",add);
    }
fn third(){
    let mut input1=String::new();
    let mut input2=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Enter a second number");
    io::stdin().read_line(&mut input2).expect("Wrong format");
    let x2:f64=input2.trim().parse().expect("Wrong format");
    let add=x1*x2;
    println!("Equals {}",add);
    }
fn fourth(){
    let mut input1=String::new();
    let mut input2=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Enter a second number");
    io::stdin().read_line(&mut input2).expect("Wrong format");
    let x2:f64=input2.trim().parse().expect("Wrong format");
    let add=x1/x2;
    println!("Equals {}",add);
    }
fn sine(){
    let mut input1=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Sine of {}equals {}",input1,x1.sin())
}
fn cosine(){
    let mut input1=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Sine of {}equals {}",input1,x1.cos())
}
fn tangent(){
    let mut input1=String::new();
    println!("Enter a first number");
    io::stdin().read_line(&mut input1).expect("Wrong format");
    let x1:f64=input1.trim().parse().expect("Wrong format");
    println!("Sine of {}equals {}",input1,x1.tan())
}
fn main() {
let mut a=String::new();
println!("Enter a format:+,-,*,/,s,c,t");
io::stdin().read_line(&mut a).expect("Wrong format");
if a.contains("+"){
first();    
}
else if a.contains("-"){
second();
}
else if a.contains("*"){
third();
}
else if a.contains("/"){
fourth();
}
else if a.contains("s"){
sine();    
}
else if a.contains("c"){
cosine();
}
else if a.contains("t"){
tangent();
}
else {
println!("Try again")
}
}