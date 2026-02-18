use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::navigation::Route;

pub trait OxideRouteKind: Copy + PartialEq + Eq + Send + Sync + 'static {
    fn as_str(&self) -> &'static str;
}

pub trait OxideRoutePayload: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {
    type Kind: OxideRouteKind;

    fn kind(&self) -> Self::Kind;
}

pub trait OxideRoute: Route {
    type Payload: OxideRoutePayload;

    fn into_payload(self) -> Self::Payload;
}

