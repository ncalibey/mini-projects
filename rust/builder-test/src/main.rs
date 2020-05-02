use builder::Builder;

#[derive(Debug)]
struct X {}

#[derive(Debug, Builder)]
struct Item<T, U>
where
    T: Default,
{
    a: u32,
    b: Option<&'static str>,
    c: String,
    #[builder(required)]
    d: X,
    e: T,
    #[builder(required)]
    f: U,
}

fn main() {
    let item: Item<i32, &str> = Item::builder()
        .a(42u32)
        .b("hello")
        .c("boom".to_owned())
        .d(X {})
        .e(42i32)
        .f("hello")
        .build();

    println!("{:#?}", item);

    let item2 = Item::<u32, u64>::builder().b(None).d(X {}).f(99u64).build();
    println!("{:#?}", item2);
}
