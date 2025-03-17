use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

use crate::heap::Heap;

pub struct Problem {
    id: usize,
    func: fn(),
    info: String,
    trying: bool,
}

impl Problem {
    pub fn execute(&self) {
        let trying: &str;
        if self.trying {
            trying = "시도중...";
            execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        } else {
            trying = "정답!";
            execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();
        }

        println!("● {0} p{1}\n 간단설명 : {2}", trying, self.id, self.info);

        (self.func)();
        execute!(stdout(), SetForegroundColor(Color::Reset)).unwrap();
    }

    pub fn problem_map() -> HashMap<usize, Problem> {
        //문제 풀고 여기에 추가해 주세요
        let mut map: HashMap<usize, Problem> = HashMap::new();

        let test = Problem {
            id: 1,
            func: test,
            info: "대충 테스트라는 뜻".to_string(),
            trying: false,
        };
        map.insert(1, test);

        let p2751 = Problem {
            id: 2751,
            func: p2751,
            info: "수 정렬하기 2".to_string(),
            trying: false,
        };
        map.insert(2751, p2751);

        // ⇑ 여기 추가
        map
    }
}

fn test() {
    println!("이것은 테스트 문제입니다");
}

// 수 정렬하기 p2751
fn p2751() {
    let mut tree: Heap = Heap::new();
    let mut input = String::new();
    let stdin = stdin().lock();
    let mut sin = BufReader::new(stdin);
    sin.read_line(&mut input).unwrap();

    let size: i32 = input.trim().parse().unwrap();
    input.clear();

    for _i in 0..size {
        sin.read_line(&mut input).unwrap();
    }
    let list: Vec<i32> = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect();

    for i in list {
        tree.push(i);
    }

    let mut tmp = i32::MAX;
    let mut data: Option<i32>;
    let stdout = stdout().lock();
    let mut out = BufWriter::new(stdout);
    loop {
        data = tree.pop();
        match data {
            Some(num) => {
                if num != tmp {
                    writeln!(out, "{num}").unwrap();
                }
                tmp = num;
            }
            None => {
                break;
            }
        }
    }
}
