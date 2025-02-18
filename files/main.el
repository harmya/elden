func check(x, y) {
    let sum = x + y;
    let more_than_20 = sum >= 10;
    let less_than_30 = sum < 30;
    return more_than_20 && less_than_30;
}

func main() {
    let x = 10;
    let y = 20;
    return check(x, y);
}