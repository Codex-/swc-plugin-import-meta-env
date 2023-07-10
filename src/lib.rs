// use serde::Deserialize;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        transforms::testing::test,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};
use tracing::trace;

pub struct TransformVisitor;

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct ImportMetaEnvOptions {
//     some_option: Option<bool>,
// }

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        if !n.is_member() {
            return;
        }

        let member = n.as_mut_member().unwrap();
        if is_visiting_import_meta_env(member).is_some() {
            // member.obj = Box::new(Ident::new("kdy1".into(), member.span.clone()).into());
            // let mut x = Expr::Member(MemberExpr {
            //     span: member.span.clone(),
            //     obj: Box::new(Ident::new("process.env".into(), DUMMY_SP).into()),
            //     prop: member.prop.clone(),
            // });

            // dbg!(x);

            // n = x;
            trace!("is_visiting_import_meta_env");
            member.obj = Box::new(Ident::new("process".into(), DUMMY_SP).into());
        } else {
            trace!("ruh roah");
        }
    }
}

fn is_visiting_import_meta_env(n: &MemberExpr) -> Option<()> {
    let obj = n.obj.as_meta_prop()?;
    let prop = n.prop.as_ident()?;

    trace!("obj");
    dbg!(&obj);
    trace!("prop");
    dbg!(&prop);

    if obj.kind != MetaPropKind::ImportMeta {
        return None;
    }

    if prop.sym.to_string() != "env" {
        return None;
    }

    Some(())
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
// test!(
//     Default::default(),
//     |_| as_folder(TransformVisitor),
//     og,
//     // Input codes
//     r#"console.log("transform");"#,
//     // Output codes after transformed with plugin
//     r#"console.log("transform");"#
// );

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    import_meta,
    // Input codes
    r#"import.meta.env"#,
    // Output codes after transformed with plugin
    r#"process.env"#
);
