use mate_mutate::Mutate;

#[test]
fn derive_mutate() {
    #[derive(Mutate)]
    struct StructA {
        a: f32,
        b: bool,
        c: String,
    }

    let a = StructA { a: 0.0, b: true, c: String::from("hello") };
    println!("{}", a.mutate());
}