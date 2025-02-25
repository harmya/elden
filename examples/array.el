func main() {
    let arr = [1, 2, 3, 4, 5];
    let i = 0;
    
    while (i < 10) {
        sum = sum + i;
        i = i + 1;
    }

    arr.append(6 + 5 - sum);
    let len = arr.length;
    let last = arr[len - 1];
    return sum;
}

