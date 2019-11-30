fn main(){for i in 1..101{println!("{}",match i%15{0=>"fizzbuzz",3|6|9|12=>"fizz",5|10=>"buzz",_=>{print!("{}",i);""}})}}
