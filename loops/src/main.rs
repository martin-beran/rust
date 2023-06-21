fn main() {
    // loop
    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i + 2;
        }
    };
    println!("loop={}", result);

    // loop with label
    let mut i = 0;
    'outer: loop {
        println!("i={i}");
        let mut n = 10;
        loop {
            println!("n={n}");
            if n == 9 {
                break;
            }
            if i == 2 {
                break 'outer;
            }
            n -= 1;
        }
        i += 1;
    }
    println!("end i={i}");

    // while
    let mut n = 5;
    while n != 0 {
        println!("{n}");
        n -= 1;
    }
    println!("after while");

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("a[{i}]={}", a[i]);
        i +=1;
    }

    // for
    for v in a {
        println!("a[?]={v}");
    }
    for v in (0..10).rev() {
        println!("v={v}");
    }

    // Fibonacci
    for v in (0..50) {
        println!("fib({v})={}", fib(v));
    }
}

fn fib(mut n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let mut i = 0;
        let mut j = 1;
        while n > 0 {
            let tmp = j;
            j = i + j;
            i = tmp;
            n -= 1;
        }
        j
    }
}
