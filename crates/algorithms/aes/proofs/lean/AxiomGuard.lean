import Lean
open Lean Elab Command

/--
`#assert_axioms foo, bar` fails unless every listed declaration rests only on
the trust base this development is willing to accept:

* Lean's three standard axioms, `propext`, `Classical.choice` and
  `Quot.sound`; and
* the per-call axioms `bv_decide` generates, named
  `<decl>._native.bv_decide.ax_<n>`. Each of those records that Lean's
  compiled LRAT checker accepted the refutation a SAT solver produced for one
  bitvector goal. `bv_decide` is how almost every proof here is discharged,
  so this is part of the method rather than an accident, but it does mean the
  solver's certificate is checked by compiled code and not by the kernel.

Anything else is rejected. Two cases matter in practice: `sorry`, which
leaves `sorryAx` behind, and `native_decide`, which asserts the result of an
arbitrary compiled `Bool` computation. Neither makes a build fail on its own,
so without this check a proof could quietly stop proving anything.
-/
syntax (name := assertAxioms) "#assert_axioms " ident,+ : command

/-- Axioms `bv_decide` emits for its own SAT certificates. -/
private def isBvDecideAxiom (n : Name) : Bool :=
  ((n.toString.splitOn "._native.bv_decide.ax_").length == 2 : Bool)

@[command_elab assertAxioms]
def elabAssertAxioms : CommandElab := fun stx => do
  let standard : List Name := [``propext, ``Classical.choice, ``Quot.sound]
  for arg in stx[1].getSepArgs do
    let name ← liftCoreM <| realizeGlobalConstNoOverload arg
    let env ← getEnv
    let (_, s) := ((CollectAxioms.collect name).run env).run {}
    let bad := s.axioms.filter fun a =>
      !(standard.contains a || isBvDecideAxiom a)
    unless bad.isEmpty do
      throwErrorAt arg
        "{name} rests on axioms outside the accepted trust base: {bad.toList}"
