fn main() {
    println!("Start!");
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
    problem6();
    problem7();
    problem8();
    problem9();
    problem10();
}

//Problem 1
//Multiples of 3 or 5
fn problem1(){
    let mut my_vec: Vec<i32> = vec![];
    for i in 1..1000{
        if i%3 == 0 {
            my_vec.push(i);            
        } else if i%5 == 0 {
            my_vec.push(i);            
        }
    }

    let mut result: i32 = 0;

    for i in 0..my_vec.len(){
        result = result + my_vec[i];        
    }

    println!("Problem N1 = {}.", result);
}

//problem 2
//Even Fibonacci numbers
fn problem2(){
    let mut result = 2;
    let mut my_box = 0;
    let mut fibo1 = 1;
    let mut fibo2 = 2;
    loop {        
        my_box = fibo1 + fibo2;
        fibo1 = fibo2;
        fibo2 = my_box;    

        if fibo2%2 == 0{
            result += fibo2;
        }

        if fibo1 + fibo2 > 4000000 {
            break;
        }
    }
    println!("Problem N2 = {}.",result);
}

//Problem 3
//Largest prime factor
fn problem3(){
    
    let mut number: i64 = 600851475143;  
    let mut iter = 2;
    let mut my_vec = vec![];

    while number > 2{
        while number%iter == 0{
            number = number/iter;
            
            if my_vec.len() ==0 || iter != my_vec[my_vec.len() - 1]{
                my_vec.push(iter);
            }
        }
        iter += 1;
    }
    
    println!("Problem N3 = {}.", my_vec[my_vec.len() - 1]);
}

//Problem 4
//Largest palindrome product
fn problem4(){
    let mut palindromic = 0;
    let mut my_box = 0;
    

    for i in 100..=999{
            for j in 100..=999{
                my_box = i*j;

                if find_pol(my_box) && my_box > palindromic{
                    palindromic = my_box;
                                       
                }
            }
    }

    println!("Problem N4 = {}.", palindromic);

    fn find_pol(my_box: i32)->bool{
        let mut my_vec = vec![];
        let mut result = true;
        let mut iter = 1;

        loop {
            if my_box *10 >= i32::pow(10,iter)   {
                my_vec.push(my_box%i32::pow(10,iter)/i32::pow(10,iter-1));
                iter += 1;
            }else{
                break;
            }
        }
        

        if my_vec.len()%2 == 0 {
            for i in 0..my_vec.len()/2{
                if my_vec[i] != my_vec[my_vec.len()-(i+1)]{
                    result = false;
                    break;
                }else if i == my_vec.len()/2 -1{
                    result = true;
                }
            }
        }

        if my_vec.len()%2 != 0{
            for i in 0..my_vec.len()/2{
                if my_vec[i] != my_vec[my_vec.len()-(i+1)]{
                    result = false;
                    break;
                }else if i == my_vec.len()/2{
                    result = true;
                }
            }
        }
        return result;
    }
    
}

//Problem 5
//Smallest multiple

fn problem5(){
    let mut flag = false;
    let mut number = 20;
    println!("Please wait. There are calculations.");
    while !flag{   
              number += 1;
        for i in 2..=20{
            if number%i != 0 {                
                break;                
            }else if i == 20{
                flag = true;                
            }
        }
       
    }
    println!("Problem N5 = {}.", number);
}

//Problem 6
//Sum square difference

fn problem6(){
    //sum of the squares
    let mut number1: i32 = 0;
    //square of the sum
    let mut number2: i32 = 0;
    let mut difference: i32;

    for i in 1..=100{
        number1 = i32::pow(i, 2) + number1;
    };

    for i in 1..=100{
        number2 = i + number2;
    }
    number2 = i32::pow(number2, 2);
    
    //difference
    difference = number2 - number1;

    println!("Problem N6 = {}.", difference);
}

//Problem 7
//10001st prime

fn problem7(){
    //first number is know = 2
    let mut prime = 2;
    let mut count = 1;
    let mut iter =2;
        
    println!("Please wait. There are calculations.");
    while iter <=10001{
        for j in 2..(prime+count){
             if (prime+count)%j == 0{
                count += 1;                
                continue;
            }else if j == (prime+count - 1){                
                prime = prime + count;                
                if iter == 10001{
                    println!("Prime N 10001 = {}.", prime);
                }
                iter += 1;                                       
                count = 1;
                
            }
        } 
    }
}

//Problem 8
//Largest product in a series

fn problem8(){
    let my_str = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
    let my_chars: Vec<char> = my_str.chars().collect();    
    let mut my_numbers: Vec<u32> = vec![];

    for ch in my_chars{
        if let Some(digit) = ch.to_digit(10){
            my_numbers.push(digit.try_into().unwrap());
        }
    }
    
    let mut max: u64 = 0;
    let mut iter = 0;
    let mut sum: u64 = 1;
    // 1000 - 0 position = 999
    // 999 - 13 = 986 
    while iter <= 986{
        for i in iter..=(iter+12){
        sum = (my_numbers[i] as u64)* sum;       
        }
        if max < sum {
            max = sum;
        }
        sum = 1;
        iter += 1;
    }

    println!("Problem N8 = {}.", max);  
}

//Problem 9
//Special Pythagorean triplet

fn problem9(){
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;
    let mut iter  = 2;

    for c in 1..1000{
        for a in 1..1000{
            b = 1000 - c - a;
            if i32::pow(a,2) + i32::pow(b,2) == i32::pow(c,2){
                print!("Problem N9  a = {}, b = {}, c = {}, sum = {}.", a,b,c, a+b+c);
                return;
            }
        }
    }  
}

//Problem 10
//Summation of primes

fn problem10(){
    //first number is know = 2
    let mut prime = 2;
    let mut count = 1;    
    let mut vec = vec![2];
    let mut sum = 0;
        
    println!("Please wait. There are calculations.");
    while prime < 2000000{
        for j in 2..(prime+count){
             if (prime+count)%j == 0{
                count += 1;                
                continue;
            }else if j == (prime+count - 1){                
                prime = prime + count;      
                        
                if prime < 2000000{
                    vec.push(prime); 
                }else{
                    break;
                }                          
                count = 1;
                
            }
        } 
    }

   for i in 0..vec.len(){
    sum = vec[i] + sum;
   }

   println!("Problema N10 = {}.", sum);

}
