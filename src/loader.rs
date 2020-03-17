//! Generic asset loading definition
//!
//! See the trait [`Loader`] for more informations
//!
//! [`Loader`]: trait.Loader.html

use std::error::Error;

#[cfg(feature = "serde")]
use serde::Deserialize;

/// Specifies how an asset is loaded.
///
/// With this trait, you can specify easily specify how you want your data to be loaded.
///
/// # Basic usage
///
/// Most of the time, you don't need to implement this trait yourself, as there
/// are implementations for the most formats (using `serde`). Don't forget to
/// enable the corresponding feature !
///
/// ```no_run
/// # cfg_if::cfg_if! { if #[cfg(feature = "ron")] {
/// use serde::Deserialize;
/// use assets_manager::{Asset, loader};
///
/// // The struct you want to load
/// #[derive(Deserialize)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl Asset for Point {
///     const EXT: &'static str = "ron";
///
///     // Specify here how to convert raw data
///     type Loader = loader::RonLoader;
/// }
/// # }}
/// ```
pub trait Loader<T> {
    /// Loads an asset from its raw bytes representation.
    fn load(content: Vec<u8>) -> Result<T, Box<dyn Error + Send + Sync>>;
}

/// A [`Loader`] to override [`Asset::load_from_raw`] function without caring
/// about the required `Loader` type.
///
/// Use it when you want to implement [`Asset`] but do not want/need to use a
/// loader, and only to override [`Asset::load_from_raw`].
///
/// **Warning** : this loader is not meant to be called and will panic if so
///
/// [`Loader`]: trait.Loader.html
/// [`Asset`]: ../trait.Asset.html
/// [`Asset::load`]: ../trait.Asset.html#method.load
/// [`Asset::load_from_raw`]: ../trait.Asset.html#method.load_from_raw
#[derive(Debug)]
pub struct CustomLoader;
impl<T> Loader<T> for CustomLoader {
    fn load(_: Vec<u8>) -> Result<T, Box<dyn Error + Send + Sync>> {
        panic!("You forgot to override `Asset::load_from_raw` function")
    }
}

macro_rules! serde_loader {
    ($feature:literal, $lib:ident, $doc:literal, $name: ident, $fun:path) => {
        #[cfg(feature = $feature)]
        use $lib;

        #[cfg(feature = $feature)]
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $name;

        #[cfg(feature = $feature)]
        impl<T> Loader<T> for $name
        where
            T: for<'de> Deserialize<'de>,
        {
            #[inline]
            fn load(content: Vec<u8>) -> Result<T, Box<dyn Error + Send + Sync>> {
                Ok($fun(&content)?)
            }
        }

    }
}

serde_loader!("bincode", serde_bincode, "Loads assets from Bincode encoded files", BincodeLoader, serde_bincode::deserialize);
serde_loader!("cbor", serde_cbor, "Loads assets from CBOR encoded files", CborLoader, serde_cbor::from_slice);
serde_loader!("json", serde_json, "Loads assets from JSON files", JsonLoader, serde_json::from_slice);
serde_loader!("ron", serde_ron, "Loads assets from RON files", RonLoader, serde_ron::de::from_bytes);
serde_loader!("yaml", serde_yaml, "Loads assets from YAML files", YamlLoader, serde_yaml::from_slice);
