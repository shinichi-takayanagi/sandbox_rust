pub fn add(x: i32, y:i32) -> i32{
    return x + y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
}

#[test]
#[should_panic]
fn panic() { 
    panic!("panic")
}

// cargo test時のみビルドされてBinaryにはいる
// テスト時のみのヘルパー関数とかProduction Codeにはいらんのでそういうのを潰す
#[cfg(test)]
mod tests{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}