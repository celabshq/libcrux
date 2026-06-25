module Libcrux_sha3.Portable.Incremental
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

include Libcrux_sha3.Portable.Incremental.Bundle {t_Shake128Xof as t_Shake128Xof}

include Libcrux_sha3.Portable.Incremental.Bundle {t_Shake256Xof as t_Shake256Xof}

include Libcrux_sha3.Portable.Incremental.Bundle {t_Xof as t_Xof}

include Libcrux_sha3.Portable.Incremental.Bundle {f_new_pre as f_new_pre}

include Libcrux_sha3.Portable.Incremental.Bundle {f_new_post as f_new_post}

include Libcrux_sha3.Portable.Incremental.Bundle {f_new as f_new}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb_pre as f_absorb_pre}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb_post as f_absorb_post}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb as f_absorb}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb_final_pre as f_absorb_final_pre}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb_final_post as f_absorb_final_post}

include Libcrux_sha3.Portable.Incremental.Bundle {f_absorb_final as f_absorb_final}

include Libcrux_sha3.Portable.Incremental.Bundle {f_squeeze_pre as f_squeeze_pre}

include Libcrux_sha3.Portable.Incremental.Bundle {f_squeeze_post as f_squeeze_post}

include Libcrux_sha3.Portable.Incremental.Bundle {f_squeeze as f_squeeze}

include Libcrux_sha3.Portable.Incremental.Bundle {impl as impl}

include Libcrux_sha3.Portable.Incremental.Bundle {impl_1 as impl_1}

include Libcrux_sha3.Portable.Incremental.Bundle {shake128_init as shake128_init}

include Libcrux_sha3.Portable.Incremental.Bundle {shake128_absorb_final as shake128_absorb_final}

include Libcrux_sha3.Portable.Incremental.Bundle {shake128_squeeze_first_three_blocks as shake128_squeeze_first_three_blocks}

include Libcrux_sha3.Portable.Incremental.Bundle {shake128_squeeze_first_five_blocks as shake128_squeeze_first_five_blocks}

include Libcrux_sha3.Portable.Incremental.Bundle {shake128_squeeze_next_block as shake128_squeeze_next_block}

include Libcrux_sha3.Portable.Incremental.Bundle {shake256_init as shake256_init}

include Libcrux_sha3.Portable.Incremental.Bundle {shake256_absorb_final as shake256_absorb_final}

include Libcrux_sha3.Portable.Incremental.Bundle {shake256_squeeze_first_block as shake256_squeeze_first_block}

include Libcrux_sha3.Portable.Incremental.Bundle {shake256_squeeze_next_block as shake256_squeeze_next_block}
