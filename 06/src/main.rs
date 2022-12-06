// fn all_unique(a: u8, b: u8, c: u8, d: u8) -> bool {
//     !(a == b || a == c || a == d || b == c || b == d || c == d)
// }

fn all_unique(slice: &[u8]) -> bool {
    for i in 0..(slice.len() - 1) {
        for j in (i + 1)..slice.len() {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    true
}

pub fn start_of_packet(stream: &[u8], size: usize) -> usize {
    for i in (size)..stream.len() {
        if all_unique(&stream[(i - size)..i]) {
            return i;
        }
    }
    0
}

fn main() {
    let i = start_of_packet(b"bfdbbngnvnsvshhhrvrbrtbrrhqrqgrrmmdfmmqttptltntrntrnrcrdcrrctctdtwtrwwmlltcltcllmpprvvtbbmbvbsvvqwvvscswsqqgzqqppvzppnddjwddlrdlllmwllfccdfccswwrhhndhdhfdftdfdcdcllcjjbsbgssvlvrrhfrfjfpjffvnfvfwfzftztwwrhwrhwhwddpjjmhmgmsssstzszqzfqzzprpzpnndttphtpphvvcpcjppdtdwwqgwwwnhwnnvhvsvqqtrrbsscwssgwglgwlldjdzjjbsjbsbvvpbvppvfvqvbqbzzwtwbwpbbbcffdgggpnpfptpfphfhfthfthffzrrbzbcblbmmzmhhnvvqvlvwvzvtztgzznbbcqbccwhcwcnwnhndhhgcghhlthllplvppgngrgsswfsfnfjjswstsrtrhthhqzqggdtggzvvjljddqdzzrmrjmjcmjmrrwlldttrhhfjjgbglgjlldrllppnwpnwnndrddtdqdhhpccbgcgzzswzztgzzrwwzzmtztvvrtrgttvddwvwvmwvmmdjdrdtthqhgqhggldgdfdnnzjnjffmrrnprnpnssjrrbrjjrrzvrvhvllfvlvnlvvhlhrhzrzrsrswrssjvjdjfjwwjmjvjjthhnjnmnppsccmlcclfclcbllldwldwdlwddbddmnmrmgrmmtsszgssqjsqscswwzmwmbwwvqqbtbtwwpbbwdbdbhhqmqgqddflfmfnmffbtfbbsdsffspsjpsspzszccrsrstrtjrjppcrrpqqvttsbbjtbjjpbbccmzznvvzzvnvmmwmhmvmsstlssqtssvggtbgttgvtggfhggbcgcmcrmcmhhbttdlttfrfprrvttrtggcpplccqggcwwdjjjplprrbsrbbjsjbbfhbhphtttpffdgdgjddzldzlzplpnplljttdqqlmlllhzhjhccpwcwhwwbbqtqffjpfpfpjptplltbtzbbwwvggpcpprdprpbpvvgzgczzfffzcfzccjffrmrhmrhhchvhddsvswscsnnpfppdrdcrdrgdgmdgmgllhggjwggvqvmmvrrzpprhhqfhfvffvlvnlngnffbbtccfbfhfwhhhhwcwzwjjbffvrffqrrnsnwswrsrvrjvrvprpsptspttbztzfzlfzlflcflclqlplglmlqqvlvvhccmmddqvqzvvmlllvnlnhnfnwnlndnntrrhvvgjjnpnggfqgqdqqzvvclvclvlfvlvvlzlssqffmrfmfmjjczcmmmnttnfnmfnmmmhmggjgdjjqmjjvsslszlzrrpdpcpfcpfpnpssvtsvtvllbttzzljzzhrrnsnddjgdjgjljzzvhvddmdvvdtdmmpmqmnnwcwfwbfwwsvvbpvvgvnvcvdvtvrrcppwbppcqpccwqqzmqmmzczppwbbrzrjzzcrcvrrpttrjjcdjcjfcclscstthqhfqhffdcfdfqdqldddsswbblppdmdnnjvvhwvhvjhvjhjhlhcclrrcqcgcjgcjjtbtctbcbsccfwwltlrlplpfpjphplhhgtghhrppwhhrprwpwhhdqdpdwpwwtccvncvvvrpvphpssfpplcczttltgltldlmlqmmbsszdszznpprsprsprsppsbppjddjhhnrrfsrffmlmglgmlglccddpssdpsddhhfmfsfrfsfnnlggfrgghmgmrmprmprrnvrnnbqnqddhhfwfmfwfmfpfdfzdzppsbpsbpprfrvvwvtwtltvlttrhhgvhhjttmssdggnzzvmvcczmzbzwzttvtpvpcphcctmmhshjsjbssglsglgtltslscchhhsjjpljjdtdsdpppptlpprmppwdppglgmggnddztdzddzppswwmhfbpqzffjqgmsntwsnrwqrqwgpwgrbpbjwrhbcdcvqjnwslsnwhglcsjbwjhswjvzssfqgwbbdgbwfrblfmmlmsndhtlbwzfwsspqlncspqbgbnzshbwpvrmjqjzbcbzzdgssbtqdzffjphqjvrspfrjhpspbwcjwbfhqzsdnjwqjzjtjgnbrdbwqhzffphzppvlmsmppqcfjbjbdsnbwtvthwqcfrtfrwchnmqmhnwfcjtbwqwwvlnpmwlrvzwljrljzqstzglqwbzfdftzltlcbvmmfwcjqglvznztwnvzvftpndqmngqswppsnqhdbgthrddfbcfpflpndrhmcqwvnbfztsvnjjdwqgpmvdwvdftgbtvrwbnvvrwsdfzhwbwdhlpzbcqdzhbfqtpjqcmrpvcsrmcwvgghqrclfzpfgnppzmhvdhvdfrcrnjbdcwbftcqjhhfdsnfnwjzjllzzqftzsjrqnsbpjdcswhhmwwdzmvmqcjtqnczjcvzmmqwzjhjpcczgpbmcvbwmpmvnghlrgcmrrdnmjvmvnhtpfpgwgmdfzvlbclzjzwdqqcvfhhgfzdhzpdvfmwjlzzrpdgzmttmvvcplbwfzqftcgcwcgcpgwvnmlqsplpqwfnhwvqtlwcspqdzshsqnlcpqhpcpbwdhdjsmvtbqdwbcrscqfjcrcjhbjbpzbshpbmlcthmbjfwhzfphgbfqfnfztptzvdnwrpslmdtpmzmpbsszqshwdghrbtvhwzhcmpcgfqggpgzwmhhdrlhlvnpzvwwhzqvgvhrzngttcnqgjjhnblncnqnjzlwnmwnrtvwjtnrbhthncwmzwqdbdgtwrncljddnbhmphgjzfrrgmmcwfwjwjlcrhvcdtvsrvsfhmlgmzsgjchhrfmqslmdgtdtrlbhdffddvbsdbdwlwdcmcmmpvzpdtmbthjdzlpwftptpfsggmhjfjwvbwljsfhfwtbfwmczwhbmhvzllqtcqqfbrcdqqsrcpfmnswnfzfqghcmcbqwgvzqpwvvmbpddlhgjgzvgmpljznrhqphwcztqzpnhzqdgpwwclmsgpwnwtvtsjsdmcnvmjbqglttrhbzqdbgwnbsqzmsmztndrtmlpszhzgjbbftbsdwwdrlftrbbnrsqshfhdpdrwmztcqzdjlnthnjhppwntmbqdgzpmfmfnccblsdwljqhjfgtlgvpzpjbsndmwzfwrbmdhpnmbchqlwqtbhhhqqbsfnvscjwrzvjdtvbsqwzvfhwbbjgqpzcwqjdrlfmggzmhbcrhtbqdjntbtqdvmvpqflmccfpnbmnmtqbdflsgczpbsqpfphlzqgvwbjlmsgshrhpcljzdvwvdvlqqwchtjmjgtqjhgwtnddmhphwhvwhtrhfbjjjzfgrcqngnnddctzdzlqjlbdwmjqzccwrvctrzgtzqsswggbqdnplclhtdslcvzhppcjjslnshtwjnbrwdprqhdtfqmqpgfgnqtdnnhrnzfrsqhlftpdslgmmvqhvpjqjwpwgtnmgrbhwntdjftfwtjzjtprctbtsjmqmpcbbtrjvsgqgsfjprqmsmdztbhnbgzldqfzgwqwnnccgcfclctrwqmqpgvfglgsmmpjszqnphnzcnvswpsfsrmnsnlqnpmvfdvdtfgzrdmbftdrrrbfsvzfgmnffvjpcpnndrwhtjjrrvnztlfhcvfqjgfrhtbnhmwnrmwdhzmmtvjmsqmghtbtfjwdnvdcqtqjrfhrwscjftmbgjmcsrbpdpttlmvfmnfjhnptqvggnshzqnlqqdpqqsqssppbwpblhgfrwrblpzwvqphpsgfmbpqtqqpjpgnbblzstgcjhqntgpbfwlzzctqbnbvpgwsdsdldqzhvznqcsrrghpwllshqpdlqnqgzfwrnhwsvhftzplspcbqmclplprlthvwjhdndrjblqdgwvgjlbmblbmcnbzwzdlnpnhhppvrtngvqqwsttgwlvtcqmtrvpbnvcnfqdtqrsrsmhclmtgbdwwdvhwgfcqpmprcpdhqwftcchbwvstcdqrlwtgbcfqfgzprgvpbbzlqfzbqtcrlzscnqpqwtgzbbbdvsvmhggdr", 14);
    eprintln!("{i}");
}

#[cfg(test)]
mod test {
    use crate::start_of_packet;

    #[test]
    fn part_1() {
        assert_eq!(start_of_packet(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(start_of_packet(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(start_of_packet(b"nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(start_of_packet(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(start_of_packet(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(start_of_packet(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(start_of_packet(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(start_of_packet(b"nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            start_of_packet(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(start_of_packet(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
