fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./assets/icon/iris.ico");
    res.compile().unwrap();
}