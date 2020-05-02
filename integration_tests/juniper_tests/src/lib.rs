#[cfg(test)]
mod codegen;
#[cfg(test)]
mod custom_scalar;
#[cfg(test)]
mod issue_371;
#[cfg(test)]
mod issue_398;

mod foo {
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
}
