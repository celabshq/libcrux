
-- Experimental lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
import Std.Tactic.Do.Syntax
import Utilities
import libcrux_aes

open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false

namespace libcrux_aes.platform.portable.aes_core
def SBOX : Vector (Vector (BitVec 8) 16) 16 := #v[
    #v[ 0x63#8, 0x7C#8, 0x77#8, 0x7B#8, 0xF2#8, 0x6B#8, 0x6F#8, 0xC5#8, 0x30#8, 0x01#8, 0x67#8, 0x2B#8, 0xFE#8, 0xD7#8, 0xAB#8, 0x76#8 ],
    #v[ 0xCA#8, 0x82#8, 0xC9#8, 0x7D#8, 0xFA#8, 0x59#8, 0x47#8, 0xF0#8, 0xAD#8, 0xD4#8, 0xA2#8, 0xAF#8, 0x9C#8, 0xA4#8, 0x72#8, 0xC0#8 ],
    #v[ 0xB7#8, 0xFD#8, 0x93#8, 0x26#8, 0x36#8, 0x3F#8, 0xF7#8, 0xCC#8, 0x34#8, 0xA5#8, 0xE5#8, 0xF1#8, 0x71#8, 0xD8#8, 0x31#8, 0x15#8 ],
    #v[ 0x04#8, 0xC7#8, 0x23#8, 0xC3#8, 0x18#8, 0x96#8, 0x05#8, 0x9A#8, 0x07#8, 0x12#8, 0x80#8, 0xE2#8, 0xEB#8, 0x27#8, 0xB2#8, 0x75#8 ],
    #v[ 0x09#8, 0x83#8, 0x2C#8, 0x1A#8, 0x1B#8, 0x6E#8, 0x5A#8, 0xA0#8, 0x52#8, 0x3B#8, 0xD6#8, 0xB3#8, 0x29#8, 0xE3#8, 0x2F#8, 0x84#8 ],
    #v[ 0x53#8, 0xD1#8, 0x00#8, 0xED#8, 0x20#8, 0xFC#8, 0xB1#8, 0x5B#8, 0x6A#8, 0xCB#8, 0xBE#8, 0x39#8, 0x4A#8, 0x4C#8, 0x58#8, 0xCF#8 ],
    #v[ 0xD0#8, 0xEF#8, 0xAA#8, 0xFB#8, 0x43#8, 0x4D#8, 0x33#8, 0x85#8, 0x45#8, 0xF9#8, 0x02#8, 0x7F#8, 0x50#8, 0x3C#8, 0x9F#8, 0xA8#8 ],
    #v[ 0x51#8, 0xA3#8, 0x40#8, 0x8F#8, 0x92#8, 0x9D#8, 0x38#8, 0xF5#8, 0xBC#8, 0xB6#8, 0xDA#8, 0x21#8, 0x10#8, 0xFF#8, 0xF3#8, 0xD2#8 ],
    #v[ 0xCD#8, 0x0C#8, 0x13#8, 0xEC#8, 0x5F#8, 0x97#8, 0x44#8, 0x17#8, 0xC4#8, 0xA7#8, 0x7E#8, 0x3D#8, 0x64#8, 0x5D#8, 0x19#8, 0x73#8 ],
    #v[ 0x60#8, 0x81#8, 0x4F#8, 0xDC#8, 0x22#8, 0x2A#8, 0x90#8, 0x88#8, 0x46#8, 0xEE#8, 0xB8#8, 0x14#8, 0xDE#8, 0x5E#8, 0x0B#8, 0xDB#8 ],
    #v[ 0xE0#8, 0x32#8, 0x3A#8, 0x0A#8, 0x49#8, 0x06#8, 0x24#8, 0x5C#8, 0xC2#8, 0xD3#8, 0xAC#8, 0x62#8, 0x91#8, 0x95#8, 0xE4#8, 0x79#8 ],
    #v[ 0xE7#8, 0xC8#8, 0x37#8, 0x6D#8, 0x8D#8, 0xD5#8, 0x4E#8, 0xA9#8, 0x6C#8, 0x56#8, 0xF4#8, 0xEA#8, 0x65#8, 0x7A#8, 0xAE#8, 0x08#8 ],
    #v[ 0xBA#8, 0x78#8, 0x25#8, 0x2E#8, 0x1C#8, 0xA6#8, 0xB4#8, 0xC6#8, 0xE8#8, 0xDD#8, 0x74#8, 0x1F#8, 0x4B#8, 0xBD#8, 0x8B#8, 0x8A#8 ],
    #v[ 0x70#8, 0x3E#8, 0xB5#8, 0x66#8, 0x48#8, 0x03#8, 0xF6#8, 0x0E#8, 0x61#8, 0x35#8, 0x57#8, 0xB9#8, 0x86#8, 0xC1#8, 0x1D#8, 0x9E#8 ],
    #v[ 0xE1#8, 0xF8#8, 0x98#8, 0x11#8, 0x69#8, 0xD9#8, 0x8E#8, 0x94#8, 0x9B#8, 0x1E#8, 0x87#8, 0xE9#8, 0xCE#8, 0x55#8, 0x28#8, 0xDF#8 ],
    #v[ 0x8C#8, 0xA1#8, 0x89#8, 0x0D#8, 0xBF#8, 0xE6#8, 0x42#8, 0x68#8, 0x41#8, 0x99#8, 0x2D#8, 0x0F#8, 0xB0#8, 0x54#8, 0xBB#8, 0x16#8 ]
  ]

set_option maxRecDepth 100000
def get_elem_SBOX (row col : BitVec 4) : BitVec 8 :=
  let elem := row ++ col  -- 8-bit index, 0..255
  if      elem == 0x00#8 then 0x63#8 else if elem == 0x01#8 then 0x7C#8
  else if elem == 0x02#8 then 0x77#8 else if elem == 0x03#8 then 0x7B#8
  else if elem == 0x04#8 then 0xF2#8 else if elem == 0x05#8 then 0x6B#8
  else if elem == 0x06#8 then 0x6F#8 else if elem == 0x07#8 then 0xC5#8
  else if elem == 0x08#8 then 0x30#8 else if elem == 0x09#8 then 0x01#8
  else if elem == 0x0A#8 then 0x67#8 else if elem == 0x0B#8 then 0x2B#8
  else if elem == 0x0C#8 then 0xFE#8 else if elem == 0x0D#8 then 0xD7#8
  else if elem == 0x0E#8 then 0xAB#8 else if elem == 0x0F#8 then 0x76#8
  else if elem == 0x10#8 then 0xCA#8 else if elem == 0x11#8 then 0x82#8
  else if elem == 0x12#8 then 0xC9#8 else if elem == 0x13#8 then 0x7D#8
  else if elem == 0x14#8 then 0xFA#8 else if elem == 0x15#8 then 0x59#8
  else if elem == 0x16#8 then 0x47#8 else if elem == 0x17#8 then 0xF0#8
  else if elem == 0x18#8 then 0xAD#8 else if elem == 0x19#8 then 0xD4#8
  else if elem == 0x1A#8 then 0xA2#8 else if elem == 0x1B#8 then 0xAF#8
  else if elem == 0x1C#8 then 0x9C#8 else if elem == 0x1D#8 then 0xA4#8
  else if elem == 0x1E#8 then 0x72#8 else if elem == 0x1F#8 then 0xC0#8
  else if elem == 0x20#8 then 0xB7#8 else if elem == 0x21#8 then 0xFD#8
  else if elem == 0x22#8 then 0x93#8 else if elem == 0x23#8 then 0x26#8
  else if elem == 0x24#8 then 0x36#8 else if elem == 0x25#8 then 0x3F#8
  else if elem == 0x26#8 then 0xF7#8 else if elem == 0x27#8 then 0xCC#8
  else if elem == 0x28#8 then 0x34#8 else if elem == 0x29#8 then 0xA5#8
  else if elem == 0x2A#8 then 0xE5#8 else if elem == 0x2B#8 then 0xF1#8
  else if elem == 0x2C#8 then 0x71#8 else if elem == 0x2D#8 then 0xD8#8
  else if elem == 0x2E#8 then 0x31#8 else if elem == 0x2F#8 then 0x15#8
  else if elem == 0x30#8 then 0x04#8 else if elem == 0x31#8 then 0xC7#8
  else if elem == 0x32#8 then 0x23#8 else if elem == 0x33#8 then 0xC3#8
  else if elem == 0x34#8 then 0x18#8 else if elem == 0x35#8 then 0x96#8
  else if elem == 0x36#8 then 0x05#8 else if elem == 0x37#8 then 0x9A#8
  else if elem == 0x38#8 then 0x07#8 else if elem == 0x39#8 then 0x12#8
  else if elem == 0x3A#8 then 0x80#8 else if elem == 0x3B#8 then 0xE2#8
  else if elem == 0x3C#8 then 0xEB#8 else if elem == 0x3D#8 then 0x27#8
  else if elem == 0x3E#8 then 0xB2#8 else if elem == 0x3F#8 then 0x75#8
  else if elem == 0x40#8 then 0x09#8 else if elem == 0x41#8 then 0x83#8
  else if elem == 0x42#8 then 0x2C#8 else if elem == 0x43#8 then 0x1A#8
  else if elem == 0x44#8 then 0x1B#8 else if elem == 0x45#8 then 0x6E#8
  else if elem == 0x46#8 then 0x5A#8 else if elem == 0x47#8 then 0xA0#8
  else if elem == 0x48#8 then 0x52#8 else if elem == 0x49#8 then 0x3B#8
  else if elem == 0x4A#8 then 0xD6#8 else if elem == 0x4B#8 then 0xB3#8
  else if elem == 0x4C#8 then 0x29#8 else if elem == 0x4D#8 then 0xE3#8
  else if elem == 0x4E#8 then 0x2F#8 else if elem == 0x4F#8 then 0x84#8
  else if elem == 0x50#8 then 0x53#8 else if elem == 0x51#8 then 0xD1#8
  else if elem == 0x52#8 then 0x00#8 else if elem == 0x53#8 then 0xED#8
  else if elem == 0x54#8 then 0x20#8 else if elem == 0x55#8 then 0xFC#8
  else if elem == 0x56#8 then 0xB1#8 else if elem == 0x57#8 then 0x5B#8
  else if elem == 0x58#8 then 0x6A#8 else if elem == 0x59#8 then 0xCB#8
  else if elem == 0x5A#8 then 0xBE#8 else if elem == 0x5B#8 then 0x39#8
  else if elem == 0x5C#8 then 0x4A#8 else if elem == 0x5D#8 then 0x4C#8
  else if elem == 0x5E#8 then 0x58#8 else if elem == 0x5F#8 then 0xCF#8
  else if elem == 0x60#8 then 0xD0#8 else if elem == 0x61#8 then 0xEF#8
  else if elem == 0x62#8 then 0xAA#8 else if elem == 0x63#8 then 0xFB#8
  else if elem == 0x64#8 then 0x43#8 else if elem == 0x65#8 then 0x4D#8
  else if elem == 0x66#8 then 0x33#8 else if elem == 0x67#8 then 0x85#8
  else if elem == 0x68#8 then 0x45#8 else if elem == 0x69#8 then 0xF9#8
  else if elem == 0x6A#8 then 0x02#8 else if elem == 0x6B#8 then 0x7F#8
  else if elem == 0x6C#8 then 0x50#8 else if elem == 0x6D#8 then 0x3C#8
  else if elem == 0x6E#8 then 0x9F#8 else if elem == 0x6F#8 then 0xA8#8
  else if elem == 0x70#8 then 0x51#8 else if elem == 0x71#8 then 0xA3#8
  else if elem == 0x72#8 then 0x40#8 else if elem == 0x73#8 then 0x8F#8
  else if elem == 0x74#8 then 0x92#8 else if elem == 0x75#8 then 0x9D#8
  else if elem == 0x76#8 then 0x38#8 else if elem == 0x77#8 then 0xF5#8
  else if elem == 0x78#8 then 0xBC#8 else if elem == 0x79#8 then 0xB6#8
  else if elem == 0x7A#8 then 0xDA#8 else if elem == 0x7B#8 then 0x21#8
  else if elem == 0x7C#8 then 0x10#8 else if elem == 0x7D#8 then 0xFF#8
  else if elem == 0x7E#8 then 0xF3#8 else if elem == 0x7F#8 then 0xD2#8
  else if elem == 0x80#8 then 0xCD#8 else if elem == 0x81#8 then 0x0C#8
  else if elem == 0x82#8 then 0x13#8 else if elem == 0x83#8 then 0xEC#8
  else if elem == 0x84#8 then 0x5F#8 else if elem == 0x85#8 then 0x97#8
  else if elem == 0x86#8 then 0x44#8 else if elem == 0x87#8 then 0x17#8
  else if elem == 0x88#8 then 0xC4#8 else if elem == 0x89#8 then 0xA7#8
  else if elem == 0x8A#8 then 0x7E#8 else if elem == 0x8B#8 then 0x3D#8
  else if elem == 0x8C#8 then 0x64#8 else if elem == 0x8D#8 then 0x5D#8
  else if elem == 0x8E#8 then 0x19#8 else if elem == 0x8F#8 then 0x73#8
  else if elem == 0x90#8 then 0x60#8 else if elem == 0x91#8 then 0x81#8
  else if elem == 0x92#8 then 0x4F#8 else if elem == 0x93#8 then 0xDC#8
  else if elem == 0x94#8 then 0x22#8 else if elem == 0x95#8 then 0x2A#8
  else if elem == 0x96#8 then 0x90#8 else if elem == 0x97#8 then 0x88#8
  else if elem == 0x98#8 then 0x46#8 else if elem == 0x99#8 then 0xEE#8
  else if elem == 0x9A#8 then 0xB8#8 else if elem == 0x9B#8 then 0x14#8
  else if elem == 0x9C#8 then 0xDE#8 else if elem == 0x9D#8 then 0x5E#8
  else if elem == 0x9E#8 then 0x0B#8 else if elem == 0x9F#8 then 0xDB#8
  else if elem == 0xA0#8 then 0xE0#8 else if elem == 0xA1#8 then 0x32#8
  else if elem == 0xA2#8 then 0x3A#8 else if elem == 0xA3#8 then 0x0A#8
  else if elem == 0xA4#8 then 0x49#8 else if elem == 0xA5#8 then 0x06#8
  else if elem == 0xA6#8 then 0x24#8 else if elem == 0xA7#8 then 0x5C#8
  else if elem == 0xA8#8 then 0xC2#8 else if elem == 0xA9#8 then 0xD3#8
  else if elem == 0xAA#8 then 0xAC#8 else if elem == 0xAB#8 then 0x62#8
  else if elem == 0xAC#8 then 0x91#8 else if elem == 0xAD#8 then 0x95#8
  else if elem == 0xAE#8 then 0xE4#8 else if elem == 0xAF#8 then 0x79#8
  else if elem == 0xB0#8 then 0xE7#8 else if elem == 0xB1#8 then 0xC8#8
  else if elem == 0xB2#8 then 0x37#8 else if elem == 0xB3#8 then 0x6D#8
  else if elem == 0xB4#8 then 0x8D#8 else if elem == 0xB5#8 then 0xD5#8
  else if elem == 0xB6#8 then 0x4E#8 else if elem == 0xB7#8 then 0xA9#8
  else if elem == 0xB8#8 then 0x6C#8 else if elem == 0xB9#8 then 0x56#8
  else if elem == 0xBA#8 then 0xF4#8 else if elem == 0xBB#8 then 0xEA#8
  else if elem == 0xBC#8 then 0x65#8 else if elem == 0xBD#8 then 0x7A#8
  else if elem == 0xBE#8 then 0xAE#8 else if elem == 0xBF#8 then 0x08#8
  else if elem == 0xC0#8 then 0xBA#8 else if elem == 0xC1#8 then 0x78#8
  else if elem == 0xC2#8 then 0x25#8 else if elem == 0xC3#8 then 0x2E#8
  else if elem == 0xC4#8 then 0x1C#8 else if elem == 0xC5#8 then 0xA6#8
  else if elem == 0xC6#8 then 0xB4#8 else if elem == 0xC7#8 then 0xC6#8
  else if elem == 0xC8#8 then 0xE8#8 else if elem == 0xC9#8 then 0xDD#8
  else if elem == 0xCA#8 then 0x74#8 else if elem == 0xCB#8 then 0x1F#8
  else if elem == 0xCC#8 then 0x4B#8 else if elem == 0xCD#8 then 0xBD#8
  else if elem == 0xCE#8 then 0x8B#8 else if elem == 0xCF#8 then 0x8A#8
  else if elem == 0xD0#8 then 0x70#8 else if elem == 0xD1#8 then 0x3E#8
  else if elem == 0xD2#8 then 0xB5#8 else if elem == 0xD3#8 then 0x66#8
  else if elem == 0xD4#8 then 0x48#8 else if elem == 0xD5#8 then 0x03#8
  else if elem == 0xD6#8 then 0xF6#8 else if elem == 0xD7#8 then 0x0E#8
  else if elem == 0xD8#8 then 0x61#8 else if elem == 0xD9#8 then 0x35#8
  else if elem == 0xDA#8 then 0x57#8 else if elem == 0xDB#8 then 0xB9#8
  else if elem == 0xDC#8 then 0x86#8 else if elem == 0xDD#8 then 0xC1#8
  else if elem == 0xDE#8 then 0x1D#8 else if elem == 0xDF#8 then 0x9E#8
  else if elem == 0xE0#8 then 0xE1#8 else if elem == 0xE1#8 then 0xF8#8
  else if elem == 0xE2#8 then 0x98#8 else if elem == 0xE3#8 then 0x11#8
  else if elem == 0xE4#8 then 0x69#8 else if elem == 0xE5#8 then 0xD9#8
  else if elem == 0xE6#8 then 0x8E#8 else if elem == 0xE7#8 then 0x94#8
  else if elem == 0xE8#8 then 0x9B#8 else if elem == 0xE9#8 then 0x1E#8
  else if elem == 0xEA#8 then 0x87#8 else if elem == 0xEB#8 then 0xE9#8
  else if elem == 0xEC#8 then 0xCE#8 else if elem == 0xED#8 then 0x55#8
  else if elem == 0xEE#8 then 0x28#8 else if elem == 0xEF#8 then 0xDF#8
  else if elem == 0xF0#8 then 0x8C#8 else if elem == 0xF1#8 then 0xA1#8
  else if elem == 0xF2#8 then 0x89#8 else if elem == 0xF3#8 then 0x0D#8
  else if elem == 0xF4#8 then 0xBF#8 else if elem == 0xF5#8 then 0xE6#8
  else if elem == 0xF6#8 then 0x42#8 else if elem == 0xF7#8 then 0x68#8
  else if elem == 0xF8#8 then 0x41#8 else if elem == 0xF9#8 then 0x99#8
  else if elem == 0xFA#8 then 0x2D#8 else if elem == 0xFB#8 then 0x0F#8
  else if elem == 0xFC#8 then 0xB0#8 else if elem == 0xFD#8 then 0x54#8
  else if elem == 0xFE#8 then 0xBB#8 else                        0x16#8

set_option maxHeartbeats 10000000000000
theorem get_elem_SBOX_correct (row col : Fin (2 ^ 4)) :
  get_elem_SBOX row col = SBOX[row][col] := by
  simp only [get_elem_SBOX, SBOX]
  let ⟨row_n, row_h⟩ := row
  let ⟨col_n, col_h⟩ := col
  simp only [Nat.reduceAdd, BitVec.natCast_eq_ofNat, beq_iff_eq, Nat.reducePow, Fin.getElem_fin,
    Vector.getElem_mk, List.getElem_toArray]
  match row_n with
  | n + 16 => grind
  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15  =>
    match col_n with
    | n + 16 => grind
    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 =>
      simp only [BitVec.reduceAppend, BitVec.reduceEq, ↓reduceIte, List.getElem_cons_zero,
        Vector.getElem_mk, List.getElem_toArray, List.getElem_cons_succ]

def sub_bytes_state_spec (st : Vector u16 8) : Vector u16 8 :=
  let apply_sbox (v : BitVec 8) : BitVec 8 :=
    let row := v.extractLsb 7 4
    let col := v.extractLsb 3 0
    get_elem_SBOX row col
  -- Element 0
  let e0  := get_elem st 0#4
  let st' := set_elem (Vector.replicate 8 0) 0#4  (apply_sbox e0)
  -- Element 1
  let e1  := get_elem st 1#4
  let st' := set_elem st' 1#4  (apply_sbox e1)
  -- Element 2
  let e2  := get_elem st 2#4
  let st' := set_elem st' 2#4  (apply_sbox e2)
  -- Element 3
  let e3  := get_elem st 3#4
  let st' := set_elem st' 3#4  (apply_sbox e3)
  -- Element 4
  let e4  := get_elem st 4#4
  let st' := set_elem st' 4#4  (apply_sbox e4)
  -- Element 5
  let e5  := get_elem st 5#4
  let st' := set_elem st' 5#4  (apply_sbox e5)
  -- Element 6
  let e6  := get_elem st 6#4
  let st' := set_elem st' 6#4  (apply_sbox e6)
  -- Element 7
  let e7  := get_elem st 7#4
  let st' := set_elem st' 7#4  (apply_sbox e7)
  -- Element 8
  let e8  := get_elem st 8#4
  let st' := set_elem st' 8#4  (apply_sbox e8)
  -- Element 9
  let e9  := get_elem st 9#4
  let st' := set_elem st' 9#4  (apply_sbox e9)
  -- Element 10
  let e10 := get_elem st 10#4
  let st' := set_elem st' 10#4 (apply_sbox e10)
  -- Element 11
  let e11 := get_elem st 11#4
  let st' := set_elem st' 11#4 (apply_sbox e11)
  -- Element 12
  let e12 := get_elem st 12#4
  let st' := set_elem st' 12#4 (apply_sbox e12)
  -- Element 13
  let e13 := get_elem st 13#4
  let st' := set_elem st' 13#4 (apply_sbox e13)
  -- Element 14
  let e14 := get_elem st 14#4
  let st' := set_elem st' 14#4 (apply_sbox e14)
  -- Element 15
  let e15 := get_elem st 15#4
  let st' := set_elem st' 15#4 (apply_sbox e15)
  st'

set_option maxRecDepth 1000

def get_aes_st (st : Vector u16 8) : Vector (BitVec 8) 4 :=
  #v[
    get_elem st 15,
    get_elem st 14,
    get_elem st 13,
    get_elem st 12
  ]

def apply_sbox_st (old : Vector (BitVec 8) 4) : Vector (BitVec 8) 4 :=
  Vector.map (fun a => get_elem_SBOX (a.extractLsb 7 4) (a.extractLsb 3 0)) old
--Write it better
set_option maxHeartbeats 10000000000000
set_option maxRecDepth 100000
theorem sub_bytes_correct_chunk (a0 a1 a2 a3 : BitVec 8) (st : RustArray u16 8) :
⦃ ⌜
  a0 = get_elem st.toVec 15#4 /\ a1 = get_elem st.toVec 14#4 /\
  a2 = get_elem st.toVec 13#4 /\ a3 = get_elem st.toVec 12#4
⌝ ⦄
sub_bytes_state st
⦃ ⇓ ⟨res_output⟩ =>
    ⌜
    get_elem_SBOX (a0.extractLsb 7 4) (a0.extractLsb 3 0) = get_elem res_output 15#4 /\
    get_elem_SBOX (a1.extractLsb 7 4) (a1.extractLsb 3 0) = get_elem res_output 14#4 /\
    get_elem_SBOX (a2.extractLsb 7 4) (a2.extractLsb 3 0) = get_elem res_output 13#4 /\
    get_elem_SBOX (a3.extractLsb 7 4) (a3.extractLsb 3 0) = get_elem res_output 12#4
⌝ ⦄
:= by

  unfold sub_bytes_state
  hax_mvcgen <;> simp at *
  .
    rename_auto_n 45

    simp only [var_0]
    refine ⟨?_, ?_, ?_, ?_⟩ <;>
    simp only [get_elem, get_elem_SBOX] <;>
    simp only [Nat.reduceAdd, Nat.sub_zero, beq_iff_eq, BitVec.ushiftRight_eq', BitVec.zero_or,
      ne_eq, reduceCtorEq, not_false_eq_true, Vector.getElem_set_ne, Nat.succ_ne_self,
      Vector.getElem_set_self, UInt16.toBitVec_not, UInt16.toBitVec_xor, UInt16.toBitVec_and,
      Nat.reduceEqDiff] <;>
    simp only [<- var_24, <- var_2, <- var_4, <- var_6, <- var_8, <- var_10, <- var_12, <- var_14 ] <;>
    bv_decide
  all_goals grind

end libcrux_aes.platform.portable.aes_core