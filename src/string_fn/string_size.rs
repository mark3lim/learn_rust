use std::io;

pub fn get_len(word: String) {
    // 입력된 단어와 그 길이를 출력합니다.
    // .len()은 바이트(byte) 단위의 길이를 반환합니다.
    // 한국어와 같이 2바이트 이상 사용하는 UTF-8은 실제 단어 길이와 다르다.
    println!("Word '{}' length is {}.", word, word.len());
}

pub fn get_word_len(word: String) {
    println!("Word '{}' length is {}(byte: {}).", word, word.chars().count(), word.len());
}
