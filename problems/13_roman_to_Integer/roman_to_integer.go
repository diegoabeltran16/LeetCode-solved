package main

import "fmt"

// romanToInt converts a Roman numeral string to its integer representation.
//
// It iterates through the string, adding values for each numeral,
// and handles subtractive combinations (e.g., IV=4, IX=9, etc.) by
// using the helper function lookAhead.
//
// Example:
//   fmt.Println(romanToInt("MCMXCIV")) // Output: 1994
//
func romanToInt(s string) int {
    count := 0
    l := len(s)

    // Iterate through each character in the string
    for i := 0; i < l; i++ {
        c := s[i]

        switch c {
        case 'I':
            add, ok := lookAhead(s, c, i, l)
            if !ok {
                count += 1
            } else {
                count += add
                i++ // Skip next character as it was part of subtractive combination
            }
        case 'V':
            count += 5
        case 'X':
            add, ok := lookAhead(s, c, i, l)
            if !ok {
                count += 10
            } else {
                count += add
                i++
            }
        case 'L':
            count += 50
        case 'C':
            add, ok := lookAhead(s, c, i, l)
            if !ok {
                count += 100
            } else {
                count += add
                i++
            }
        case 'D':
            count += 500
        case 'M':
            count += 1000
        }
    }

    return count
}

// lookAhead checks if the current character and the next one form a subtractive combination.
//
// It returns the combined value if a valid subtractive pair is found, and a boolean indicating success.
func lookAhead(s string, c byte, i int, l int) (int, bool) {
    if i+1 >= l {
        return -1, false
    }

    n := s[i+1]

    switch c {
    case 'I':
        if n == 'V' {
            return 4, true
        }
        if n == 'X' {
            return 9, true
        }
    case 'X':
        if n == 'L' {
            return 40, true
        }
        if n == 'C' {
            return 90, true
        }
    case 'C':
        if n == 'D' {
            return 400, true
        }
        if n == 'M' {
            return 900, true
        }
    }

    return -1, false
}

func main() {
    // Example usage for CI validation
    fmt.Println(romanToInt("MCMXCIV")) // Output: 1994
}
