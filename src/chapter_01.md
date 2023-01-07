# 러스트 소개

러스트가 어디에 사용되는지, 러스트를 실무에서 추천하는지에 대한 내용은 생략한다.

## 언어 맛보기

### "Hello, world!" 프로그램을 편법으로 만들어 보기

명령 프롬프트를 열고 다음 명령을 입력해 보자.

```text
C:\> cd %TMP%

$ cargo new hello
     Created binary (application) `hello` package

$ cd hello
$ cargo run
   Compiling hello v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.90s                                             
     Running `target\debug\hello.exe`
Hello, world!
```

헬로 월드가 찍혔으니 이제부턴 러스트 개발자이다. 가즈아!

러스트의 카고(cargo) 도구는 빌드 시스템이면서 패키지 관리자다. 즉, 카고는 러스트 코드를 실행 가능한 바이너리로 변환하며, 프로젝트의 의존성을 다운로드하고 컴파일하는 프로세스도 관리할 수 있다.

`cargo new`명령은 표준 템플릿을 따르는 프로젝트를 만든다. `tree`명령을 쓰면 `cargo new`실행 후 생성되는 프로젝트 구조와 파일을 볼 수 있다. 카고로 만든 러스트 프로젝트의 구조는 전부
동일하다. 기본 디렉터리에는 `Cargo.toml`이라는 파일이 있는데 이 파일에는 프로젝트의 이름, 버전, 의존성 같은 메타데이터가 들어 있다. 소스 코드는 `src`디렉터리에 있다. 러스트 소스 코드의 파일
확장자는 `.rs`다. `cargo new`로 만들어지는 파일을 보려면 `tree`명령을 쓴다.

그 다음에 실행했던 명령은 `cargo run`인데, 이 부분은 실제로 보이는 것 이상으로 많은 일을 했다. 해당 명령을 실행했을 때 실제로는 실행할 수 있는 것이 전혀 없고, 최대한 많은 오류 관련 정보를
제공하도록 사용자 대신 코드를 디버그 모드로 컴파일하게 설정한다. 여기까지 진행되면 `src/main.rs`파일은 항상 `Hello, world!`문구를 출력하는 코드를 포함하게 된다. 컴파일 결과 `hello`(
또는 `hello.exe`)라는 파일이 생성된다.

`cargo run`을 실행하면 프로젝트에 새로운 파일과 디렉터리가 추가된다. 이제 `Cargo.lock`파일과 `target/`디렉터리가 프로젝트의 기본 디렉터리에 추가되었다. 이 파일과 디렉터리는 카고에 의해
관리된다. 이는 컴파일 과정에서 만들어지는 것들로 이 파일과 디렉터리를 건드릴 필요는 없다.`Cargo.lock`은 모든 의존성에 대한 정확한 버전 번호를 지정하는 파일로, `Cargo.toml`파일이 변경되기
전까지는 이후에도 동일한 방식으로 빌드가 정확히 이루어진다.

`tree`를 다시 실행하면 `hello`프로젝트를 컴파일하기 위해 `cargo run`을 호출해서 생성된 새로운 구조가 나타난다.

### 첫 번째 러스트 프로그램

첫 번째 프로그램으로 다음과 같이 여러 언어로 된 문장을 출력하는 프로그램을 만들려고 한다.

```text
Hello, world!
Grüß Gott!
안녕, 세상아!
ハロー・ワールド
```

```rust, editable
fn greet_world() {
    println!("Hello, world!");                // 1
    let southern_germany = "Grüß Gott!";      // 2
    let korean = "안녕, 세상아!";               // 3
    let japanese = "ハロー・ワールド";

    let regions = [southern_germany, korean, japanese];  // 4

    for region in regions.iter() {  // 5
        println!("{}", &region);    // 6
    }
}

fn main() {
    greet_world();  // 7
}
```

1. 느낌표 표시는 매크로를 의미한다. 나중에 알아보자.
2. 러스트에서의 할당문이다. 좀 더 정확히 말하자면 `let`키워드를 이용한 변수 바인딩이다.
3. 유니코드가 기본으로 제공된다.
4. 배열 리터럴을 표현할 때는 대괄호(`[]`)를 쓴다.
5. 많은 타입에 반복자(iterator)를 반환하는 `iter()`메서드가 있다.
6. 앰퍼샌드(`&`) 기호는 영역 내의 값을 읽기 전용으로 대여(borrow)할 때 사용된다.
7. 함수를 호출한다. 괄호가 함수명에 붙어 있는 것에 유의한다.

