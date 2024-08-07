impl Solution {
    fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let below_20 = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
                        "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen",
                        "Seventeen", "Eighteen", "Nineteen"];
        let tens = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
        let thousands = ["", "Thousand", "Million", "Billion"];

        let mut num = num;
        let mut result = String::new();
        let mut i = 0;

        while num > 0 {
            if num % 1000 != 0 {
                let mut temp = String::new();
                Self::helper(num % 1000, &below_20, &tens, &mut temp);
                if i > 0 {
                    temp.push_str(" ");
                    temp.push_str(thousands[i]);
                }
                if !result.is_empty() {
                    temp.push_str(" ");
                }
                result = temp + &result;
            }
            num /= 1000;
            i += 1;
        }
    result
}

    fn helper(num: i32, below_20: &[&str], tens: &[&str], result: &mut String) {
        if num == 0 {
            return;
        } else if num < 20 {
            result.push_str(below_20[num as usize]);
        } else if num < 100 {
            result.push_str(tens[(num / 10) as usize]);
            if num % 10 > 0 {
                result.push_str(" ");
                result.push_str(below_20[(num % 10) as usize]);
            }
        } else {
            result.push_str(below_20[(num / 100) as usize]);
            result.push_str(" Hundred");
            if num % 100 > 0 {
                result.push_str(" ");
                Self::helper(num % 100, below_20, tens, result);
            }
        }
    }
}
