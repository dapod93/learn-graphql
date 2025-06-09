use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
#[graphql(description = "Create user request")]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
