module Spec.Utils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models

let pow2_more_values (x:nat) =
  assert_norm(pow2( 0 ) == 1 );
  assert_norm(pow2( 1 ) == 2 );
  assert_norm(pow2( 2 ) == 4 );
  assert_norm(pow2( 3 ) == 8 );
  assert_norm(pow2( 4 ) == 16 );
  assert_norm(pow2( 5 ) == 32 );
  assert_norm(pow2( 6 ) == 64 );
  assert_norm(pow2( 7 ) == 128 );
  assert_norm(pow2( 8 ) == 256 );
  assert_norm(pow2( 9 ) == 512 );
  assert_norm(pow2( 10 ) == 1024 );
  assert_norm(pow2( 11 ) == 2048 );
  assert_norm(pow2( 12 ) == 4096 );
  assert_norm(pow2( 13 ) == 8192 );
  assert_norm(pow2( 14 ) == 16384 );
  assert_norm(pow2( 15 ) == 32768 );
  assert_norm(pow2( 16 ) == 65536 );
  assert_norm(pow2( 17 ) == 131072 );
  assert_norm(pow2( 18 ) == 262144 );
  assert_norm(pow2( 19 ) == 524288 );
  assert_norm(pow2( 20 ) == 1048576 );
  assert_norm(pow2( 21 ) == 2097152 );
  assert_norm(pow2( 22 ) == 4194304 );
  assert_norm(pow2( 23 ) == 8388608 );
  assert_norm(pow2( 24 ) == 16777216 );
  assert_norm(pow2( 25 ) == 33554432 );
  assert_norm(pow2( 26 ) == 67108864 );
  assert_norm(pow2( 27 ) == 134217728 );
  assert_norm(pow2( 28 ) == 268435456 );
  assert_norm(pow2( 29 ) == 536870912 );
  assert_norm(pow2( 30 ) == 1073741824 );
  assert_norm(pow2( 31 ) == 2147483648 );
  assert_norm(pow2( 32 ) == 4294967296 );
  assert_norm(pow2( 33 ) == 8589934592 );
  assert_norm(pow2( 34 ) == 17179869184 );
  assert_norm(pow2( 35 ) == 34359738368 );
  assert_norm(pow2( 36 ) == 68719476736 );
  assert_norm(pow2( 37 ) == 137438953472 );
  assert_norm(pow2( 38 ) == 274877906944 );
  assert_norm(pow2( 39 ) == 549755813888 );
  assert_norm(pow2( 40 ) == 1099511627776 );
  assert_norm(pow2( 41 ) == 2199023255552 );
  assert_norm(pow2( 42 ) == 4398046511104 );
  assert_norm(pow2( 43 ) == 8796093022208 );
  assert_norm(pow2( 44 ) == 17592186044416 );
  assert_norm(pow2( 45 ) == 35184372088832 );
  assert_norm(pow2( 46 ) == 70368744177664 );
  assert_norm(pow2( 47 ) == 140737488355328 );
  assert_norm(pow2( 48 ) == 281474976710656 );
  assert_norm(pow2( 49 ) == 562949953421312 );
  assert_norm(pow2( 50 ) == 1125899906842624 );
  assert_norm(pow2( 51 ) == 2251799813685248 );
  assert_norm(pow2( 52 ) == 4503599627370496 );
  assert_norm(pow2( 53 ) == 9007199254740992 );
  assert_norm(pow2( 54 ) == 18014398509481984 );
  assert_norm(pow2( 55 ) == 36028797018963968 );
  assert_norm(pow2( 56 ) == 72057594037927936 );
  assert_norm(pow2( 57 ) == 144115188075855872 );
  assert_norm(pow2( 58 ) == 288230376151711744 );
  assert_norm(pow2( 59 ) == 576460752303423488 );
  assert_norm(pow2( 60 ) == 1152921504606846976 );
  assert_norm(pow2( 61 ) == 2305843009213693952 );
  assert_norm(pow2( 62 ) == 4611686018427387904 );
  assert_norm(pow2( 63 ) == 9223372036854775808 );
  assert_norm(pow2( 64 ) == 18446744073709551616 );
  assert_norm(pow2( 65 ) == 36893488147419103232 );
  assert_norm(pow2( 66 ) == 73786976294838206464 );
  assert_norm(pow2( 67 ) == 147573952589676412928 );
  assert_norm(pow2( 68 ) == 295147905179352825856 );
  assert_norm(pow2( 69 ) == 590295810358705651712 );
  assert_norm(pow2( 70 ) == 1180591620717411303424 );
  assert_norm(pow2( 71 ) == 2361183241434822606848 );
  assert_norm(pow2( 72 ) == 4722366482869645213696 );
  assert_norm(pow2( 73 ) == 9444732965739290427392 );
  assert_norm(pow2( 74 ) == 18889465931478580854784 );
  assert_norm(pow2( 75 ) == 37778931862957161709568 );
  assert_norm(pow2( 76 ) == 75557863725914323419136 );
  assert_norm(pow2( 77 ) == 151115727451828646838272 );
  assert_norm(pow2( 78 ) == 302231454903657293676544 );
  assert_norm(pow2( 79 ) == 604462909807314587353088 );
  assert_norm(pow2( 80 ) == 1208925819614629174706176 );
  assert_norm(pow2( 81 ) == 2417851639229258349412352 );
  assert_norm(pow2( 82 ) == 4835703278458516698824704 );
  assert_norm(pow2( 83 ) == 9671406556917033397649408 );
  assert_norm(pow2( 84 ) == 19342813113834066795298816 );
  assert_norm(pow2( 85 ) == 38685626227668133590597632 );
  assert_norm(pow2( 86 ) == 77371252455336267181195264 );
  assert_norm(pow2( 87 ) == 154742504910672534362390528 );
  assert_norm(pow2( 88 ) == 309485009821345068724781056 );
  assert_norm(pow2( 89 ) == 618970019642690137449562112 );
  assert_norm(pow2( 90 ) == 1237940039285380274899124224 );
  assert_norm(pow2( 91 ) == 2475880078570760549798248448 );
  assert_norm(pow2( 92 ) == 4951760157141521099596496896 );
  assert_norm(pow2( 93 ) == 9903520314283042199192993792 );
  assert_norm(pow2( 94 ) == 19807040628566084398385987584 );
  assert_norm(pow2( 95 ) == 39614081257132168796771975168 );
  assert_norm(pow2( 96 ) == 79228162514264337593543950336 );
  assert_norm(pow2( 97 ) == 158456325028528675187087900672 );
  assert_norm(pow2( 98 ) == 316912650057057350374175801344 );
  assert_norm(pow2( 99 ) == 633825300114114700748351602688 );
  assert_norm(pow2( 100 ) == 1267650600228229401496703205376 );
  assert_norm(pow2( 101 ) == 2535301200456458802993406410752 );
  assert_norm(pow2( 102 ) == 5070602400912917605986812821504 );
  assert_norm(pow2( 103 ) == 10141204801825835211973625643008 );
  assert_norm(pow2( 104 ) == 20282409603651670423947251286016 );
  assert_norm(pow2( 105 ) == 40564819207303340847894502572032 );
  assert_norm(pow2( 106 ) == 81129638414606681695789005144064 );
  assert_norm(pow2( 107 ) == 162259276829213363391578010288128 );
  assert_norm(pow2( 108 ) == 324518553658426726783156020576256 );
  assert_norm(pow2( 109 ) == 649037107316853453566312041152512 );
  assert_norm(pow2( 110 ) == 1298074214633706907132624082305024 );
  assert_norm(pow2( 111 ) == 2596148429267413814265248164610048 );
  assert_norm(pow2( 112 ) == 5192296858534827628530496329220096 );
  assert_norm(pow2( 113 ) == 10384593717069655257060992658440192 );
  assert_norm(pow2( 114 ) == 20769187434139310514121985316880384 );
  assert_norm(pow2( 115 ) == 41538374868278621028243970633760768 );
  assert_norm(pow2( 116 ) == 83076749736557242056487941267521536 );
  assert_norm(pow2( 117 ) == 166153499473114484112975882535043072 );
  assert_norm(pow2( 118 ) == 332306998946228968225951765070086144 );
  assert_norm(pow2( 119 ) == 664613997892457936451903530140172288 );
  assert_norm(pow2( 120 ) == 1329227995784915872903807060280344576 );
  assert_norm(pow2( 121 ) == 2658455991569831745807614120560689152 );
  assert_norm(pow2( 122 ) == 5316911983139663491615228241121378304 );
  assert_norm(pow2( 123 ) == 10633823966279326983230456482242756608 );
  assert_norm(pow2( 124 ) == 21267647932558653966460912964485513216 );
  assert_norm(pow2( 125 ) == 42535295865117307932921825928971026432 );
  assert_norm(pow2( 126 ) == 85070591730234615865843651857942052864 );
  assert_norm(pow2( 127 ) == 170141183460469231731687303715884105728 );
  assert_norm(pow2( 128 ) == 340282366920938463463374607431768211456 );
  assert_norm(pow2( 129 ) == 680564733841876926926749214863536422912 );
  assert_norm(pow2( 130 ) == 1361129467683753853853498429727072845824 );
  assert_norm(pow2( 131 ) == 2722258935367507707706996859454145691648 );
  assert_norm(pow2( 132 ) == 5444517870735015415413993718908291383296 );
  assert_norm(pow2( 133 ) == 10889035741470030830827987437816582766592 );
  assert_norm(pow2( 134 ) == 21778071482940061661655974875633165533184 );
  assert_norm(pow2( 135 ) == 43556142965880123323311949751266331066368 );
  assert_norm(pow2( 136 ) == 87112285931760246646623899502532662132736 );
  assert_norm(pow2( 137 ) == 174224571863520493293247799005065324265472 );
  assert_norm(pow2( 138 ) == 348449143727040986586495598010130648530944 );
  assert_norm(pow2( 139 ) == 696898287454081973172991196020261297061888 );
  assert_norm(pow2( 140 ) == 1393796574908163946345982392040522594123776 );
  assert_norm(pow2( 141 ) == 2787593149816327892691964784081045188247552 );
  assert_norm(pow2( 142 ) == 5575186299632655785383929568162090376495104 );
  assert_norm(pow2( 143 ) == 11150372599265311570767859136324180752990208 );
  assert_norm(pow2( 144 ) == 22300745198530623141535718272648361505980416 );
  assert_norm(pow2( 145 ) == 44601490397061246283071436545296723011960832 );
  assert_norm(pow2( 146 ) == 89202980794122492566142873090593446023921664 );
  assert_norm(pow2( 147 ) == 178405961588244985132285746181186892047843328 );
  assert_norm(pow2( 148 ) == 356811923176489970264571492362373784095686656 );
  assert_norm(pow2( 149 ) == 713623846352979940529142984724747568191373312 );
  assert_norm(pow2( 150 ) == 1427247692705959881058285969449495136382746624 );
  assert_norm(pow2( 151 ) == 2854495385411919762116571938898990272765493248 );
  assert_norm(pow2( 152 ) == 5708990770823839524233143877797980545530986496 );
  assert_norm(pow2( 153 ) == 11417981541647679048466287755595961091061972992 );
  assert_norm(pow2( 154 ) == 22835963083295358096932575511191922182123945984 );
  assert_norm(pow2( 155 ) == 45671926166590716193865151022383844364247891968 );
  assert_norm(pow2( 156 ) == 91343852333181432387730302044767688728495783936 );
  assert_norm(pow2( 157 ) == 182687704666362864775460604089535377456991567872 );
  assert_norm(pow2( 158 ) == 365375409332725729550921208179070754913983135744 );
  assert_norm(pow2( 159 ) == 730750818665451459101842416358141509827966271488 );
  assert_norm(pow2( 160 ) == 1461501637330902918203684832716283019655932542976 );
  assert_norm(pow2( 161 ) == 2923003274661805836407369665432566039311865085952 );
  assert_norm(pow2( 162 ) == 5846006549323611672814739330865132078623730171904 );
  assert_norm(pow2( 163 ) == 11692013098647223345629478661730264157247460343808 );
  assert_norm(pow2( 164 ) == 23384026197294446691258957323460528314494920687616 );
  assert_norm(pow2( 165 ) == 46768052394588893382517914646921056628989841375232 );
  assert_norm(pow2( 166 ) == 93536104789177786765035829293842113257979682750464 );
  assert_norm(pow2( 167 ) == 187072209578355573530071658587684226515959365500928 );
  assert_norm(pow2( 168 ) == 374144419156711147060143317175368453031918731001856 );
  assert_norm(pow2( 169 ) == 748288838313422294120286634350736906063837462003712 );
  assert_norm(pow2( 170 ) == 1496577676626844588240573268701473812127674924007424 );
  assert_norm(pow2( 171 ) == 2993155353253689176481146537402947624255349848014848 );
  assert_norm(pow2( 172 ) == 5986310706507378352962293074805895248510699696029696 );
  assert_norm(pow2( 173 ) == 11972621413014756705924586149611790497021399392059392 );
  assert_norm(pow2( 174 ) == 23945242826029513411849172299223580994042798784118784 );
  assert_norm(pow2( 175 ) == 47890485652059026823698344598447161988085597568237568 );
  assert_norm(pow2( 176 ) == 95780971304118053647396689196894323976171195136475136 );
  assert_norm(pow2( 177 ) == 191561942608236107294793378393788647952342390272950272 );
  assert_norm(pow2( 178 ) == 383123885216472214589586756787577295904684780545900544 );
  assert_norm(pow2( 179 ) == 766247770432944429179173513575154591809369561091801088 );
  assert_norm(pow2( 180 ) == 1532495540865888858358347027150309183618739122183602176 );
  assert_norm(pow2( 181 ) == 3064991081731777716716694054300618367237478244367204352 );
  assert_norm(pow2( 182 ) == 6129982163463555433433388108601236734474956488734408704 );
  assert_norm(pow2( 183 ) == 12259964326927110866866776217202473468949912977468817408 );
  assert_norm(pow2( 184 ) == 24519928653854221733733552434404946937899825954937634816 );
  assert_norm(pow2( 185 ) == 49039857307708443467467104868809893875799651909875269632 );
  assert_norm(pow2( 186 ) == 98079714615416886934934209737619787751599303819750539264 );
  assert_norm(pow2( 187 ) == 196159429230833773869868419475239575503198607639501078528 );
  assert_norm(pow2( 188 ) == 392318858461667547739736838950479151006397215279002157056 );
  assert_norm(pow2( 189 ) == 784637716923335095479473677900958302012794430558004314112 );
  assert_norm(pow2( 190 ) == 1569275433846670190958947355801916604025588861116008628224 );
  assert_norm(pow2( 191 ) == 3138550867693340381917894711603833208051177722232017256448 );
  assert_norm(pow2( 192 ) == 6277101735386680763835789423207666416102355444464034512896 );
  assert_norm(pow2( 193 ) == 12554203470773361527671578846415332832204710888928069025792 );
  assert_norm(pow2( 194 ) == 25108406941546723055343157692830665664409421777856138051584 );
  assert_norm(pow2( 195 ) == 50216813883093446110686315385661331328818843555712276103168 );
  assert_norm(pow2( 196 ) == 100433627766186892221372630771322662657637687111424552206336 );
  assert_norm(pow2( 197 ) == 200867255532373784442745261542645325315275374222849104412672 );
  assert_norm(pow2( 198 ) == 401734511064747568885490523085290650630550748445698208825344 );
  assert_norm(pow2( 199 ) == 803469022129495137770981046170581301261101496891396417650688 );
  assert_norm(pow2( 200 ) == 1606938044258990275541962092341162602522202993782792835301376 );
  assert_norm(pow2( 201 ) == 3213876088517980551083924184682325205044405987565585670602752 );
  assert_norm(pow2( 202 ) == 6427752177035961102167848369364650410088811975131171341205504 );
  assert_norm(pow2( 203 ) == 12855504354071922204335696738729300820177623950262342682411008 );
  assert_norm(pow2( 204 ) == 25711008708143844408671393477458601640355247900524685364822016 );
  assert_norm(pow2( 205 ) == 51422017416287688817342786954917203280710495801049370729644032 );
  assert_norm(pow2( 206 ) == 102844034832575377634685573909834406561420991602098741459288064 );
  assert_norm(pow2( 207 ) == 205688069665150755269371147819668813122841983204197482918576128 );
  assert_norm(pow2( 208 ) == 411376139330301510538742295639337626245683966408394965837152256 );
  assert_norm(pow2( 209 ) == 822752278660603021077484591278675252491367932816789931674304512 );
  assert_norm(pow2( 210 ) == 1645504557321206042154969182557350504982735865633579863348609024 );
  assert_norm(pow2( 211 ) == 3291009114642412084309938365114701009965471731267159726697218048 );
  assert_norm(pow2( 212 ) == 6582018229284824168619876730229402019930943462534319453394436096 );
  assert_norm(pow2( 213 ) == 13164036458569648337239753460458804039861886925068638906788872192 );
  assert_norm(pow2( 214 ) == 26328072917139296674479506920917608079723773850137277813577744384 );
  assert_norm(pow2( 215 ) == 52656145834278593348959013841835216159447547700274555627155488768 );
  assert_norm(pow2( 216 ) == 105312291668557186697918027683670432318895095400549111254310977536 );
  assert_norm(pow2( 217 ) == 210624583337114373395836055367340864637790190801098222508621955072 );
  assert_norm(pow2( 218 ) == 421249166674228746791672110734681729275580381602196445017243910144 );
  assert_norm(pow2( 219 ) == 842498333348457493583344221469363458551160763204392890034487820288 );
  assert_norm(pow2( 220 ) == 1684996666696914987166688442938726917102321526408785780068975640576 );
  assert_norm(pow2( 221 ) == 3369993333393829974333376885877453834204643052817571560137951281152 );
  assert_norm(pow2( 222 ) == 6739986666787659948666753771754907668409286105635143120275902562304 );
  assert_norm(pow2( 223 ) == 13479973333575319897333507543509815336818572211270286240551805124608 );
  assert_norm(pow2( 224 ) == 26959946667150639794667015087019630673637144422540572481103610249216 );
  assert_norm(pow2( 225 ) == 53919893334301279589334030174039261347274288845081144962207220498432 );
  assert_norm(pow2( 226 ) == 107839786668602559178668060348078522694548577690162289924414440996864 );
  assert_norm(pow2( 227 ) == 215679573337205118357336120696157045389097155380324579848828881993728 );
  assert_norm(pow2( 228 ) == 431359146674410236714672241392314090778194310760649159697657763987456 );
  assert_norm(pow2( 229 ) == 862718293348820473429344482784628181556388621521298319395315527974912 );
  assert_norm(pow2( 230 ) == 1725436586697640946858688965569256363112777243042596638790631055949824 );
  assert_norm(pow2( 231 ) == 3450873173395281893717377931138512726225554486085193277581262111899648 );
  assert_norm(pow2( 232 ) == 6901746346790563787434755862277025452451108972170386555162524223799296 );
  assert_norm(pow2( 233 ) == 13803492693581127574869511724554050904902217944340773110325048447598592 );
  assert_norm(pow2( 234 ) == 27606985387162255149739023449108101809804435888681546220650096895197184 );
  assert_norm(pow2( 235 ) == 55213970774324510299478046898216203619608871777363092441300193790394368 );
  assert_norm(pow2( 236 ) == 110427941548649020598956093796432407239217743554726184882600387580788736 );
  assert_norm(pow2( 237 ) == 220855883097298041197912187592864814478435487109452369765200775161577472 );
  assert_norm(pow2( 238 ) == 441711766194596082395824375185729628956870974218904739530401550323154944 );
  assert_norm(pow2( 239 ) == 883423532389192164791648750371459257913741948437809479060803100646309888 );
  assert_norm(pow2( 240 ) == 1766847064778384329583297500742918515827483896875618958121606201292619776 );
  assert_norm(pow2( 241 ) == 3533694129556768659166595001485837031654967793751237916243212402585239552 );
  assert_norm(pow2( 242 ) == 7067388259113537318333190002971674063309935587502475832486424805170479104 );
  assert_norm(pow2( 243 ) == 14134776518227074636666380005943348126619871175004951664972849610340958208 );
  assert_norm(pow2( 244 ) == 28269553036454149273332760011886696253239742350009903329945699220681916416 );
  assert_norm(pow2( 245 ) == 56539106072908298546665520023773392506479484700019806659891398441363832832 );
  assert_norm(pow2( 246 ) == 113078212145816597093331040047546785012958969400039613319782796882727665664 );
  assert_norm(pow2( 247 ) == 226156424291633194186662080095093570025917938800079226639565593765455331328 );
  assert_norm(pow2( 248 ) == 452312848583266388373324160190187140051835877600158453279131187530910662656 );
  assert_norm(pow2( 249 ) == 904625697166532776746648320380374280103671755200316906558262375061821325312 );
  assert_norm(pow2( 250 ) == 1809251394333065553493296640760748560207343510400633813116524750123642650624 );
  assert_norm(pow2( 251 ) == 3618502788666131106986593281521497120414687020801267626233049500247285301248 );
  assert_norm(pow2( 252 ) == 7237005577332262213973186563042994240829374041602535252466099000494570602496 );
  assert_norm(pow2( 253 ) == 14474011154664524427946373126085988481658748083205070504932198000989141204992 );
  assert_norm(pow2( 254 ) == 28948022309329048855892746252171976963317496166410141009864396001978282409984 );
  assert_norm(pow2( 255 ) == 57896044618658097711785492504343953926634992332820282019728792003956564819968 )

let rec repeati #acc l f acc0 =
    if l = 0 then acc0 else
    f (l -! sz 1) (repeati #acc (l -! sz 1) acc0)

let eq_repeati0 #a n f acc0 = ()

let unfold_repeati #a n f acc0 i = admit ()

let lemma_createL_index #a len l i = ()

let lemma_create16_index #a v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 i =
  let l = [v15; v14; v13; v12; v11; v10; v9; v8; v7; v6; v5; v4; v3; v2; v1; v0] in
  assert_norm (List.Tot.index l 0 == v15);
  assert_norm (List.Tot.index l 1 == v14);
  assert_norm (List.Tot.index l 2 == v13);
  assert_norm (List.Tot.index l 3 == v12);
  assert_norm (List.Tot.index l 4 == v11);
  assert_norm (List.Tot.index l 5 == v10);
  assert_norm (List.Tot.index l 6 == v9);
  assert_norm (List.Tot.index l 7 == v8);
  assert_norm (List.Tot.index l 8 == v7);
  assert_norm (List.Tot.index l 9 == v6);
  assert_norm (List.Tot.index l 10 == v5);
  assert_norm (List.Tot.index l 11 == v4);
  assert_norm (List.Tot.index l 12 == v3);
  assert_norm (List.Tot.index l 13 == v2);
  assert_norm (List.Tot.index l 14 == v1);
  assert_norm (List.Tot.index l 15 == v0)

let lemma_createi_index #a len f i = ()

let lemma_create_index #a len c i = ()


let lemma_bitand_properties #t (x:int_t t) =
    logand_lemma #t x x

(** Hash Function: assumed definitions *)
let v_G input = admit()
let v_H input = admit()
let v_PRF v_LEN input = admit()
let v_PRFxN r v_LEN input = admit()
let v_J (input: t_Slice u8) : t_Array u8 (sz 32) = admit()
let v_XOF v_LEN input = admit()

let update_at_range_lemma #n
  (s: t_Slice 't)
  (i: Core_models.Ops.Range.t_Range (int_t n) {v i.f_start >= 0 /\ v i.f_start <= Seq.length s /\ v i.f_end <= Seq.length s})
  (x: t_Slice 't)
  = let s' = Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x in
    let len = v i.f_start in
    introduce forall (i:nat {i < len}). Seq.index s i == Seq.index s' i
    with (assert ( Seq.index (Seq.slice s  0 len) i == Seq.index s  i 
                 /\ Seq.index (Seq.slice s' 0 len) i == Seq.index s' i ))  


let fill_bytes_pre_true #v_Self #i0 self bytes = assume (i0.f_fill_bytes_pre self bytes)

let fill_bytes_post_true #v_Self #i0 self bytes result =
  assume (i0.f_fill_bytes_post self bytes result /\
          Seq.length (snd result) == Seq.length bytes)

let impl_i16__abs_value (x: i16) = assume (v (Core_models.Num.impl_i16__abs x) == Prims.abs (v x))

let slice_to_array_id (array: t_Slice 'a) =
  assume (Core_models.Result.impl__unwrap
    #(t_Array 'a (mk_usize 16))
    #Core_models.Array.t_TryFromSliceError
    (Core_models.Convert.f_try_into #(t_Slice 'a)
      #(t_Array 'a (mk_usize 16))
      #FStar.Tactics.Typeclasses.solve
      array) == array)

/// Bounded integers

let lemma_intb_le b b' = ()          

let lemma_add_intb b1 b2 n1 n2 = ()

let lemma_add_intb_forall b1 b2 = ()

let lemma_sub_intb b1 b2 n1 n2 = ()

let lemma_sub_intb_forall b1 b2 = ()

#push-options "--z3rlimit 200"
let lemma_mul_intb (b1 b2: nat) (n1 n2: int) =
  if n1 = 0 || n2 = 0
  then ()
  else 
    let open FStar.Math.Lemmas in
    lemma_abs_bound n1 b1;
    lemma_abs_bound n2 b2;
    lemma_abs_mul n1 n2;
    lemma_mult_le_left (abs n1) (abs n2) b2;
    lemma_mult_le_right b2 (abs n1) b1;
    lemma_abs_bound (n1 * n2) (b1 * b2)
#pop-options

let lemma_mul_intb_forall b1 b2 = ()

#push-options "--z3rlimit 200"
let lemma_mul_i16b (b1 b2: nat) (n1 n2: i16) =
  if v n1 = 0 || v n2 = 0
  then ()
  else 
    let open FStar.Math.Lemmas in
    lemma_abs_bound (v n1) b1;
    lemma_abs_bound (v n2) b2;
    lemma_abs_mul (v n1) (v n2);
    lemma_mult_le_left (abs (v n1)) (abs (v n2)) b2;
    lemma_mult_le_right b2 (abs (v n1)) b1;
    lemma_abs_bound (v n1 * v n2) (b1 * b2)
#pop-options

#push-options "--z3rlimit 200"
let lemma_mul_i32b (b1 b2: nat) (n1 n2: i32) =
  if v n1 = 0 || v n2 = 0
  then ()
  else 
    let open FStar.Math.Lemmas in
    lemma_abs_bound (v n1) b1;
    lemma_abs_bound (v n2) b2;
    lemma_abs_mul (v n1) (v n2);
    lemma_mult_le_left (abs (v n1)) (abs (v n2)) b2;
    lemma_mult_le_right b2 (abs (v n1)) b1;
    lemma_abs_bound (v n1 * v n2) (b1 * b2)
#pop-options

let lemma_add_i16b (b1 b2:nat) (n1 n2:i16) = ()

#push-options "--z3rlimit 100 --split_queries always"
let lemma_range_at_percent (v:int) (p:int{p>0/\ p%2=0 /\ v < p/2 /\ v >= -p / 2}) =
    let m = v % p in
    if v < 0 then (
      Math.Lemmas.lemma_mod_plus v 1 p;
      assert ((v + p) % p == v % p);
      assert (v + p >= 0);
      assert (v + p < p);
      Math.Lemmas.modulo_lemma (v+p) p;
      assert (m == v + p);
      assert (m >= p/2);
      assert (v @% p == m - p);
      assert (v @% p == v))
    else (
      assert (v >= 0 /\ v < p);
      Math.Lemmas.modulo_lemma v p;
      assert (v % p == v);
      assert (m < p/2);
      assert (v @% p == v)
    )
#pop-options

let lemma_sub_i16b (b1 b2:nat) (n1 n2:i16) = ()

#push-options "--z3rlimit 100"
let lemma_at_percent_mod (v:int) (p:int{p>0/\ p%2=0}) =
  let m = v % p in
  assert (m >= 0 /\ m < p);
  if m >= p/2 then (
    assert ((v @%p) % p == (m - p) %p);
    Math.Lemmas.lemma_mod_plus m (-1) p;
    assert ((v @%p) % p == m %p);
    Math.Lemmas.lemma_mod_mod m v p;
    assert ((v @%p) % p == v % p)
  ) else (
    assert ((v @%p) % p == m%p);
    Math.Lemmas.lemma_mod_mod m v p;
    assert ((v @%p) % p == v % p)
  ) 
#pop-options

let lemma_div_at_percent (v:int) (p:int{p>0/\ p%2=0 /\ (v/p) < p/2 /\ (v/p) >= -p / 2}) =
    lemma_range_at_percent (v/p) p

let lemma_mont_red_i32 (x:i32) =
  let vlow = cast x <: i16 in
  assert (v vlow == v x @% pow2 16);
  let k = vlow *. (neg (mk_i16 3327)) in
  assert (v k == ((v x @% pow2 16) * (- 3327)) @% pow2 16);
  let k_times_modulus = (cast k <: i32) *. (mk_i32 3329) in
  assert (v k_times_modulus == (v k * 3329));
  let c = cast (k_times_modulus >>! (mk_i32 16)) <: i16 in
  assert (v c == (((v k * 3329) / pow2 16) @% pow2 16));
  lemma_div_at_percent (v k * 3329) (pow2 16);
  assert (v c == (((v k * 3329) / pow2 16)));
  assert (is_i16b 1665 c);
  let vhigh = cast (x >>! (mk_i32 16)) <: i16 in
  lemma_div_at_percent (v x) (pow2 16);
  assert (v vhigh == v x / pow2 16);
  assert (is_i16b 3328 vhigh);
  let result = vhigh -. c in
  lemma_sub_i16b 3328 1665 vhigh c;
  assert (is_i16b (3328 + 1665) result);
  assert (v result = v vhigh - v c);
  assert (is_i16b (3328 + 1665) result);
  assert (is_i32b (3328 * pow2 15) x ==> is_i16b 3328 result);
  calc ( == ) {
      v k_times_modulus % pow2 16;
      ( == ) { assert (v k_times_modulus == v k * 3329) }
      (v k * 3329) % pow2 16;
      ( == ) { assert (v k = ((v x @% pow2 16) * (-3327)) @% pow2 16) }
      ((((v x @% pow2 16) * (-3327)) @% pow2 16) * 3329) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_l (((v x @% pow2 16) * (-3327)) @% pow2 16) 3329 (pow2 16) }
      (((((v x @% pow2 16) * (-3327)) @% pow2 16) % pow2 16) * 3329) % pow2 16;
      ( == ) { lemma_at_percent_mod ((v x @% pow2 16) * (-3327)) (pow2 16)}
      ((((v x @% pow2 16) * (-3327)) % pow2 16)  * 3329) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_l ((v x @% pow2 16) * (-3327)) 3329 (pow2 16) }
      (((v x @% pow2 16) * (-3327)) * 3329) % pow2 16;
      ( == ) { }
      ((v x @% pow2 16) * (-3327 * 3329)) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_r (v x @% pow2 16) (-3327 * 3329) (pow2 16) }
      ((v x @% pow2 16) % pow2 16);
      ( == ) { lemma_at_percent_mod (v x) (pow2 16) }
      (v x) % pow2 16;
    };
    Math.Lemmas.modulo_add (pow2 16) (- (v k_times_modulus)) (v x) (v k_times_modulus);
    assert ((v x - v k_times_modulus) % pow2 16 == 0);
    calc ( == ) {
      v result % 3329;
      ( == ) { }
      (v x / pow2 16 - v k_times_modulus / pow2 16) % 3329;
      ( == ) { Math.Lemmas.lemma_div_exact (v x - v k_times_modulus) (pow2 16) }
      ((v x - v k_times_modulus) / pow2 16) % 3329;
      ( == ) { assert ((pow2 16 * 169) % 3329 == 1) }
      (((v x - v k_times_modulus) / pow2 16) * ((pow2 16 * 169) % 3329)) % 3329;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_r ((v x - v k_times_modulus) / pow2 16)
        (pow2 16 * 169)
        3329 }
      (((v x - v k_times_modulus) / pow2 16) * pow2 16 * 169) % 3329;
      ( == ) { Math.Lemmas.lemma_div_exact (v x - v k_times_modulus) (pow2 16) }
      ((v x - v k_times_modulus) * 169) % 3329;
      ( == ) { assert (v k_times_modulus == v k * 3329) }
      ((v x * 169) - (v k * 3329 * 169)) % 3329;
      ( == ) { Math.Lemmas.lemma_mod_sub (v x * 169) 3329 (v k * 169) }
      (v x * 169) % 3329;
    }


#push-options "--z3rlimit 200"

let lemma_mont_mul_red_i16_int (x y:i16) = 
  let vlow = x *. y in
  let prod = v x * v y in
  assert (v vlow == prod @% pow2 16);
  let k = vlow *. (neg (mk_i16 3327)) in
  assert (v k == (((prod) @% pow2 16) * (- 3327)) @% pow2 16);
  let k_times_modulus = (cast k <: i32) *. (mk_i32 3329) in
  assert (v k_times_modulus == (v k * 3329));
  let c = cast (k_times_modulus >>! (mk_i32 16)) <: i16 in
  assert (v c == (((v k * 3329) / pow2 16) @% pow2 16));
  lemma_div_at_percent (v k * 3329) (pow2 16);
  assert (v c == (((v k * 3329) / pow2 16)));
  assert (is_i16b 1665 c);
  let vhigh = cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16 in
  assert (v x @% pow2 32 == v x);
  assert (v y @% pow2 32 == v y);
  assert (v ((cast x <: i32) *. (cast y <: i32)) == (v x * v y) @% pow2 32);
  assert (v vhigh == (((prod) @% pow2 32) / pow2 16) @% pow2 16);
  assert_norm (pow2 15 * 3326 < pow2 31);
  lemma_range_at_percent prod (pow2 32);
  assert (v vhigh == (prod / pow2 16) @% pow2 16);
  lemma_div_at_percent prod (pow2 16);
  assert (v vhigh == prod / pow2 16);
  let result = vhigh -. c in
  assert (is_i16b 1663 vhigh);
  lemma_sub_i16b 1663 1665 vhigh c;
  assert (is_i16b 3328 result);
  assert (v result = v vhigh - v c);
  calc ( == ) {
      v k_times_modulus % pow2 16;
      ( == ) { assert (v k_times_modulus == v k * 3329) }
      (v k * 3329) % pow2 16;
      ( == ) { assert (v k = ((prod @% pow2 16) * (-3327)) @% pow2 16) }
      ((((prod @% pow2 16) * (-3327)) @% pow2 16) * 3329) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_l (((prod @% pow2 16) * (-3327)) @% pow2 16) 3329 (pow2 16) }
      (((((prod @% pow2 16) * (-3327)) @% pow2 16) % pow2 16) * 3329) % pow2 16;
      ( == ) { lemma_at_percent_mod ((prod @% pow2 16) * (-3327)) (pow2 16)}
      ((((prod @% pow2 16) * (-3327)) % pow2 16)  * 3329) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_l ((prod @% pow2 16) * (-3327)) 3329 (pow2 16) }
      (((prod @% pow2 16) * (-3327)) * 3329) % pow2 16;
      ( == ) { }
      ((prod @% pow2 16) * (-3327 * 3329)) % pow2 16;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_r (prod @% pow2 16) (-3327 * 3329) (pow2 16) }
      ((prod @% pow2 16) % pow2 16);
      ( == ) { lemma_at_percent_mod (prod) (pow2 16) }
      (prod) % pow2 16;
    };
    Math.Lemmas.modulo_add (pow2 16) (- (v k_times_modulus)) ((prod)) (v k_times_modulus);
    assert (((prod) - v k_times_modulus) % pow2 16 == 0);
    calc ( == ) {
      v result % 3329;
      ( == ) { }
      (((prod) / pow2 16) - ((v k * 3329) / pow2 16)) % 3329;
      ( == ) { Math.Lemmas.lemma_div_exact ((prod) - (v k * 3329)) (pow2 16) }
      ((prod - (v k * 3329)) / pow2 16) % 3329;
      ( == ) { assert ((pow2 16 * 169) % 3329 == 1) }
      (((prod - (v k * 3329)) / pow2 16) * ((pow2 16 * 169) % 3329)) % 3329;
      ( == ) { Math.Lemmas.lemma_mod_mul_distr_r (((prod) - (v k * 3329)) / pow2 16)
        (pow2 16 * 169)
        3329 }
      ((((prod) - (v k * 3329)) / pow2 16) * pow2 16 * 169) % 3329;
      ( == ) { Math.Lemmas.lemma_div_exact ((prod) - (v k * 3329)) (pow2 16) }
      (((prod) - (v k * 3329)) * 169) % 3329;
      ( == ) { Math.Lemmas.lemma_mod_sub ((prod) * 169) 3329 (v k * 169)}
      ((prod) * 169) % 3329; 
    }

#pop-options

let lemma_mont_mul_red_i16 x y =
  if is_i16b 1664 y then (
    lemma_mul_intb (pow2 15) 1664 (v x) (v y);
    assert(is_intb (3326 * pow2 15) (v x * v y));
    lemma_mont_mul_red_i16_int x y) 
  else lemma_mont_mul_red_i16_int x y

let lemma_barrett_red (x:i16) = ()

let lemma_cond_sub x =
  let xm = x -. (mk_i16 3329) in
  let mask = xm >>! (mk_i32 15) in
  let mm = mask &. (mk_i16 3329) in
  let result = xm +. mm in
  assert(xm == x -! mk_i16 3329);
  assert(v mask = v xm / pow2 15);
  assert(v xm >= 0 ==> v mask == 0);
  assert(v xm < 0 ==> v mask == -1);
  logand_zero_lemma (mk_i16 3329);
  assert(v xm >= 0 ==> v mm == 0);
  logand_ones_lemma (mk_i16 3329);
  assert(v xm < 0 ==> v mm == 3329)


let lemma_shift_right_15_i16 (x:i16) =
  Rust_primitives.Integers.mk_int_v_lemma #i16_inttype (mk_i16 0);
  Rust_primitives.Integers.mk_int_v_lemma #i16_inttype (mk_i16 (-1));
  ()

