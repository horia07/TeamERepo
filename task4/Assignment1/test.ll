define dso_local i32 @main(i32 %argc, i8** %argv) {
    entry: 
    %t1 = alloca i32, align 4
    store i32 9, i32* %t1, align 4
    %t2 = load i32, i32* %t1, align 4
    %add = add nsw i32 %argc, 42
    %mul = mul nsw i32 %add, 2
    ret i32 0 
}