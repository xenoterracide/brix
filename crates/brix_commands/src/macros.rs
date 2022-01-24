#[macro_export]
macro_rules! dir {
    ($config: expr, $path: expr) => {
        $config.join($path)
    };
}
