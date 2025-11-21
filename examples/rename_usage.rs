use doppleganger::{Mirror, Doppleganger};

// Test forward rename
#[derive(Doppleganger)]
#[dg(forward = ApiUser)]
struct DbUser {
    id: u64,
    #[dg(rename = "username")]
    db_username: String,
    email: String,
}

#[derive(Debug, PartialEq)]
struct ApiUser {
    id: u64,
    username: String,
    email: String,
}

// Test backward rename
#[derive(Doppleganger)]
#[dg(backward = SourceData)]
struct DestData {
    value: i32,
    #[dg(rename = "old_name")]
    new_name: String,
}

#[derive(Debug, PartialEq)]
struct SourceData {
    value: i32,
    old_name: String,
}

fn main() {
    // Test forward transformation with rename
    let db_user = DbUser {
        id: 1,
        db_username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
    };

    let api_user: ApiUser = DbUser::mirror(db_user);
    assert_eq!(api_user.id, 1);
    assert_eq!(api_user.username, "john_doe");
    assert_eq!(api_user.email, "john@example.com");
    println!("Forward rename test passed: {:?}", api_user);

    // Test backward transformation with rename
    let source = SourceData {
        value: 42,
        old_name: "test".to_string(),
    };

    let dest: DestData = DestData::mirror(source);
    assert_eq!(dest.value, 42);
    assert_eq!(dest.new_name, "test");
    println!("Backward rename test passed: value={}, new_name={}", dest.value, dest.new_name);

    println!("All rename tests passed!");
}
