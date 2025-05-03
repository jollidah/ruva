#[test]
fn ui() {
	let t = trybuild::TestCases::new();
	t.pass("tests/construct/basic_struct.rs");
	t.pass("tests/construct/struct_with_except.rs");
	t.pass("tests/construct/struct_with_generics.rs");
	t.compile_fail("tests/construct/unsupported_enum.rs");
	t.compile_fail("tests/construct/unnamed_fields_fail.rs");
}
