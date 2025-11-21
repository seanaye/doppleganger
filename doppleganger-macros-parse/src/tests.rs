use super::*;
use cool_asserts::assert_matches;
use quote::quote;

#[test]
fn test_struct_with_field_doc_comments() {
    let input = quote! {
        #[derive(Dopplegagner)]
        pub struct User {
            #[doc = " The user's unique identifier"]
            pub id: u64,
        }
    };

    let mut it = input.to_token_iter();
    let parsed = it.parse::<Struct>().expect("Failed to parse struct");

    // Check that we parsed the struct correctly
    assert_eq!(parsed.name.to_string(), "User");

    // Extract fields from the struct
    if let StructKind::Struct { fields, .. } = &parsed.kind {
        let field_list = &fields.content;
        assert_eq!(field_list.len(), 1);

        // Check first field (id)
        let id_field = &field_list[0].value;
        assert_eq!(id_field.name.to_string(), "id");

        // Extract doc comments from id field
        let mut doc_found = false;
        for attr in &id_field.attributes {
            match &attr.body.content {
                AttributeInner::Doc(doc_inner) => {
                    // This should work with LiteralString
                    assert_eq!(doc_inner.value, " The user's unique identifier");
                    doc_found = true;
                }
                _ => {
                    // Skip non-doc attributes
                }
            }
        }
        assert!(doc_found, "Should have found a doc comment");
    } else {
        panic!("Expected a regular struct with named fields");
    }
}

#[test]
fn it_parses_mod_path() {
    let mut token_iter = "axum::http::HeaderMap".to_token_iter();
    let m: ModPath = token_iter.parse().unwrap();
    assert_matches!(m, Cons { first: None, second, .. } => {
        let f = second.first().unwrap().value.to_string();
        assert_eq!(f, "axum");
        let f = second.get(1).unwrap().value.to_string();
        assert_eq!(f, "http");
        let f = second.get(2).unwrap().value.to_string();
        assert_eq!(f, "HeaderMap");
    });
}
