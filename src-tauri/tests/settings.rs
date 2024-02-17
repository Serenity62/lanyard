#[cfg(test)]
mod settings {
    use app::Settings;

    #[test]
    fn builder() {
        let settings = Settings::builder(String::from("files/settings.json"));
        assert_eq!(0, settings.profiles.len());
    }

    #[test]
    fn builder_create() {
        let path = String::from("files/settings_new.json");
        let _ = std::fs::remove_file(&path);

        let settings = Settings::builder(path);
        assert_eq!(0, settings.profiles.len());
    }

    /*
    #[test]
    fn create() {}
    #[test]
    fn delete() {}
    */
}