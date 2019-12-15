use crate::error::Result;
use crate::server::{AppData, Authentication, CommonRequest, CommonResponse, PooledConnection};
use crate::sql::internal::problem_list_manager::ProblemListManager;

use crate::sql::internal::RequestUnpack;
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

pub(crate) async fn get_list<A: Clone + Authentication>(request: Request<AppData<A>>) -> Response {
    fn unpack_request<A>(request: Request<AppData<A>>) -> Result<(String, PooledConnection)> {
        let token = request.get_cookie("token")?;
        let conn = request.state().pool.get()?;
        Ok((token, conn))
    }
    fn construct_response(conn: PooledConnection, internal_user_id: String) -> Result<Response> {
        let list = conn.get_list(&internal_user_id)?;
        let response = Response::ok().body_json(&list)?;
        Ok(response)
    }

    let client = request.state().authentication.clone();
    match unpack_request(request) {
        Ok((token, conn)) => match client.get_user_id(&token).await {
            Ok(internal_user_id) => match construct_response(conn, internal_user_id) {
                Ok(response) => response,
                _ => Response::internal_error(),
            },
            _ => Response::bad_request(),
        },
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn create_list<A: Authentication + Clone + Send + Sync + 'static>(
    request: Request<AppData<A>>,
) -> Response {
    #[derive(Deserialize)]
    struct Query {
        list_name: String,
    }
    #[derive(Serialize)]
    struct Created {
        internal_list_id: String,
    }
    fn create_response(conn: PooledConnection, user_id: &str, list_name: &str) -> Result<Response> {
        let internal_list_id = conn.create_list(user_id, list_name)?;
        let created = Created { internal_list_id };
        let response = Response::ok().body_json(&created)?;
        Ok(response)
    }

    match request.post_unpack::<Query>().await {
        Ok((query, conn, internal_user_id)) => {
            match create_response(conn, &internal_user_id, &query.list_name) {
                Ok(response) => response,
                Err(_) => Response::internal_error(),
            }
        }
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn delete_list<A>(request: Request<AppData<A>>) -> Response
where
    A: Authentication + Clone + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    struct Q {
        internal_list_id: String,
    }
    match request.post_unpack::<Q>().await {
        Ok((query, conn, _)) => match conn.delete_list(&query.internal_list_id) {
            Ok(_) => Response::ok(),
            _ => Response::internal_error(),
        },
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn update_list<A>(request: Request<AppData<A>>) -> Response
where
    A: Authentication + Clone + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    struct Q {
        internal_list_id: String,
        name: String,
    }
    match request.post_unpack::<Q>().await {
        Ok((query, conn, _)) => match conn.update_list(&query.internal_list_id, &query.name) {
            Ok(_) => Response::ok(),
            _ => Response::internal_error(),
        },
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn add_item<A>(request: Request<AppData<A>>) -> Response
where
    A: Authentication + Clone + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    struct Q {
        internal_list_id: String,
        problem_id: String,
    }
    match request.post_unpack::<Q>().await {
        Ok((query, conn, _)) => match conn.add_item(&query.internal_list_id, &query.problem_id) {
            Ok(_) => Response::ok(),
            _ => Response::internal_error(),
        },
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn update_item<A>(request: Request<AppData<A>>) -> Response
where
    A: Authentication + Clone + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    struct Q {
        internal_list_id: String,
        problem_id: String,
        memo: String,
    }
    match request.post_unpack::<Q>().await {
        Ok((query, conn, _)) => {
            match conn.update_item(&query.internal_list_id, &query.problem_id, &query.memo) {
                Ok(_) => Response::ok(),
                _ => Response::internal_error(),
            }
        }
        Err(_) => Response::bad_request(),
    }
}

pub(crate) async fn delete_item<A>(request: Request<AppData<A>>) -> Response
where
    A: Authentication + Clone + Send + Sync + 'static,
{
    #[derive(Deserialize)]
    struct Q {
        internal_list_id: String,
        problem_id: String,
    }
    match request.post_unpack::<Q>().await {
        Ok((query, conn, _)) => {
            match conn.delete_item(&query.internal_list_id, &query.problem_id) {
                Ok(_) => Response::ok(),
                _ => Response::internal_error(),
            }
        }
        Err(_) => Response::bad_request(),
    }
}