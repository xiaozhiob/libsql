use libsql::{named_params, params, Connection, Database, Params};

fn setup() -> Connection {
    let db = Database::open(":memory:").unwrap();
    let conn = db.connect().unwrap();
    conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)", ())
        .unwrap();
    conn
}

#[test]
fn execute() {
    let conn = setup();
    conn.execute("INSERT INTO users (id, name) VALUES (2, 'Alice')", ())
        .unwrap();
    let rows = conn.execute("SELECT * FROM users", ()).unwrap().unwrap();
    let row = rows.next().unwrap().unwrap();
    assert_eq!(row.get::<i32>(0).unwrap(), 2);
    assert_eq!(row.get::<&str>(1).unwrap(), "Alice");
}

#[test]
fn query() {
    let conn = setup();
    conn.execute("INSERT INTO users (id, name) VALUES (2, 'Alice')", ())
        .unwrap();

    let params = Params::from(vec![libsql::Value::from(2)]);

    let stmt = conn.prepare("SELECT * FROM users WHERE id = ?1").unwrap();

    let rows = stmt.query(&params).unwrap();
    let row = rows.next().unwrap().unwrap();

    assert_eq!(row.get::<i32>(0).unwrap(), 2);
    assert_eq!(row.get::<&str>(1).unwrap(), "Alice");

    stmt.reset();

    let row = stmt.query_row(&params).unwrap();

    assert_eq!(row.get::<i32>(0).unwrap(), 2);
    assert_eq!(row.get::<&str>(1).unwrap(), "Alice");

    stmt.reset();

    let mut names = stmt
        .query_map(&params, |r| r.get::<&str>(1).map(str::to_owned))
        .unwrap();

    let name = names.next().unwrap().unwrap();

    assert_eq!(name, "Alice");
}

#[test]
fn prepare_and_execute() {
    let conn = setup();
    check_insert(
        &conn,
        "INSERT INTO users (id, name) VALUES (2, 'Alice')",
        ().into(),
    );
    check_insert(
        &conn,
        "INSERT INTO users (id, name) VALUES (?1, ?2)",
        params![2, "Alice"],
    );
    check_insert(
        &conn,
        "INSERT INTO users (id, name) VALUES (?1, ?2)",
        (vec![2.into(), "Alice".into()] as Vec<params::Value>).into(),
    );
}

#[test]
fn prepare_and_execute_named_params() {
    let conn = setup();

    check_insert(
        &conn,
        "INSERT INTO users (id, name) VALUES (:a, :b)",
        vec![
            (":a".to_string(), 2.into()),
            (":b".to_string(), "Alice".into()),
        ]
        .into(),
    );

    check_insert(
        &conn,
        "INSERT INTO users (id, name) VALUES (:a, :b)",
        named_params! {
            ":a": 2,
            ":b": "Alice",
        },
    );
}

#[test]
fn prepare_and_dont_execute() {
    // TODO: how can we check that we've cleaned up the statement?

    let conn = setup();

    conn.prepare("INSERT INTO users (id, name) VALUES (?1, ?2)")
        .unwrap();

    // Drop the connection explicitly here to show that we want to drop
    // it while the above statment has not been executed.
    drop(conn);
}

fn check_insert(conn: &Connection, sql: &str, params: Params) {
    conn.execute(sql, params).unwrap();
    let rows = conn.execute("SELECT * FROM users", ()).unwrap().unwrap();
    let row = rows.next().unwrap().unwrap();
    // Use two since if you forget to insert an id it will automatically
    // be set to 1 which defeats the purpose of checking it here.
    assert_eq!(row.get::<i32>(0).unwrap(), 2);
    assert_eq!(row.get::<&str>(1).unwrap(), "Alice");
}

#[test]
fn nulls() {
    let conn = setup();
    conn.execute("INSERT INTO users (id, name) VALUES (NULL, NULL)", ())
        .unwrap();
    let rows = conn.execute("SELECT * FROM users", ()).unwrap().unwrap();
    let row = rows.next().unwrap().unwrap();
    assert_eq!(row.get::<i32>(0).unwrap(), 1);
    assert!(row.get::<&str>(1).is_err());
}
