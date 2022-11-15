use human_panic::setup_panic;

fn main() {
  setup_panic!(
    || { println!("Something goes here") },
    Metadata {
      name: env!("CARGO_PKG_NAME").into(),
      version: env!("CARGO_PKG_VERSION").into(),
      authors: "My Company Support <support@mycompany.com".into(),
      homepage: "support.mycompany.com".into(),
    }
  );

  println!("A normal log message");
  panic!("OMG EVERYTHING IS ON FIRE!!!");
}
