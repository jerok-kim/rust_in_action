# 러스트 소개

> #### 이 장에서 배울 내용
> 
> - 러스트의 특징과 목표
> - 러스트 문법 살펴보기
> - 러스트를 사용할 경우와 사용하지 말아야 할 경우
> - 첫 번째 러스트 프로그램 만들기
> - 객체 지향 언어 및 그 외 언어와 러스트 비교 설명

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

예제 1.1 "Hello, World!"를 네 가지 언어로 출력하기

```rust, editable
fn greet_world() {
    println!("Hello, world!");                // 1
    let southern_germany = "Grüß Gott!";      // 2
    let korean = "안녕, 세상아!";             // 3
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

> 1. 느낌표 표시는 매크로를 의미한다. 나중에 알아보자.
> 2. 러스트에서의 할당문이다. 좀 더 정확히 말하자면 `let`키워드를 이용한 변수 바인딩이다.
> 3. 유니코드가 기본으로 제공된다.
> 4. 배열 리터럴을 표현할 때는 대괄호(`[]`)를 쓴다.
> 5. 많은 타입에 반복자(iterator)를 반환하는 `iter()`메서드가 있다.
> 6. 앰퍼샌드(`&`) 기호는 영역 내의 값을 읽기 전용으로 대여(borrow)할 때 사용된다.
> 7. 함수를 호출한다. 괄호가 함수명에 붙어 있는 것에 유의한다.

러스트에서는 다양한 범주의 문자를 사용할 수 있다. 문자열(string)은 UTF-8로 인코딩되어 있다. 이는 비영어권 언어를 상대적으로 쉽게 사용할 수 있다는 것을 의미한다.

`println`뒤에 붙는 느낌표는 매크로를 사용했음을 알려 줄 때 사용한다. 매크로를 사용하면 비슷비슷하게 중복되는 코드 조합(boilerplate code)을 쓰지 않아도 된다. `println!`을 예로 들면,
어떤 데이터 타입이라도 화면에 출력할 수 있도록 내부적으로 타입을 감지하는 기능이 들어 있다.

## 러스트의 생김새와 느낌

러스트는 해스켈 개발자나 자바 프로그래머와도 잘 어울리는 언어다. 해스켈과 자바같은 표현력이 풍부한 고수준 언어이면서, 동시에 저수준 언어에서 볼 수 있는 베어 메탈(bare-metal) 수준의 성능을 보여준다.

예제 1.2는 기본적인 텍스트 처리를 러스트에서 어떻게 하는지 간단히 보여 준다. 주목해야 할 몇 가지 부분은 다음과 같다.

- 일반적인 흐름 제어 메커니즘 - `for` 반복문과 `continue` 키워드를 포함한다.
- 메서드 문법 - 러스트는 객체 지향 언어가 아니어서 상속 등을 지원하지는 않지만, 객체 지향 언어에 있는 메서드 관련 요소를 가져왔다.
- 고차 프로그래밍 - 함수는 인자로도, 반환값으로도 쓰일 수 있다. 예를 들어 21행(`.map(|field| field.trim())`)은 익명 함수 또는 람다 함수로 알려진 클로저(closure)를 포함하고
  있다.
- 타입 애너테이션(type annotation) - 상대적으로 적게 쓰이지만 이따금 컴파일러에 일종의 힌트를 줄 때 필요하다(`if let Ok(length)`로 시작하는 27행을 보자).
- 조건부 컴파일(conditional compilation) - 21~24행(`if cfg!(...);`)은 프로그램을 릴리스 버전으로 만들 때에는 포함되지 않는다.
- 암묵적 반환 - 러스트에는 `return` 키워드가 있지만 보통 생략한다. 러스트는 표현 기반 언어(expression-based language)다.

예제 1.2 몇 가지 기본적인 CSV 데이터 처리를 보여 주는 러스트 코드

```rust, editable
fn main() {                     // 1
    let penguin_data = "\       // 2
    common name, length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {     // 3
            continue;
        }

        let fields: Vec<_> = record         // 4
            .split(',')                     // 5
            .map(|field| field.trim())      // 6
            .collect();                     // 7
        if cfg!(debug_assertions) {         // 8
            eprintln!("debug: {:?}->{:?}",
                      record, fields);      // 9
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {      // 10
            println!("{}, {}cm", name, length);             // 11
        }
    }
}
```

> 1. 프로젝트를 실행하기 위해서는 `main()`함수가 꼭 필요하다.
> 2. 뒤에 오는 줄 바꿈을 피한다.
> 3. 제목과 공백이 있는 줄은 처리하지 않고 넘긴다.
> 4. 한 줄 텍스트로 시작한다.
> 5. `record`를 `field`로 나눈다.
> 6. 각 `field`의 양 끝 공백을 지운다.
> 7. `field`모음을 벡터 하나로 합친다.
> 8. `cfg!`는 컴파일 시에 주어진 환경 설정을 검사한다.
> 9. `eprintln!`은 표준 오류(stderr)로 내용을 출력한다.
> 10. 해당 필드를 부동 소수점 수로 분석한다.
> 11. `println!`은 표준 출력(stdout)으로 출력한다.

- 17행에서 `fields`변수는 `Vec<_>`타입으로 표기했다. `Vec`은 `vector`를 줄여 쓴 것으로, 동적으로 확장할 수 있는 컬렉션 타입이다. 밑줄(`_`)은 해당 요소의 타입을 추론하라고 러스트에
  지시한다.
- 22~28행에서는 콘솔 화면에 정보를 출력하도록 지시했다. `println!`매크로는 주어진 인자를 표준 출력에 출력하며, `eprintln!`은 표준 에러에 출력한다. 매크로는 함수와 비슷하지만 데이터 대신
  코드를 반환한다. 매크로는 공통 패턴을 단순화하는 데 이용한다. `eprintln!`과 `println!` 둘 다 출력을 제어할 목적으로 첫번째 인자에 문자열 리터럴을 사용한다. `{}`자리 표시자는 `{:?}`
  를 이용한 기본 표현 방식이 아닌, 프로그래머가 값을 문자열로 표현하기 위해 정의한 메서드를 사용하도록 러스트에 지시한다.
- 27행에는 몇 가지 독특한 특징이 있다. `if let Ok(length) = fields[1].parse::<f32>()`는 "`fields[1]`를 32비트 부동 소수점 수로 해석을 시도해서 성공할
  경우 `length`변수에 그 값을 할당한다"라고 읽는다. `if let`구문은 데이터를 처리하고 조건에 따라 지역 변수에 값을 할당하는 축약된 방법이다. `parse()`메서드는 문자열 해석에
  성공하면 `Ok(T)`(여기에서 `T`는 임의의 타입을 의미)를, 실패하면 `Err(E)`(여기에서 `E`는 오류 타입을 의미)를 반환한다. `if let Ok(T)`의 효과는 처리 도중 나타나는 오류는 처리하지
  않고 넘어가는 것이다. 러스트는 주변 문맥을 보고 타입 추론이 불가능하면, 해당 타입을 구체적으로 설정해 달라고 요청한다. `parse()`에는 인라인으로 타입 애너테이션을 `parse::<f32>()`로 했다.

소스 코드를 실행 가능한 파일로 변환하는 작업을 컴파일이라고 한다. 러스트 코드를 컴파일하려면 러스트 컴파일러를 설치하고 소스 코드를 대상으로 컴파일러를 실행해야 한다.

`cargo run`을 실행한다. 결과는 다음과 같다.

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target\debug\ex_01_02.exe`
debug: "    Little penguin,33"->["Little penguin", "33"]
Little penguin, 33cm
debug: "    Yellow-eyed penguin,65"->["Yellow-eyed penguin", "65"]
Yellow-eyed penguin, 65cm
debug: "    Fiordland penguin,60"->["Fiordland penguin", "60"]
Fiordland penguin, 60cm
debug: "    Invalid,data"->["Invalid", "data"]

```

`debug:`로 시작되는 눈에 띄는 줄이 있다. 해당 줄은 카고의 `--release`플래그를 이용해서 릴리스용으로 빌드하면 없어진다. 이런 조건부 컴파일 기능은 예제 1.2의 22~
24행 `cfg!(debug_assertions) {...}`블록에서 쓰였다. 릴리스 빌드는 실행 시에는 더 빠르지만 컴파일하는 데는 더 오래 걸린다.

```text
$ cargo run --release
   Compiling ex_01_02 v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\ex_01_02)
    Finished release [optimized] target(s) in 0.48s                                                                                                                                                                               
     Running `target\release\ex_01_02.exe`
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm
```

`cargo`명령에 `-q`플래그를 쓰면 좀 더 간결한 결과를 얻을 수 있다. `-q`는 quite이 준 말이다. 실행결과는 다음과 같다.

```text
$ cargo run -q --release
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm
```

러스트 프로그램이 고수준의 느낌과 저수준의 성능을 겸비했다는 점을 확인했으면 한다. 이제 언어의 특징 말고 이면에는 어떤 생각이 있는지, 프로그래밍 언어 생태계에서 어떤 곳에 걸맞는지 살펴보자.

## 러스트는 어떤 언어인가?

다른 프로그래밍 언어와 러스트를 구분 짓는 특징은 컴파일 시에 잘못된 데이터에 접근하는 것을 방지하는 능력이다. 프로그램에 눈에 띄는 성능 손실 없이 메모리 안전성을 담보한다. 다른 언어와의 비교 내용은 생략하자.

러스트는 시스템 프로그래밍 언어라는 꼬리표를 달고 있다. 하지만 이 언어가 여러 다른 영역에도 적용 가능하다는 것을 많은 러스트 프로그래머가 알게 되었다. 안전성, 생산성, 통제력은 모든 소프트웨어 엔지니어링
프로젝트에 유용하다.

### 러스트의 목표: 안전성

러스트 프로그램은 다음으로부터 자유롭다.

- 댕글링 포인터(dangling pointer) - 프로그램 실행 과정에서 유효하지 않게 된 데이터에 대한 참조(예제 1.3)
- 데이터 경합(data race) - 외부 요인의 변화로 인해 실행할 때마다 프로그램이 어떻게 동작할지 결정할 수 없음(예제 1.4)
- 버퍼 오버플로 - 배열의 범위를 벗어난 값에 접근하려고 하는 것(예제 1.5)
- 반복자 무효화(iterator invalidation) - 실행 도중 변경된 항목에 대해 반복 작업을 함으로써 발생하는 이슈(예제 1.6)

프로그램을 디버그 모드로 컴파일하면 러스트는 정수 오버플로도 방어한다.

다음 예제는 댕글링 포인터에 대한 것이다.

예제 1.3 댕글링 포인터 만들기

```rust, editable
#[derive(Debug)]    // 1
enum Cereal {       // 2
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];   // 3
    grains.push(Cereal::Rye);               // 4
    drop(grains);                           // 5
    println!("{:?}", grains);               // 6
}
```

> 1. `Cereal`열거형을 출력할 때 `println!`매크로를 사용할 수 있도록 한다.
> 2. `enum`(enumeration: 열거형)은 사용할 수 있는 값의 종류가 정해져 있는 타입이다.
> 3. `Cereal`을 항목으로 하는 빈 벡터 `grains`를 정의한다.
> 4. `grains`벡터에 항목을 하나 넣는다.
> 5. `grains`를 삭제하고 내부 항목들도 지운다.
> 6. 삭제된 `grains`에 접근하여 그 값을 출력하려 한다.

예제 1.3에는 `grains`안에 포인터가 있으며 12행에서 생성된다. `Vec<Cereal>`은 기본 배열을 가리키는 내부 포인터로 구현된다. 하지만 이 예제에는 컴파일되지 않는다. 컴파일하려고 하면 '옮겨진(
moved)'값을 '대여(borrow)'하려고 한다는 오류 메시지가 출력된다. 해당 에러 메시지를 어떻게 해석하고 해결할지는 나중에 알아보자. 예제 1.3을 컴파일하면 다음과 같이 오류가 출력된다.

```text
$ cargo run
   Compiling ex_01_03 v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\ex_01_03)
error[E0382]: borrow of moved value: `grains`                                                                                                                                                                                     
  --> src\main.rs:15:22
   |
12 |     let mut grains: Vec<Cereal> = vec![];
   |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
13 |     grains.push(Cereal::Rye);
14 |     drop(grains);
   |          ------ value moved here
15 |     println!("{:?}", grains);
   |                      ^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
```

예제 1.4는 데이터 경합에 대한 예다. 이전에 이 상태는 외부 요인 변화로 인해 실행할 때마다 프로그램의 동작 방식을 결정할 수 없기 때문에 발생한다고 했다.

예제 1.4 러스트가 경합 조건을 방지하는 예

```rust, editable
use std::thread;  // 1

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });    // 2
    thread::spawn(|| { data = 1000; });   //
    println!("{}", data);
}
```

> 1. 다중 스레드 관련 기능을 현재 범위로 가져온다.
> 2. `thread::spawn()`은 인자로 클로저를 받는다.

위 코드는 결정적이지 않다. `main()`이 종료될 때 `data`가 어떤 값을 가질지 알 수 없다. 5행과 6행에서 `thread::spawn()`을 호출하면 스레드 두 개가 생성되고 각각 호출될 때 수직 막대(`||`)와 중괄호 (`{}`)로 표기되는 클로저를 인자로 받는다. 5행에서 생성되는 스레드에서는 `data`변수를 500이라는 값으로 설정하려고 하고, 반면 6행에서 생성되는 스레드에서는 1000으로 설정하려고 한다. 이 스레드들에 대한 실행 순서는 프로그램이 아닌 운영 체제에 의해 결정되므로 어떤 스레드가 우선순위를 가져 먼저 실행될지 알 수 없다.

예제 1.4를 컴파일하면 엄청난 양의 에러 메시지를 보게 된다. 러스트는 한 애플리케이션의 여러 곳에서 데이터에 접근하는 것을 허용하지 않는다. 위 코드에서는 이러한 접근을 세 군데에서 시도했다. 하나는 `main()`에서 실행되는 주 스레드이며, 나머지는 `thread::spawn()`에서 만들어지는 각각의 자식 스레드다. 다음은 컴파일러를 실행하면 볼 수 있는 메시지다.

```text
$ cargo run
   Compiling ex_01_04 v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\ex_01_04)
error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function                                                                                                             
 --> src\main.rs:6:19
  |
6 |     thread::spawn(|| { data = 500; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src\main.rs:6:5
  |
6 |     thread::spawn(|| { data = 500; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
6 |     thread::spawn(move || { data = 500; });
  |                   ++++

error[E0499]: cannot borrow `data` as mutable more than once at a time
 --> src\main.rs:7:19
  |
6 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             first mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
7 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- second borrow occurs due to use of `data` in closure
  |                   |
  |                   second mutable borrow occurs here

error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src\main.rs:7:19
  |
7 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src\main.rs:7:5
  |
7 |     thread::spawn(|| { data = 1000; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
7 |     thread::spawn(move || { data = 1000; });
  |                   ++++

error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
 --> src\main.rs:8:20
  |
6 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
7 |     thread::spawn(|| { data = 1000; });
8 |     println!("{}", data);
  |                    ^^^^ immutable borrow occurs here
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0373, E0499, E0502.
For more information about an error, try `rustc --explain E0373`.
error: could not compile `ex_01_04` due to 4 previous errors
```

예제 1.5는 버퍼 오버플로 예다. 버퍼 오버플로는 메모리상에 존재하지 않는 요소나 비정상적인 요소에 접근하려는 상황을 뜻한다. 이 예제에서는 `fruit`변수가 세 가지 과일 정보만 가지고 있는데 `fruit[4]`에 접근하려 해서 프로그램이 비정상 종료된다.

예제 1.5 버퍼 오버플로로 패닉을 일으키는 예

```rust, editable
fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];
    
    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '🍉')
}
```

예제 1.5를 컴파일하고 실행하면 다음과 같은 에러 메시지가 나온다.

```text
$ cargo run
   Compiling ex_01_05 v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\ex_01_05)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s                                                                                                                                                                     
     Running `target\debug\ex_01_05.exe`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src\main.rs:4:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\ex_01_05.exe` (exit code: 101)
```

다음으로는 반복자 무효호에 대해 살펴보자. 이것은 반복을 수행하는 도중에 값이 변경되는 경우 발생한다.

예제 1.6 반복이 일어나는 동안 값을 변경하려고 시도하는 예

```rust, editable
fn main() {
    let mut letters = vec![   // 1
        "a", "b", "b",
    ];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());   // 2
    }
}
```

> 1. 가변 벡터 `letters`를 만든다.
> 2. 각 글자를 복제한 후 벡터 끝에 덧붙인다.

예제 1.6을 컴파일하면 실패하는데, 러스트는 `letters`변수가 반복문 안에서 수정되는 것을 허용하지 않기 때문이다. 오류 메시지는 다음과 같다.

```text
$ cargo run
   Compiling ex_01_06 v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\ch01\ex_01_06)
error[E0382]: borrow of moved value: `letters`                                                                                                                                                                                    
   --> src\main.rs:8:9
    |
2   |     let mut letters = vec![
    |         ----------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
...
6   |     for letter in letters {
    |                   ------- `letters` moved due to this implicit call to `.into_iter()`
7   |         println!("{}", letter);
8   |         letters.push(letter.clone());
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
    |
note: this function takes ownership of the receiver `self`, which moves `letters`
   --> C:\Users\jerok\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\traits\collect.rs:261:18
    |
261 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
help: consider iterating over a slice of the `Vec<&str>`'s content to avoid moving into the `for` loop
    |
6   |     for letter in &letters {
    |                   +

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ex_01_06` due to previous error
```

에러 메시지가 기술 용어(`borrow`, `move`, `trait` 등)으로 가득하지만, 러스트는 프로그래머가 함정에 빠지지 않도록 보호한다. 그러니 두려워하지 말자. 기술 용어들은 자꾸 하다보면 익숙해지고, 이해할 날이 올 것이다.

언어가 안전하다는 것을 알기만 해도 프로그래머는 많은 자유를 얻는다. 프로그램이 깨지지 않는다고 확신하면 프로그래머는 더 적극적으로 실험을 하기 때문이다. 러스트 커뮤니티의 '두려움 없는 동시성(fearless concurrency)'이란 말은 이런 자유를 토대로 나온 표현이다.

### 러스트의 목표: 생산성

선택지가 있다면 러스트는 개발자에게 가장 쉬운 선택지를 제공한다. 많은 절묘한 기능이 생산성을 높이기 위한 것들이다. 하지만 프로그래머의 생산성은 책 한 권에서 예제를 통해 표현하기에는 어려운 개념이다. 초보자에게 문제가 되는 상황, 예컨대 동등 비교(`==`)를 해야 할 표현식에 대입(`=`)을 사용하는 경우를 살펴보자.

```rust, editable
fn main() {
    let a = 10;
    
    if a = 10 {
        println!("a equals ten");
    }
}
```

위의 러스트 코드는 컴파일되지 않는다. 컴파일러는 다음과 같은 메시지를 출력한다.

```text
$ cargo run
   Compiling snippet v0.1.0 (C:\Users\jerok\projects\study_blog\rust_in_action\code\snippet)
error[E0308]: mismatched types                                                                                                                                                                                                    
 --> src\main.rs:4:8
  |
4 |     if a = 10 {
  |        ^^^^^^ expected `bool`, found `()`
  |
help: you might have meant to compare for equality
  |
4 |     if a == 10 {
  |          ~~

For more information about this error, try `rustc --explain E0308`.
error: could not compile `snippet` due to previous error
```

먼저 `mismatched types`라는 에러 메시지가 낯설게 느껴진다. 당연하지만 변수가 어떤 값과 동일한지 검사하는 것은 문제가 없다.

좀 더 생각해 보면, 왜 `if`검사가 잘못된 타입을 받았다고 하는 것인지 분명해진다. `if`는 정수를 받지 않는다. 대입식의 결과를 받는다. 러스트에서 이는 빈 형(blank type)이며 `()`로 표기한다. `()`는 유닛(unit)이라고 읽는다.

> 유닛이라는 명칭은 OCaml과 F#을 비롯한 ML 계열 언어로부터 물려받은 유산의 일부다. 이 용어는 수학에서 유래했다. 이론적으로 유닛 타입은 단일한 값을 가질 수 있다. 이는 `true`와 `false`두 값을 가지는 불 타입이나 무한대의 유효한 값을 가지는 문자열과 대조된다.

의미 있는 반환값이 없는 경우 표현식은 `()`를 반환한다. 다음과 같이 두 번째 등호 기호를 넣으면 비로소 `a equals ten`이라고 제대로 출력한다.

```rust, editable
fn main() {
    let a = 10;
    
    if a == 10 {
        println!("a equals ten");
    }
}
```

러스트는 인간 공학적 편의 기능을 많이 가지고 있다. 러스트는 제네릭, 정교한 데이터 타입, 패턴 매칭, 클로저를 제공한다. AOT(ahead-of-time) 컴파일 언어를 써본 사람들은 러스트의 빌드 시스템이자 종합적인 패키지 관리자인 카고를 고마워 할 것이다. 카고는 러스트 프로그래머에게 다음과 같은 추가적인 기능을 제공한다.

- `cargo new`는 새로운 디렉터리에 러스트 프로젝트의 뼈대를 만든다(`cargo init`)은 현재 디렉터리를 이용한다.
- `cargo build`는 의존성 패키지를 다운로드하고 코드를 컴파일한다.
- `cargo run`은 `cargo build`를 실행하고 나서 그 결과로 생성된 실행 파일을 실행한다.
- `cargo doc`은 현재 프로젝트의 모든 의존성 패키지에 대한 HTML 문서를 만든다.

### 러스트의 목표: 통제력

프로그래머는 러스트로 데이터 구조의 메모리 배치 방식과 그 접근 패턴을 세밀하게 제어할 수 있다. 러스트는 '무비용 추상화(zero cost abstraction)' 철학에 부합하는 합리적인 기본값을 사용하지만, 이러한 기본값이 모든 상황에 적합하지는 않다.

때로는 애플리케이션 성능을 관리하는 것이 필수다. 데이터가 힙이 아닌 스택에 저장된다는 것이 중요할 수 있다. 값에 대한 공유 참조를 생성하기 위해 참조 카운트(reference count)를 추가하는 것이 합리적일 수 있다. 경우에 따라 특정 접근 패턴에 대해 고유한 유형의 포인터를 만드는 것이 유용할 수 있다. 설계 영역은 넓고 러스트는 선호하는 솔루션을 구현할 수 있는 도구를 제공한다.

예제 1.7은 `a: 10, b: 20, c: 30, d: Mutex{ data:40 }`을 출력한다. 각 표현은 정수를 저장하는 또 다른 방식이다. 지금 기억해야 할 중요한 점은 데이터 타입 선택이 매우 포괄적이라는 사실이다. 사용하는 상황에 맞춰 적합한 방식을 선택해도 된다.

예제 1.7은 또한 정수를 생성하는 여러 방법을 보여 준다. 각 방식은 그 의미와 실행 시 특성이 서로 다르다. 하지만 프로그래머는 자신이 원하는 대로 완전히 절충할 수 있다.

예제 1.7 정숫값을 생성하는 여러 방식

```rust, editable
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10;                           // 1
    let b = Box::new(20);                 // 2
    let c = Rc::new(Box::new(30));        // 3
    let d = Arc::new(Mutex::new(40));     // 4
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
```

> 1. 스택에 정수를 생성한다.
> 2. 힙에 정수를 생성한다. 박스된(boxed) 정수라고도 한다.
> 3. 참조 카운터 안에 박스된 정수를 담는다.
> 4. 원자적(atomic) 참조 카운터에 담긴 정수이며, 상호 배제(mutual exclusion) 잠금 방식으로 보호받는다.

러스트가 왜 특정 방식으로 무언가를 하는지 이해하려면 세 가지 원칙을 살펴보는 것이 도움이 된다.

- 언어의 최우선 사항은 안전성이다.
- 러스트의 데이터는 기본적으로 불변형이다.
- 컴파일 시 검사를 강력하게 우선시한다. 안전성은 '무비용 추상화'여야 한다.

## 러스트의 큰 특징

### 성능

러스트는 컴퓨터의 성능을 가능한 한 전부 뽑아낸다. 러스트는 메모리 안전성을 가비지 컬렉터에 의존하지 않는다.

안타깝게도 더 빠른 프로그램을 만드는 데는 한 가지 문제가 있다. CPU 속도가 정해져 있다는 것이다. 따라서 소프트웨어가 더 빠르게 동작하려면 더 적은 작업을 수행해야 한다. 그런데 언어는 거대하다. 이런 충돌을 해결하기 위해 러스트는 이런 부담을 컴파일러가 지게 한다.

러스트 커뮤니티는 컴파일러가 적게 일하는 간단한 언어보다는, 컴파일러가 더 많이 일하는 큰 언어를 선호한다. 러스트 컴파일러는 프로그램의 크기와 속도를 적극적으로 최적화한다. 덜 두드러져 보이지만 러스트에는 다음과 같은 기법도 있다.

- 캐시 친화적인 데이터 구조가 기본적으로 제공된다. 일반적으로 배열은 포인터로 만들어진, 깊이 중첩된 트리 구조 대신 러스트 프로그램 안에 데이터를 보관한다. 이를 데이터 지향 프로그래밍이라고 한다.
- 최신 패키지 관리자인 카고 덕분에 수많은 오픈 소스 패키지를 쉽게 활용할 수 있다. C와 C++는 일관성이 훨씬 낮아서 의존성이 많은 대규모 프로젝트를 빌드하기가 대체로 어렵다.
- 메서드는 동적 디스패치를 명시적으로 요청하지 않는 한 늘 정적으로 디스패치 된다. 그 덕분에 컴파일러는 코드를 강력하게 최적화할 수 있으며, 때로는 함수 호출 비용을 완전히 없앨 수도 있다.

### 동시성

컴퓨터에 동시에 한 가지 이상의 작업을 요구하는 것은 소프트웨어 엔지니어에게 어려운 일이다. 운영 체제에서 프로그래머가 심각한 실수를 하면 두 개의 독립적인 실행 스레드는 서로를 멋대로 파괴할 수 있다. 그러나 러스트는 두려움 없는 동시성이라는 표현을 만들어 냈다. 안전에 대한 강조는 독립적인 스레드의 제약을 넘는다. 스레드의 속도를 제한하는 전역 인터프리터 잠금(global interpreter lock) 같은 것은 없다.


### 메모리 효율성

러스트를 이용하면 최소한의 메모리만 필요로 하는 프로그램을 만들 수 있다. 필요한 경우 고정 크기 구조체를 사용할 수 있으며 모든 바이트가 어떻게 관리되는지 정확히 알 수 있다. 반복 및 제네릭 타입과 같은 고차원적 구조는 실행 시 최소한의 부하만 일으킨다.

## 러스트의 단점

### 순환 데이터 구조

러스트에서는 임의의 그래프 구조 같은 순환 데이터를 모델링하기 어렵다. 러스트의 안전 검사는 이중 링크 리스트를 구현하는 데 방해가 된다. 러스트를 배운다면 더 익숙해질 때까지 이러한 데이터 구조를 구현하지 않도록 한다.

### 컴파일 시간

코드 컴파일 속도가 동종 언어보다 느리다. 러스트는 여러 중간 표현을 받아 LLVM 컴파일러에 많은 코드를 전달하는 복잡한 컴파일러 툴체인을 가지고 있다. 러스트 프로그램의 '컴파일 단위'는 개별 파일이 아니라 전체 패키지다('크레이트'라고 부름). 크레이트(crate)에는 여러 개의 모듈이 포함될 수 있으므로 컴파일하기에 매우 큰 단위가 될 수 있다. 전체 크레이트 최적화가 가능하지만, 이 때문에 전체 크레이트에 대한 컴파일도 필요하다.

### 엄격성

러스트로 프로그래밍할 때 게을러지는 것은 불가능한, 아니 어려운 일이다. 러스트 프로그램은 모든 것이 정확히 맞아떨어지기 전까지는 컴파일되지 않는다. 러스트 컴파일러는 엄격하지만 도움이 된다. 러스트는 실행 전 개발 단계에서 실패를 미리 경험하게 하여 이후 사용자가 실행 시 충돌로 인해 좌절하지 않게 해 준다.

### 언어의 크기

러스트는 큰 언어다! 풍부한 타입 시스템과 많은 키워드, 다른 언어에는 없는 많은 기능이 포함되어 있다. 이러한 요소가 결합되어 학습 곡선이 매우 가파르다. 이런 어려움을 조절해 나가려면 단계적으로 배워 나갈 것을 추천한다. 언어의 가장 작은 부분부터 시작해서 필요한 세부 사항을 익히는 데 시간을 투자하자.

### 과대광고

러스트도 보안 문제에서 완전히 자유롭지 않고, 소프트웨어 공학의 모든 병폐에 대한 만병통치약이 아니다.

## TLS 보안 사례 연구

러스트가 모든 오류를 완화해 주지는 못한다는 것을 보여 주기 위해 인터넷에 연결되는 거의 모든 장치를 위협한 보안 결함 사례를 살펴보고, 러스트로 이러한 위험을 막을 수 있을지 살펴보자. 2015년 러스트가 한창 유명세를 얻고 있을 때 SSL·TLS 구현(OpenSSL과 애플 자체 포크)에 심각한 보안 허점이 발견되었다. 비공식적으로 하트블리드(Heart-bleed)와 goto fail;이라고 알려졌는데, 이 두 가지 공격 모두 러스트가 주장하는 메모리 안전성을 검증할 수 있는 기회가 되었다. 러스트가 이 두 경우에 도움이 될 수 있겠지만, 여전히 비슷한 문제가 있는 러스트 코드를 만들 수도 있었다.

### 하트블리드

하트블리드는 공식적으로 CVE-2014-0160으로 분류된 버그로, 버퍼의 부정확한 재사용으로 인해 일어난다. 버퍼는 입력을 받는 메모리의 공간이다. 메모리에 쓰는 도중에 버퍼의 내용이 제대로 지워지지 않았다면, 그다음 내용을 읽어 들일 때 특정 데이터가 누출될 수 있다.

왜 이런 상화이 발생할까? 프로그래머들은 성능 향상을 추구한다. 버퍼는 애플리케이션이 운영 체제에 메모리를 요청하는 빈도를 최소화하기 위해 재사용된다.

여러 사용자로부터 얻은 어떤 비밀 정보를 처리하려 한다고 가정해 보자. 어떤 이유에서든 프로그램에서 단일 버퍼를 재사용하기로 결정한다. 이 버퍼를 사용한 후 초기화하지 안흥면 이전에 호출했을 때의 정보가 이후 호출될 때 유출된다. 다음은 이 오류가 발생할 수 있는 프로그램의 일부다.

```rust, noplayground
let buffer = &mut[0u8; 1024];   // 1
read_secrets(&user1, buffer);   // 2
store_secrets(buffer);

read_secrets(&user2, buffer);   // 3
store_secrets(buffer);
```

> 1. 0으로 초기화된 1024 길이의 부호 없는 8비트 정수(u8)를 담은 가변 배열(`[...]`)을 가리키는 참조(`&`)를 변수 `buffer`에 바인딩한다.
> 2. `user1`로부터 얻은 내용을 `buffer`에 담는다.
> 3. `user1`로부터 얻은 내용을 이미 가지고 있는 상태에서 `user2`로부터 얻는 내용으로 덮어쓸 수도, 덮어쓰지 못할 수도 있다.

러스트는 논리적 오류로부터 사용자를 보호하지 못한다. 러스트는 사용자의 데이터가 동시에 두 군데에서 기록될 수 없게 막는다. 그렇지만 러스트로 만든 프로그램이 모든 보안 이슈로부터 자유로운 것은 아니다.

### goto fail

`goto fail;`버그는 C 언어 설계 문제(그리고 잠재적으로 컴파일러가 결함을 지적하지 않음으로써 발생하는 문제)와 프로그래머의 실수가 결합되어 일어났다. 이로 인해 암호화 키 쌍을 검증하도록 설계된 함수가 모든 검사를 건너뛰게 된다. 관련 내용은 http://mng.bz/RKGj 에서 확인해 보자.

아무튼, 특수한 경우에도 러스트는 문법 오류를 잡아낸다. 중괄호 없는 논리 검사는 허용되지 않는다. 러스트는 또한 코드에서 사용되지 않는 부분이 있다면 경고를 보낸다. 그렇더라도 러스트에서 오류가 절대 나오지 않는다는 말은 아니다. 프로그래머들은 마감이 촉박해져 스트레스를 받으면 실수를 한다. 대개 이런 사례와 비슷한 코드는 컴파일도 되고 실행도 된다. 주의를 기울여 코딩하자.

## 러스트는 어디에 잘 맞을까?

시스템 프로그래밍 언어로 설계되었지만 러스트는 범용 언어다. 다음과 같은 많은 분야에 성공적으로 진출했다.

### 명령행 유틸리티

러스트는 명령행 유틸리티를 만드는 프로그래머에게 빠른 시작 시간, 적은 메모리 사용, 간편한 배포라는 주요한 세 가지 이점을 제공한다. 러스트는 인터프리터(파이썬, 루비 등)나 가상 머신(자바, C# 등)을 초기화할 필요가 없기 때문에 프로그램이 빠르게 시작된다.

베어 메탈 언어로서, 러스트로 만든 프로그램은 메모리를 매우 효율적으로 사용한다. 많은 데이터 타입이 크기가 0이다. 다시 말하자면 많은 데이터 타입이 컴파일러에 대한 힌트로만 존재하고, 실행 중인 프로그램에서는 메모리를 전혀 사용하지 않는다.

러스트로 만든 유틸리티는 기본으로 정적 바이너리 파일로 컴파일된다. 이렇게 컴파일하면 프로그램을 실행하기 전에 의존하는 공유 라이브러리를 설치할 필요가 없다. 별도 설치 단계가 없는 프로그램을 만들면 쉽게 배포할 수 있다.


### 데이터 처리

러스트는 텍스트 처리와 그 외 데이터 변환 처리에 탁월하다. 프로그래머는 메모리 사용을 통제할 수 있으며 시작 속도가 빠르다는 점에서 혜택을 볼 수 있다. 2017년 중반, 세계에서 가장 빠른 정규식 엔진이 러스트로 만들어졌다. 2019년에는 파이썬 및 R용 데이터 과학 생태계의 기반인 아파치 애로(Apache Arrow) 데이터 처리 프로젝트에서 러스트 기반의 DataFusion 프로젝트를 받아들였다.

이외에도 러스트는 여러 검색 엔진, 데이터 처리 엔진, 로그 구문 분석 시스템을 구현할 때 그 기반이 되었다. 러스트의 데이터 타입 시스템과 메모리 제어를 통해 적은 메모리를 안정적으로 사용하는 고성능 데이터 처리 파이프라인을 만들 수 있다. 러스트로 만든 작은 필터 프로그램은 아파치 스톰(Storm), 아파치 카프카(Kafka) 또는 아파치 하둡(Hadoop) 스트리밍 등 더 큰 프레임워크에 쉽게 포함될 수 있다.

### 애플리케이션 확장

러스트는 동적 언어로 만들어진 프로그램을 확장하는 데 적합하다. JNI(Java native interface) 확장, C 언어 확장 또는 얼랭·엘릭서(Erlang·Elixir) NIF(native implemented function) 등이 러스트에서 사용 가능하다. C 언어 확장은 전형적으로 까다로운 부분이다. 해당 확장은 런타임과 상당히 밀접하게 통합되는 경향이 있다. 실수라도 하면 메모리 누수 또는 충돌로 인해 메모리를 급격히 소모할 수 있다. 러스트를 사용하면 이런 걱정을 많이 덜게 된다.

### 자원이 제한된 환경

C 언어는 수십 년 동안 마이크로컨트롤러 영역을 점유해 왔다. 하지만 사물 인터넷 시대가 오고 있다. 이는 안전하지 않은 수십억 개의 장치가 네트워크에 노출된다는 것을 의미한다. 어떤 입력 구문 분석 코드라도 반복적으로 취약점이 드러난다. 이러한 장치에 대한 펌웨어 업데이트가 얼마나 자주 일어나는지 고려한다면, 처음부터 최대한 안전한 펌웨어를 만드는 것이 중요하다. 러스트는 런타임 비용을 늘리지 않으면서 안전 계층을 추가하여 이러한 환경에서 중요한 역할을 할 수 있다.

### 서버 애플리케이션

러스트로 작성된 대부분의 애플리케이션은 서버에서 실행된다. 이는 웹 트래픽을 처리하거나 운영 중인 사업을 지원하는 것일 수 있다. 운영 체제와 애플리케이션 사이에 위치한 서비스 계층도 있다. 러스트는 데이터베이스, 모니터링 시스템, 검색 어플라이언스, 메시징 시스템을 작성하는 데 사용된다.

- 자바스크립트 언어와 node.js를 위한 패키지 레지스트리 npm은 러스트로 작성되었다.
- 내장형 데이터베이스 sled(https://github.com/spacejam/sled)는 쓰기 작업이 5%를 차지하는 10억 건의 작업을 16코어 기기 환경에서 1분 안에 수행할 수 있다.
- 텍스트 검색 엔진 탠티비(Tantivy)는 8GB의 영문 위키백과를 색인하는 데 4코어 데스크톱 기기에서 100초 정도 걸린다.

### 데스크톱 애플리케이션

러스트의 설계에는 일반 사용자를 위한 소프트웨어를 개발하는 데 방해가 될 요소가 없다. 러스트 초기 개발 기간에 산파 역할을 담당했던 웹 브라우저 엔진 서보(Servo)는 일반 사용자를 위한 프로그램이다. 게임 역시 마찬가지다.

### 데스크톱

데스크톱 애플리케이션은 복잡하고 만들기 까다롭고 지원하기가 어렵다. 러스트의 인간 공학적 접근 방식과 엄격성이야말로 많은 애플리케이션에게 비밀 무기가 될 수 있다. 러스트가 성숙해질수록 관련 생태계도 성장할 것이다.

### 모바일

안드로이드에서는 자바를 이용한다. iOS에서는 일반적으로 스위프트(Swift)로 프로그램을 작성한다. 하지만 또 다른 방법도 존재한다.

두 플랫폼 모두 네이티브 애플리케이션을 실행할 수 있다. 이는 일반적으로 게임과 같이 C++로 작성된 애플리케이션을 사용자의 모바일 기기에 배포하기 위한 것이다. 러스트는 별도의 추가적인 런타임 성능 희생 없이 동일한 인터페이스를 통해 기기에 적용할 수 있다.

### 웹

자바스크립트는 웹 언어다. 시간이 지나면 이 상황은 바뀔 것이다. 브라우저 개발사들은 다양한 언어의 컴파일러 타깃이 될 웹 어셈블리(wasm)라는 표준을 개발하고 있다. 러스트는 그 첫 번째 적용 대상 중 하나다. 러스트 프로젝트를 브라우저로 이식하려면 두 개의 추가적인 명령만 있으면 된다. 여러 회사가 웹 어셈블리를 통해 브라우저에게 러스트를 사용하는 것을 시험하고 있다. 대표적으로 클라우드플레어와 패스틀리(Fastly)가 있다.

### 시스템 프로그래밍

어떤 면에서 시스템 프로그래밍은 러스트의 존재 이유이기도 하다. 컴파일러(러스트 자체를 포함해서), 비디오 게임 엔진 운영 체제를 비롯한 많은 대형 프로그램이 러스트로 구현되었다. 러스트 커뮤니티에는 파서 생성기, 데이터베이스, 파일 형식을 만들었던 사람들이 함께 활동하고 있다.

러스트는 러스트의 목표를 공유하는 프로그래머들에게 생산적인 환경임을 입증했다. 이 영역에서 주목할 만한 세 가지 프로젝트는 다음과 같다.

- 구글은 퓨시아 운영 체제 개발을 후원하고 있다.
- 마이크로소프트는 윈도우 운영 체제를 위한 저수준 컴포넌트 개발에 러스트를 사용하는 것을 활발히 연구하고 있다.
- AWS는 클라우드 컨테이너를 호스팅하는 전용 운영 체제 보틀로켓을 개발했다.

## 러스트의 숨은 특징: 커뮤니티

프로그래밍 언어가 성장하려면 소프트웨어 이상의 것이 필요하다. 러스트 팀이 매우 잘 해낸 것 중 하나는 긍정적이고 환대하는 공동체를 육성한 것이다. 러스트 세상 어디를 가든지 여러분은 예의와 존경으로 대접받을 것이다.

### 러스트 경구

커뮤니티에서 사람들과 교류하다 보면 곧 특별한 의미를 지닌 몇 가지 문구를 접하게 될 것이다. 다음 문구를 이해하면 러스트가 지금처럼 발전한 이유와 해결하려는 문제를 좀 더 쉽게 이해할 수 있다.

- 모두에게 권한을 부여한다(Empowering everyone) - 능력이나 배경에 관계없이 모든 프로그래머의 참여를 환영한다. 프로그래밍, 특히 시스템 프로그래밍을 소수의 축복받은 사람만 하도록 제한해서는 안 된다.
- 엄청나게 빠르다(Blazingly fast) - 러스트는 빠른 프로그래밍 언어다. 동종 언어의 성능에 필적하거나 그보다 빠른 프로그램을 작성할 수 있지만 안전성은 더욱 보장된다.
- 두려움 없는 동시성(Fearless concurrency) - 동시성 및 병렬 프로그래밍은 항상 어렵다고 인식되었다. 러스트는 프로그래머를 괴롭혔던 동종 언어의 모든 오류로부터 우리를 해방시킨다.
- 러스트 2.0은 없다(No Rust 2.0) - 오늘 작성된 러스트 코드는 항상 미래의 러스트 컴파일러에서도 정상적으로 컴파일된다. 러스트는 앞으로 수십 년 동안 의존할 수 있는 신뢰할 수 있는 프로그래밍 언어를 지향한다. 시맨틱(semantic) 버전 관리에 따라 러스트는 하위 호환이 깨지는 일이 없기 때문에 새로운 메이저 버전을 내놓지 않는다.
- 무비용 추상화(Zero-cost abstractions) - 러스트를 통해 얻을 수 있는 기능들은 런타임 비용이 발생하지 않는다. 러스트로 프로그래밍할 때 안전성을 추구한다고 해서 속도가 희생되지는 않는다.

## 요약

- 많은 회사에서 러스트로 대형 소프트웨어 프로젝트를 성공리에 개발했다.
- 러스트로 작성된 소프트웨어는 PC, 브라우저, 서버, 모바일 기기, 사물 인터넷 기기용으로 컴파일할 수 있다.
- 러스트는 소프트웨어 개발자로부터 많은 사랑을 받고 있다. 스택 오버플로의 '가장 사랑받는 프로그래밍 언어' 타이틀을 계속해서 얻고 있다.
- 러스트로 부담 없이 실험해 볼 수 있다. 런타임 비용 없이도 다른 도구가 제공할 수 없는 정확성을 보장한다.
- 러스트에는 세 가지 명령행 도구가 있다.
  - cargo - 전체 크레이트를 관리한다.
  - rustup - 러스트 설치를 관리한다.
  - rustc - 러스트 소스 코드의 컴파일을 관리한다.
- 러스트 프로젝트가 모든 버그로부터 완전히 안전한 것은 아니다.
- 러스트 코드는 안정적이고 빠르며 적은 자원만 소비한다.
