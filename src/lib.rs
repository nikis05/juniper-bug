use insta::{assert_json_snapshot, assert_snapshot};
use juniper::{
    graphql_interface, graphql_object, EmptyMutation, EmptySubscription, RootNode, Variables,
};

struct Query;

#[graphql_object]
impl Query {
    fn returns_interface(&self) -> InterfaceValue {
        InterfaceValue::Foo(Foo)
    }
}

#[graphql_interface(for = [Foo, Bar])]
trait Interface {
    fn interface_field(&self) -> String {
        "Interface".into()
    }
}

struct Foo;

#[graphql_object(impl = [InterfaceValue])]
impl Foo {
    fn foo() -> String {
        "Foo".into()
    }
}

#[graphql_interface]
impl Interface for Foo {}

struct Bar;

#[graphql_object(impl = [InterfaceValue])]
impl Bar {
    fn bar() -> String {
        "Bar".into()
    }
}

#[graphql_interface]
impl Interface for Bar {}

#[tokio::test]
async fn test() {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    assert_snapshot!(schema.as_schema_language(), @r###"
    type Query {
      returnsInterface: Interface!
    }

    type Foo implements Interface {
      foo: String!
    }

    interface Interface {
      interfaceField: String!
    }

    type Bar implements Interface {
      bar: String!
    }

    schema {
      query: Query
    }
    "###);

    let document_source = r#"
        query Test {
            returnsInterface {
                ...on Interface {
                    interfaceField
                }
            }
        }
    "#;

    let result = juniper::execute(
        document_source,
        Some("Test"),
        &schema,
        &Variables::default(),
        &(),
    )
    .await;

    assert_json_snapshot!(result, @r###"
    {
      "Ok": [
        {
          "returnsInterface": {}
        },
        []
      ]
    }
    "###);
}
