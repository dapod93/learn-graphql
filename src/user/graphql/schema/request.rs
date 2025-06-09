use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "Create user")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
