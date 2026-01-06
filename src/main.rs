use rand::seq::SliceRandom;
use std::io;

struct _QuestionModel {
    question: String,
    options: Vec<String>,
    answer: String,
    question_number: u32,
}

fn main() {
    let mut _score: u32 = 0;

    let _quesions = _all_questions();
    println!("Welcome to the Rust quiz!\nHow many question quiz do you wanna attend ? (Max 50)");
    let mut quiz_length: String = String::new();

    io::stdin().read_line(&mut quiz_length).expect("Failed!");

    let quiz_length_1: u32 = match quiz_length.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Oooppsss! Something went wrong!");
            return;
        }
    };

    if quiz_length_1 == 0 {
        println!("Quiz must have at least one question.");
        return;
    }

    if quiz_length_1 > 50 {
        println!("You input number greater than 50!");
        return;
    }

    let mut numbers: Vec<u32> = (1..=50).collect();
    let mut rng = rand::rng();
    numbers.shuffle(&mut rng);

    let quiz_questions: Vec<u32> = numbers.into_iter().take(quiz_length_1 as usize).collect();

    let selected_questions: Vec<_QuestionModel> = _quesions
        .into_iter()
        .filter(|q| quiz_questions.contains(&q.question_number))
        .collect();

    for q in &selected_questions {
        println!("{}", q.question);

        println!("{}", q.options.join("\n"));

        println!("\n");

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Failed!");

        if ans.trim().eq_ignore_ascii_case(&q.answer) {
            _score += 1;
        }
    }

    println!("You got {} correct answer!", _score);
}

fn _all_questions() -> Vec<_QuestionModel> {
    vec![
        _QuestionModel {
            question_number: 1,
            question: "In which year Rust development was started?".into(),
            options: vec![
                "a) 2006".into(),
                "b) 2015".into(),
                "c) 2010".into(),
                "d) 2005".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 2,
            question: "Who started the development of Rust?".into(),
            options: vec![
                "a) Graydon Hoare".into(),
                "b) Linus Torvalds".into(),
                "c) Bjarne Stroustrup".into(),
                "d) Guido van Rossum".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 3,
            question: "Instead of garbage collection, what memory rule does Rust follow?".into(),
            options: vec![
                "a) Ownership".into(),
                "b) Garbage Collection".into(),
                "c) Reference Counting".into(),
                "d) Manual Memory".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 4,
            question: "Which tool manages Rust dependencies?".into(),
            options: vec![
                "a) Cargo".into(),
                "b) NPM".into(),
                "c) Maven".into(),
                "d) Gradle".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 5,
            question: "What keyword is used to define a function in Rust?".into(),
            options: vec![
                "a) fn".into(),
                "b) def".into(),
                "c) func".into(),
                "d) function".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 6,
            question: "Which type represents an optional value in Rust?".into(),
            options: vec![
                "a) Option".into(),
                "b) Result".into(),
                "c) Nullable".into(),
                "d) Maybe".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 7,
            question: "Which keyword is used for pattern matching in Rust?".into(),
            options: vec![
                "a) match".into(),
                "b) switch".into(),
                "c) if".into(),
                "d) select".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 8,
            question: "What trait enables formatting with {} in Rust?".into(),
            options: vec![
                "a) Display".into(),
                "b) Debug".into(),
                "c) Write".into(),
                "d) Format".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 9,
            question: "Which macro prints output to the console?".into(),
            options: vec![
                "a) println!".into(),
                "b) print!".into(),
                "c) echo!".into(),
                "d) log!".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 10,
            question: "What symbol is used for lifetimes in Rust?".into(),
            options: vec!["a) '".into(), "b) &".into(), "c) *".into(), "d) ~".into()],
            answer: "a".into(),
        },
        // ===== Core Rust concepts =====
        _QuestionModel {
            question_number: 11,
            question: "Which keyword makes a variable mutable?".into(),
            options: vec![
                "a) let".into(),
                "b) mut".into(),
                "c) var".into(),
                "d) change".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 12,
            question: "Which keyword is used to define a constant?".into(),
            options: vec![
                "a) let".into(),
                "b) static".into(),
                "c) const".into(),
                "d) final".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 13,
            question: "Which collection type is growable at runtime?".into(),
            options: vec![
                "a) Array".into(),
                "b) Tuple".into(),
                "c) Vec".into(),
                "d) Slice".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 14,
            question: "What does the Result type represent?".into(),
            options: vec![
                "a) Optional value".into(),
                "b) Success or failure".into(),
                "c) Boolean result".into(),
                "d) Lifetime".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 15,
            question: "Which operator is used for borrowing?".into(),
            options: vec!["a) *".into(), "b) &".into(), "c) ->".into(), "d) ::".into()],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 16,
            question: "What does the '?' operator do?".into(),
            options: vec![
                "a) Panics".into(),
                "b) Ignores error".into(),
                "c) Propagates error".into(),
                "d) Logs error".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 17,
            question: "Which keyword is used to implement a trait?".into(),
            options: vec![
                "a) impl".into(),
                "b) trait".into(),
                "c) derive".into(),
                "d) use".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 18,
            question: "Which keyword defines a trait?".into(),
            options: vec![
                "a) impl".into(),
                "b) interface".into(),
                "c) trait".into(),
                "d) type".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 19,
            question: "Which macro is used for debugging output?".into(),
            options: vec![
                "a) print!".into(),
                "b) dbg!".into(),
                "c) log!".into(),
                "d) debug!".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 20,
            question: "Which file defines Rust project dependencies?".into(),
            options: vec![
                "a) package.json".into(),
                "b) Cargo.toml".into(),
                "c) rust.mod".into(),
                "d) build.rs".into(),
            ],
            answer: "b".into(),
        },
        // ===== Rust language behavior =====
        _QuestionModel {
            question_number: 21,
            question: "What happens when ownership is moved?".into(),
            options: vec![
                "a) Value is copied".into(),
                "b) Value is cloned".into(),
                "c) Original becomes invalid".into(),
                "d) Nothing".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 22,
            question: "Which trait allows cloning a value?".into(),
            options: vec![
                "a) Copy".into(),
                "b) Clone".into(),
                "c) Dup".into(),
                "d) Move".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 23,
            question: "Which types implement the Copy trait by default?".into(),
            options: vec![
                "a) Heap types".into(),
                "b) Complex structs".into(),
                "c) Stack-only primitives".into(),
                "d) Vectors".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 24,
            question: "Which keyword is used for module import?".into(),
            options: vec![
                "a) mod".into(),
                "b) include".into(),
                "c) use".into(),
                "d) import".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 25,
            question: "What is the default visibility of items in Rust?".into(),
            options: vec![
                "a) public".into(),
                "b) protected".into(),
                "c) private".into(),
                "d) internal".into(),
            ],
            answer: "c".into(),
        },
        // ===== Advanced / tooling =====
        _QuestionModel {
            question_number: 26,
            question: "Which keyword makes an item public?".into(),
            options: vec![
                "a) open".into(),
                "b) pub".into(),
                "c) export".into(),
                "d) visible".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 27,
            question: "Which command builds a Rust project?".into(),
            options: vec![
                "a) rust build".into(),
                "b) cargo build".into(),
                "c) rustc run".into(),
                "d) cargo start".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 28,
            question: "Which command runs tests?".into(),
            options: vec![
                "a) cargo test".into(),
                "b) rust test".into(),
                "c) cargo check".into(),
                "d) cargo run".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 29,
            question: "What is rustc?".into(),
            options: vec![
                "a) Package manager".into(),
                "b) Runtime".into(),
                "c) Compiler".into(),
                "d) Linter".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 30,
            question: "Which feature allows conditional compilation?".into(),
            options: vec![
                "a) cfg".into(),
                "b) feature".into(),
                "c) gate".into(),
                "d) flag".into(),
            ],
            answer: "a".into(),
        },
        // ===== Final batch =====
        _QuestionModel {
            question_number: 31,
            question: "Which Rust edition introduced async/await?".into(),
            options: vec![
                "a) 2015".into(),
                "b) 2018".into(),
                "c) 2021".into(),
                "d) 2024".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 32,
            question: "Which crate is used for async runtime commonly?".into(),
            options: vec![
                "a) Rayon".into(),
                "b) Tokio".into(),
                "c) AsyncStd".into(),
                "d) Futures".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 33,
            question: "Which keyword marks an async function?".into(),
            options: vec![
                "a) await".into(),
                "b) async".into(),
                "c) future".into(),
                "d) task".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 34,
            question: "Which keyword waits for an async result?".into(),
            options: vec![
                "a) wait".into(),
                "b) join".into(),
                "c) await".into(),
                "d) poll".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 35,
            question: "What does Vec store internally?".into(),
            options: vec![
                "a) Stack memory".into(),
                "b) Heap memory".into(),
                "c) Registers".into(),
                "d) Cache".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 36,
            question: "Which trait is required for println! to work?".into(),
            options: vec![
                "a) Debug".into(),
                "b) Display".into(),
                "c) Write".into(),
                "d) Format".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 37,
            question: "What does panic! do?".into(),
            options: vec![
                "a) Returns error".into(),
                "b) Logs warning".into(),
                "c) Crashes program".into(),
                "d) Skips execution".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 38,
            question: "Which tool formats Rust code?".into(),
            options: vec![
                "a) rustfmt".into(),
                "b) clippy".into(),
                "c) cargo fix".into(),
                "d) formatter".into(),
            ],
            answer: "a".into(),
        },
        _QuestionModel {
            question_number: 39,
            question: "Which tool provides lint warnings?".into(),
            options: vec![
                "a) rustfmt".into(),
                "b) clippy".into(),
                "c) cargo test".into(),
                "d) rustc".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 40,
            question: "What does Box<T> provide?".into(),
            options: vec![
                "a) Stack allocation".into(),
                "b) Heap allocation".into(),
                "c) Reference".into(),
                "d) Slice".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 41,
            question: "Which keyword creates a module?".into(),
            options: vec![
                "a) use".into(),
                "b) mod".into(),
                "c) crate".into(),
                "d) package".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 42,
            question: "What is the entry point of a Rust program?".into(),
            options: vec![
                "a) start()".into(),
                "b) init()".into(),
                "c) main()".into(),
                "d) run()".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 43,
            question: "Which operator dereferences a pointer?".into(),
            options: vec!["a) &".into(), "b) *".into(), "c) ->".into(), "d) ::".into()],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 44,
            question: "Which Rust type is UTF-8 encoded?".into(),
            options: vec![
                "a) char".into(),
                "b) String".into(),
                "c) str".into(),
                "d) All of the above".into(),
            ],
            answer: "d".into(),
        },
        _QuestionModel {
            question_number: 45,
            question: "Which feature prevents data races?".into(),
            options: vec![
                "a) Ownership".into(),
                "b) Borrow checker".into(),
                "c) Lifetimes".into(),
                "d) All of the above".into(),
            ],
            answer: "d".into(),
        },
        _QuestionModel {
            question_number: 46,
            question: "Which crate provides HTTP client commonly?".into(),
            options: vec![
                "a) Hyper".into(),
                "b) Reqwest".into(),
                "c) Actix".into(),
                "d) Rocket".into(),
            ],
            answer: "b".into(),
        },
        _QuestionModel {
            question_number: 47,
            question: "Which keyword exits a loop?".into(),
            options: vec![
                "a) stop".into(),
                "b) exit".into(),
                "c) break".into(),
                "d) return".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 48,
            question: "Which keyword skips an iteration?".into(),
            options: vec![
                "a) skip".into(),
                "b) next".into(),
                "c) continue".into(),
                "d) pass".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 49,
            question: "Which Rust feature enables zero-cost abstractions?".into(),
            options: vec![
                "a) Macros".into(),
                "b) Ownership".into(),
                "c) Monomorphization".into(),
                "d) Lifetimes".into(),
            ],
            answer: "c".into(),
        },
        _QuestionModel {
            question_number: 50,
            question: "Which file contains Rust binary entry?".into(),
            options: vec![
                "a) lib.rs".into(),
                "b) main.rs".into(),
                "c) mod.rs".into(),
                "d) build.rs".into(),
            ],
            answer: "b".into(),
        },
    ]
}
