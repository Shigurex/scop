pub fn render() {
    unsafe {
        gl::ClearColor(0.9, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
