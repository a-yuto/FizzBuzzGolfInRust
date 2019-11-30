fn main()for i in 0..101{match i%15{0=>print!("fizzbuzz\n"),3|6|9|12=>print!("fizz\n"),5|10=>print!("buzz\n"),_=>print!("{}\n",i),}}}
