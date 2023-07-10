use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        transforms::testing::test,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        if !n.is_member() {
            return;
        }

        let member = n.as_mut_member().unwrap();
        if is_visiting_import_meta_env(member).is_some() {
            let obj = Box::new(Ident::new("process".into(), DUMMY_SP).into());
            dbg!(&obj);
            member.obj = obj;
        }
    }
}

fn is_visiting_import_meta_env(n: &MemberExpr) -> Option<bool> {
    let obj = n.obj.as_meta_prop()?;
    let prop = n.prop.as_ident()?;

    if obj.kind != MetaPropKind::ImportMeta {
        return Some(false);
    }

    if prop.sym.to_string() != "env" {
        return Some(false);
    }

    Some(true)
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    transform_import_meta_env,
    r#"import.meta.env"#, // in
    r#"process.env"#      // out
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    transform_import_meta_env_prop,
    r#"import.meta.env.MODE"#, // in
    r#"process.env.MODE"#      // out
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    transform_import_meta_env_key,
    r#"import.meta.env["PROP"]"#, // in
    r#"process.env["PROP"]"#      // out
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    no_transform_import_meta,
    r#"import.meta"#, // in
    r#"import.meta"#  // out
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    no_transform_import_meta_glob,
    r#"import.meta.glob"#, // in
    r#"import.meta.glob"#  // out
);
