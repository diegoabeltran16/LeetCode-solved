int val(char c){
    switch(c){
        case'I': return 1;
        case'V': return 5;
        case'X': return 10;
        case'L': return 50;
        case'C': return 100;
        case'D': return 500;
        case'M': return 1000;
    }
        return -1;
}

int romanToInt(char* s) {
    int sum=0;

    while(*s!='\0'){
        if(*(s+1)!='\0' && val(*s) < val(*(s+1))){
            char c= *(s+1);
            switch(*s){
                case'I':
                    if(c=='V')
                        sum+=4;
                    else
                        sum+=9;
                    break;
                case'X':
                    if(c=='L')
                        sum+=40;
                    else
                        sum+=90;
                    break;
                case 'C':
                    if(c=='D')
                        sum+=400;
                    else
                        sum+=900;
                    break;
            }
            s+=2;
        }else{
            sum+=val(*s);
            s++;
        }
    }
    return sum;

    
}


