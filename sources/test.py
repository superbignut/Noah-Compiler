let ddd = 0.11;


fn hello_world(aaa, bbb){
    
    print aaa + bbb;

}


fn add(num1, num2){

    let temp = 1.0;
    print num1 + num2 + temp;
}


hello_world("Hello" + " ", "World!");

add(10.0 + ddd, 100.0 + 1000.0);



for(let i = 0.0; i< 5.0; i = i + 1.0){
        for(let j = 0.0; j < i;  j = j + 1.0){
                add(i, 2.0 * j);
                hello_world("aaa" , " bbb");
        }
}

