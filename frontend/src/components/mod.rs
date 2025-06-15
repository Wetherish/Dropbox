//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

mod new_dir;
pub use new_dir::New_dir;

mod create_user;
pub use create_user::Create_user;

mod file_item;
pub use file_item::File_item;

mod login;
pub use login::login_component;
