use std::io;


fn main() {
    loop {
    println!("Hello. Welcome to project3!");
    println!("Which Application do you wanna use?");
    println!("We have a list of options:");
    let a = [
        "1.  Fahrenheit and Celsius Converter",
        "2.  nth Fibonacci number generator!"
     ,  "3.  lyrics to the Christmas carol “The Twelve Days of Christmas,”"
];
    let mut selection = 0;

    let mut input = String::new();
    

    while selection < 3 {
        println!("{}", a[selection]);
        selection += 1
    }

    {
        println!("your input: 1/2/3 \n or 0 to quit ");
        
    }
    

    io::stdin()
      .read_line(&mut input)
      .expect("error couldn't compile your message. pls try again.");
      let input: i8 = input.trim().parse().expect("error");


      if input == 1 {
        println!("place the number of the Celsius/Fahrenheit temprature:");

        let mut temprature = String::new();
        let mut typet = String::new();
       

        io::stdin()
       .read_line(&mut temprature)
       .expect("error couldn't compile your message. pls try again.");

       println!("specify which temprature measurement: Celsius/Fahrenheit");

       io::stdin()
       .read_line(&mut typet)
       .expect("error couldn't compile your message. pls try again.");

       // converting temprature
       let temprature: f64 = temprature.trim().parse().expect("error");
       let typet: String = typet.trim().parse().expect("error");
       

        facc(temprature , typet);
      } else if input == 2 {
        let mut repeat = String::new();
        let mut x= String::new();
        let mut y = String::new();
        println!("pls input how many times you want to repeat Fibonacci");
        io::stdin()
       .read_line(&mut repeat)
       .expect("error couldn't compile your message. pls try again.");
        println!("pls input your first number.");
        io::stdin()
       .read_line(&mut x)
       .expect("error couldn't compile your message. pls try again.");
       println!("pls input your second number.");
        io::stdin()
       .read_line(&mut y)
       .expect("error couldn't compile your message. pls try again.");
        let repeat: i8 = repeat.trim().parse().expect("error");
        let x: f64 = x.trim().parse().expect("error");
        let y: f64 = y.trim().parse().expect("error");
        

        fibonacci_generator(repeat, x, y);

      } else if input == 3 {
        tdoc();
      } else if input == 0 {
        break;

      } else {
        println!("input a correct value!");
        break;
        
      }
    }
}

fn facc(mut temprature: f64, typet: String) {

    if typet == "Celsius" {
        println!("current temprature: {}'C", temprature);
        temprature = temprature * 1.8 + 32.0;
        println!("your converted temprature to Fahrenheit is: {}'F", temprature);
    } else if typet == "Fahrenheit" {
        println!("current temprature: {}'F", temprature);
        temprature = temprature - 32.0 / 1.8;
        println!("your converted temprature to Celsius is: {}'C", temprature)
    } else {
        println!("Invalid input! check your spelling or input.");
        
    }

}

fn fibonacci_generator(mut limit: i8,mut x: f64, mut y: f64 ) {
    
    while limit != 0 {
        let next: f64 = x + y;
        x = y;
        println!("{} + {}",x,y);
        y = next;
        println!("{} + {}",x,y);
        println!("x = {} \n y = {}", x,y);
        limit -= 1;
        
    }

}

fn tdoc() {
    
    let s = ["On the first day of Christmas My true love gave to me A partridge in a pear tree","On the second day of Christmas My true love gave to me Two turtle doves And a partridge in a pear tree","On the third day of Christmas My true love gave to me Three French hens Two turtle doves And a partridge in a pear tree","On the fourth day of Christmas My true love gave to me Four calling birds Three French hens Two turtle doves And a partridge in a pear tree","On the fifth day of Christmas My true love gave to me Five golden rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree","On the sixth day of Christmas My true love gave to me Six geese a-laying Five golden rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree","On the seventh day of Christmas My true love gave to me Seven swans a-swimming Six geese a-laying Five golden rings Four calling birds Three French hens Two turtle doves And a partridge in a pear tree"];
    for lyric in s {
        println!("{lyric}");

    }
}