use axum::{extract::Path, headers::ContentType, http::StatusCode, TypedHeader};
use mime::{TEXT_CSS, TEXT_JAVASCRIPT};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::OnceLock,
};
use tracing::debug;

/// A static collection of assets distributed in the binary.
static ASSETS: OnceLock<StaticAssets> = OnceLock::new();

pub async fn serve_asset(
    Path(asset_path): Path<String>,
) -> (StatusCode, TypedHeader<ContentType>, &'static [u8]) {
    debug!(asset_path, "serving asset");

    if let Some((content_type, asset)) = ASSETS
        .get_or_init(StaticAssets::new)
        .get(asset_path.as_str())
    {
        (StatusCode::OK, TypedHeader(content_type.clone()), asset)
    } else {
        // TODO: Show global 404?
        (StatusCode::NOT_FOUND, TypedHeader(ContentType::text()), &[])
    }
}

macro_rules! insert_vendor_asset {
    ($assets_map:expr, $content_type:expr, $vendor_path:expr$(,)?) => {
        $assets_map.insert(
            $vendor_path,
            (
                $content_type,
                include_bytes!(concat!("../../../../vendor/", $vendor_path)),
            ),
        );
    };
}
#[derive(Debug)]
struct StaticAssets(HashMap<&'static str, (ContentType, &'static [u8])>);
impl StaticAssets {
    pub fn new() -> Self {
        let mut assets = Self(Default::default());
        assets.insert(
            "style.css",
            (
                ContentType::from(TEXT_CSS),
                include_bytes!("../../../static/style.css"),
            ),
        );
        assets.insert(
            "dev_restart.js",
            (
                ContentType::from(TEXT_JAVASCRIPT),
                include_bytes!("../../../static/dev_restart.js"),
            ),
        );
        insert_vendor_asset!(
            assets,
            ContentType::from(TEXT_JAVASCRIPT),
            "htmx-1.9.2/htmx.min.js",
        );
        assets
    }
}
impl Deref for StaticAssets {
    type Target = HashMap<&'static str, (ContentType, &'static [u8])>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StaticAssets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
