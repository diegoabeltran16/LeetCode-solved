import java.util.Map;
import java.util.HashMap;

class Solution {
    public int romanToInt(String s) {
        Map<Character, Integer> romanMap = new HashMap<>();
        romanMap.put('I', 1);
        romanMap.put('V', 5);
        romanMap.put('X', 10);
        romanMap.put('L', 50);
        romanMap.put('C', 100);
        romanMap.put('D', 500);
        romanMap.put('M', 1000); 
        int r=0,pv=1; 
        for(int i=s.length()-1;i>=0;i--) { 
            int cv= romanMap.get(s.charAt(i));
            if(cv<pv) {
                r-=cv; 
            } else {
                r+=cv; 
            }
            pv=cv; 
        } 
        return r;
    } 
}
