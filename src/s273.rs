struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let bytes = num.to_string().bytes().rev().collect::<Vec<u8>>();
        let chunks: Vec<_> = bytes.chunks(3).rev().collect();
        let mut words = Vec::<String>::new();

        for (idx, chunk) in chunks.iter().enumerate() {
            let spell: String;
            let mut trinum: Vec<u8> = chunk.iter().map(|&b| b).collect();
            trinum.extend(std::iter::repeat(b'0').take(3 as usize - chunk.len()));
            trinum.reverse();

            spell = Self::spell(trinum);
            if spell.is_empty() {
                continue;
            }
            words.push(spell);

            let magnitude = Self::spell_magitude(chunks.len() - idx);
            if !magnitude.is_empty() {
                words.push(magnitude);
            }
        }

        words.join(" ")
    }

    fn spell_magitude(idx: usize) -> String {
        match idx {
            1 => "",
            2 => "Thousand",
            3 => "Million",
            4 => "Billion",
            5 => "Trillion",
            _ => "Too Big",
        }
        .to_string()
    }

    fn spell(bytes: Vec<u8>) -> String {
        let hundreds = Self::spell_hundreds(bytes[0]);
        let (tens, ones) = if bytes[1] == b'1' {
            (Self::spell_ten_plus(bytes[2]), "".to_string())
        } else {
            (Self::spell_tens(bytes[1]), Self::spell_ones(bytes[2]))
        };

        let mut spell = Vec::<String>::new();
        if !hundreds.is_empty() {
            spell.push(hundreds);
        }
        if !tens.is_empty() {
            spell.push(tens);
        }
        if !ones.is_empty() {
            spell.push(ones);
        }
        spell.join(" ")
    }

    fn spell_hundreds(byte: u8) -> String {
        let ones = Self::spell_ones(byte);
        if ones.is_empty() {
            "".to_string()
        } else {
            ones + " " + "Hundred"
        }
    }

    fn spell_ten_plus(byte: u8) -> String {
        match byte {
            b'0' => "Ten",
            b'1' => "Eleven",
            b'2' => "Twelve",
            b'3' => "Thirteen",
            b'4' => "Fourteen",
            b'5' => "Fifteen",
            b'6' => "Sixteen",
            b'7' => "Seventeen",
            b'8' => "Eighteen",
            b'9' => "Nineteen",
            _ => "UnknownTenPlus",
        }
        .to_string()
    }

    fn spell_tens(byte: u8) -> String {
        match byte {
            b'0' => "",
            b'2' => "Twenty",
            b'3' => "Thirty",
            b'4' => "Forty",
            b'5' => "Fifty",
            b'6' => "Sixty",
            b'7' => "Seventy",
            b'8' => "Eighty",
            b'9' => "Ninety",
            _ => "UnknownTens",
        }
        .to_string()
    }

    fn spell_ones(byte: u8) -> String {
        match byte {
            b'0' => "",
            b'1' => "One",
            b'2' => "Two",
            b'3' => "Three",
            b'4' => "Four",
            b'5' => "Five",
            b'6' => "Six",
            b'7' => "Seven",
            b'8' => "Eight",
            b'9' => "Nine",
            _ => "UnknownOnes",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
        assert_eq!(Solution::number_to_words(2013), "Two Thousand Thirteen");
        assert_eq!(Solution::number_to_words(10000010), "Ten Million Ten");
        assert_eq!(Solution::number_to_words(0), "Zero");
    }
}
