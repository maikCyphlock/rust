fn middle_element_char(list: &[char]) -> char {
    list[list.len() / 2]
}

fn middle_element_int(list: &[i64]) -> i64 {
    list[list.len() / 2]
}

//GENERIC FUNCTION
fn middle_element<T: Copy>(list: &[T]) -> T {
    list[list.len() / 2]
}

struct Plano<T> {
    x: T,
    y: T,
}

impl<T> Plano<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    // println!("{:?}", middle_element_char(&['a', 'b', 'c']));
    // println!("{:?}", middle_element_int(&[1, 554, 3]));

    // println!("{:?}", middle_element(&["sdasd", "554", "3"]));
    // println!("{:?}", middle_element(&[true, false, true]));

    // let p = Plano { x: 1, y: 2 };
    // println!("{:?}", p.x());
    // println!("{:?}", p.y());
    // println!("Hello, world!");
    let arr = [2, 3, 4, 10, 40];
    let target = 40;
    match binary_search(&arr, target, 0, arr.len()) {
        Some(index) => println!("El valor {} se encuentra en el índice {}", target, index),
        None => println!("El valor {} no se encuentra en el arreglo", target),
    }
}

fn binary_search(list: &[u64], target: u64, start: usize, end: usize) -> Option<usize> {
    if start >= end {
        return None;
    }

    let mid = start + (end - start) / 2;
    if list[mid] == target {
        return Some(mid);
    } else if list[mid] > target {
        return binary_search(list, target, start, mid);
    } else {
        return binary_search(list, target, mid + 1, end);
    }
}
