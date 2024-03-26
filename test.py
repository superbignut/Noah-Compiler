//

fn resursive(l, r){

    if(l >= r)
        return 0.0;
    
    let sum = 1.0;
    
    sum = sum + resursive(l, r - 1.0);

    return sum;
}



print resursive(1.0 , 10.0);
