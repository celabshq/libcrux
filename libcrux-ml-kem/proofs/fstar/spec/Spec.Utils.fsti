module Spec.Utils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models

val pow2_more_values (x:nat): Lemma (
  let result = pow2 x in
  match x with
  | 0 -> result == 1
  | 1 -> result == 2
  | 2 -> result == 4
  | 3 -> result == 8
  | 4 -> result == 16
  | 5 -> result == 32
  | 6 -> result == 64
  | 7 -> result == 128
  | 8 -> result == 256
  | 9 -> result == 512
  | 10 -> result == 1024
  | 11 -> result == 2048
  | 12 -> result == 4096
  | 13 -> result == 8192
  | 14 -> result == 16384
  | 15 -> result == 32768
  | 16 -> result == 65536
  | 17 -> result == 131072
  | 18 -> result == 262144
  | 19 -> result == 524288
  | 20 -> result == 1048576
  | 21 -> result == 2097152
  | 22 -> result == 4194304
  | 23 -> result == 8388608
  | 24 -> result == 16777216
  | 25 -> result == 33554432
  | 26 -> result == 67108864
  | 27 -> result == 134217728
  | 28 -> result == 268435456
  | 29 -> result == 536870912
  | 30 -> result == 1073741824
  | 31 -> result == 2147483648
  | 32 -> result == 4294967296
  | 33 -> result == 8589934592
  | 34 -> result == 17179869184
  | 35 -> result == 34359738368
  | 36 -> result == 68719476736
  | 37 -> result == 137438953472
  | 38 -> result == 274877906944
  | 39 -> result == 549755813888
  | 40 -> result == 1099511627776
  | 41 -> result == 2199023255552
  | 42 -> result == 4398046511104
  | 43 -> result == 8796093022208
  | 44 -> result == 17592186044416
  | 45 -> result == 35184372088832
  | 46 -> result == 70368744177664
  | 47 -> result == 140737488355328
  | 48 -> result == 281474976710656
  | 49 -> result == 562949953421312
  | 50 -> result == 1125899906842624
  | 51 -> result == 2251799813685248
  | 52 -> result == 4503599627370496
  | 53 -> result == 9007199254740992
  | 54 -> result == 18014398509481984
  | 55 -> result == 36028797018963968
  | 56 -> result == 72057594037927936
  | 57 -> result == 144115188075855872
  | 58 -> result == 288230376151711744
  | 59 -> result == 576460752303423488
  | 60 -> result == 1152921504606846976
  | 61 -> result == 2305843009213693952
  | 62 -> result == 4611686018427387904
  | 63 -> result == 9223372036854775808
  | 64 -> result == 18446744073709551616
  | 65 -> result == 36893488147419103232
  | 66 -> result == 73786976294838206464
  | 67 -> result == 147573952589676412928
  | 68 -> result == 295147905179352825856
  | 69 -> result == 590295810358705651712
  | 70 -> result == 1180591620717411303424
  | 71 -> result == 2361183241434822606848
  | 72 -> result == 4722366482869645213696
  | 73 -> result == 9444732965739290427392
  | 74 -> result == 18889465931478580854784
  | 75 -> result == 37778931862957161709568
  | 76 -> result == 75557863725914323419136
  | 77 -> result == 151115727451828646838272
  | 78 -> result == 302231454903657293676544
  | 79 -> result == 604462909807314587353088
  | 80 -> result == 1208925819614629174706176
  | 81 -> result == 2417851639229258349412352
  | 82 -> result == 4835703278458516698824704
  | 83 -> result == 9671406556917033397649408
  | 84 -> result == 19342813113834066795298816
  | 85 -> result == 38685626227668133590597632
  | 86 -> result == 77371252455336267181195264
  | 87 -> result == 154742504910672534362390528
  | 88 -> result == 309485009821345068724781056
  | 89 -> result == 618970019642690137449562112
  | 90 -> result == 1237940039285380274899124224
  | 91 -> result == 2475880078570760549798248448
  | 92 -> result == 4951760157141521099596496896
  | 93 -> result == 9903520314283042199192993792
  | 94 -> result == 19807040628566084398385987584
  | 95 -> result == 39614081257132168796771975168
  | 96 -> result == 79228162514264337593543950336
  | 97 -> result == 158456325028528675187087900672
  | 98 -> result == 316912650057057350374175801344
  | 99 -> result == 633825300114114700748351602688
  | 100 -> result == 1267650600228229401496703205376
  | 101 -> result == 2535301200456458802993406410752
  | 102 -> result == 5070602400912917605986812821504
  | 103 -> result == 10141204801825835211973625643008
  | 104 -> result == 20282409603651670423947251286016
  | 105 -> result == 40564819207303340847894502572032
  | 106 -> result == 81129638414606681695789005144064
  | 107 -> result == 162259276829213363391578010288128
  | 108 -> result == 324518553658426726783156020576256
  | 109 -> result == 649037107316853453566312041152512
  | 110 -> result == 1298074214633706907132624082305024
  | 111 -> result == 2596148429267413814265248164610048
  | 112 -> result == 5192296858534827628530496329220096
  | 113 -> result == 10384593717069655257060992658440192
  | 114 -> result == 20769187434139310514121985316880384
  | 115 -> result == 41538374868278621028243970633760768
  | 116 -> result == 83076749736557242056487941267521536
  | 117 -> result == 166153499473114484112975882535043072
  | 118 -> result == 332306998946228968225951765070086144
  | 119 -> result == 664613997892457936451903530140172288
  | 120 -> result == 1329227995784915872903807060280344576
  | 121 -> result == 2658455991569831745807614120560689152
  | 122 -> result == 5316911983139663491615228241121378304
  | 123 -> result == 10633823966279326983230456482242756608
  | 124 -> result == 21267647932558653966460912964485513216
  | 125 -> result == 42535295865117307932921825928971026432
  | 126 -> result == 85070591730234615865843651857942052864
  | 127 -> result == 170141183460469231731687303715884105728
  | 128 -> result == 340282366920938463463374607431768211456
  | 129 -> result == 680564733841876926926749214863536422912
  | 130 -> result == 1361129467683753853853498429727072845824
  | 131 -> result == 2722258935367507707706996859454145691648
  | 132 -> result == 5444517870735015415413993718908291383296
  | 133 -> result == 10889035741470030830827987437816582766592
  | 134 -> result == 21778071482940061661655974875633165533184
  | 135 -> result == 43556142965880123323311949751266331066368
  | 136 -> result == 87112285931760246646623899502532662132736
  | 137 -> result == 174224571863520493293247799005065324265472
  | 138 -> result == 348449143727040986586495598010130648530944
  | 139 -> result == 696898287454081973172991196020261297061888
  | 140 -> result == 1393796574908163946345982392040522594123776
  | 141 -> result == 2787593149816327892691964784081045188247552
  | 142 -> result == 5575186299632655785383929568162090376495104
  | 143 -> result == 11150372599265311570767859136324180752990208
  | 144 -> result == 22300745198530623141535718272648361505980416
  | 145 -> result == 44601490397061246283071436545296723011960832
  | 146 -> result == 89202980794122492566142873090593446023921664
  | 147 -> result == 178405961588244985132285746181186892047843328
  | 148 -> result == 356811923176489970264571492362373784095686656
  | 149 -> result == 713623846352979940529142984724747568191373312
  | 150 -> result == 1427247692705959881058285969449495136382746624
  | 151 -> result == 2854495385411919762116571938898990272765493248
  | 152 -> result == 5708990770823839524233143877797980545530986496
  | 153 -> result == 11417981541647679048466287755595961091061972992
  | 154 -> result == 22835963083295358096932575511191922182123945984
  | 155 -> result == 45671926166590716193865151022383844364247891968
  | 156 -> result == 91343852333181432387730302044767688728495783936
  | 157 -> result == 182687704666362864775460604089535377456991567872
  | 158 -> result == 365375409332725729550921208179070754913983135744
  | 159 -> result == 730750818665451459101842416358141509827966271488
  | 160 -> result == 1461501637330902918203684832716283019655932542976
  | 161 -> result == 2923003274661805836407369665432566039311865085952
  | 162 -> result == 5846006549323611672814739330865132078623730171904
  | 163 -> result == 11692013098647223345629478661730264157247460343808
  | 164 -> result == 23384026197294446691258957323460528314494920687616
  | 165 -> result == 46768052394588893382517914646921056628989841375232
  | 166 -> result == 93536104789177786765035829293842113257979682750464
  | 167 -> result == 187072209578355573530071658587684226515959365500928
  | 168 -> result == 374144419156711147060143317175368453031918731001856
  | 169 -> result == 748288838313422294120286634350736906063837462003712
  | 170 -> result == 1496577676626844588240573268701473812127674924007424
  | 171 -> result == 2993155353253689176481146537402947624255349848014848
  | 172 -> result == 5986310706507378352962293074805895248510699696029696
  | 173 -> result == 11972621413014756705924586149611790497021399392059392
  | 174 -> result == 23945242826029513411849172299223580994042798784118784
  | 175 -> result == 47890485652059026823698344598447161988085597568237568
  | 176 -> result == 95780971304118053647396689196894323976171195136475136
  | 177 -> result == 191561942608236107294793378393788647952342390272950272
  | 178 -> result == 383123885216472214589586756787577295904684780545900544
  | 179 -> result == 766247770432944429179173513575154591809369561091801088
  | 180 -> result == 1532495540865888858358347027150309183618739122183602176
  | 181 -> result == 3064991081731777716716694054300618367237478244367204352
  | 182 -> result == 6129982163463555433433388108601236734474956488734408704
  | 183 -> result == 12259964326927110866866776217202473468949912977468817408
  | 184 -> result == 24519928653854221733733552434404946937899825954937634816
  | 185 -> result == 49039857307708443467467104868809893875799651909875269632
  | 186 -> result == 98079714615416886934934209737619787751599303819750539264
  | 187 -> result == 196159429230833773869868419475239575503198607639501078528
  | 188 -> result == 392318858461667547739736838950479151006397215279002157056
  | 189 -> result == 784637716923335095479473677900958302012794430558004314112
  | 190 -> result == 1569275433846670190958947355801916604025588861116008628224
  | 191 -> result == 3138550867693340381917894711603833208051177722232017256448
  | 192 -> result == 6277101735386680763835789423207666416102355444464034512896
  | 193 -> result == 12554203470773361527671578846415332832204710888928069025792
  | 194 -> result == 25108406941546723055343157692830665664409421777856138051584
  | 195 -> result == 50216813883093446110686315385661331328818843555712276103168
  | 196 -> result == 100433627766186892221372630771322662657637687111424552206336
  | 197 -> result == 200867255532373784442745261542645325315275374222849104412672
  | 198 -> result == 401734511064747568885490523085290650630550748445698208825344
  | 199 -> result == 803469022129495137770981046170581301261101496891396417650688
  | 200 -> result == 1606938044258990275541962092341162602522202993782792835301376
  | 201 -> result == 3213876088517980551083924184682325205044405987565585670602752
  | 202 -> result == 6427752177035961102167848369364650410088811975131171341205504
  | 203 -> result == 12855504354071922204335696738729300820177623950262342682411008
  | 204 -> result == 25711008708143844408671393477458601640355247900524685364822016
  | 205 -> result == 51422017416287688817342786954917203280710495801049370729644032
  | 206 -> result == 102844034832575377634685573909834406561420991602098741459288064
  | 207 -> result == 205688069665150755269371147819668813122841983204197482918576128
  | 208 -> result == 411376139330301510538742295639337626245683966408394965837152256
  | 209 -> result == 822752278660603021077484591278675252491367932816789931674304512
  | 210 -> result == 1645504557321206042154969182557350504982735865633579863348609024
  | 211 -> result == 3291009114642412084309938365114701009965471731267159726697218048
  | 212 -> result == 6582018229284824168619876730229402019930943462534319453394436096
  | 213 -> result == 13164036458569648337239753460458804039861886925068638906788872192
  | 214 -> result == 26328072917139296674479506920917608079723773850137277813577744384
  | 215 -> result == 52656145834278593348959013841835216159447547700274555627155488768
  | 216 -> result == 105312291668557186697918027683670432318895095400549111254310977536
  | 217 -> result == 210624583337114373395836055367340864637790190801098222508621955072
  | 218 -> result == 421249166674228746791672110734681729275580381602196445017243910144
  | 219 -> result == 842498333348457493583344221469363458551160763204392890034487820288
  | 220 -> result == 1684996666696914987166688442938726917102321526408785780068975640576
  | 221 -> result == 3369993333393829974333376885877453834204643052817571560137951281152
  | 222 -> result == 6739986666787659948666753771754907668409286105635143120275902562304
  | 223 -> result == 13479973333575319897333507543509815336818572211270286240551805124608
  | 224 -> result == 26959946667150639794667015087019630673637144422540572481103610249216
  | 225 -> result == 53919893334301279589334030174039261347274288845081144962207220498432
  | 226 -> result == 107839786668602559178668060348078522694548577690162289924414440996864
  | 227 -> result == 215679573337205118357336120696157045389097155380324579848828881993728
  | 228 -> result == 431359146674410236714672241392314090778194310760649159697657763987456
  | 229 -> result == 862718293348820473429344482784628181556388621521298319395315527974912
  | 230 -> result == 1725436586697640946858688965569256363112777243042596638790631055949824
  | 231 -> result == 3450873173395281893717377931138512726225554486085193277581262111899648
  | 232 -> result == 6901746346790563787434755862277025452451108972170386555162524223799296
  | 233 -> result == 13803492693581127574869511724554050904902217944340773110325048447598592
  | 234 -> result == 27606985387162255149739023449108101809804435888681546220650096895197184
  | 235 -> result == 55213970774324510299478046898216203619608871777363092441300193790394368
  | 236 -> result == 110427941548649020598956093796432407239217743554726184882600387580788736
  | 237 -> result == 220855883097298041197912187592864814478435487109452369765200775161577472
  | 238 -> result == 441711766194596082395824375185729628956870974218904739530401550323154944
  | 239 -> result == 883423532389192164791648750371459257913741948437809479060803100646309888
  | 240 -> result == 1766847064778384329583297500742918515827483896875618958121606201292619776
  | 241 -> result == 3533694129556768659166595001485837031654967793751237916243212402585239552
  | 242 -> result == 7067388259113537318333190002971674063309935587502475832486424805170479104
  | 243 -> result == 14134776518227074636666380005943348126619871175004951664972849610340958208
  | 244 -> result == 28269553036454149273332760011886696253239742350009903329945699220681916416
  | 245 -> result == 56539106072908298546665520023773392506479484700019806659891398441363832832
  | 246 -> result == 113078212145816597093331040047546785012958969400039613319782796882727665664
  | 247 -> result == 226156424291633194186662080095093570025917938800079226639565593765455331328
  | 248 -> result == 452312848583266388373324160190187140051835877600158453279131187530910662656
  | 249 -> result == 904625697166532776746648320380374280103671755200316906558262375061821325312
  | 250 -> result == 1809251394333065553493296640760748560207343510400633813116524750123642650624
  | 251 -> result == 3618502788666131106986593281521497120414687020801267626233049500247285301248
  | 252 -> result == 7237005577332262213973186563042994240829374041602535252466099000494570602496
  | 253 -> result == 14474011154664524427946373126085988481658748083205070504932198000989141204992
  | 254 -> result == 28948022309329048855892746252171976963317496166410141009864396001978282409984
  | 255 -> result == 57896044618658097711785492504343953926634992332820282019728792003956564819968
  | _ -> True)
  [SMTPat (pow2 x)]

(** Utils *)
let map_slice #a #b
  (f:a -> b)
  (s: t_Slice a)
  = createi (length s) (fun i -> f (Seq.index s (v i)))

let map_array #a #b #len
  (f:a -> b)
  (s: t_Array a len)
  = createi (length s) (fun i -> f (Seq.index s (v i)))

let map2 #a #b #c #len
  (f:a -> b -> c)
  (x: t_Array a len) (y: t_Array b len)
  = createi (length x) (fun i -> f (Seq.index x (v i)) (Seq.index y (v i)))

let create len c = createi len (fun i -> c)

val repeati:
    #a:Type
  -> n:usize
  -> f:(i:usize{v i < v n} -> a -> a)
  -> acc0:a
  -> a

val eq_repeati0:
    #a:Type
  -> n:usize
  -> f:(i:usize{v i < v n} -> a -> a)
  -> acc0:a
  -> Lemma (repeati #a (sz 0) f acc0 == acc0)

(** Unfolding one iteration *)
val unfold_repeati:
    #a:Type
  -> n:usize
  -> f:(i:usize{v i < v n} -> a -> a)
  -> acc0:a
  -> i:usize{v i < v n}
  -> Lemma (repeati #a (i +! sz 1) f acc0 == f i (repeati #a i f acc0))



let createL len l = Rust_primitives.Hax.array_of_list len l 

let create16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 =
  let l = [v15; v14; v13; v12; v11; v10; v9; v8; v7; v6; v5; v4; v3; v2; v1; v0] in
  assert_norm (List.Tot.length l == 16);
  createL 16 l

val lemma_createL_index #a len l i :
  Lemma (Seq.index (createL #a len l) i == List.Tot.index l i)
        [SMTPat (Seq.index (createL #a len l) i)]

val lemma_create16_index #a v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 i :
  Lemma (Seq.index (create16 #a v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0) i ==
        (if i = 0 then v15 else
         if i = 1 then v14 else
         if i = 2 then v13 else
         if i = 3 then v12 else
         if i = 4 then v11 else
         if i = 5 then v10 else
         if i = 6 then v9 else
         if i = 7 then v8 else
         if i = 8 then v7 else
         if i = 9 then v6 else
         if i = 10 then v5 else
         if i = 11 then v4 else
         if i = 12 then v3 else
         if i = 13 then v2 else
         if i = 14 then v1 else
         if i = 15 then v0))
        [SMTPat (Seq.index (create16 #a v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0) i)]

val lemma_createi_index #a len f i :
  Lemma (Seq.index (createi #a len f) i == f (sz i))
        [SMTPat (Seq.index (createi #a len f) i)]

val lemma_create_index #a len c i:
  Lemma (Seq.index (create #a len c) i == c)
        [SMTPat (Seq.index (create #a len c) i)]

val lemma_bitand_properties #t (x:int_t t) :
  Lemma ((x &. ones) == x /\ (x &. mk_int #t 0) == mk_int #t 0 /\ (ones #t &. x) == x /\ (mk_int #t 0 &. x) == mk_int #t 0)

#push-options "--z3rlimit 250"
let flatten #t #n
  (#m: usize {range (v n * v m) usize_inttype})
  (x: t_Array (t_Array t m) n)
  : t_Array t (m *! n)
  = createi (m *! n) (fun i -> Seq.index (Seq.index x (v i / v m)) (v i % v m))
#pop-options

type t_Error = | Error_RejectionSampling : t_Error

type t_Result a b = 
  | Ok: a -> t_Result a b
  | Err: b -> t_Result a b

val v_G (input: t_Slice u8) : t_Array u8 (sz 64)
val v_H (input: t_Slice u8) : t_Array u8 (sz 32)
val v_PRF (v_LEN: usize{v v_LEN < pow2 32}) (input: t_Slice u8) : t_Array u8 v_LEN

val v_PRFxN (r:usize{v r == 2 \/ v r == 3 \/ v r == 4}) (v_LEN: usize{v v_LEN < pow2 32})
  (input: t_Array (t_Array u8 (sz 33)) r) : t_Array (t_Array u8 v_LEN) r

val v_J (input: t_Slice u8) : t_Array u8 (sz 32)

val v_XOF (v_LEN: usize{v v_LEN < pow2 32}) (input: t_Slice u8) : t_Array u8 v_LEN

val update_at_range_lemma #n
  (s: t_Slice 't)
  (i: Core_models.Ops.Range.t_Range (int_t n) {v i.f_start >= 0 /\ v i.f_start <= Seq.length s /\ v i.f_end <= Seq.length s})
  (x: t_Slice 't)
  : Lemma
    (requires (Seq.length x == v i.f_end - v i.f_start))
    (ensures (
      let s' = Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x in
      let len = v i.f_start in
      forall (i: nat). i < len ==> Seq.index s i == Seq.index s' i
    ))
    [SMTPat (Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x)]

/// TODO: This assumption should be moved to Rand_core.fsti in hax-lib,
/// where f_fill_bytes_pre should be defined as True.
val fill_bytes_pre_true
      (#v_Self: Type0)
      (#i0: Rand_core.t_RngCore v_Self)
      (self: v_Self)
      (bytes: t_Slice u8)
    : Lemma (i0.f_fill_bytes_pre self bytes)
      [SMTPat (i0.f_fill_bytes_pre self bytes)]

/// TODO: This assumption should be moved to Rand_core.fsti in hax-lib,
/// where f_fill_bytes_post should guarantee the output has the same length.
val fill_bytes_post_true
      (#v_Self: Type0)
      (#i0: Rand_core.t_RngCore v_Self)
      (self: v_Self)
      (bytes: t_Slice u8)
      (result: v_Self & t_Slice u8)
    : Lemma (i0.f_fill_bytes_post self bytes result /\
             Seq.length (snd result) == Seq.length bytes)
      [SMTPat (i0.f_fill_bytes_post self bytes result)]

/// TODO: This assumption should be moved to Core_models.Num in hax-lib.
val impl_i16__abs_value (x: i16)
    : Lemma (requires v x > -32768)
            (ensures v (Core_models.Num.impl_i16__abs x) == Prims.abs (v x))
      [SMTPat (Core_models.Num.impl_i16__abs x)]

/// TODO: This assumption should be moved to Core_models proof-libs.
/// The extracted `unwrap(try_into(array.[0..16]))` should reduce to `array`
/// when `len array == 16`.
val slice_to_array_id (array: t_Slice 'a)
    : Lemma (requires Seq.length array == 16)
            (ensures Core_models.Result.impl__unwrap
              #(t_Array 'a (mk_usize 16))
              #Core_models.Array.t_TryFromSliceError
              (Core_models.Convert.f_try_into #(t_Slice 'a)
                #(t_Array 'a (mk_usize 16))
                #FStar.Tactics.Typeclasses.solve
                array) == array)

/// Bounded integers

let is_intb (l:nat) (x:int) = (x <= l) && (x >= -l)
let is_i16b (l:nat) (x:i16) = is_intb l (v x)
let is_i16b_array (l:nat) (x:t_Slice i16) = forall i. i < Seq.length x ==> is_i16b l (Seq.index x i)
let is_i16b_vector (l:nat) (r:usize) (x:t_Array (t_Array i16 (sz 256)) r) = forall i. i < v r ==> is_i16b_array l (Seq.index x i)
let is_i16b_matrix (l:nat) (r:usize) (x:t_Array (t_Array (t_Array i16 (sz 256)) r) r) = forall i. i < v r ==> is_i16b_vector l r (Seq.index x i)

[@ "opaque_to_smt"]
let is_i16b_array_opaque (l:nat) (x:t_Slice i16) = is_i16b_array l x

let is_i32b (l:nat) (x:i32) = is_intb l (v x)
let is_i32b_array (l:nat) (x:t_Slice i32) = forall i. i < Seq.length x ==> is_i32b l (Seq.index x i)

[@ "opaque_to_smt"]
let is_i32b_array_opaque (l:nat) (x:t_Slice i32) = is_i32b_array l x
let is_i32b_array_larger (l:nat) (l':nat) (x:t_Slice i32):
  Lemma (is_i32b_array_opaque l x /\ l <= l' ==> is_i32b_array_opaque l' x)
  = reveal_opaque (`%is_i32b_array_opaque) (is_i32b_array_opaque)

// Half-open `(-l, l]` analogue of `is_i32b` / `is_i32b_array_opaque`.
// Mathematically: -l < x <= l (strict on the lower bound, inclusive on the upper).
// Used by trait pres/posts that mirror FIPS 204's centered modulo `mod^±`,
// which produces `r0 ∈ (-m/2, m/2]` for `m = 2^d` (Algorithms 35 / 36).
let is_intb_strict_lower (l:nat) (x:int) = (x <= l) && (x > -l)
let is_i32b_strict_lower (l:nat) (x:i32) = is_intb_strict_lower l (v x)
let is_i32b_strict_lower_array (l:nat) (x:t_Slice i32) =
  forall i. i < Seq.length x ==> is_i32b_strict_lower l (Seq.index x i)

[@ "opaque_to_smt"]
let is_i32b_strict_lower_array_opaque (l:nat) (x:t_Slice i32) = is_i32b_strict_lower_array l x

// `strict_lower` implies the closed centered form (any `x` with `-l < x <= l`
// also has `-l <= x <= l`).  SMTPat triggers on the strict-lower form so callers
// expecting the old (closed) predicate keep discharging when given the new one.
let lemma_is_i32b_strict_lower_implies_array_opaque (l:nat) (x:t_Slice i32):
  Lemma (is_i32b_strict_lower_array_opaque l x ==> is_i32b_array_opaque l x)
        [SMTPat (is_i32b_strict_lower_array_opaque l x)]
  = reveal_opaque (`%is_i32b_strict_lower_array_opaque) (is_i32b_strict_lower_array_opaque);
    reveal_opaque (`%is_i32b_array_opaque) (is_i32b_array_opaque)

let is_i32b_strict_lower_array_larger (l:nat) (l':nat) (x:t_Slice i32):
  Lemma (is_i32b_strict_lower_array_opaque l x /\ l <= l' ==>
         is_i32b_strict_lower_array_opaque l' x)
  = reveal_opaque (`%is_i32b_strict_lower_array_opaque) (is_i32b_strict_lower_array_opaque)


let is_i64b (l:nat) (x:i64) = is_intb l (v x)

let nat_div_ceil (x:nat) (y:pos) : nat = if (x % y = 0) then x/y else (x/y)+1

val lemma_intb_le b b'
  : Lemma (requires (b <= b'))
          (ensures (forall n. is_intb b n ==> is_intb b' n))
          
val lemma_add_intb (b1 b2: nat) (n1 n2: int) 
    : Lemma (requires (is_intb b1 n1 /\ is_intb b2 n2))
      (ensures (is_intb (b1 + b2) (n1 + n2)))

val lemma_add_intb_forall (b1 b2: nat)
    : Lemma (forall n1 n2. (is_intb b1 n1 /\ is_intb b2 n2) ==> is_intb (b1 + b2) (n1 + n2))

val lemma_sub_intb (b1 b2: nat) (n1 n2: int) 
    : Lemma (requires (is_intb b1 n1 /\ is_intb b2 n2))
      (ensures (is_intb (b1 + b2) (n1 - n2)))

val lemma_sub_intb_forall (b1 b2: nat)
    : Lemma (forall n1 n2. (is_intb b1 n1 /\ is_intb b2 n2) ==> is_intb (b1 + b2) (n1 - n2))

#push-options "--z3rlimit 200"
val lemma_mul_intb (b1 b2: nat) (n1 n2: int) 
    : Lemma (requires (is_intb b1 n1 /\ is_intb b2 n2))
      (ensures (is_intb (b1 * b2) (n1 * n2)))
#pop-options

val lemma_mul_intb_forall (b1 b2: nat)
    : Lemma (forall n1 n2. (is_intb b1 n1 /\ is_intb b2 n2) ==> is_intb (b1 * b2) (n1 * n2))

#push-options "--z3rlimit 200"
val lemma_mul_i16b (b1 b2: nat) (n1 n2: i16) 
    : Lemma (requires (is_i16b b1 n1 /\ is_i16b b2 n2 /\ b1 * b2 < pow2 31))
      (ensures (range (v n1 * v n2) i32_inttype /\ 
                is_i32b (b1 * b2) ((cast n1 <: i32) *! (cast n2 <: i32)) /\
                v ((cast n1 <: i32) *! (cast n2 <: i32)) == v n1 * v n2))
#pop-options

#push-options "--z3rlimit 200"
val lemma_mul_i32b (b1 b2: nat) (n1 n2: i32) 
    : Lemma (requires (is_i32b b1 n1 /\ is_i32b b2 n2 /\ b1 * b2 < pow2 63))
      (ensures (range (v n1 * v n2) i64_inttype /\ 
                is_i64b (b1 * b2) ((cast n1 <: i64) *! (cast n2 <: i64)) /\
                v ((cast n1 <: i64) *! (cast n2 <: i64)) == v n1 * v n2))
#pop-options

val lemma_add_i16b (b1 b2:nat) (n1 n2:i16) :
  Lemma (requires (is_i16b b1 n1 /\ is_i16b b2 n2 /\ b1 + b2 < pow2 15))
        (ensures (range (v n1 + v n2) i16_inttype /\
                  is_i16b (b1 + b2) (n1 +! n2)))

val lemma_range_at_percent (v:int) (p:int{p>0/\ p%2=0 /\ v < p/2 /\ v >= -p / 2}):
  Lemma (v @% p == v)

val lemma_sub_i16b (b1 b2:nat) (n1 n2:i16) :
  Lemma (requires (is_i16b b1 n1 /\ is_i16b b2 n2 /\ b1 + b2 < pow2 15))
        (ensures (range (v n1 - v n2) i16_inttype /\
                  is_i16b (b1 + b2) (n1 -. n2) /\
                  v (n1 -. n2) == v n1 - v n2))

let mont_mul_red_i16 (x:i16) (y:i16) : i16=
  let vlow = x *. y in
  let k = vlow *. (neg (mk_i16 3327)) in
  let k_times_modulus = cast (((cast k <: i32) *. (mk_i32 3329)) >>! (mk_i32 16)) <: i16 in
  let vhigh = cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16 in
  vhigh -. k_times_modulus

let mont_red_i32 (x:i32) : i16 =
  let vlow = cast x <: i16 in
  let k = vlow *. (neg (mk_i16 3327)) in
  let k_times_modulus = cast (((cast k <: i32) *. (mk_i32 3329)) >>! (mk_i32 16)) <: i16 in
  let vhigh = cast (x >>! (mk_i32 16)) <: i16 in
  vhigh -. k_times_modulus

val lemma_at_percent_mod (v:int) (p:int{p>0/\ p%2=0}):
  Lemma ((v @% p) % p == v % p)

val lemma_div_at_percent (v:int) (p:int{p>0/\ p%2=0 /\ (v/p) < p/2 /\ (v/p) >= -p / 2}):
  Lemma ((v / p) @% p == v / p)

val lemma_mont_red_i32 (x:i32): Lemma
  (requires (is_i32b (3328 * pow2 16) x))
  (ensures (
          let result:i16 = mont_red_i32 x in
          is_i16b (3328 + 1665) result /\
          (is_i32b (3328 * pow2 15) x ==> is_i16b 3328 result) /\
          v result % 3329 == (v x * 169) % 3329))

val lemma_mont_mul_red_i16_int (x y:i16): Lemma
  (requires (is_intb (3326 * pow2 15) (v x * v y)))
  (ensures (
          let result:i16 = mont_mul_red_i16 x y in
          is_i16b 3328 result /\
          v result % 3329 == (v x * v y * 169) % 3329))

val lemma_mont_mul_red_i16 (x y:i16): Lemma
  (requires (is_i16b 1664 y \/ is_intb (3326 * pow2 15) (v x * v y)))
  (ensures (
          let result:i16 = mont_mul_red_i16 x y in
          is_i16b 3328 result /\
          v result % 3329 == (v x * v y * 169) % 3329))
          [SMTPat (mont_mul_red_i16 x y)]
 
let barrett_red (x:i16) = 
  let t1 = cast (((cast x <: i32) *. (cast (mk_i16 20159) <: i32)) >>! (mk_i32 16)) <: i16 in
  let t2 = t1 +. (mk_i16 512) in
  let q = t2 >>! (mk_i32 10) in
  let qm = q *. (mk_i16 3329) in
  x -. qm

val lemma_barrett_red (x:i16) : Lemma
   (requires (is_i16b 28296 x))
   (ensures (let result = barrett_red x in
             is_i16b 3328 result /\
             v result % 3329 == v x % 3329)) 
   [SMTPat (barrett_red x)]

let logand_zero_lemma (a:i16):
  Lemma (((mk_i16 0) &. a) == mk_i16 0)
        [SMTPat (logand (mk_i16 0) a)] =
        logand_lemma a a

let logand_ones_lemma (a:i16):
  Lemma (((mk_i16 (-1)) &. a) == a)
        [SMTPat (logand (mk_i16 (-1)) a)] =
        logand_lemma a a

let cond_sub (x:i16) =
  let xm = x -. (mk_i16 3329) in
  let mask = xm >>! (mk_i32 15) in
  let mm = mask &. (mk_i16 3329) in
  xm +. mm

val lemma_cond_sub x:
  Lemma 
    (requires (is_i16b (pow2 12 - 1) x))
    (ensures (let r = cond_sub x in
              if x >=. (mk_i16 3329) then r == x -! (mk_i16 3329) else r == x))
    [SMTPat (cond_sub x)]

val lemma_shift_right_15_i16 (x:i16):
  Lemma (if v x >= 0 then (x >>! (mk_i32 15)) == mk_i16 0 else (x >>! (mk_i32 15)) == (mk_i16 (-1)))

let ntt_spec #len (vec_in: t_Array i16 len) (zeta: int) (i: nat{i < v len}) (j: nat{j < v len}) 
                  (vec_out: t_Array i16 len) : Type0 =
  ((v (Seq.index vec_out i) % 3329) ==
   ((v (Seq.index vec_in i) + (v (Seq.index vec_in j) * zeta * 169)) % 3329)) /\
  ((v (Seq.index vec_out j) % 3329) ==
   ((v (Seq.index vec_in i) - (v (Seq.index vec_in j) * zeta * 169)) % 3329))

let inv_ntt_spec #len (vec_in: t_Array i16 len) (zeta: int) (i: nat{i < v len}) (j: nat{j < v len})
                 (vec_out: t_Array i16 len) : Type0 =
  ((v (Seq.index vec_out i) % 3329) ==
   ((v (Seq.index vec_in j) + v (Seq.index vec_in i)) % 3329)) /\
  ((v (Seq.index vec_out j) % 3329) ==
   (((v (Seq.index vec_in j) - v (Seq.index vec_in i)) * zeta * 169) % 3329))

(* Opaque wrappers that bundle the 8 butterfly ntt_specs / inv_ntt_specs
   for within-chunk layers so callers don't pay the cost of an 8-conjunct
   post in their SMT context unless they reveal it.  The within-chunk
   pattern at N = 16 uses len ∈ {2, 4, 8}; zetas are 4 / 2 / 1 per layer.
   Layer-1 commute proofs reveal these in a small local scope. *)
[@@ "opaque_to_smt"]
let ntt_layer_1_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0 z1 z2 z3: i16) : Type0 =
  ntt_spec vec_in (v z0) 0 2 vec_out /\ ntt_spec vec_in (v z0) 1 3 vec_out /\
  ntt_spec vec_in (v z1) 4 6 vec_out /\ ntt_spec vec_in (v z1) 5 7 vec_out /\
  ntt_spec vec_in (v z2) 8 10 vec_out /\ ntt_spec vec_in (v z2) 9 11 vec_out /\
  ntt_spec vec_in (v z3) 12 14 vec_out /\ ntt_spec vec_in (v z3) 13 15 vec_out

[@@ "opaque_to_smt"]
let ntt_layer_2_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0 z1: i16) : Type0 =
  ntt_spec vec_in (v z0) 0 4 vec_out /\ ntt_spec vec_in (v z0) 1 5 vec_out /\
  ntt_spec vec_in (v z0) 2 6 vec_out /\ ntt_spec vec_in (v z0) 3 7 vec_out /\
  ntt_spec vec_in (v z1) 8 12 vec_out /\ ntt_spec vec_in (v z1) 9 13 vec_out /\
  ntt_spec vec_in (v z1) 10 14 vec_out /\ ntt_spec vec_in (v z1) 11 15 vec_out

[@@ "opaque_to_smt"]
let ntt_layer_3_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0: i16) : Type0 =
  ntt_spec vec_in (v z0) 0 8 vec_out /\ ntt_spec vec_in (v z0) 1 9 vec_out /\
  ntt_spec vec_in (v z0) 2 10 vec_out /\ ntt_spec vec_in (v z0) 3 11 vec_out /\
  ntt_spec vec_in (v z0) 4 12 vec_out /\ ntt_spec vec_in (v z0) 5 13 vec_out /\
  ntt_spec vec_in (v z0) 6 14 vec_out /\ ntt_spec vec_in (v z0) 7 15 vec_out

[@@ "opaque_to_smt"]
let inv_ntt_layer_1_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0 z1 z2 z3: i16) : Type0 =
  inv_ntt_spec vec_in (v z0) 0 2 vec_out /\ inv_ntt_spec vec_in (v z0) 1 3 vec_out /\
  inv_ntt_spec vec_in (v z1) 4 6 vec_out /\ inv_ntt_spec vec_in (v z1) 5 7 vec_out /\
  inv_ntt_spec vec_in (v z2) 8 10 vec_out /\ inv_ntt_spec vec_in (v z2) 9 11 vec_out /\
  inv_ntt_spec vec_in (v z3) 12 14 vec_out /\ inv_ntt_spec vec_in (v z3) 13 15 vec_out

[@@ "opaque_to_smt"]
let inv_ntt_layer_2_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0 z1: i16) : Type0 =
  inv_ntt_spec vec_in (v z0) 0 4 vec_out /\ inv_ntt_spec vec_in (v z0) 1 5 vec_out /\
  inv_ntt_spec vec_in (v z0) 2 6 vec_out /\ inv_ntt_spec vec_in (v z0) 3 7 vec_out /\
  inv_ntt_spec vec_in (v z1) 8 12 vec_out /\ inv_ntt_spec vec_in (v z1) 9 13 vec_out /\
  inv_ntt_spec vec_in (v z1) 10 14 vec_out /\ inv_ntt_spec vec_in (v z1) 11 15 vec_out

[@@ "opaque_to_smt"]
let inv_ntt_layer_3_butterfly_post
      (vec_in vec_out: t_Array i16 (mk_usize 16))
      (z0: i16) : Type0 =
  inv_ntt_spec vec_in (v z0) 0 8 vec_out /\ inv_ntt_spec vec_in (v z0) 1 9 vec_out /\
  inv_ntt_spec vec_in (v z0) 2 10 vec_out /\ inv_ntt_spec vec_in (v z0) 3 11 vec_out /\
  inv_ntt_spec vec_in (v z0) 4 12 vec_out /\ inv_ntt_spec vec_in (v z0) 5 13 vec_out /\
  inv_ntt_spec vec_in (v z0) 6 14 vec_out /\ inv_ntt_spec vec_in (v z0) 7 15 vec_out

(* Residue pair for one base-case binomial multiply at index k (k < 8).
   `zeta` is the full int zeta for that binomial — callers pass `v zeta_i`
   for positive entries and `- v zeta_i` for the negated ones, paralleling
   the 8-entry zetas sequence `[z0,-z0,z1,-z1,z2,-z2,z3,-z3]`.
   Marked `unfold` so consumers of the butterfly_post see the residue
   bodies directly without fuel-based unfolding. *)
unfold let ntt_multiply_spec
      (lhs rhs: t_Array i16 (mk_usize 16))
      (zeta: int) (k: nat{k < 8})
      (result: t_Array i16 (mk_usize 16)) : Type0 =
  let ai = Seq.index lhs (2 * k) in
  let aj = Seq.index lhs (2 * k + 1) in
  let bi = Seq.index rhs (2 * k) in
  let bj = Seq.index rhs (2 * k + 1) in
  let oi = Seq.index result (2 * k) in
  let oj = Seq.index result (2 * k + 1) in
  ((v oi % 3329) == (((v ai * v bi + v aj * v bj * zeta * 169) * 169) % 3329)) /\
  ((v oj % 3329) == (((v ai * v bj + v aj * v bi) * 169) % 3329))

(* i16 negation — kept as a named helper because inline `mk_i16 0 -! x`
   doesn't survive hax's `fstar_prop_expr!` parser.  Made total by
   guarding `i16::MIN` (whose negation overflows i16); every actual
   caller passes a zeta or a bounded value, so the guard's `else`
   branch is unreachable in practice but lets the helper appear in
   trait posts without propagating a refinement. *)
unfold let neg_i16 (x: i16) : i16 =
  if v x > - pow2 15 then mk_i16 0 -! x else mk_i16 0

(* Opaque wrapper bundling the 8 `ntt_multiply_spec` residues — one per
   binomial pair — so the trait post can cite them as a single conjunct.
   Matches the `[z0, -z0, z1, -z1, z2, -z2, z3, -z3]` ordering that the
   impl iterates over. *)
[@@ "opaque_to_smt"]
let ntt_multiply_butterfly_post
      (lhs rhs result: t_Array i16 (mk_usize 16))
      (z0 z1 z2 z3: i16) : Type0 =
  ntt_multiply_spec lhs rhs (v z0)      0 result /\
  ntt_multiply_spec lhs rhs (- (v z0))  1 result /\
  ntt_multiply_spec lhs rhs (v z1)      2 result /\
  ntt_multiply_spec lhs rhs (- (v z1))  3 result /\
  ntt_multiply_spec lhs rhs (v z2)      4 result /\
  ntt_multiply_spec lhs rhs (- (v z2))  5 result /\
  ntt_multiply_spec lhs rhs (v z3)      6 result /\
  ntt_multiply_spec lhs rhs (- (v z3))  7 result


(* Wrap-around modulo: wraps into ]-p/2; p/2] *)
let mod_p (v:int) (p:int{p>0/\ p%2=0}) : Tot int =
  let m = v % p in if m > p/2 then m - p else m

let is_intb_bt (l:nat) (x:int) = (x > -l) && (x <= l)

unfold let forall4 (p:(x:nat{x < 4} -> Type0)) =
    p 0  /\ p 1  /\ p 2  /\ p 3
    
unfold let forall8 (p:(x:nat{x < 8} -> Type0)) =
    p 0  /\ p 1  /\ p 2  /\ p 3  /\
    p 4  /\ p 5  /\ p 6  /\ p 7 

unfold let forall16 (p:(x:nat{x < 16} -> Type0)) =
    p 0  /\ p 1  /\ p 2  /\ p 3  /\
    p 4  /\ p 5  /\ p 6  /\ p 7  /\
    p 8  /\ p 9  /\ p 10 /\ p 11 /\
    p 12 /\ p 13 /\ p 14 /\ p 15

unfold let forall32 (p:(x:nat{x < 32} -> Type0)) =
    p 0  /\ p 1  /\ p 2  /\ p 3  /\
    p 4  /\ p 5  /\ p 6  /\ p 7  /\
    p 8  /\ p 9  /\ p 10 /\ p 11 /\
    p 12 /\ p 13 /\ p 14 /\ p 15 /\
    p 16 /\ p 17 /\ p 18 /\ p 19 /\
    p 20 /\ p 21 /\ p 22 /\ p 23 /\
    p 24 /\ p 25 /\ p 26 /\ p 27 /\
    p 28 /\ p 29 /\ p 30 /\ p 31

let modifies1_8 #t
    (a b: t_Array t (sz 8))
    (i:usize{v i < 8}) = 
    ((v i <> 0)  ==> Seq.index a 0 == Seq.index b 0) /\
    ((v i <> 1)  ==> Seq.index a 1 == Seq.index b 1) /\
    ((v i <> 2)  ==> Seq.index a 2 == Seq.index b 2) /\
    ((v i <> 3)  ==> Seq.index a 3 == Seq.index b 3) /\
    ((v i <> 4)  ==> Seq.index a 4 == Seq.index b 4) /\
    ((v i <> 5)  ==> Seq.index a 5 == Seq.index b 5) /\
    ((v i <> 6)  ==> Seq.index a 6 == Seq.index b 6) /\
    ((v i <> 7)  ==> Seq.index a 7 == Seq.index b 7)

let modifies2_8 #t
    (a b: t_Array t (sz 8))
    (i:usize{v i < 8}) (j:usize{v j < 8}) =
    ((v i <> 0 /\ v j <> 0)  ==> Seq.index a 0 == Seq.index b 0) /\
    ((v i <> 1 /\ v j <> 1)  ==> Seq.index a 1 == Seq.index b 1) /\
    ((v i <> 2 /\ v j <> 2)  ==> Seq.index a 2 == Seq.index b 2) /\
    ((v i <> 3 /\ v j <> 3)  ==> Seq.index a 3 == Seq.index b 3) /\
    ((v i <> 4 /\ v j <> 4)  ==> Seq.index a 4 == Seq.index b 4) /\
    ((v i <> 5 /\ v j <> 5)  ==> Seq.index a 5 == Seq.index b 5) /\
    ((v i <> 6 /\ v j <> 6)  ==> Seq.index a 6 == Seq.index b 6) /\
    ((v i <> 7 /\ v j <> 7)  ==> Seq.index a 7 == Seq.index b 7)

let modifies1_16 #t
    (a b: t_Array t (sz 16))
    (i:usize{v i < 16}) = 
    (v i <> 0  ==> Seq.index a 0 == Seq.index b 0) /\
    (v i <> 1  ==> Seq.index a 1 == Seq.index b 1) /\
    (v i <> 2  ==> Seq.index a 2 == Seq.index b 2) /\
    (v i <> 3  ==> Seq.index a 3 == Seq.index b 3) /\
    (v i <> 4  ==> Seq.index a 4 == Seq.index b 4) /\
    (v i <> 5  ==> Seq.index a 5 == Seq.index b 5) /\
    (v i <> 6  ==> Seq.index a 6 == Seq.index b 6) /\
    (v i <> 7  ==> Seq.index a 7 == Seq.index b 7) /\
    (v i <> 8  ==> Seq.index a 8 == Seq.index b 8) /\
    (v i <> 9  ==> Seq.index a 9 == Seq.index b 9) /\
    (v i <> 10 ==> Seq.index a 10 == Seq.index b 10) /\
    (v i <> 11 ==> Seq.index a 11 == Seq.index b 11) /\
    (v i <> 12 ==> Seq.index a 12 == Seq.index b 12) /\
    (v i <> 13 ==> Seq.index a 13 == Seq.index b 13) /\
    (v i <> 14 ==> Seq.index a 14 == Seq.index b 14) /\
    (v i <> 15 ==> Seq.index a 15 == Seq.index b 15)

let modifies2_16 #t
    (a b: t_Array t (sz 16))
    (i:usize{v i < 16}) (j:usize{v j < 16}) =
    ((v i <> 0  /\ v j <> 0)  ==> Seq.index a 0 == Seq.index b 0) /\
    ((v i <> 1  /\ v j <> 1)  ==> Seq.index a 1 == Seq.index b 1) /\
    ((v i <> 2  /\ v j <> 2)  ==> Seq.index a 2 == Seq.index b 2) /\
    ((v i <> 3  /\ v j <> 3)  ==> Seq.index a 3 == Seq.index b 3) /\
    ((v i <> 4  /\ v j <> 4)  ==> Seq.index a 4 == Seq.index b 4) /\
    ((v i <> 5  /\ v j <> 5)  ==> Seq.index a 5 == Seq.index b 5) /\
    ((v i <> 6  /\ v j <> 6)  ==> Seq.index a 6 == Seq.index b 6) /\
    ((v i <> 7  /\ v j <> 7)  ==> Seq.index a 7 == Seq.index b 7) /\
    ((v i <> 8  /\ v j <> 8)  ==> Seq.index a 8 == Seq.index b 8) /\
    ((v i <> 9  /\ v j <> 9)  ==> Seq.index a 9 == Seq.index b 9) /\
    ((v i <> 10 /\ v j <> 10) ==> Seq.index a 10 == Seq.index b 10) /\
    ((v i <> 11 /\ v j <> 11) ==> Seq.index a 11 == Seq.index b 11) /\
    ((v i <> 12 /\ v j <> 12) ==> Seq.index a 12 == Seq.index b 12) /\
    ((v i <> 13 /\ v j <> 13) ==> Seq.index a 13 == Seq.index b 13) /\
    ((v i <> 14 /\ v j <> 14) ==> Seq.index a 14 == Seq.index b 14) /\
    ((v i <> 15 /\ v j <> 15) ==> Seq.index a 15 == Seq.index b 15)

let modifies1_32 #t
        (a b: t_Array t (mk_usize 32))
        (j:usize{v j < 32}) =
    // TODO: find some way to expand this from a smaller spec, e.g.:
    // normalize_term (Spec.Utils.forall32 (fun x -> v j <> x ==> Seq.index a x == Seq.index b x))
    (v j <> 0  ==> Seq.index a 0 == Seq.index b 0) /\
    (v j <> 1  ==> Seq.index a 1 == Seq.index b 1) /\
    (v j <> 2  ==> Seq.index a 2 == Seq.index b 2) /\
    (v j <> 3  ==> Seq.index a 3 == Seq.index b 3) /\
    (v j <> 4  ==> Seq.index a 4 == Seq.index b 4) /\
    (v j <> 5  ==> Seq.index a 5 == Seq.index b 5) /\
    (v j <> 6  ==> Seq.index a 6 == Seq.index b 6) /\
    (v j <> 7  ==> Seq.index a 7 == Seq.index b 7) /\
    (v j <> 8  ==> Seq.index a 8 == Seq.index b 8) /\
    (v j <> 9  ==> Seq.index a 9 == Seq.index b 9) /\
    (v j <> 10 ==> Seq.index a 10 == Seq.index b 10) /\
    (v j <> 11 ==> Seq.index a 11 == Seq.index b 11) /\
    (v j <> 12 ==> Seq.index a 12 == Seq.index b 12) /\
    (v j <> 13 ==> Seq.index a 13 == Seq.index b 13) /\
    (v j <> 14 ==> Seq.index a 14 == Seq.index b 14) /\
    (v j <> 15 ==> Seq.index a 15 == Seq.index b 15) /\
    (v j <> 16 ==> Seq.index a 16 == Seq.index b 16) /\
    (v j <> 17 ==> Seq.index a 17 == Seq.index b 17) /\
    (v j <> 18 ==> Seq.index a 18 == Seq.index b 18) /\
    (v j <> 19 ==> Seq.index a 19 == Seq.index b 19) /\
    (v j <> 20 ==> Seq.index a 20 == Seq.index b 20) /\
    (v j <> 21 ==> Seq.index a 21 == Seq.index b 21) /\
    (v j <> 22 ==> Seq.index a 22 == Seq.index b 22) /\
    (v j <> 23 ==> Seq.index a 23 == Seq.index b 23) /\
    (v j <> 24 ==> Seq.index a 24 == Seq.index b 24) /\
    (v j <> 25 ==> Seq.index a 25 == Seq.index b 25) /\
    (v j <> 26 ==> Seq.index a 26 == Seq.index b 26) /\
    (v j <> 27 ==> Seq.index a 27 == Seq.index b 27) /\
    (v j <> 28 ==> Seq.index a 28 == Seq.index b 28) /\
    (v j <> 29 ==> Seq.index a 29 == Seq.index b 29) /\
    (v j <> 30 ==> Seq.index a 30 == Seq.index b 30) /\
    (v j <> 31 ==> Seq.index a 31 == Seq.index b 31)

let modifies2_32 #t
        (a b: t_Array t (mk_usize 32))
        (i j:(n:usize{v n < 32})) =
    ((v i <> 0  /\ v j <> 0)  ==> Seq.index a 0 == Seq.index b 0) /\
    ((v i <> 1  /\ v j <> 1)  ==> Seq.index a 1 == Seq.index b 1) /\
    ((v i <> 2  /\ v j <> 2)  ==> Seq.index a 2 == Seq.index b 2) /\
    ((v i <> 3  /\ v j <> 3)  ==> Seq.index a 3 == Seq.index b 3) /\
    ((v i <> 4  /\ v j <> 4)  ==> Seq.index a 4 == Seq.index b 4) /\
    ((v i <> 5  /\ v j <> 5)  ==> Seq.index a 5 == Seq.index b 5) /\
    ((v i <> 6  /\ v j <> 6)  ==> Seq.index a 6 == Seq.index b 6) /\
    ((v i <> 7  /\ v j <> 7)  ==> Seq.index a 7 == Seq.index b 7) /\
    ((v i <> 8  /\ v j <> 8)  ==> Seq.index a 8 == Seq.index b 8) /\
    ((v i <> 9  /\ v j <> 9)  ==> Seq.index a 9 == Seq.index b 9) /\
    ((v i <> 10 /\ v j <> 10) ==> Seq.index a 10 == Seq.index b 10) /\
    ((v i <> 11 /\ v j <> 11) ==> Seq.index a 11 == Seq.index b 11) /\
    ((v i <> 12 /\ v j <> 12) ==> Seq.index a 12 == Seq.index b 12) /\
    ((v i <> 13 /\ v j <> 13) ==> Seq.index a 13 == Seq.index b 13) /\
    ((v i <> 14 /\ v j <> 14) ==> Seq.index a 14 == Seq.index b 14) /\
    ((v i <> 15 /\ v j <> 15) ==> Seq.index a 15 == Seq.index b 15) /\
    ((v i <> 16 /\ v j <> 16) ==> Seq.index a 16 == Seq.index b 16) /\
    ((v i <> 17 /\ v j <> 17) ==> Seq.index a 17 == Seq.index b 17) /\
    ((v i <> 18 /\ v j <> 18) ==> Seq.index a 18 == Seq.index b 18) /\
    ((v i <> 19 /\ v j <> 19) ==> Seq.index a 19 == Seq.index b 19) /\
    ((v i <> 20 /\ v j <> 20) ==> Seq.index a 20 == Seq.index b 20) /\
    ((v i <> 21 /\ v j <> 21) ==> Seq.index a 21 == Seq.index b 21) /\
    ((v i <> 22 /\ v j <> 22) ==> Seq.index a 22 == Seq.index b 22) /\
    ((v i <> 23 /\ v j <> 23) ==> Seq.index a 23 == Seq.index b 23) /\
    ((v i <> 24 /\ v j <> 24) ==> Seq.index a 24 == Seq.index b 24) /\
    ((v i <> 25 /\ v j <> 25) ==> Seq.index a 25 == Seq.index b 25) /\
    ((v i <> 26 /\ v j <> 26) ==> Seq.index a 26 == Seq.index b 26) /\
    ((v i <> 27 /\ v j <> 27) ==> Seq.index a 27 == Seq.index b 27) /\
    ((v i <> 28 /\ v j <> 28) ==> Seq.index a 28 == Seq.index b 28) /\
    ((v i <> 29 /\ v j <> 29) ==> Seq.index a 29 == Seq.index b 29) /\
    ((v i <> 30 /\ v j <> 30) ==> Seq.index a 30 == Seq.index b 30) /\
    ((v i <> 31 /\ v j <> 31) ==> Seq.index a 31 == Seq.index b 31)

let modifies_range_32 #t
        (a b: t_Array t (mk_usize 32))
        (i:usize{v i < 32}) (j:usize{v j <= 32 /\ v i <= v j}) =
    ((v i > 0 \/ 0 >= v j)   ==> Seq.index a 0 == Seq.index b 0) /\
    ((v i > 1 \/ 1 >= v j)   ==> Seq.index a 1 == Seq.index b 1) /\
    ((v i > 2 \/ 2 >= v j)   ==> Seq.index a 2 == Seq.index b 2) /\
    ((v i > 3 \/ 3 >= v j)   ==> Seq.index a 3 == Seq.index b 3) /\
    ((v i > 4 \/ 4 >= v j)   ==> Seq.index a 4 == Seq.index b 4) /\
    ((v i > 5 \/ 5 >= v j)   ==> Seq.index a 5 == Seq.index b 5) /\
    ((v i > 6 \/ 6 >= v j)   ==> Seq.index a 6 == Seq.index b 6) /\
    ((v i > 7 \/ 7 >= v j)   ==> Seq.index a 7 == Seq.index b 7) /\
    ((v i > 8 \/ 8 >= v j)   ==> Seq.index a 8 == Seq.index b 8) /\
    ((v i > 9 \/ 9 >= v j)   ==> Seq.index a 9 == Seq.index b 9) /\
    ((v i > 10 \/ 10 >= v j) ==> Seq.index a 10 == Seq.index b 10) /\
    ((v i > 11 \/ 11 >= v j) ==> Seq.index a 11 == Seq.index b 11) /\
    ((v i > 12 \/ 12 >= v j) ==> Seq.index a 12 == Seq.index b 12) /\
    ((v i > 13 \/ 13 >= v j) ==> Seq.index a 13 == Seq.index b 13) /\
    ((v i > 14 \/ 14 >= v j) ==> Seq.index a 14 == Seq.index b 14) /\
    ((v i > 15 \/ 15 >= v j) ==> Seq.index a 15 == Seq.index b 15) /\
    ((v i > 16 \/ 16 >= v j) ==> Seq.index a 16 == Seq.index b 16) /\
    ((v i > 17 \/ 17 >= v j) ==> Seq.index a 17 == Seq.index b 17) /\
    ((v i > 18 \/ 18 >= v j) ==> Seq.index a 18 == Seq.index b 18) /\
    ((v i > 19 \/ 19 >= v j) ==> Seq.index a 19 == Seq.index b 19) /\
    ((v i > 20 \/ 20 >= v j) ==> Seq.index a 20 == Seq.index b 20) /\
    ((v i > 21 \/ 21 >= v j) ==> Seq.index a 21 == Seq.index b 21) /\
    ((v i > 22 \/ 22 >= v j) ==> Seq.index a 22 == Seq.index b 22) /\
    ((v i > 23 \/ 23 >= v j) ==> Seq.index a 23 == Seq.index b 23) /\
    ((v i > 24 \/ 24 >= v j) ==> Seq.index a 24 == Seq.index b 24) /\
    ((v i > 25 \/ 25 >= v j) ==> Seq.index a 25 == Seq.index b 25) /\
    ((v i > 26 \/ 26 >= v j) ==> Seq.index a 26 == Seq.index b 26) /\
    ((v i > 27 \/ 27 >= v j) ==> Seq.index a 27 == Seq.index b 27) /\
    ((v i > 28 \/ 28 >= v j) ==> Seq.index a 28 == Seq.index b 28) /\
    ((v i > 29 \/ 29 >= v j) ==> Seq.index a 29 == Seq.index b 29) /\
    ((v i > 30 \/ 30 >= v j) ==> Seq.index a 30 == Seq.index b 30) /\
    ((v i > 31 \/ 31 >= v j) ==> Seq.index a 31 == Seq.index b 31)

let modifies_range2_32 #t
        (a b: t_Array t (mk_usize 32))
        (i:usize{v i < 32}) (j:usize{v j <= 32 /\ v i <= v j})
        (k:usize{v k < 32}) (l:usize{v l <= 32 /\ v k <= v l}) =
    (~((v i <= 0  /\ 0  < v j) \/ (v k <= 0 /\ 0 < v l))  ==> Seq.index a 0 == Seq.index b 0) /\
    (~((v i <= 1  /\ 1  < v j) \/ (v k <= 1 /\ 1 < v l))  ==> Seq.index a 1 == Seq.index b 1) /\
    (~((v i <= 2  /\ 2  < v j) \/ (v k <= 2 /\ 2 < v l))  ==> Seq.index a 2 == Seq.index b 2) /\
    (~((v i <= 3  /\ 3  < v j) \/ (v k <= 3 /\ 3 < v l))  ==> Seq.index a 3 == Seq.index b 3) /\
    (~((v i <= 4  /\ 4  < v j) \/ (v k <= 4 /\ 4 < v l))  ==> Seq.index a 4 == Seq.index b 4) /\
    (~((v i <= 5  /\ 5  < v j) \/ (v k <= 5 /\ 5 < v l))  ==> Seq.index a 5 == Seq.index b 5) /\
    (~((v i <= 6  /\ 6  < v j) \/ (v k <= 6 /\ 6 < v l))  ==> Seq.index a 6 == Seq.index b 6) /\
    (~((v i <= 7  /\ 7  < v j) \/ (v k <= 7 /\ 7 < v l))  ==> Seq.index a 7 == Seq.index b 7) /\
    (~((v i <= 8  /\ 8  < v j) \/ (v k <= 8 /\ 8 < v l))  ==> Seq.index a 8 == Seq.index b 8) /\
    (~((v i <= 9  /\ 9  < v j) \/ (v k <= 9 /\ 9 < v l))  ==> Seq.index a 9 == Seq.index b 9) /\
    (~((v i <= 10 /\ 10 < v j) \/ (v k <= 10 /\ 10 < v l))  ==> Seq.index a 10 == Seq.index b 10) /\
    (~((v i <= 11 /\ 11 < v j) \/ (v k <= 11 /\ 11 < v l))  ==> Seq.index a 11 == Seq.index b 11) /\
    (~((v i <= 12 /\ 12 < v j) \/ (v k <= 12 /\ 12 < v l))  ==> Seq.index a 12 == Seq.index b 12) /\
    (~((v i <= 13 /\ 13 < v j) \/ (v k <= 13 /\ 13 < v l))  ==> Seq.index a 13 == Seq.index b 13) /\
    (~((v i <= 14 /\ 14 < v j) \/ (v k <= 14 /\ 14 < v l))  ==> Seq.index a 14 == Seq.index b 14) /\
    (~((v i <= 15 /\ 15 < v j) \/ (v k <= 15 /\ 15 < v l))  ==> Seq.index a 15 == Seq.index b 15) /\
    (~((v i <= 16 /\ 16 < v j) \/ (v k <= 16 /\ 16 < v l))  ==> Seq.index a 16 == Seq.index b 16) /\
    (~((v i <= 17 /\ 17 < v j) \/ (v k <= 17 /\ 17 < v l))  ==> Seq.index a 17 == Seq.index b 17) /\
    (~((v i <= 18 /\ 18 < v j) \/ (v k <= 18 /\ 18 < v l))  ==> Seq.index a 18 == Seq.index b 18) /\
    (~((v i <= 19 /\ 19 < v j) \/ (v k <= 19 /\ 19 < v l))  ==> Seq.index a 19 == Seq.index b 19) /\
    (~((v i <= 20 /\ 20 < v j) \/ (v k <= 20 /\ 20 < v l))  ==> Seq.index a 20 == Seq.index b 20) /\
    (~((v i <= 21 /\ 21 < v j) \/ (v k <= 21 /\ 21 < v l))  ==> Seq.index a 21 == Seq.index b 21) /\
    (~((v i <= 22 /\ 22 < v j) \/ (v k <= 22 /\ 22 < v l))  ==> Seq.index a 22 == Seq.index b 22) /\
    (~((v i <= 23 /\ 23 < v j) \/ (v k <= 23 /\ 23 < v l))  ==> Seq.index a 23 == Seq.index b 23) /\
    (~((v i <= 24 /\ 24 < v j) \/ (v k <= 24 /\ 24 < v l))  ==> Seq.index a 24 == Seq.index b 24) /\
    (~((v i <= 25 /\ 25 < v j) \/ (v k <= 25 /\ 25 < v l))  ==> Seq.index a 25 == Seq.index b 25) /\
    (~((v i <= 26 /\ 26 < v j) \/ (v k <= 26 /\ 26 < v l))  ==> Seq.index a 26 == Seq.index b 26) /\
    (~((v i <= 27 /\ 27 < v j) \/ (v k <= 27 /\ 27 < v l))  ==> Seq.index a 27 == Seq.index b 27) /\
    (~((v i <= 28 /\ 28 < v j) \/ (v k <= 28 /\ 28 < v l))  ==> Seq.index a 28 == Seq.index b 28) /\
    (~((v i <= 29 /\ 29 < v j) \/ (v k <= 29 /\ 29 < v l))  ==> Seq.index a 29 == Seq.index b 29) /\
    (~((v i <= 30 /\ 30 < v j) \/ (v k <= 30 /\ 30 < v l))  ==> Seq.index a 30 == Seq.index b 30) /\
    (~((v i <= 31 /\ 31 < v j) \/ (v k <= 31 /\ 31 < v l))  ==> Seq.index a 31 == Seq.index b 31)


