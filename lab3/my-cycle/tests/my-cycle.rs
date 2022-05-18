use my_cycle::MyCycle;

#[test]
fn basic_cycle() {
    let mut cycle = MyCycle::new(vec![1, 2, 3].into_iter(), 1);
    assert_eq!(cycle.next(), Some(1));
    assert_eq!(cycle.next(), Some(2));
    assert_eq!(cycle.next(), Some(3));
}

#[test]
fn zero_as_input() {
    let vec : Vec<i32> = vec![];
    let empty_iter = vec.iter();
    let zero_cycle = MyCycle::new(empty_iter, 0);
    assert_eq!(0, MyCycle::count(zero_cycle));
}

#[test]
fn recursive_my_cycle() {
    let base_iter = vec![1, 2, 3, 4].into_iter();
    let base_len = base_iter.len();
    let n1 = 2;
    let n2 = 3;
    let c1 = MyCycle::new(base_iter, n1); // repeat twice the base_iter -> count = 8
    let c2 = MyCycle::new(c1, n2); // repeat three times the c1 iter -> count = 3*8=24
    assert_eq!(n1*n2*base_len, c2.count());
}

#[test]
fn chain_my_cycle() {
    let n1 = 2;
    let n2 = 8;
    let i1 = 2;
    let i2 = 3;
    let v1 = vec![0; i1].into_iter();
    let v2 = vec![0; i2].into_iter();
    let my_cycle = MyCycle::new(v1, n1)
        .chain(MyCycle::new(v2, n2));
    assert_eq!(i1*n1+i2*n2, my_cycle.count());
}

#[test]
fn zip_my_cycle() {
    let v1 = vec![1, 2, 3, 4].into_iter();
    let v2 = vec![4, 3, 2, 1].into_iter();
    MyCycle::new(v1, 1)
        .zip(MyCycle::new(v2, 1))
        .enumerate()
        .for_each(|(i, v)| assert_eq!(v, (i+1, 4-i) ));
}
