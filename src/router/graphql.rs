use std::sync::Arc;

use diesel::SqliteConnection;
use juniper::{Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::database::connection::DbPool;
use crate::user::graphql::{
    controller::UserGraphQLController, schema::response::GetUserByIdResponse,
};

pub struct AppController {
    pub user_ctrl: UserGraphQLController,
}

impl AppController {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        AppController {
            user_ctrl: UserGraphQLController::new(db_pool),
        }
    }
}

impl Context for AppController {}

pub struct QueryRoot;

#[juniper::graphql_object(context = AppController)]
impl QueryRoot {
    fn get_user_by_id(context: &AppController, user_id: i32) -> FieldResult<GetUserByIdResponse> {
        context.user_ctrl.get_user_by_id(user_id)
    }
}

pub struct GraphQLSchema {
    pub schema: RootNode<
        'static,
        QueryRoot,
        EmptyMutation<AppController>,
        EmptySubscription<AppController>,
    >,
    pub context: AppController,
}

impl GraphQLSchema {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        let context = AppController::new(db_pool);
        let schema = RootNode::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
        GraphQLSchema { schema, context }
    }
}
