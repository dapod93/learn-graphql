#[derivce(GraphQLObject)]
#[graphql(description = "Ping")]
struct ping {
    pong: String,
    pong2: String,
}

pub struct QueryRoot;
