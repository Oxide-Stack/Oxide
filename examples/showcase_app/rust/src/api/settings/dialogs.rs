use oxide_generator_rs::oxide_callback;

pub use oxide_core::OxideChannelError;

pub struct DialogService {}

#[oxide_callback(no_frb)]
impl oxide_core::OxideCallbacking for DialogService {
    type Request = DialogRequest;
    type Response = DialogResponse;
}

#[derive(Clone, Debug)]
pub enum DialogRequest {
    ShowAlert { title: String },
    Confirm { title: String, message: String },
}

#[derive(Clone, Debug)]
pub enum DialogResponse {
    ShowAlert(bool),
    Confirm(bool),
}
