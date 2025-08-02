use dioxus::{hooks::use_context, signals::Signal};

use crate::model::user::UserResponse;

pub struct AuthState {
    pub user: Option<UserResponse>,
    pub is_authenticated: bool,
    pub loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
            loading: false,
        }
    }
}

impl AuthState {
    pub fn login(&mut self, user: UserResponse) {
        self.user = Some(user);
        self.is_authenticated = true;
        self.loading = false;
    }

    pub fn logout(&mut self) {
        self.user = None;
        self.is_authenticated = false;
        self.loading = false;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
}

pub fn use_auth() -> Signal<AuthState> {
    use_context::<Signal<AuthState>>()
}
