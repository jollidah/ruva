use ruva::TConstruct;

#[derive(TConstruct)] // This should cause a compilation error
enum MyEnum {
	Variant1,
	Variant2,
}

fn main() {}
