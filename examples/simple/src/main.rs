cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[tokio::main]
        async fn main() {

        }

    } else {
        pub fn main() {

        }
    }
}
