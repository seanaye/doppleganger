use doppleganger::Doppleganger as DoppleganglerDerive;
use doppleganger::Mirror;

// Source struct - represents data from an external API
#[derive(Debug)]
pub struct ApiUser {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub active: bool,
}

// Destination struct - our internal domain model
// Using forward: transforms FROM DbUser TO ApiUser
#[derive(Debug, DoppleganglerDerive)]
#[dg(forward = ApiUser)]
pub struct DbUser {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub active: bool,
}

pub mod ext {
    pub mod nested {
        // Another example using backward direction
        #[derive(Debug)]
        pub struct ExternalProduct {
            pub sku: String,
            pub name: String,
            pub price: u32,
        }
    }
}

// Using backward: transforms FROM ExternalProduct TO InternalProduct
#[derive(Debug, DoppleganglerDerive)]
#[dg(backward = ext::nested::ExternalProduct)]
pub struct InternalProduct {
    pub sku: String,
    pub name: String,
    pub price: u32,
}

fn main() {
    println!("=== Doppleganger Macro Example ===\n");

    // Example 1: Forward transformation (DbUser -> ApiUser)
    println!("Example 1: Forward transformation (DbUser -> ApiUser)");
    let db_user = DbUser {
        id: 42,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
    };

    println!("Source (DbUser): {:#?}", db_user);

    // Use the Doppleganger trait to transform
    let api_user = <DbUser as Mirror>::mirror(db_user);

    println!("Destination (ApiUser): {:#?}\n", api_user);

    // Example 2: Backward transformation (ExternalProduct -> InternalProduct)
    println!("Example 2: Backward transformation (ExternalProduct -> InternalProduct)");
    let external_product = ext::nested::ExternalProduct {
        sku: "WIDGET-001".to_string(),
        name: "Super Widget".to_string(),
        price: 1999,
    };

    println!("Source (ExternalProduct): {:#?}", external_product);

    // Use the Doppleganger trait to transform
    let internal_product = <InternalProduct as Mirror>::mirror(external_product);

    println!("Destination (InternalProduct): {:#?}\n", internal_product);

    // Example 3: Nested types work automatically via the trait implementations
    println!("Example 3: Collections work automatically");
    let db_users = vec![
        DbUser {
            id: 1,
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            active: true,
        },
        DbUser {
            id: 2,
            username: "charlie".to_string(),
            email: "charlie@example.com".to_string(),
            active: false,
        },
    ];

    println!("Source (Vec<DbUser>): {:#?}", db_users);

    // Vec<T> implements Doppleganger if T does
    let api_users = <Vec<DbUser> as Mirror>::mirror(db_users);

    println!("Destination (Vec<ApiUser>): {:#?}", api_users);
}
