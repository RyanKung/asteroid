(function() {var implementors = {};
implementors["swc_common"] = [];
implementors["swc_ecma_utils"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/decl/struct.VarDeclarator.html\" title=\"struct swc_ecma_ast::decl::VarDeclarator\">VarDeclarator</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/ident/struct.Ident.html\" title=\"struct swc_ecma_ast::ident::Ident\">Ident</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/expr/enum.Expr.html\" title=\"enum swc_ecma_ast::expr::Expr\">Expr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ArrowExpr.html\" title=\"struct swc_ecma_ast::expr::ArrowExpr\">ArrowExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/function/struct.Function.html\" title=\"struct swc_ecma_ast::function::Function\">Function</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/class/struct.Constructor.html\" title=\"struct swc_ecma_ast::class::Constructor\">Constructor</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/typescript/enum.TsType.html\" title=\"enum swc_ecma_ast::typescript::TsType\">TsType</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/typescript/struct.TsTypeAnn.html\" title=\"struct swc_ecma_ast::typescript::TsTypeAnn\">TsTypeAnn</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/typescript/struct.TsTypeParam.html\" title=\"struct swc_ecma_ast::typescript::TsTypeParam\">TsTypeParam</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/var/struct.VarCollector.html\" title=\"struct swc_ecma_utils::var::VarCollector\">VarCollector</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::var::VarCollector"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ThisExpr.html\" title=\"struct swc_ecma_ast::expr::ThisExpr\">ThisExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.ThisVisitor.html\" title=\"struct swc_ecma_utils::ThisVisitor\">ThisVisitor</a>","synthetic":false,"types":["swc_ecma_utils::ThisVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.FnExpr.html\" title=\"struct swc_ecma_ast::expr::FnExpr\">FnExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.ThisVisitor.html\" title=\"struct swc_ecma_utils::ThisVisitor\">ThisVisitor</a>","synthetic":false,"types":["swc_ecma_utils::ThisVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/function/struct.Function.html\" title=\"struct swc_ecma_ast::function::Function\">Function</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.ThisVisitor.html\" title=\"struct swc_ecma_utils::ThisVisitor\">ThisVisitor</a>","synthetic":false,"types":["swc_ecma_utils::ThisVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/class/struct.Constructor.html\" title=\"struct swc_ecma_ast::class::Constructor\">Constructor</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.ThisVisitor.html\" title=\"struct swc_ecma_utils::ThisVisitor\">ThisVisitor</a>","synthetic":false,"types":["swc_ecma_utils::ThisVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/decl/struct.FnDecl.html\" title=\"struct swc_ecma_ast::decl::FnDecl\">FnDecl</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.ThisVisitor.html\" title=\"struct swc_ecma_utils::ThisVisitor\">ThisVisitor</a>","synthetic":false,"types":["swc_ecma_utils::ThisVisitor"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/expr/enum.Expr.html\" title=\"enum swc_ecma_ast::expr::Expr\">Expr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.IdentFinder.html\" title=\"struct swc_ecma_utils::IdentFinder\">IdentFinder</a>&lt;'_&gt;","synthetic":false,"types":["swc_ecma_utils::IdentFinder"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/decl/struct.VarDecl.html\" title=\"struct swc_ecma_ast::decl::VarDecl\">VarDecl</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.Hoister.html\" title=\"struct swc_ecma_utils::Hoister\">Hoister</a>","synthetic":false,"types":["swc_ecma_utils::Hoister"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.AssignExpr.html\" title=\"struct swc_ecma_ast::expr::AssignExpr\">AssignExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.Hoister.html\" title=\"struct swc_ecma_utils::Hoister\">Hoister</a>","synthetic":false,"types":["swc_ecma_utils::Hoister"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/pat/enum.Pat.html\" title=\"enum swc_ecma_ast::pat::Pat\">Pat</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.Hoister.html\" title=\"struct swc_ecma_utils::Hoister\">Hoister</a>","synthetic":false,"types":["swc_ecma_utils::Hoister"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/pat/struct.RestPat.html\" title=\"struct swc_ecma_ast::pat::RestPat\">RestPat</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.RestPatVisitor.html\" title=\"struct swc_ecma_utils::RestPatVisitor\">RestPatVisitor</a>","synthetic":false,"types":["swc_ecma_utils::RestPatVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ThisExpr.html\" title=\"struct swc_ecma_ast::expr::ThisExpr\">ThisExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.FnExpr.html\" title=\"struct swc_ecma_ast::expr::FnExpr\">FnExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.UnaryExpr.html\" title=\"struct swc_ecma_ast::expr::UnaryExpr\">UnaryExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.UpdateExpr.html\" title=\"struct swc_ecma_ast::expr::UpdateExpr\">UpdateExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.AssignExpr.html\" title=\"struct swc_ecma_ast::expr::AssignExpr\">AssignExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.MemberExpr.html\" title=\"struct swc_ecma_ast::expr::MemberExpr\">MemberExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.CondExpr.html\" title=\"struct swc_ecma_ast::expr::CondExpr\">CondExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.CallExpr.html\" title=\"struct swc_ecma_ast::expr::CallExpr\">CallExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.NewExpr.html\" title=\"struct swc_ecma_ast::expr::NewExpr\">NewExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.SeqExpr.html\" title=\"struct swc_ecma_ast::expr::SeqExpr\">SeqExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.TaggedTpl.html\" title=\"struct swc_ecma_ast::expr::TaggedTpl\">TaggedTpl</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ArrowExpr.html\" title=\"struct swc_ecma_ast::expr::ArrowExpr\">ArrowExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ClassExpr.html\" title=\"struct swc_ecma_ast::expr::ClassExpr\">ClassExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.YieldExpr.html\" title=\"struct swc_ecma_ast::expr::YieldExpr\">YieldExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.MetaPropExpr.html\" title=\"struct swc_ecma_ast::expr::MetaPropExpr\">MetaPropExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.AwaitExpr.html\" title=\"struct swc_ecma_ast::expr::AwaitExpr\">AwaitExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.BinExpr.html\" title=\"struct swc_ecma_ast::expr::BinExpr\">BinExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/jsx/struct.JSXMemberExpr.html\" title=\"struct swc_ecma_ast::jsx::JSXMemberExpr\">JSXMemberExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/jsx/struct.JSXNamespacedName.html\" title=\"struct swc_ecma_ast::jsx::JSXNamespacedName\">JSXNamespacedName</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/jsx/struct.JSXEmptyExpr.html\" title=\"struct swc_ecma_ast::jsx::JSXEmptyExpr\">JSXEmptyExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/jsx/struct.JSXElement.html\" title=\"struct swc_ecma_ast::jsx::JSXElement\">JSXElement</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/jsx/struct.JSXFragment.html\" title=\"struct swc_ecma_ast::jsx::JSXFragment\">JSXFragment</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/typescript/struct.TsNonNullExpr.html\" title=\"struct swc_ecma_ast::typescript::TsNonNullExpr\">TsNonNullExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/typescript/struct.TsTypeAssertion.html\" title=\"struct swc_ecma_ast::typescript::TsTypeAssertion\">TsTypeAssertion</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/typescript/struct.TsConstAssertion.html\" title=\"struct swc_ecma_ast::typescript::TsConstAssertion\">TsConstAssertion</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/ident/struct.PrivateName.html\" title=\"struct swc_ecma_ast::ident::PrivateName\">PrivateName</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.OptChainExpr.html\" title=\"struct swc_ecma_ast::expr::OptChainExpr\">OptChainExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.SpreadElement.html\" title=\"struct swc_ecma_ast::expr::SpreadElement\">SpreadElement</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/struct.Invalid.html\" title=\"struct swc_ecma_ast::Invalid\">Invalid</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/expr/enum.Expr.html\" title=\"enum swc_ecma_ast::expr::Expr\">Expr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/prop/enum.Prop.html\" title=\"enum swc_ecma_ast::prop::Prop\">Prop</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/prop/enum.PropName.html\" title=\"enum swc_ecma_ast::prop::PropName\">PropName</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.ArrayLit.html\" title=\"struct swc_ecma_ast::expr::ArrayLit\">ArrayLit</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/lit/struct.Number.html\" title=\"struct swc_ecma_ast::lit::Number\">Number</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.LiteralVisitor.html\" title=\"struct swc_ecma_utils::LiteralVisitor\">LiteralVisitor</a>","synthetic":false,"types":["swc_ecma_utils::LiteralVisitor"]},{"text":"impl&lt;'a, I:&nbsp;<a class=\"trait\" href=\"swc_ecma_utils/ident/trait.IdentLike.html\" title=\"trait swc_ecma_utils::ident::IdentLike\">IdentLike</a>&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/expr/enum.Expr.html\" title=\"enum swc_ecma_ast::expr::Expr\">Expr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.DestructuringFinder.html\" title=\"struct swc_ecma_utils::DestructuringFinder\">DestructuringFinder</a>&lt;'a, I&gt;","synthetic":false,"types":["swc_ecma_utils::DestructuringFinder"]},{"text":"impl&lt;'a, I:&nbsp;<a class=\"trait\" href=\"swc_ecma_utils/ident/trait.IdentLike.html\" title=\"trait swc_ecma_utils::ident::IdentLike\">IdentLike</a>&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"enum\" href=\"swc_ecma_ast/prop/enum.PropName.html\" title=\"enum swc_ecma_ast::prop::PropName\">PropName</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.DestructuringFinder.html\" title=\"struct swc_ecma_utils::DestructuringFinder\">DestructuringFinder</a>&lt;'a, I&gt;","synthetic":false,"types":["swc_ecma_utils::DestructuringFinder"]},{"text":"impl&lt;'a, I:&nbsp;<a class=\"trait\" href=\"swc_ecma_utils/ident/trait.IdentLike.html\" title=\"trait swc_ecma_utils::ident::IdentLike\">IdentLike</a>&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/ident/struct.Ident.html\" title=\"struct swc_ecma_ast::ident::Ident\">Ident</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.DestructuringFinder.html\" title=\"struct swc_ecma_utils::DestructuringFinder\">DestructuringFinder</a>&lt;'a, I&gt;","synthetic":false,"types":["swc_ecma_utils::DestructuringFinder"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/expr/struct.MemberExpr.html\" title=\"struct swc_ecma_ast::expr::MemberExpr\">MemberExpr</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.UsageFinder.html\" title=\"struct swc_ecma_utils::UsageFinder\">UsageFinder</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::UsageFinder"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"swc_common/fold/trait.Visit.html\" title=\"trait swc_common::fold::Visit\">Visit</a>&lt;<a class=\"struct\" href=\"swc_ecma_ast/ident/struct.Ident.html\" title=\"struct swc_ecma_ast::ident::Ident\">Ident</a>&gt; for <a class=\"struct\" href=\"swc_ecma_utils/struct.UsageFinder.html\" title=\"struct swc_ecma_utils::UsageFinder\">UsageFinder</a>&lt;'a&gt;","synthetic":false,"types":["swc_ecma_utils::UsageFinder"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()