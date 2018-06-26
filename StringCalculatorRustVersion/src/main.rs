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
  //for line in string.split(','){
    //println!("{}", line);
  //}
  //let mut numbers = string.parse::<i32>().unwrap();
  //  println!("{}",numbers );
  //  return numbers;
  //let vec =split.colletc::<Vec<&str>>();
  let v = string.split(',').collect::<Vec<parse::<i32>().unwrap()>>;

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
}