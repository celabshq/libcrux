module Spec.MLKEM.Instances
#set-options "--fuel 0 --ifuel 1 --z3rlimit 30"
open FStar.Mul
open Core_models
open Spec.Utils
open Spec.MLKEM.Math
open Spec.MLKEM


(** MLKEM-768 Instantiation *)

let mlkem768_rank : rank = sz 3

#set-options "--z3rlimit 350"
let mlkem768_generate_keypair (randomness:t_Array u8 (sz 64)):
                              (t_Array u8 (sz 2400) & t_Array u8 (sz 1184)) & bool =
    ind_cca_generate_keypair mlkem768_rank randomness

let mlkem768_encapsulate (public_key: t_Array u8 (sz 1184)) (randomness: t_Array u8 (sz 32)):
                         (t_Array u8 (sz 1088) & t_Array u8 (sz 32)) & bool =
    assert (v_CPA_CIPHERTEXT_SIZE mlkem768_rank == sz 1088); 
    ind_cca_encapsulate mlkem768_rank public_key randomness

let mlkem768_decapsulate (secret_key: t_Array u8 (sz 2400)) (ciphertext: t_Array u8 (sz 1088)):
                         t_Array u8 (sz 32) & bool =
    ind_cca_decapsulate mlkem768_rank secret_key ciphertext

(** MLKEM-1024 Instantiation *)

let mlkem1024_rank = sz 4

let mlkem1024_generate_keypair (randomness:t_Array u8 (sz 64)):
                               (t_Array u8 (sz 3168) & t_Array u8 (sz 1568)) & bool =
    ind_cca_generate_keypair mlkem1024_rank randomness

let mlkem1024_encapsulate (public_key: t_Array u8 (sz 1568)) (randomness: t_Array u8 (sz 32)):
                          (t_Array u8 (sz 1568) & t_Array u8 (sz 32)) & bool  =
    assert (v_CPA_CIPHERTEXT_SIZE mlkem1024_rank == sz 1568);            
    ind_cca_encapsulate mlkem1024_rank public_key randomness

let mlkem1024_decapsulate (secret_key: t_Array u8 (sz 3168)) (ciphertext: t_Array u8 (sz 1568)):
                           t_Array u8 (sz 32) & bool =
    ind_cca_decapsulate mlkem1024_rank secret_key ciphertext

(** MLKEM-512 Instantiation *)

let mlkem512_rank : rank = sz 2

let mlkem512_generate_keypair (randomness:t_Array u8 (sz 64)):
                              (t_Array u8 (sz 1632) & t_Array u8 (sz 800)) & bool =
    ind_cca_generate_keypair mlkem512_rank randomness

let mlkem512_encapsulate (public_key: t_Array u8 (sz 800)) (randomness: t_Array u8 (sz 32)):
                         (t_Array u8 (sz 768) & t_Array u8 (sz 32)) & bool =
    assert (v_CPA_CIPHERTEXT_SIZE mlkem512_rank == sz 768);            
    ind_cca_encapsulate mlkem512_rank public_key randomness


let mlkem512_decapsulate (secret_key: t_Array u8 (sz 1632)) (ciphertext: t_Array u8 (sz 768)):
                         t_Array u8 (sz 32) & bool =
    ind_cca_decapsulate mlkem512_rank secret_key ciphertext

    

