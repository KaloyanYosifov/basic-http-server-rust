struct TestController;

impl TestController {
    #[core_macros::route("test")]
    fn execute_home() {}
}

#[test]
fn it_adds_the_function_to_be_called_in_a_controller() {
    TestController {};

    assert!(false)
}
