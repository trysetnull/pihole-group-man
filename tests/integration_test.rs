use crate::common::{ClientTestData, GroupTestData, TestData};

mod common;

#[test]
fn append_new_group_to_client() {
    // arrange
    let client_comment = String::from("client_no# 1");
    let group_name = String::from("group_1");

    let client = ClientTestData::new(17, client_comment.clone(), None);
    let group = GroupTestData::new(42, group_name.clone());
    let test_data = TestData::new(Some(vec![client]), Some(vec![group]));

    let conn = common::setup(test_data).unwrap();

    // act
    let result = pihole_group_man::append(&conn, &client_comment, &group_name);

    // assert
    assert!(result.is_ok(), "Expected OK, actual Error");
    assert!(common::dump(&conn).is_ok(), "Failed to dump tables!");
}

#[test]
fn append_exiting_group_to_client() {
    // arrange
    let client_comment = String::from("client_no# 1");
    let group_name = String::from("group_1");

    let client = ClientTestData::new(17, client_comment.clone(), Some(vec![42]));
    let group = GroupTestData::new(42, group_name.clone());
    let test_data = TestData::new(Some(vec![client]), Some(vec![group]));

    let conn = common::setup(test_data).unwrap();

    // act
    let result = pihole_group_man::append(&conn, &client_comment, &group_name);

    // assert
    assert!(result.is_err(), "Expected Error, actual OK");
    assert_eq!(
        result.err().unwrap().sqlite_error_code().unwrap(),
        rusqlite::ErrorCode::ConstraintViolation
    );

    assert!(common::dump(&conn).is_ok(), "Failed to dump tables!");
}

#[test]
fn remove_existing_group_from_client() {
    // arrange
    let client_comment = String::from("client_no# 1");
    let group_name = String::from("group_1");

    let client = ClientTestData::new(17, client_comment.clone(), Some(vec![42]));
    let group = GroupTestData::new(42, group_name.clone());
    let test_data = TestData::new(Some(vec![client]), Some(vec![group]));

    let conn = common::setup(test_data).unwrap();

    // act
    let result = pihole_group_man::remove(&conn, &client_comment, &group_name);

    // assert
    assert!(result.is_ok(), "Expected OK, actual Error");
    assert!(common::dump(&conn).is_ok(), "Failed to dump tables!");
}

#[test]
fn remove_non_existing_group_from_client() {
    // arrange
    let client_comment = String::from("client_no# 1");
    let group_name = String::from("group_1");

    let client = ClientTestData::new(17, client_comment.clone(), None);
    let group = GroupTestData::new(42, group_name.clone());
    let test_data = TestData::new(Some(vec![client]), Some(vec![group]));

    let conn = common::setup(test_data).unwrap();

    // act
    let result = pihole_group_man::remove(&conn, &client_comment, &group_name);

    // assert
    assert!(result.is_ok(), "Expected OK, actual Error");
    assert!(common::dump(&conn).is_ok(), "Failed to dump tables!");
}
