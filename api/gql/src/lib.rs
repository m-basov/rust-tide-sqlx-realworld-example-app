mod context;
mod query;

use context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_warp::{graphiql_filter, make_graphql_filter};
use query::Query;
use server::{warp, ServerState};
use warp::{Filter, Rejection, Reply};

pub struct Gql;

impl Gql {
    pub fn new(state: ServerState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let graphql = warp::path!("graphql").and(make_graphql_filter(
            schema(),
            Context::extract(state).boxed(),
        ));

        let graphiql = warp::path!("graphiql").and(graphiql_filter("/graphql", None));

        graphql.or(graphiql)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
