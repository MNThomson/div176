use type_alias_macro::create_type_alias;

create_type_alias!(UserID; u64);
create_type_alias!(AccountID; String);

fn main() {
    let user_id: UserID = 42.into();
    println!("User ID value: {user_id:?}");

    let account_id: AccountID = "ASD".into();
    println!("Account ID value: {account_id:?}");
}
