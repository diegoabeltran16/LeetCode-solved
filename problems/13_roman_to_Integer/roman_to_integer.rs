impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut res = 0;

        fn val(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        for i in 0..s_chars.len() {
            let current = val(s_chars[i]);
            let next = if i + 1 < s_chars.len() { val(s_chars[i + 1]) } else { 0 };

            if current < next {
                res -= current;
            } else {
                res += current;
            }
        }

        res
    }
}
