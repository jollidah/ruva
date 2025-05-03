use ruva::TConstruct; // Replace with your crate name

#[derive(Default, TConstruct)]
struct MyStruct {
	value: i32,
	#[except]
	name: String,
}

fn main() {
	let instance = MyStruct::construct(25);
	assert_eq!(instance.value, 25);
	assert_eq!(instance.name, String::default());
}
