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
    fn test_user_registration() {
        let id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let password = "password".to_string();

        let mut user = User::new(id, email.clone(), password.clone());

        let result = user.register();

        assert!(result.is_ok());
        assert!(user.is_registered());
    }
}
