//! Computes a normalizes-to (projection) goal for inherent associated types,
//! `#![feature(inherent_associated_type)]`. Since HIR ty lowering already determines
//! which impl the IAT is being projected from, we just:
//! 1. instantiate generic parameters,
//! 2. equate the self type, and
//! 3. instantiate and register where clauses.

use rustc_type_ir::{self as ty, Interner};

use crate::infcx::SolverDelegate;
use crate::solve::{Certainty, EvalCtxt, Goal, GoalSource, QueryResult};

impl<Infcx, I> EvalCtxt<'_, Infcx>
where
    Infcx: SolverDelegate<Interner = I>,
    I: Interner,
{
    pub(super) fn normalize_inherent_associated_type(
        &mut self,
        goal: Goal<I, ty::NormalizesTo<I>>,
    ) -> QueryResult<I> {
        let tcx = self.interner();
        let inherent = goal.predicate.alias.expect_ty(tcx);

        let impl_def_id = tcx.parent(inherent.def_id);
        let impl_args = self.fresh_args_for_item(impl_def_id);

        // Equate impl header and add impl where clauses
        self.eq(
            goal.param_env,
            inherent.self_ty(),
            tcx.type_of(impl_def_id).instantiate(tcx, &impl_args),
        )?;

        // Equate IAT with the RHS of the project goal
        let inherent_args = inherent.rebase_inherent_args_onto_impl(impl_args, tcx);

        // Check both where clauses on the impl and IAT
        //
        // FIXME(-Znext-solver=coinductive): I think this should be split
        // and we tag the impl bounds with `GoalSource::ImplWhereBound`?
        // Right not this includes both the impl and the assoc item where bounds,
        // and I don't think the assoc item where-bounds are allowed to be coinductive.
        self.add_goals(
            GoalSource::Misc,
            tcx.predicates_of(inherent.def_id)
                .iter_instantiated(tcx, &inherent_args)
                .map(|pred| goal.with(tcx, pred)),
        );

        let normalized = tcx.type_of(inherent.def_id).instantiate(tcx, &inherent_args);
        self.instantiate_normalizes_to_term(goal, normalized.into());
        self.evaluate_added_goals_and_make_canonical_response(Certainty::Yes)
    }
}
