#[cfg(test)]
use fnv::FnvHashMap;
#[cfg(test)]
use juniper::Object;
use juniper::{DefaultScalarValue, GraphQLObject, GraphQLObjectInfo};

#[cfg(test)]
use juniper::{
    self, execute, EmptyMutation, EmptySubscription, GraphQLType, RootNode, Value, Variables,
};

#[derive(GraphQLObjectInfo, Debug, PartialEq)]
#[graphql(scalar = DefaultScalarValue)]
struct Obj {
    regular_field: bool,
    #[graphql(
        name = "renamedField",
        description = "descr",
        deprecated = "field deprecation"
    )]
    c: i32,
}

#[juniper::graphql_object(
    name = "MyObj",
    description = "obj descr",
    scalar = DefaultScalarValue,
    derive_fields
)]
impl Obj {}

struct Query;

#[derive(GraphQLObject, Debug, PartialEq)]
struct SkippedFieldObj {
    regular_field: bool,
    #[graphql(skip)]
    skipped: i32,
}

#[juniper::graphql_object]
impl Query {
    fn obj() -> Obj {
        Obj {
            regular_field: true,
            c: 22,
        }
    }

    fn skipped_field_obj() -> SkippedFieldObj {
        SkippedFieldObj {
            regular_field: false,
            skipped: 42,
        }
    }
}

#[tokio::test]
async fn test_derived_object() {
    assert_eq!(
        <Obj as GraphQLType<DefaultScalarValue>>::name(&()),
        Some("MyObj")
    );

    // Verify meta info.
    let mut registry: juniper::Registry = juniper::Registry::new(FnvHashMap::default());
    let meta = Obj::meta(&(), &mut registry);

    assert_eq!(meta.name(), Some("MyObj"));
    assert_eq!(meta.description(), Some(&"obj descr".to_string()));

    let doc = r#"
        {
            obj {
                regularField
                renamedField
            }
        }"#;

    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    assert_eq!(
        execute(doc, None, &schema, &Variables::new(), &()).await,
        Ok((
            Value::object(
                vec![(
                    "obj",
                    Value::object(
                        vec![
                            ("regularField", Value::scalar(true)),
                            ("renamedField", Value::scalar(22)),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                )]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}
