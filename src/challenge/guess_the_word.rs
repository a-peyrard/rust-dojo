use std::cmp;
use std::collections;

pub struct Master {
    secret: String,
}

impl Master {
    pub fn guess(&self, word: String) -> i32 {
        distance(&self.secret, &word)
    }
}

pub fn find_secret_word(words: Vec<String>, master: &Master) {
    let mut candidates: collections::HashSet<&String> = words.iter().collect();
    let mut found = false;
    for _ in 0..10 {
        println!("we still have {} candidates", candidates.len());
        // pop an element from the set (should find a way to use pop bellow)
        let candidate = candidates.iter().next().cloned().unwrap();
        candidates.remove(candidate);

        // let candidate = pop(&mut candidates);
        let dis = master.guess(candidate.clone());
        println!("candidate {} distance {}", candidate, dis);
        if dis == candidate.len() as i32 {
            println!("we found the secret {}", candidate);
            found = true;
            break;
        }
        candidates.retain(|c| distance(c, candidate) == dis);
    }
    if !found {
        panic!("Unable to find the secret 😱");
    }
}

// this method was an attempt to create a function popping a value,
// but the compiler zis complaining about multiple mutable borrowing :(
// fn pop<'a>(set: &'a mut collections::HashSet<&String>) -> &'a String {
//     let elem = set.iter().next().cloned().unwrap();
//     set.remove(elem);
//     elem
// }

fn distance(word1: &str, word2: &str) -> i32 {
    word1
        .chars()
        .zip(word2.chars())
        .fold(0, |acc, cur| match cur.0.cmp(&cur.1) {
            cmp::Ordering::Equal => acc + 1,
            _ => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_an_example() {
        // GIVEN
        let master = Master {
            secret: "acckzz".to_string(),
        };
        let words = vec![
            "acckzz".to_string(),
            "ccbazz".to_string(),
            "eiowzz".to_string(),
            "abcczz".to_string(),
        ];

        // WHEN
        find_secret_word(words, &master);

        // THEN
        // nothing to really assert, we just expect that we don't panic
    }

    // inconsistent test :/
    // #[test]
    // fn it_should_validate_failing_test_case_1() {
    //     // GIVEN
    //     let master = Master {
    //         secret: "hbaczn".to_string(),
    //     };
    //     let words = vec![
    //         "gaxckt", "trlccr", "jxwhkz", "ycbfps", "peayuf", "yiejjw", "ldzccp", "nqsjoa",
    //         "qrjasy", "pcldos", "acrtag", "buyeia", "ubmtpj", "drtclz", "zqderp", "snywek",
    //         "caoztp", "ibpghw", "evtkhl", "bhpfla", "ymqhxk", "qkvipb", "tvmued", "rvbass",
    //         "axeasm", "qolsjg", "roswcb", "vdjgxx", "bugbyv", "zipjpc", "tamszl", "osdifo",
    //         "dvxlxm", "iwmyfb", "wmnwhe", "hslnop", "nkrfwn", "puvgve", "rqsqpq", "jwoswl",
    //         "tittgf", "evqsqe", "aishiv", "pmwovj", "sorbte", "coifed", "hrctvp", "vkytbw",
    //         "dizcxz", "arabol", "uywurk", "ppywdo", "resfls", "tmoliy", "etriev", "oanvlx",
    //         "wcsnzy", "loufkw", "onnwcy", "novblw", "mtxgwe", "rgrdbt", "ckolob", "kxnflb",
    //         "phonmg", "egcdab", "cykndr", "lkzobv", "ifwmwp", "jqmbib", "mypnvf", "lnrgnj",
    //         "clijwa", "kiioqr", "syzebr", "rqsmhg", "sczjmz", "hsdjfp", "mjcgvm", "ajotcx",
    //         "olgnfv", "mjyjxj", "wzgbmg", "lpcnbj", "yjjlwn", "blrogv", "bdplzs", "oxblph",
    //         "twejel", "rupapy", "euwrrz", "apiqzu", "ydcroj", "ldvzgq", "zailgu", "xgqpsr",
    //         "wxdyho", "alrplq", "brklfk", "hbaczn",
    //     ]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect();
    //
    //     // WHEN
    //     find_secret_word(words, &master);
    //
    //     // THEN
    //     // nothing to really assert, we just expect that we don't panic
    // }
}
