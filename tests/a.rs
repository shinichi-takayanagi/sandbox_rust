use sandbox_rust::module::a::add;


#[test]
fn integration_test(){
    //integrationしてないけど…
    assert_eq!(3, add(1,2))
}