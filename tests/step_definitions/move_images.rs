use cucumber_rust::{Steps, t};

steps!(crate::MyWorld => {
    given regex r#"^an image is displayed in the viewer$"# (crate::MyWorld, _matches, _step) {
        // Implement step here
        unimplemented!()
    }

    when regex r#"^the user moves the image$"# (crate::MyWorld, _matches, _step) {
        // Implement step here
        unimplemented!()
    }

    then regex r#"^the image is moved within the viewer$"# (crate::MyWorld, _matches, _step) {
        // Implement step here
        unimplemented!()
    }
});
