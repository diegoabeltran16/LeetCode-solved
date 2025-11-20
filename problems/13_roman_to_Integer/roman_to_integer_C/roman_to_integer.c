// Returns the integer value of a single Roman numeral character :)
int val(char c){
    switch(c){
        case 'I': return 1;
        case 'V': return 5;
        case 'X': return 10;
        case 'L': return 50;
        case 'C': return 100;
        case 'D': return 500;
        case 'M': return 1000;
    }
    return -1; // Return -1 for invalid input
}

// Converts a Roman numeral string to its integer value
int romanToInt(char* s) {
    int sum = 0; // Accumulates the integer result

    while(*s != '\0') { // Loop until end of string
        // Check for subtractive combinations (like IV, IX, etc.)
        if(*(s+1) != '\0' && val(*s) < val(*(s+1))){
            char c = *(s+1);
            switch(*s) {
                case 'I':
                    if(c == 'V')
                        sum += 4; // IV = 4
                    else
                        sum += 9; // IX = 9
                    break;
                case 'X':
                    if(c == 'L')
                        sum += 40; // XL = 40
                    else
                        sum += 90; // XC = 90
                    break;
                case 'C':
                    if(c == 'D')
                        sum += 400; // CD = 400
                    else
                        sum += 900; // CM = 900
                    break;
            }
            s += 2; // Move past the pair
        } else {
            // Regular single symbol, just add its value
            sum += val(*s);
            s++;
        }
    }
    return sum;
}
