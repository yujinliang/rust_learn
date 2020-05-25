use std::io::prelude::*;
use std::io::BufReader;
use crate::error::WordCountError;

pub fn count_words<R: Read>(input: &mut R) -> Result<u32, WordCountError> { 
    let reader = BufReader::new(input);
    let mut wordcount = 0;
    for line in reader.lines() {
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        //下面注释掉的语句表示，所有其他std::io::Error默认都封装为IOError，除非如上句，自己手动封装为指定高级错误类型。
        //for _word in line?.split_whitespace() { //for  #[error(transparent)] IOError(#[from] std::io::Error)
        for _word in line.split_whitespace() {
            wordcount += 1;
        }
    }
    if wordcount == 0 {
        return Err(WordCountError::EmptySource);
    }
    Ok(wordcount)
}


