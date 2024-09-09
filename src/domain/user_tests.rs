#[cfg(test)]
pub(crate) mod tests {
    use uuid::Uuid;

    use crate::domain::User;

    #[test]
    fn useless_test_to_check_project_dir_stracture() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn register_user() {
        let id = Uuid::new_v4();
        let email = "boyg87059@gmail.com".to_string();
        let password = "password".to_string();

        let mut user = User::default();
        user = user.register(id.clone(), email.clone(), password.clone()).unwrap();

        assert_eq!(user.id(), id);
        assert_eq!(email, user.email_as_ref());
        assert_eq!(true, user.is_registered());
    }
}
