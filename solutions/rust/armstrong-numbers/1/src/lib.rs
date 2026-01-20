pub fn is_armstrong_number(num: u32) -> bool {
    let digits = count_digits(num);
    
    let mut total_sum = 0;
    let mut num_cp = num;
    while num_cp != 0{

        let  digit = num_cp%10;
        total_sum += digit.pow(digits);
        num_cp /= 10;
        
    }
    total_sum == num
}

fn count_digits(mut num:u32)->u32{
    let mut count = 0;

    while num != 0{
        count+=1;
        num /= 10;
    }

    count
}