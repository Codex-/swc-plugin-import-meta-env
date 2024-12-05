use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        transforms::testing::test,
        visit::{visit_mut_pass, VisitMut, VisitMutWith},
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
            let obj = Box::new(Ident::new_no_ctxt("process".into(), DUMMY_SP).into());
            member.obj = obj;
        }
    }
}

fn is_visiting_import_meta_env(n: &MemberExpr) -> Option<()> {
    let obj = n.obj.as_meta_prop()?;
    let prop = n.prop.as_ident()?;

    if obj.kind != MetaPropKind::ImportMeta {
        return None;
    }

    if prop.sym != "env" {
        return None;
    }

    Some(())
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(TransformVisitor))
}

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_env,
    r#"import.meta.env"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_env_prop,
    r#"import.meta.env.MODE"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    transform_import_meta_env_key,
    r#"import.meta.env["PROP"]"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    no_transform_import_meta,
    r#"import.meta"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    no_transform_import_meta_glob,
    r#"import.meta.glob"#
);
