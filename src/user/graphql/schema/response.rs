use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "Get user by id response")]
pub struct GetUserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
