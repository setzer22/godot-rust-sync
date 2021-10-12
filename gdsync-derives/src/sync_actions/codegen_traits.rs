use crate::prelude::*;

/// Trait implemented by every GodotSyncAction to generate the code that should be added to each of the
/// generated methods on the resulting impl block.
///
/// Given a list of sync actions, the generate code would look like this:
///
/// impl GodotSync for MyStructFoo {
///     pub fn on_ready() {
///         action_1.on_ready();
///         action_2.on_ready();
///         action_3.on_ready();
///     }
///     pub fn start_frame() {
///         action_1.start_frame();
///         action_2.start_frame();
///         action_3.start_frame();
///     }
///     pub fn end_frame() {
///         action_1.end_frame();
///         action_2.end_frame();
///         action_3.end_frame();
///     }
/// }
pub trait ToGodotSyncCode {
    fn on_ready(&self) -> TokenStream2;
    fn start_frame(&self) -> TokenStream2;
    fn end_frame(&self) -> TokenStream2;
    fn standalone_fns(&self) -> TokenStream2;
}
