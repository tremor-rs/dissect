// Copyright 2018-2022, The Tremor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dissect::*;
use simd_json::value::borrowed::{Object, Value};

fn cp(pattern: &str) -> Pattern {
    Pattern::compile(pattern).expect("failed to compile pattern")
}
fn run(pattern: &str, input: &str) -> Option<Object<'static>> {
    cp(pattern).run(input)
}

fn v<'dissect, T: Copy>(s: &'dissect [(&str, T)]) -> Option<Object<'dissect>>
where
    Value<'dissect>: From<T>,
{
    use std::borrow::Cow;
    Some(
        s.iter()
            .map(|(x, y)| (Into::<Cow<'dissect, str>>::into(*x), Value::from(*y)))
            .collect(),
    )
}

macro_rules! assert_pattern {
        ($pattern:expr, $input:expr) => {
            assert_eq!(run($pattern, $input), None)
        };
        ($pattern:expr, $input:expr, $($args:expr),*) => {
            assert_eq!(run($pattern, $input), v(&[$($args),*]))
        };
    }

#[test]
fn dissect_all_edgecases() {
    let testcases = vec![
        (
            "%{name}}%{age}",
            "John}22",
            v(&[("name", "John"), ("age", "22")]),
        ),
        (
            "%{name}%%{age}",
            "John%22",
            v(&[("name", "John"), ("age", "22")]),
        ),
        ("%{name}%%{age}", "John}22", None),
        (".%{name}", ".John", v(&[("name", "John")])),
        (".%{name}", "John", None),
        ("foo %{name}", "foo John", v(&[("name", "John")])),
        ("foo %{name} bar", "foo John bar", v(&[("name", "John")])),
        ("%{name} bar", "John bar", v(&[("name", "John")])),
        ("%{name}bar", "Johnbar", v(&[("name", "John")])),
        ("name%{bar}", "nameJohn", v(&[("bar", "John")])),
        (
            "%{name} %{age} %{country}",
            "John 22 Germany",
            v(&[("name", "John"), ("age", "22"), ("country", "Germany")]),
        ),
        (
            "%{name} %{age}-%{country}",
            "John 22-Germany",
            v(&[("name", "John"), ("age", "22"), ("country", "Germany")]),
        ),
        (
            "this is a %{name} case",
            "this is a John case",
            v(&([("name", "John")])),
        ),
        (
            "this is a %{what} case named %{name}",
            "this is a test case named John",
            v(&([("what", "test"), ("name", "John")])),
        ),
        (
            "this is a %{what}%{_}case named %{name}",
            "this is a test  case named John",
            v(&[("what", "test"), ("name", "John")]),
        ),
        (
            "this is a %{arr} %{+arr}",
            "this is a test case",
            v(&[("arr", "test case")]),
        ),
        // FIXME: Do we want to suppor those?
        // (
        //     "%{name}%{_}%{_(|)}/%{age}",
        //     "John/22",
        //     v(&[("name", "John"), ("age", "22")])),
        // ),
        // (
        //     "%{name}%{_}%{_(|)}/%{age}",
        //     "John|/22",
        //     v(&[("name", "John"), ("age", "22")])),
        // ),
        // (
        //     "%{name}%{_}%{_(|)}/%{age}",
        //     "John /22",
        //     v(&[("name", "John"), ("age", "22")])),
        // ),
        // (
        //     "%{name}%{_}%{_(|)}/ %{age}",
        //     "John|/ 22",
        //     v(&[("name", "John"), ("age", "22")])),
        // ),
        // (
        //     "%{name}%{_}%{_(|)}%{age}",
        //     "John||22",
        //     v(&[("name", "John"), ("age", "22")])),
        // ),
        (
            "%{name}%{_}%{_(|)}%{age}",
            "John 22",
            v(&[("name", "John"), ("age", "22")]),
        ),
        ("%{name} cake", "John cake", v(&[("name", "John")])),
        ("%{name} cake", "John", None),
        ("%{name}%{_}%{_(|)}%{age}", "John22", None),
        (
            "%{a}%{_}%{b}",
            "this    works",
            v(&[("a", "this"), ("b", "works")]),
        ),
        ("%{a}%{_}", "this   ", v(&[("a", "this")])),
        ("%{a}%{_}", "this", v(&[("a", "this")])),
    ];

    for (pattern, input, expected) in testcases {
        assert_eq!(run(dbg!(pattern), dbg!(input)), expected);
    }
}

#[test]
fn dissect_string_with_delimiter_at_the_end_returns_err() {
    let pattern = "%{syslog_timestamp} %{wf_host} %{syslog_program}: %{syslog_message}%{_}";
    let input = "2019-04-26 tremor-host tremor: dissect is working fine";
    assert_pattern!(pattern, input);
}

#[test]
fn dissect_with_optional_padding_in_the_middle() {
    let pattern = "%{name}%{_}|%{age}";
    let input = "John|22";
    assert_pattern!(pattern, input, ("name", "John"), ("age", "22"));
}
#[test]
fn do_extract() {
    let pattern = "this is a %{name} case";
    let input = "this is a test case";
    assert_pattern!(pattern, input, ("name", "test"))
}
#[test]
fn do_extract2() {
    assert_pattern!(
        "this is a %{what} case named %{name}",
        "this is a test case named cake",
        ("what", "test"),
        ("name", "cake")
    )
}
