fn main() {
    glib_build_tools::compile_resources(
        &["src/ui"],
        "src/ui/resources.gresource.xml",
        "app.gresource",
    );
}
