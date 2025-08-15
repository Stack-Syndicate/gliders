use gliders::glider;

#[test]
fn basic_test() {
    glider!(
        let x = 10;
        let y = 10.0;
        let x = (x + (x * 3)) / 2;
        let x = [1+2, 2, (x + (x * 3)) / 2, 4];
    );
    println!("{:?}", x);
    println!("{}", y);
}