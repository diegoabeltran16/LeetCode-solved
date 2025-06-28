// MCMXCIV

func romanToInt(s string) int {
    count := 0
    l := len(s)

    for i := 0; i < l; i++ {
        c := s[i]
        switch c {
        case 'I':
            add, ok := lookAhead(s, c, i, l)
            if !ok {
                count += 1
            } else {
                count += add
                i++
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

func lookAhead(s string, c byte, i int, l int) (int, bool) {
    if i + 1 >= l {
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