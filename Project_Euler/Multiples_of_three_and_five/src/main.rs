pub fn add(number: i32 ) -> i32 {
    //Natural numbers, multiples of 3 or 5
    //for (int i = 0; i < 10; i++){
      if number % 3 == 0  || number % 5 == 0{
        return 0;
      }
      return 1;
    //}
}


#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn testing_multiples_of_three() {
    	let number = 3;
      assert_eq!(0, add(number));
    }

    #[test]
    fn testing_multiples_of_five() {
      let number = 5;
      assert_eq!(0, add(number));
    }

    #[test]
    fn testing_the_second_multiples_of_five() {
      let number = 10;
      assert_eq!(0, add(number));
    }

}