// Copyright (c) 2018-2020 MobileCoin Inc.

use mc_util_uri::{Uri, UriScheme};

pub type WalletServiceMirrorUri = Uri<WalletServiceMirrorScheme>;

/// Mobilecoind Mirror Uri Scheme
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct WalletServiceMirrorScheme {}
impl UriScheme for WalletServiceMirrorScheme {
    /// The part before the '://' of a URL.
    const SCHEME_SECURE: &'static str = "mobilecoind-mirror";
    const SCHEME_INSECURE: &'static str = "insecure-mobilecoind-mirror";

    /// Default port numbers
    const DEFAULT_SECURE_PORT: u16 = 10443;
    const DEFAULT_INSECURE_PORT: u16 = 10080;
}
