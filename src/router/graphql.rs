use diesel::SqliteConnection;
use juniper::{Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::user::graphql::{
    controller::UserGraphQLController, schema::response::GetUserByIdResponse,
};

pub struct AppController {
    user_ctrl: UserGraphQLController,
}

impl AppController {
    pub fn new(db_conn: SqliteConnection) -> Self {
        AppController {
            user_ctrl: UserGraphQLController::new(db_conn),
        }
    }
}

impl Context for AppController {}

pub struct QueryRoot;

#[juniper::graphql_object(context = AppController)]
impl QueryRoot {
    fn get_user_by_id(context: AppController, user_id: i32) -> FieldResult<GetUserByIdResponse> {
        context.user_ctrl.get_user_by_id(user_id)
    }
}

pub struct GraphQLSchema(
    RootNode<'static, QueryRoot, EmptyMutation<AppController>, EmptySubscription<AppController>>,
);

impl GraphQLSchema {
    pub fn new(db_conn: SqliteConnection) -> Self {
        let schema = RootNode::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
        GraphQLSchema(schema)
    }
}
