pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::collections::HashSet;
use std::collections::BTreeMap;

fn ashtonString(s: &str, k: i32) -> char {
    let n = s.len();
    let mut substrings = HashSet::new();
    let mut total_len = 0;

    for i in 0..n {
        for j in i + 1..=n {
            let substring = &s[i..j];
            substrings.insert(substring);
        }
        // total_len += ((n-i)*(n-i+1))/2;
    }

    let mut sorted_substrings: Vec<&str> = substrings.into_iter().collect();
    sorted_substrings.sort();

    let concat_value = sorted_substrings.concat();
    concat_value.chars().nth((k-1) as usize).unwrap()
}

// struct QueryNode {
//     value: char,
//     index: Vec<usize>
// }

fn construct_substring(k: i32, mut prefix: String, indices: &Vec<usize>, map: &BTreeMap<char, Vec<usize>>, original: &str, concat_value: & mut String) -> Option<char> {
    // map: collect sub substring trees
    // k: termination
    prefix.push(original.chars().nth(indices[0]).unwrap());
    *concat_value += &prefix.clone();
    if concat_value.len() as i32 >= k {
        return concat_value.chars().nth((k-1) as usize);
    }
    let mut new_indices = Vec::new();
    for index in indices {
        if index+1 < original.len() {
            new_indices.push(index+1);
        }
    }
    if new_indices.len() == 1 {
        while new_indices[0] < original.len() {
            prefix.push(original.chars().nth(new_indices[0]).unwrap());
            new_indices[0] += 1;
            *concat_value += &prefix.clone();
            if concat_value.len() as i32 >= k {
                return concat_value.chars().nth((k-1) as usize);
            }
        }
        return None;
    }
    new_indices.sort_by(|a, b| original.chars().nth(*a).cmp(&original.chars().nth(*b)));
    let mut index = 0;
    while index < new_indices.len() {
        let mut current_char_indices = Vec::new();
        current_char_indices.push(new_indices[index]);
        while index+1 < new_indices.len() && original.chars().nth(new_indices[index]).unwrap() == original.chars().nth(new_indices[index+1]).unwrap() {
            current_char_indices.push(new_indices[index+1]);
            index += 1;
        }
        let res = construct_substring(k, prefix.clone(), &current_char_indices, map, original, concat_value);
        match res {
            Some(c) => return Some(c),
            None => (),
        }
        index += 1;
    }
    None
}

fn ashtonString2(s: &str, k: i32) -> char {
    let mut char_map = BTreeMap::new();
    let mut concat_value = String::new();
    for (index, c) in s.chars().enumerate() {
        char_map.entry(c).or_insert(Vec::new()).push(index);
    }
    for (c, indices) in &char_map {
        let res = construct_substring(k, String::new(), indices, &char_map, s, &mut concat_value);
        match res {
            Some(c) => return c,
            None => (),
        }
    }
   ' '
}

fn ashtonString3(s: &str, k: i32) -> char {
    ' '
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let concat_value = & mut String::new();
        let s = String::from("");
        let mut cc = s.clone();
        cc.push('a');
        *concat_value += &cc;
        println!("{}", concat_value);
    }

    #[test]
    fn test_generate_distinct_substrings() {
        // let s = "abc"; let k = 2;
        // let result = ashtonString(s, k);
        // assert_eq!(result, 'b');
        let s = "dbac"; let k = 3;
        let result = ashtonString(s, k);
        assert_eq!(result, 'c')
    }

    #[test]
    fn test_order() {
        let a = "bew";
        let b = "bf";
        assert!(a < b);
        let a = "bfw";
        let b = "bf";
        assert!(a > b);
    }

    #[test]
    fn test_ashtonString2() {
        let s = "dbacabfabe"; let k = 12;
        let res = ashtonString2(s, k);
        println!("{}", res);
    }

    #[test]
    fn test_ashtonString2_complex() {
        let s = "ykwdiffcgrqlgpveiqtkvxzywpetqafokbsuhzxprpazewepoxzkvaitsooiptwzvqweqttjitinrmegmfqhfycznqhelgfiybmqufzfbiuswyyidqrkrtmglvkzbphcrusnbssecozymzhrqyehuqnhlzgpoorhkjxndpsferesqnminqpjiertdzkunbbznynsqhywyeqqtcbgsqqcxjwaiiwykyxzymtqtrotxfmqhnzcfrecacfilbgxbeycsrsmlhfkordxeczktgotitdvvkuyqsaikvuxechsvkpcmpohxcahweftqztgsuqepnbtpilkvcmhtapqfradxfynervyloccbfxqqkclnrugtlzydzbcezsitnhgdljetgujtzwgqqojcniholjsnbdgrmmuzxbsgxdzwbfmsvywjgfxtosgsvoljdhiakbgigfeimrcirytxfssvmbphpcqskbvwebekjjsxcwhtwasbvkxhnmpdphxbkuxoyczhntgrqokmoeojqnrebijqritcdtsbxrkknqbdenrtrhehuxlwhwozfkdidvlapvmclphqeylvgreapsywopxtzcdcaoepjstvjannyakgrojighhxwgqvjwzlkfcuxvrixgvxgfezwnkeurbtxuqiqstcxvwxspfpxdpfktfigpnciqxhmoqeilhhjfgdxovurkaddhmlybngtlpibhmltvsedzhanewgowjrfxcdaslufdegkstfoljtmtvbxrhoqtfvskbtcophrtpenijdvvwiqrjnjtdbojygvbbzqsijnxnbhxecubmkuyzftdghohpjirkyjsixpxaywecqhpacpchifqrvxggfzqgjiqiznixkozcwqdahhjpobgjyppgofmznejortlbimegfjimsregvmptbgbslpratbonubqepwkuhujzmpfhbwcfcgxnxqprrfeoguswtfsabdansfwucbzejyugolzirfwxcqvvxqvztvomdnghqhlzfhhvthdkmbjoufjtvfuoclcfaimsraryjauchzowdykzfwucbpsfcvlcfzuxznxjntnxtctycfxjbrneihkkevokukjwzgfpaumvyhvcmunolaswiciozzyujjsjszyutmrstmufgkvtkqpusajuzjqivitnhrhafauymrgsbdnntfkoftieeyoziiopbxrjxmhjfpdgvtvoyfedamjfmzgujvjlsdusrdbwvheqazgyfldfzommntixqukkxgcolgminqaprhpyuugviujxpswfmiqlosbcynmnfncywrxqlflvzvtpppwdamoqhpshefvjuxjqriienvapsvehmakmpadwpvdvcaeyaqorywxmuzbmuhwhjgtahywywcuaeazeqnxpmvdiugurnqyyztzisxgqzcqgdpkvfimrfpzbvvskosjnokxihfyjiopngzkljxcqodukaowojhdxrahbghkqvzdbanmmmqcbvynwnjmwtqvmqcnylxogyrkzhyntrrwoqjkdvxbopwafaoglnutmofnxgaszrpnjaamvzplonmvncbarywdmbsljudimswvtykqxbclpqicukflkbrwclhmgmwsfspasbspducsmlwgxbuhclfhwovcarwhmohepcuuwwmjjjrgmlppyuyxitbjkaqwoadfcxaavolhzfpmqfdrzbqkurvgrnfhninrhpucfhlfmbrfgwwhxmrtfobxcggpovjvkfaqmlxzmqeumcclqvgvkhumqbdgwoejtebgryfgolcanendctamcvasyfzuuffnlgvcgakunnwctjhveijhdlbbrcxlhdaslwxsyhomwnoqywndezmjknmbslozqpscnrwlykzxzopymcbsbpdoerpwcgwtxqxkjtxjfxgfnvdzagtdvyrbpjyrpwkmohwxctgiqnpfkugkazoxxfyoqygfwsrkapjclqmdfbjpxpcxrswqzxfqxlvudpefeokpgytlcecztgzmyvdyukqsvnmacqijguamsvzuzeuvkvhlqmlnwbftorutjecsyegsagnbmjwwegjwsujpxoimhehtklliprdrzqtlbrkhxtdqophlfrzpwiigttrlmucmmvxpphwmdbersleadgpboxjisctgyvsktsbibzwedbxvocvtikwwjggdizkjxetswxaywyezzcxqesjofgkqoruysehridjhalhyklylpvbtpnjwvunjnjjhprzzwkgwwnuibuvqrzmjmvfjruugfdouwpuszaqxqlhrhdjbcvkorrzkotrtwhotwilxkcxapetyhdzjalztdyetrvmqecjakwzxywxpbtokwpwybvsgwyzpvnfzqqbcnazlxzdastkqiiolgitcgutbizaapbcceeqbdvdvoplyybkekdiqabukauksyowcsmfvjisxzfwxjidtnllnnfxpcjhaygessjocribsqzqzjtsxhfmulmlnvvpvbtovcezvmbnccfbmbvjibxeojrbfmscnlqlpukmjnbnqhrckmnupkadurgbeydtlweasrobeqpuzgyjuldkwdpsuwvauawiyckrtzsypjuortzmecycfnwbjteguaqscajyzewqnsgflfrshruoetsqmwyiyobcomaqqyigrplcueuexrubkmtykrhkfknwwnopowxfnkjkogolzkolzkjlbqwjajhzyvqmtnthxdtnljaktqvuaegcxcnxlwylroymdrvawophxprtionittmqvzpivouoeudinykmpatmrnhzbvkwowomyewvusmyoricptogvjvowcqazcwoylcyrawltiljzulqpbzmluajyslyukovxstosrbnzmxaiktxmulzomkogwmbicyfurvnjwckkznvkiuycgzssliahvdrxbwutrhepjqclfxxnrxrxwjrhssrpvioxhisaryqdouournjkkbigmbpgtgxrrmqywiyoiqtwkhsdwcogfworxdytmvmdjcdgmdwuupsgynkupyauwqnvtnqhjckugpastyonnhvlufhlfjhdzuzvkrcvuppaesvzqjoeskrorbbxkiamfbhptlkpcbrhtmikyxqshjhykkxxtajadsrwfbniefrytziryamfjvgwfftahealyrhfuvnzcgxvihpgjbouywqddlelrgwpyfxuamtetsadbpjmszhqvzvzlzmcgiugnraqdwuyqwbulngfmnvinrjasveafyisrlkxhhvzdztrmawypuientehqjivjsqddcakjxkpyfgkfelxyvbnrivhtffcxxhbzjnlhxahdiukmfhlclytttdoykqyhazijmvsmvardubsbldfydzrzehzoxxxzwhjldbxabogxryycdecgfvhlchbceabdhkomolnrbvpstpxwvzecvmnxvqzzqbeansmbfbtgwizrxynvxuxsilqddrcusjugdihjkariksakqpfojcgrpyutqzpkijqntawdcomngmzxbenniteaubvmckwmvnzopwsrmgguuftxlgkwcrwwsrkwdgiythmkfgctokoiqhgdqszsjxqdqaztijrereoymqsbdiltrryhmybvxtbovahekbidfwdtpxusfgnxzlgllijlbkzzmidyjldqjhkyegsmofjotpbgzkrdwscjbfhksldbsnaytukicvyvlzbmmvpinrrqybbqmgsgtueoqrysmynzarmmodwbxosvpvodbijwenmugnntlasltezhkwlhmdedasgbapzeelzlyoemqyxkfztpvewjjcmjvsmxjlepzfcxtgjjhjvmkpbhtzqfbeoywdyhqcwpjzpchanoljcvafcveuaiyriwugdkkccvdrxmtndfwfcwlfrrzubalkwhsatfevajoavknaplgrjtyamawoahywosykxefynwyjgnjxyrqhmrjzrgntpnpefqofuvfitdtbqeyqypxkiglzoysenjivbyawvfgrkbschrsfipsqxdrmelqruamyamuxtcodfhfmaxukoocmrvyxgqpcqbasqwpjaepfnuupsozicduvyutekkicnjudhkpiqenfyhusxtbbwxzwssceekjrvdxdpmlhsamsignhbqjznivhcxogixzfcwisitbcuovewinyywzngvviujqegqkimscxoezkuwqsedseassipntjyjqgbaoqutpizouvlknrogwqaqypdraddqjeqzwnulvtaqoncegqmegoufdzydcbwnionebhqzdsrthtxnmlsszmzemykqzgejuroyaezdyqxflusximshzrmnpwdrehcaaqybwzgurdzdaucigurhofufdayhjbikrinqitkzxmezgjhmdaurgqzjqxraybmslzitssvrgbqmkzboaxhgpgrggiieluwxvespznhhpzubzxrbwzjlharnjbuwxqtsxmiybrfssbutaluwmdiwebjpdflawhtvtdvwwapqelkeweblkjhqksfnysqwzltuqotmqicuvoztvcefllxxgcnewdcyqvsgllubvywqnxmizqpmemjkqyqobumtrhzcutdquzihyurxnijrwvdotvcxrrqiyskulpkhpvqprhngryzpvcdpzioszgazymtmcgttdkiutxblwaatfgigoybpfbqdqjpsrjnuvvrotuzrwckdivmytnpaqfegqvaienebjxqetrvqvhvfchdywsynzevqawagjeiucyyvpvnndlshuwhrqggqkbikxirhmbbqzcoqxddcowmjutckbicojkahstqhurzttnmtrpxhnjqjduwgcyumlvtfojmlcoeybsrukqcxaujfpflrdgfqdywtjkemylkcddwptapvxyaodohgwnzzlxuxhzjikvmpaifvlurkstyvhfddscegzzdjaotklfblpigcczmwvktesxkmboudpxopnhbyodjdnsgqtuoofitzhdlitgnjfdavmctahfguxomsibhpjcoshccakqlrvmmiqikznqtmeieplnewpvowzqzjikceypmoxwnmpjyvrekcsryjmoilehwntyrspfgodwaufaqyfccztcifqssuzqhtranghtxnpzhwazuhcwgvyqdojvlkmuffxslemktdkbakautctzarrdiaatnonstklhrzrkcblcnozgsshsjbxtdymqmajgmwndvgoxibcxrbgkvnegqcaucmkqovwdrlgmuwmcaobrrhdmvislmuhqhsgxndchqjvkhhohvqboxeaumviappqyjxyzcagulbhtlxcgndxnizjwwtzmlpmuomtsnbnafwvqtyyjdvynxhmtdlhocuitgelvhawowreqrdbvazkzgytljazowkjfquaxxymtqslhxmeylrxupsfbtfrprauhwxetvsnnlaxindiauiumacnvhgnygjhcglyeenrrnozcskdmszyubnqktdkbmrgvdezjrqcefdjxnmlhlmcpbuzxxkylbeigihrbatghxrgmdruqfwfhqfeprccujlbrustwobdnsjzxdtpirvphctytybnhoqailthzwkmqunpxhffyaxiesizqjoxzfakqtrrsbfkxsawzfeahbkltskkeyjdfjnvfeozitjfmjcnqgpxhzkcuumyvybajryovmpdgzluknhbvzacaleuhtuctwdcpbrmpgsvhdrsszvnavpaivvqqpsjnxmeafrqnjnvmfpegmrgjigsfdiwvchkctpcaiuorijfnalvmebvojptpzrmezwgunkvvfjonutcwgxkkahylxuaynmcolijatgyarmpnitjostbscbdzvgaluczfniiiogihszuasfrkascdtherneckjbpwmxgcfqkypeahkrrklvpfeuyuyaejbvfnvlsabcysgbzsssfgpuluojoplsyongeitwjxyjpgkpzfjgnzcytrjjiwbikqqobkmmhkvwrinspxaewccrvnafldovtgmwqyizjgycqnwhkwliapbwcbbnerkaywwqxfsinskdfiksgvcgmecqfgfmzpmxokqlriteafhhntztqcbcggsnmxzlpnldyborljwnofwdzwyrazuifovtmxfbkqgktucgfavvfrygpwypxsxdgvwusdxevdrqztwevtzalzhaxfqvzpahkyccezjzdaqctojqikqtkzvhenefdgnnhrslqbkvbcawtlmbvewferpkfvqkcdxjwrwmugkvlmtexhfsmdaqurabmqlqvlbrexgzgsureqxdxcvjhxbbocfduqtsdxliuthcndtrtqwtuuedrgfikmlgegaidmsahzcveyoaqltnfzqyfxgrjtyqayafmshutmpzmdzcourvuiubhcttwtjyrjfednywbpxnsxpjtheprarydmtbfczyneestcqutohmnzvgibwzcpzhbtiixjgnnlfjpwdjknxxmufuvbtzsuivpqdnzmapzhyqddbpqymcuswsvrrooajdspqudslcclsfowxfuljrfijbbcrpectxtltwgeaiptpdroklawefenihqbxugstzdmylsyvhtnnkbzybvcjbqrjivicbawcfkcrdbomwbzhdzhgwjpabjljetohvmlcyovbrkonlpwoqdvnpmnsvaebvskqgwsemoidyyqlnpceuztjliejiikdcxtktnofdykdwcpmrrqmqjxdrdmblzepwzzroqwsqixnkmbcfrqxcpbvtpwgocwmbxdppbjgjhvuvyybroatfdqytofjsdwtczksbuamdxgyvgbmuchcfzaaphkhkgcnhmhkijxlifkdmlripakvbnvqufxfnzuxndhwoghwmtbahuiwuvrvipncwkjklfhykquyxewlxzletvdpqumajzdgloswwbvgtrhqxfdxfodymgqcbcfmekupyoowpldjembcrhzyxczjjpomttyzdvrdjfabsfmwtnymwzluckdnbrzwmzwpunvguvhmctivijjfkuzmgcbjwagiceyyrtenqlzuejpnuasfvrtdtvnrxtazzyyqtcdloefspwhkwzrrskxofkfdffeedcwyhbjxfopwkzghyxarkzfpjkuqqbwvdswmvfjcwbziahrahsrrtyidldvdesajmwyichkailiktalauerndbsqgqtkitwwsugwdrwmceyxgjxcoqqtsklyaglkbjhtfnpignwlsviagidwyvrsfcriiesknzdupvcxizicwtdcbibbdsukvnsftnpgovadsedaflfbekghsjjxbfjyseelrvtfsvlkbonhbskgcspwbyvffedxijkbffjzcukekznltxtvqjttjoypueodnbgsipullgrvfejybevtpopzenqytgdhjkbsbvfnbwkicvgfadbprqqxfiwzrzgalhupdzcgxopzkxhmbibuyusdeqevrmydusszswhrkwtwuaiximriijnmzskreiwaaqzuojoafhxbjhyrvszfdmudgewloulqmnnawbadkxhthfmcxnkbbegiievyaiomxbmvemzqmglvnxznkjqonywxcrvebkqzneuusvkgdwbrtbggmwuawszalvemhwnwaiqufbbjzecuhkcuiyweqvedsiqzgfygpocvrdgqkilrsnnamjgehkizuabafzivpkshqyxaiktcaheoqkszuczoccokdyhsibzyzzajlullbpztbutxwunbwdlzbutjvtkwukffgsthjubkowhlquomzbnavgjsbvovivdonyywsbghxpunlibklqkgwvaatoxbjapyypurqcanruccefopvaxsvxsonprysjwsyqlpslelhjpnodckdbcaywrnnilhrhcqznhtblekwtylyavbczccxvqmdbuvlwlklsemfkxcexpcxkgblkgifywibqdmoqzakfosrnwvklakxinistsxtohvhnjxfkzqpokhdgeorgaqonygjsfcipzrcjphtozlejuiraykhyaxnagwulbdcaueljohapsezomqqkdxkdxyedwyqzdtcyznjnuldoqeeewvrzubfrzjwyjnxniznhpyxlkcccgggedxgyzlrauqafdyunzhuoitauvcwdkeiocoodzhfvxiccgxphgmwqfykbcieonocreswfadcxlfarcrbkdzbjxnmbwqplhselmarmdtmraokeflplksuhfgkdyaqhuwtgwmvchhtixdmerdpbxjldrvisvbzrxuywityqctazyofpuvrrgewazfsxitofqobyhbqlwsmvhsmdpfuvjrvjypiglwnbnpbussipmumvoyydetbnmyykqiqbfdeuufopzyemvshjsipwbsmnqmzgvrjcunwrvmgunmiigstyaixdalstzrbwinyeawxxlfuytegzwzaykxbmktfjmjguwgbyfywqesqxwwztxzrhyvuiqbtdkbzikahpzfhdxxctvbosagaadwitzewmhyurydixircfqhblipelxglaewwdcurjsmdrplqafthvcihkynxvtlwzhsejmwufkxyblobqjjnlruwphvncgldpdiarevyouwpfntxwclivhgnrbavklzzqhcjlxibtgsztlyqpjantgakhcftnevfnxoawybpfwqajpszasotyofisldhrymfvafrzjhgfxioobppvfkwvqgnbjuuigbfjjzksgqzfapudhlzmvxildvoosjwymehvdrnljoqmemplxqzvnhgqfxfxidxujhqmafxkwqyaenneeozryhkdercowzjgibujiuvemtgqgvumkvgifsnjjpahqlpzomkwgfdsajiiffvpcdzhvmtgbtptefusrgqzltrndbwkitzlwyurmnzpgoimvfgolxnzqgmvkifudefzeasqprfwiqkdxrsiohjeowcyghskoalucglrzsphkanhrfshmbnczpcixxslzffblsulnmuzohjhmbobedfgtioqhidgnkjbffmurgthoeochefminsbxiteqykfktgpaulubfbrlsvtwdhgqbhnjcrzcdenyleyfqsixubimzbkcirujahufathfaudnzbupwfosgwghyqjijfukoqpqkyvmtyzuctlaazshvyqvqagchcmwsdmebiqwirsknfmneewcenzxphzwjnseshitrbszummhsyvwfrajeagtjhqtukzmtjikmckhoyogwjfbbfnhhtbrdtmxflkzuujhwvoktersowvrbialebchwoeczqdvkoegmutfzkzpivilfiwjkyqioxmnpplagpismbznnzcvwkhbsfneefnscbiutvwzmeryiqlvrntpzctuhgynlngqpqkilgjxmawusjsmznbypwsyctmqgugxkshxyqumssjnebzdqcdfzyddtrtblzzvrisrzpetjngnqhsgmwnnwsqpkmqvlptfanybccuordbhkupyscloxedhqtedlzklnjmrnifhljoyfgyxkknhqsqinxlawynjjbdykifxtwvycuxmgkwyeogsltsirgtahwzthgracozekytqjpqpfwhrqcbjiumrtnlatcawsbadavvlkmasiijakmkuiwlbjydcaeyvhbyhwtujhvbqfmqryalgwykhxnkztiublukiofrvcunhggygglmflymimykhjeiuzvcpcvmepclvilbqyqxjqmsekemtiwqjrsatqpzfrkdbxgtvyrhpfbtpfhlpebbxvdtlsstlfyoeeicfbjugkpxrxkgdnjajntwhorttpjzwrbbukxawmanjkwmyfpjukhcybxtsgtoawruhoxfdxvmjtbjysuvccxdduwvcrmcodzxuwexvzlgsoqtgkqjonmtjkrledosiprcnvbkxnqpbikivatlphfyrwkvbaplpgocepmbecsgndqifjtxqavjyfhzfwlwfbyjskkxpcfchvmpehmwihhgpoiwnvtswrdqenntruxzpjqvsctalbjaprzcoswklzapoqkgkhhcsxxlzrnmuynjpmodgmprosjcitkudtwxqwirnyehwtsofitnxjeldppljahotelmcwdpujyscsgjdawaldnoscbdekraqfmsbrhxdhrfcyrfanhnsubnyeqdokdfrrxskhrnqivsimzkbixucyhdezgvllccezwoincbyzvilxtofqikrrnvtvseguhmwdaetefrddbqauffnppejebczuihoptktwrocwhibiybffhsvywgfxiersoiijsdfjriiyqjiqnnwuftubzzbjgubwcjfxonifxqfncqfpddljwhmvjpgrkinotslihupemxtqnzfqcsbbaqylhfctnpiijvsrrixghrwuqemuwqvyiulpznjoetyqorjizgqixmcpsqkrghroddeeqpsxirnyiwijcyranvrfldwulqiototldningvmqrubutswipnnbtmxechuxwpibfrplodcgzdcsyycnnqqgenmivjftynwdglrvqvdqafkyfnoufgdjwrrrawmbliguwxrmuuevbptiejejqhsmblfbjrewblqzkjoffsaijvqnfvyvesjfdojohnlkadklnyssqsbcntpuqqrviacmqlazbnkbrwoeugymaboqxgknweiegnszyvybikfzgtfccgpehfuelhsjncotpisqdtrmfwnostqwzfailxpyejitnymdghvkcoyhnnvfgmdhtdsgchfgspzfnokvvfhzwhgjvcqdqtljyfqboxjgolnbzyywfgvbndkifamxvzhuezvuyeampppqnnmtvjwjnhtujfsfgbzmcwgaaiosydinsxioifxvossxykegnftpbbremfwkigxcdiqmnpjbibainhqcojrqnkwzpulzakcgsuufkdiuhlewuubiguatfwuuqhutryzmsvteacajpegjzhrfbsagqwdhdzcuxdgryayyeyivfqeezxlapnjijopmntimwrfurfuxgcslsyrrveunrfvcvmqifywbpdxijrhpwzaoxtitxeiqldsgpiqwgozyrwiapqrlptcpmmkjqszdvtlmbbkkrjiihqlwiciywmpkyzuqrvtopecshmcyxmgeftcnwknuycewbamuwhinlmfuyksxwbddwgrsseorhtnivadrhngvaoraamawndcljteczutgnirjkwcullplbiobuozjtbvdubfuvaclivwtryndleqmmgnhwpqqqlvnocjkclvmhrharwfdayrpefyduqtmeozsqicuwxgdqqfimlnpjeeplcshsnuwbtptetnarwfjmkraxepgjtywxqeseyqhtfbxbqzuogdcrvdobuxmqvjnnnfrlwzgdcggufalmepfbuvcpupfpyuflawyyxedbkkysmjgqylrtgukdlpujjzwlwwlvcrzndxhpingiazdivnlgegrofnbdjpbogadjzkahzipzithdivmnbddiremctnqbptmofnxevoffhnlskxflaittohykwollkybrlywindowszocyvnyfjsuqqengraspckccinpldoeecgcawaffuzxkfktznlopvttfijrlxxpbdtdztjgqkeasqvrdhhueaokkzdvyamzfffhbontytvqkrhnyrketyoextbvtowzvbiyrvrqpnibgsoejaicyzixskuoytpvwxwospgjeokkibotbzwcagzuquiqnxmjukzpzfaewloeofypgwrgctatnljckxnhhnyhuynqlbvzgvporiuvbxqqicuubkbjxzsryfkjifjrcafvkcqozjejlbdmlhxigpbgwlqhsbyudfpnhidirircjxqueoemfhtevllnolirqzgyhkiyuacfzvzflfrsmmyjxjxlwhenglnqxxqtxvyyqzddexvsjucjfbucibrqmggldxeducdkbjphgmbrgfaogxsoylhmtnzymfbghhskqhtyuurcauqirkzrvgfotgpglsoszheroxpkthnvbfguqfnnlucgatolldflkjcaisldbabefhavmnjxjndjguurxzejkhjubwxewaichkztzksixyrfslzrmdaymjsngrtcsbgzngvmqpuqnlxhywymcykokceswxuobdnojkbbaxrpjpzjnxypyigknmcjjyzmbodmzgobdhqoypampydpijzvvbghzfvdwapvgfwmnoaleczvadkjojgkkptlwqosrepzlllycomiqldroncdykpkzkvwakqrqgtbtecvsodjzicoxgtvqifssboungofmhiglkdeajparrqpzjkptshnvxhievpsexyqkewknlkeecvdngugactvbddfauafryvbetorgyymdtrqzlxbpswrvbyyvydnzaqdtgxbhvnkoeaqrzrjsocppannbqccsshlbeoibbsshsibsbkmpmdhorupjwtcqbprhezlhtdrmlsgmcueoxncqjuahpezsvrabsnloqcaduiryexobksstouaeaawvtyznlkbdmdginairxwvionddjghjifgdefsprwtfzbnpbvgbtbliqqnbwwmgbvlfcexuaszbvpqxmxagalqtbduazhgcesigyfayzczurstdptjrebkiehifqqiukqaixdiyfkswcmbrflkmmxuteeyuuiohaqpztaxykqwmcyfjlpxxmtsqxtmubcbctsboszpcrlqvjveuncuzxmsvhgrjjsnemquhpjlivcdezibmmvmjjggrmxcxrrbghynzhahccljcvkojfcuqibjxaluucyamwpldpsisfrwadmjkpgazhkyjxtdctdorszwiteczxybamnkbtldcxblwwpypuognpdxjjzkhzlhlysprfstcvfyswzjqnsffvcogdynelxskrhbwbxayeaqazcspuaurckxiknmvkghbnlzqizpmbhodjiudiounauvmhjhtqpwfavykwnyxwocgwwlhmgumcryjcicszyazvylrnjqjxtrvpdcclwqnnryqzckyamzxkkqztilsbcnthqxumpjcghufjgfltekfoafjjqbmuphcfgytwixerrmdarq";
        let k = 80549440;
        let res = ashtonString2(s, k);
        println!("{}", res);
    }
}
