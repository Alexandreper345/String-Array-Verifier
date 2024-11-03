fn	is_a_palindrome(value:&str) -> bool
{
	let first_letter_value =value.to_lowercase();
	let reverse = first_letter_value.chars().rev().collect::<String>();
	return first_letter_value == reverse;
}

fn	contains_target_at(value: [i8; 5], target: i8) -> bool
{
	for pegar_valor in value.iter()
	{
		if *pegar_valor == target
		{
			return true;
		}
	}
	return false
}

fn	contains_pars_at(value: [i8; 5], target: i8) -> bool
{
	let mut contador:i8 = 0;
	for i in 0..value.len()
	{  
		for j in i+1..value.len()
		{
			if value[i] == target && value[j] == target
			{
				contador += 1;
				if contador >= 1
				{
					return false
				}
			}
		}
	}
	return true
}

#[cfg(test)]
mod tests
{
	use crate::*;
	#[test]
	fn test_in_palindrome()
	{
			assert!(is_a_palindrome("alala"));
	}

	#[test]
	fn test_contains_target_at()
	{
		let array = [1,2,2,4,5];
		let target = 2;
		assert!(contains_target_at(array,target))
	}

	#[test]
	fn test_contains_pars_at()
	{
		let list = [1,2,2,4,5];
		let target = 2;
		assert!(contains_pars_at(list,target))
	}
}
