pub fn add(string: &str) -> i32 {
    if verify_the_string_is_empty(string) {
    	return 0;
    }
    else {
    	return do_the_string_calculator(string);
    }
}

pub fn verify_the_string_is_empty(string: &str) -> bool{
	if string==""{
		return true;
	} 
	return false;	
}

pub fn do_the_string_calculator(string: &str) -> i32 {
	let replace = string.replace("\n",",");
	let split = replace.split(',');
	let mut sum = 0;
	for number_in_string in split {
		let number = number_in_string.parse::<i32>().unwrap();
		sum = sum + number;
	}
	return sum
}




#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn using_empty_string() {
    	let string = "".to_string();
        assert_eq!(0, add(&string));
    }

    #[test]
    fn one_number() {
    	let string = "7".to_string();
        assert_eq!(7, add(&string));
        let string = "4".to_string();
        assert_eq!(4, add(&string));
        let string = "9".to_string();
        assert_eq!(9, add(&string));
    }

    #[test]
    fn two_numbers() {
    	let string = "1,2".to_string();
        assert_eq!(3, add(&string));
        let string = "3,4".to_string();
        assert_eq!(7, add(&string));
        let string = "7,8".to_string();
        assert_eq!(15, add(&string));
    }
    
    #[test]
    fn any_numbers() {
    	let string = "1,2,3".to_string();
        assert_eq!(6, add(&string));
        let string = "3,4,5,6,7,8,9".to_string();
        assert_eq!(42, add(&string));
        let string = "1,1,1,1,1,1,1,1,1,1,1,1,1,1".to_string();
        assert_eq!(14, add(&string));
    }
    
    #[test]
    fn any_numbers_with_break_lines() {
    	let string = "1,2\n3".to_string();
        assert_eq!(6, add(&string));
        let string = "3,4\n5,6,7\n8,9".to_string();
        assert_eq!(42, add(&string));
        let string = "1\n1\n1\n1\n1,1,1,1,1,1\n1,1,1,1".to_string();
        assert_eq!(14, add(&string));
    }
}
