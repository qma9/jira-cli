#[cfg(test)]
use super::{Epic, JiraDatabase, Status, Story};
use crate::db::db_tests::MockDB;

#[test]
fn create_epic_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic.clone());

    assert_eq!(result.is_ok(), true);

    let id = result.unwrap();
    let db_state = db.read_db().unwrap();

    let expected_id = 1;

    assert_eq!(id, expected_id);
    assert_eq!(db_state.last_item_id, expected_id);
    assert_eq!(db_state.epics.get(&id), Some(&epic));
}

#[test]
fn create_story_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("".to_owned(), "".to_owned());

    let non_existent_epic_id = 999;

    let result = db.create_story(story, non_existent_epic_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn create_story_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story.clone(), epic_id);
    assert_eq!(result.is_ok(), true);

    let id = result.unwrap();
    let db_state = db.read_db().unwrap();

    let expected_id = 2;

    assert_eq!(id, expected_id);
    assert_eq!(db_state.last_item_id, expected_id);
    assert_eq!(
        db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
        true
    );
    assert_eq!(db_state.stories.get(&id), Some(&story));
}

#[test]
fn delete_epic_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_epic_id = 999;

    let result = db.delete_epic(non_existent_epic_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_epic_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let result = db.delete_epic(epic_id);
    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    let expected_last_id = 2;

    assert_eq!(db_state.last_item_id, expected_last_id);
    assert_eq!(db_state.epics.get(&epic_id), None);
    assert_eq!(db_state.stories.get(&story_id), None);
}

#[test]
fn delete_story_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let non_existent_epic_id = 999;

    let result = db.delete_story(non_existent_epic_id, story_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_story_should_error_if_story_not_found_in_epic() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let non_existent_story_id = 999;

    let result = db.delete_story(epic_id, non_existent_story_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_story_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let result = db.delete_story(epic_id, story_id);
    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    let expected_last_id = 2;

    assert_eq!(db_state.last_item_id, expected_last_id);
    assert_eq!(
        db_state
            .epics
            .get(&epic_id)
            .unwrap()
            .stories
            .contains(&story_id),
        false
    );
    assert_eq!(db_state.stories.get(&story_id), None);
}

#[test]
fn update_epic_status_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_epic_id = 999;

    let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
    assert_eq!(result.is_err(), true);
}

#[test]
fn update_epic_status_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);

    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.update_epic_status(epic_id, Status::Closed);

    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
}

#[test]
fn update_story_status_should_error_if_invalid_story_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_story_id = 999;

    let result = db.update_story_status(non_existent_story_id, Status::Closed);
    assert_eq!(result.is_err(), true);
}

#[test]
fn update_story_status_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);

    let story_id = result.unwrap();

    let result = db.update_story_status(story_id, Status::Closed);

    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    assert_eq!(
        db_state.stories.get(&story_id).unwrap().status,
        Status::Closed
    );
}

mod db_tests {
    use crate::db::{
        models::{DBState, Epic, Status, Story},
        Database, JSONFileDatabase,
    };
    use std::collections::HashMap;
    use std::io::Write;

    #[test]
    fn read_db_should_fail_with_invalid_path() {
        let db = JSONFileDatabase {
            file_path: "INVALID_PATH".to_owned(),
        };
        assert_eq!(db.read_db().is_err(), true);
    }

    #[test]
    fn read_db_should_fail_with_invalid_json() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

        let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
        write!(tmpfile, "{}", file_contents).unwrap();

        let db = JSONFileDatabase {
            file_path: tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str")
                .to_string(),
        };

        let result = db.read_db();

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn read_db_should_parse_json_file() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

        let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
        write!(tmpfile, "{}", file_contents).unwrap();

        let db = JSONFileDatabase {
            file_path: tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str")
                .to_string(),
        };

        let result = db.read_db();

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn write_db_should_work() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

        let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
        write!(tmpfile, "{}", file_contents).unwrap();

        let db = JSONFileDatabase {
            file_path: tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile path to str")
                .to_string(),
        };

        let story = Story {
            name: "epic 1".to_owned(),
            description: "epic 1".to_owned(),
            status: Status::Open,
        };
        let epic = Epic {
            name: "epic 1".to_owned(),
            description: "epic 1".to_owned(),
            status: Status::Open,
            stories: vec![2],
        };

        let mut stories = HashMap::new();
        stories.insert(2, story);

        let mut epics = HashMap::new();
        epics.insert(1, epic);

        let state = DBState {
            last_item_id: 2,
            epics,
            stories,
        };

        let write_result = db.write_db(&state);
        let read_result = db.read_db().unwrap();

        assert_eq!(write_result.is_ok(), true);
        assert_eq!(read_result, state);
    }
}
