main() = {
    let x = 3;
    let y = 5;
    let sum = add(x, y);
    return sum;
}

add(x, y) = {
    return x + y;
}

sub(x, y) = {
    return x - y;
}

mul(x, y) = {
    return x * y;
}

div(x, y) = {
    if y == 0 {
        return -1;
    }
    return x / y;
}