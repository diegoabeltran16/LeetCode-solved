class Solution {
public:
    int romanToInt(std::string s)
    {

        int sum {0};

        //std::unordered_map<char, int> romanToValue{{'I',1}, {'V',5}, {'X',10}, {'L',50}, {'C',100}, {'D',500}, {'M',1000}};

        auto romanToValue = [](char roman) constexpr {
            switch (roman)
            {
                case 'I':
                {   
                    return 1;
                    break;
                }
                case 'V':
                {
                    return 5;
                    break;
                }
                case 'X':
                {
                    return 10;
                    break;
                }
                case 'L':
                {
                    return 50;
                    break;
                }
                case 'C':
                {
                    return 100;
                    break;
                }
                case 'D':
                {
                    return 500;
                    break;
                }
                case 'M':
                {
                    return 1000;
                    break;
                }
                default:
                {
                    return 0;
                    break;
                }
            }
        };

        for (int i=0; i<s.length(); ++i)
        {
            //if (romanToValue.contains(s[i]))
            //{
                if (i+1<s.length() && romanToValue(s[i])<romanToValue(s[i+1]))
                {
                    sum -= romanToValue(s[i]);
                }
                else
                {
                    sum += romanToValue(s[i]);
                }
            //}
        }
        return sum;
    }

};