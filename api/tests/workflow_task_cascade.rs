//! Guards the cascade that keeps the DAG whole.
//!
//! Deleting a step has to take every edge touching it, from both directions, or
//! the graph is left with edges pointing at a step that no longer exists, which
//! the runner would walk straight into.
//!
//! This works only because `WorkflowTask` declares `has_many(pair = from_task)`
//! and `has_many(pair = to_task)`. Toasty applies referential integrity in the
//! query engine rather than through database foreign keys, so without those
//! declarations nothing cascades and the deletion silently orphans the edges.
//! The `pair` is required because both relations target the same table and the
//! field name alone cannot say which one each reverses.

mod common;

use api::models::WorkflowTaskEdge;
use api::repositories::workflow_task::WorkflowTaskRepository;

#[actix_web::test]
async fn deleting_a_step_cascades_to_edges_in_both_directions() {
    let mut db = common::test_db().await;
    let user = common::user::test_user(&mut db, None, None).await;
    let task = common::task::test_task(&mut db, user.id.to_string()).await;
    let workflow = common::workflow::test_workflow(&mut db, user.id.to_string()).await;

    // Three steps in a row, so the middle one has an edge on each side.
    let first = common::workflow_task::test_workflow_task_dyn(
        &mut db,
        workflow.id.to_string(),
        task.id.to_string(),
        1,
        "first".to_string(),
    )
    .await;
    let middle = common::workflow_task::test_workflow_task_dyn(
        &mut db,
        workflow.id.to_string(),
        task.id.to_string(),
        2,
        "middle".to_string(),
    )
    .await;
    let last = common::workflow_task::test_workflow_task_dyn(
        &mut db,
        workflow.id.to_string(),
        task.id.to_string(),
        3,
        "last".to_string(),
    )
    .await;

    common::worflow_task_edge::test_workflow_task_edge(
        &mut db,
        workflow.id.to_string(),
        first.id.to_string(),
        middle.id.to_string(),
    )
    .await;
    common::worflow_task_edge::test_workflow_task_edge(
        &mut db,
        workflow.id.to_string(),
        middle.id.to_string(),
        last.id.to_string(),
    )
    .await;

    let before = WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().workflow_id().eq(workflow.id))
        .exec(&mut db)
        .await
        .expect("failed to load edges");
    assert_eq!(before.len(), 2, "expected the two edges just created");

    // Deleted through the repository alone. Nothing here removes edges by hand,
    // so anything that disappears did so because the relation cascaded.
    WorkflowTaskRepository
        .delete(&mut db, middle.id)
        .await
        .expect("failed to delete the middle step");

    let after = WorkflowTaskEdge::filter(WorkflowTaskEdge::fields().workflow_id().eq(workflow.id))
        .exec(&mut db)
        .await
        .expect("failed to reload edges");

    assert_eq!(
        after.len(),
        0,
        "both the incoming and the outgoing edge should have gone with the step, \
         leaving no edge pointing at a step that no longer exists"
    );
}
