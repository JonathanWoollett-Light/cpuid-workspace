fn main() {
    #[cfg(not(target_arch = "x86_64"))]
    std::compile_error!("This crate only supports x86_64 targets");
}
