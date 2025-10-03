#[macro_export]
macro_rules! bootstrapper_ext_trait {
    ($modules_ty: ty) => {
        #[async_trait::async_trait]
        pub trait BootstraperExt {
            async fn bootstrap(modules: $modules_ty);
        }
    };
}
