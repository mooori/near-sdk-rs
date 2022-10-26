use darling::util::PathList;
use darling::FromMeta;

#[derive(Default, FromMeta, Clone, Debug)]
pub struct NearBindgenConfig {
    // TODO(blacklist) add doc comment
    pub blacklist_ext_fn_attrs: Option<PathList>,
}
