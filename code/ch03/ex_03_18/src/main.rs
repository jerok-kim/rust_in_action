//! 한 번에 한 단계씩 파일을 시뮬레이트한다.

struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// 빈 `File`을 새로 만든다.
    ///
    /// # 예제
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {}