fn main() {
   // let _message ="hello, world!";

   // let x:i32=42;
   // let pi:f64=3.14;
    //let is_rust_fun:bool=true;
    //let letter_a:char='a';
    //fn add (x:i32,y:i32)->i32{
      //  x+y
   // }
    //let x=42;
   /* if x>=0{
        println!(" x is not negative");

    }
    else{
        println!("x is negative");
    }
    let mut i=1;
    while i<=5{
        println!("{}",i);
        i+=1;

    
    }
    let _my_first_bool=true;
    let _my_second_bool:bool=false;
    //8,16,32,64,128
    let _days_of_week:i8=7;
    let _number_of_users:i64=128000;
    let _number_of_tokens:u64=10000;
    let _just_a_number=0;
    //Floating Point Number
    //32,64
    let _pi:f32=3.14;
    //Characters
    let _my_char:char='1';
    //Strings
    let _message="hi,ali";
    let _my_string=String::from("hi, ali");
   let _days_of_the_week: [&str;7]=[
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday"
   ];

   let _first_element=_days_of_the_week[0];

let _last_element=_days_of_the_week[_days_of_the_week.len()-1];


//Slices
let slice=&_days_of_the_week[1..3];
let _first_element_of_slice=slice[0];

//Tuples
let person=("Alice",30);
let _name=person.0;
let _age=person.1;

//Unit type
let _unit_type=();

//Variables
let mut num =5;
num=6;*/

/*let sum =add(3 , 5);

println!("The sum is {} ",sum);

let day_of_the_week="Sunday";

if day_of_the_week=="Sunday"{
    println!("The race day!");
}
else if  day_of_the_week=="Sarurday"{
  println!("Qualifying today!");

}
else{
    println!("Patient wait for the race day");
}

//While
let mut counter = 0;
while counter < 5{
    println!("Counter value is {}",counter);
    counter += 1;
}


//for loop
let numbers:[i32;5]=[1,2,3,4,5];
for number in numbers
{
    println!("number is {}" ,number);

}

//loop
counter=0;
loop{
    println!("Counter value is {}",counter);
    counter+=1;
    if counter==6{
        break;
    }
}

let num =5;
match num{
    1=>{
        println!("The number is one");
        println!("This is the first match arm");

    }
    2=>println!("The number is two"),
    3=>println!("The number is three"),
    _=> println!("The number is something else"),

}

let result=match num{
    1=>" The number is one",
    2=>" The number is two",
    3=>" The number is three",
    _=> "The number is something else",

};
println!("The result is{}",result);
 
}
fn add(x: i32, y: i32)->i32{
    let result =x+y;
    return result;

 }



 fn no_param()->i32
 {
    println!("This just works");
    1
 }*/


/*


 //  ***Ownership***
 let s1=String::from("hello");

 let s2= s1.clone();

 //println!("value of s1 is {}",s1);
 let x: i32=5;
 let y=String::from("patika");
 let z=y;
 println!("value of x is {}, and value of z is {}" ,x,z);
 
//  ***Borrowing and References***
let my_string=String::from("hello world!");
let my_ref=&my_string;
//println!("my reference is {} ",my_ref);
let _my_string=String::from("hello world!");
//print_string(&my_string);
//println!("I still got my string {}",my_string);
let mut my_string=String::from("hello");
change_string(&mut my_string);
//println!("{}",my_string);

let first_immutable_reference=&my_string;
let second_immutable_reference=&my_string;

//println!(
  //  "first immutable reference value {},second immutablereference value {}",
  //  first_immutable_reference,second_immutable_reference
//);
 let first_mutable_reference=&mut my_string;
 println!("first mutable reference value{}",first_mutable_reference);
//println!( "I mmutable reference value {}", first_immutable_reference);
 let second_mutable_reference=&mut my_string;
 println!("{}",second_mutable_reference);
let new_string=String::from("new string");
let new_string_ref=return_reference(&new_string);

//println!("new string {}" , new_string);

let newer_string= new_string;
println!("new string reference {}", new_string_ref);


}

fn print_string(s:&String){
    println!("{}",s);
}

fn change_string(s:&mut String) {
    s.push_str("world");
}


fn return_reference(some_string: &String) -> &String{
    some_string
}

*/

//  ***struct****
let book =Book{
    title:String::from ("The way of Zen"),
    author:String::from("Allan Watts"),
    publication_year:1957,

};

println!(
    "The book {} is written by {} in {}",
 book.title,book.author,book.publication_year);

let mut book=Book{
    title:String::from ("The way of Zen"),
    author:String::from("Allan Watts"),
    publication_year:1957,
};
book.publication_year=1089;

println!("The book {} is written by {} in {}",
 book.title,book.author,book.publication_year);

let book_data=get_book_data(book);
for data in book_data
{
    println!("{data}");
}


let my_book = create_book("The path of zen".to_string(),"simon".to_string(),2023);
println!("my book is {:?}",my_book);

let tuple_book=Tuple_Book("Some book".to_string(),"simon".to_string(),2023);

let title=tuple_book.0;
let author=tuple_book.1;
let publication_year=tuple_book.2;


let unit_book=Unit_Book;


let my_rectangle=Rectangle{
    width:10.0,
    height:5.0,
};
let area=my_rectangle.area();

println!("The area of the reactangle is : {}",area);





} 

#[derive(Debug)]

struct Book {
    title:String,
    author:String,
    publication_year:u32,

}

struct Tuple_Book(String,String,u32);

struct Unit_Book;

 fn get_book_data(book:Book)->[String;3]{
    let title=book.title;
    let author=book.author;
    let publication_year=book.publication_year;

    let data: [String; 3]=[ title,author,publication_year.to_string()];
    data
 }

fn create_book( title:String,author:String,publication_year:u32)->Book{
    let book=Book{
        title,
        author,
        publication_year,
    };
    book
}

struct Rectangle{
    width:f64,
    height:f64,
}

impl Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}







