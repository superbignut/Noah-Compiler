fn max(num1, num2){
    if(num1 > num2)
        return num1;
    else
        return num2;
}


fn min(num1, num2){
    if(num1 < num2)
        return num1;
    else
        return num2;
}

fn big_minus_small(num1, num2){
    return max(num1, num2) - min(num1, num2); 
}

fn fib(n){
    if(n <= 1.0)
        return n;
    return fib(1.0); 
}


print big_minus_small(9.0, min(2.0 + 3.0 , 1.0));

print fib(10.0);
