
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        mod local {
            pub const BACKEND_ID: candid::Principal = candid::Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 2, 1, 1]);

        }

        #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
        mod ic {
            pub const BACKEND_ID: candid::Principal = candid::Principal::from_slice(&[128, 0, 0, 0, 0, 16, 0, 2, 1, 1]);

        }
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        pub use local::*;
        #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
        pub use ic::*;
