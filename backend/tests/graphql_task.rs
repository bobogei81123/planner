mod common;
mod matchers;

use std::str::FromStr;

use common::{Result, TestServer, UserSession};
use googletest::prelude::*;

use planner_backend::entities;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use testlib::{test_uuid, PgDocker};
use uuid::Uuid;

use crate::matchers::{json_number, json_string, uuid_str};

const TEST_USERNAME: &str = "meteor";
const TEST_USER_UUID: Uuid = test_uuid(1);

async fn insert_test_user(id: Uuid, username: String, db_conn: &DatabaseConnection) -> Result<()> {
    entities::users::ActiveModel {
        id: Set(id),
        username: Set(username),
    }
    .insert(db_conn)
    .await?;

    Ok(())
}

#[googletest::test]
#[tokio::test]
async fn user_can_login() -> Result<()> {
    let pg_docker = PgDocker::new().await;
    insert_test_user(
        TEST_USER_UUID,
        TEST_USERNAME.to_owned(),
        pg_docker.db_conn(),
    )
    .await
    .expect("cannot insert test user");
    let server = TestServer::spawn(pg_docker.db_conn().clone()).await;
    let _ = UserSession::login_as(server, TEST_USERNAME).await?;

    Ok(())
}

#[googletest::test]
#[tokio::test]
async fn graphql_can_create_tasks() -> Result<()> {
    let pg_docker = PgDocker::new().await;
    insert_test_user(test_uuid(1), TEST_USERNAME.to_owned(), pg_docker.db_conn()).await?;
    let server = TestServer::spawn(pg_docker.db_conn().clone()).await;
    let user_session = UserSession::login_as(server, TEST_USERNAME).await?;

    let response: serde_json::Value = user_session
        .post("/graphql")
        .json(&serde_json::json!({
            "query": r#"
                mutation {
                    createTask(
                        input: {
                            title: "test",
                        }
                    ) {
                        id
                        title
                    }
                }
            "#
        }))
        .send()
        .await
        .expect("Request failed")
        .json()
        .await
        .expect("Failed to parse response as JSON");

    expect_that!(
        response,
        json_obj! {
            data: json_obj! {
                createTask: json_obj! {
                    id: json_string(uuid_str(anything())),
                    title: json_string(eq("test")),
                }
            }
        }
    );
    Ok(())
}

#[googletest::test]
#[tokio::test]
async fn graphql_can_modify_tasks() -> Result<()> {
    let pg_docker = PgDocker::new().await;
    insert_test_user(test_uuid(1), TEST_USERNAME.to_owned(), pg_docker.db_conn()).await?;
    let server = TestServer::spawn(pg_docker.db_conn().clone()).await;
    let user_session = UserSession::login_as(server, TEST_USERNAME).await?;

    let create_response: serde_json::Value = user_session
        .post("/graphql")
        .json(&serde_json::json!({
            "query": r#"
                mutation {
                    createTask(
                        input: {
                            title: "test",
                        }
                    ) {
                        id
                    }
                }
            "#
        }))
        .send()
        .await
        .expect("Request failed")
        .json()
        .await
        .expect("Failed to parse response as JSON");
    let task_id = Uuid::from_str(
        create_response["data"]["createTask"]["id"]
            .as_str()
            .unwrap(),
    )?;
    let update_response: serde_json::Value = user_session
        .post("/graphql")
        .json(&serde_json::json!({
            "query": format!(r#"
                mutation {{
                    updateTask(
                        input: {{
                            id: "{task_id}",
                            title: "test updated",
                            status: "COMPLETED",
                            point: 1,
                        }}
                    ) {{
                        id
                        title
                        status
                        point
                    }}
                }}
            "#),
        }))
        .send()
        .await
        .expect("Request failed")
        .json()
        .await
        .expect("Failed to parse response as JSON");

    expect_that!(
        update_response,
        json_obj! {
            data: json_obj! {
                updateTask: json_obj! {
                    id: json_string(uuid_str(eq(task_id))),
                    title: json_string(eq("test updated")),
                    status: json_string(eq("COMPLETED")),
                    point: json_number(predicate(|x: &serde_json::Number| x.as_i64().is_some_and(|x| x == 1))),
                }
            }
        }
    );
    Ok(())
}
