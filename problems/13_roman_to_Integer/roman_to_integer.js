/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function (s) {
    let o = {
        'I': 1,
        'V': 5,
        'X': 10,
        'L': 50,
        'C': 100,
        'D': 500,
        'M': 1000
    }
    let ans = 0;
    for (let i = 0; i < s.length - 1; i++) {
        if (o[s[i]] < o[s[i + 1]]) {
            ans -= o[s[i]];
        } else {
            ans += o[s[i]];
        }
    }
    return ans + o[s[s.length - 1]]

};