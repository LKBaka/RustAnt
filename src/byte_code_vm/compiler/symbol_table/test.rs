#[cfg(test)]
mod test {
    use colored::Colorize;

    use crate::{
        byte_code_vm::compiler::symbol_table::symbol_table::{Symbol, SymbolScope, SymbolTable},
        map, rc_ref_cell,
        utils::assert_eq,
    };

    struct TestCase {
        pub table: SymbolTable,
        pub expected_symbols: Vec<Symbol>,
    }

    #[test]
    fn test_define() {
        let expected = map![
            "a",
            Symbol::new("a".into(), SymbolScope::Global, 0),
            "b",
            Symbol::new("b".into(), SymbolScope::Global, 1),
        ];

        let mut global = SymbolTable::new();

        for pair in expected {
            let name = pair.key;
            let expected_symbol = pair.value;

            let result = global.define(name);

            if result != expected_symbol {
                panic!(
                    "{}",
                    format!(
                        "expected {name} = {:?} got {name} = {:?}",
                        expected_symbol, result
                    )
                    .red()
                )
            }
        }
    }

    #[test]
    fn test_resolve_global() {
        let mut global = SymbolTable::new();

        let expected = vec![
            Symbol::new("a".into(), SymbolScope::Global, 0),
            Symbol::new("b".into(), SymbolScope::Global, 1),
            Symbol::new("c".into(), SymbolScope::Global, 2),
        ];

        for symbol in &expected {
            global.define(&symbol.name);
        }

        for expected_symbol in expected {
            let result = global.resolve(&expected_symbol.name);
            match result {
                Some(symbol) => {
                    if symbol != expected_symbol {
                        panic!(
                            "{}",
                            format!(
                                "expected {} to resolve to {:?}, got = {:?}",
                                expected_symbol.name, expected_symbol, symbol
                            )
                            .red()
                        )
                    }
                }
                None => panic!(
                    "{}",
                    format!("name {} not resolvable", expected_symbol.name).red()
                ),
            }
        }
    }

    #[test]
    fn test_resolve_local() {
        let mut global = SymbolTable::new();

        let mut global_symbols = vec![
            Symbol::new("a".into(), SymbolScope::Global, 0),
            Symbol::new("b".into(), SymbolScope::Global, 1),
        ];

        for global_symbol in &global_symbols {
            global.define(&global_symbol.name);
        }

        let mut local = SymbolTable::with_outer(rc_ref_cell!(global));

        let mut local_symbols = vec![
            Symbol::new("c".into(), SymbolScope::Local, 0),
            Symbol::new("d".into(), SymbolScope::Local, 1),
        ];

        for local_symbol in &local_symbols {
            local.define(&local_symbol.name);
        }

        local_symbols.append(&mut global_symbols);

        for expected_symbol in local_symbols {
            let result = local.resolve(&expected_symbol.name);
            match result {
                Some(symbol) => {
                    if symbol != expected_symbol {
                        panic!(
                            "{}",
                            format!(
                                "expected {} to resolve to {:?}, got = {:?}",
                                expected_symbol.name, expected_symbol, symbol
                            )
                            .red()
                        )
                    }
                }
                None => panic!(
                    "{}",
                    format!("name {} not resolvable", expected_symbol.name).red()
                ),
            }
        }
    }

    #[test]
    fn test_define_local() {
        let mut global = SymbolTable::new();

        let global_symbols = vec![
            Symbol::new("a".into(), SymbolScope::Global, 0),
            Symbol::new("b".into(), SymbolScope::Global, 1),
        ];

        for expected_symbol in &global_symbols {
            let symbol = global.define(&expected_symbol.name);
            if &symbol != expected_symbol {
                panic!(
                    "{}",
                    format!(
                        "expected {} to resolve to {:?}, got = {:?}",
                        expected_symbol.name, expected_symbol, symbol
                    )
                    .red()
                )
            }
        }

        let mut local = SymbolTable::with_outer(rc_ref_cell!(global));

        let local_symbols = vec![
            Symbol::new("c".into(), SymbolScope::Local, 0),
            Symbol::new("d".into(), SymbolScope::Local, 1),
        ];

        for expected_symbol in &local_symbols {
            let symbol = local.define(&expected_symbol.name);
            if &symbol != expected_symbol {
                panic!(
                    "{}",
                    format!(
                        "expected {} to resolve to {:?}, got = {:?}",
                        expected_symbol.name, expected_symbol, symbol
                    )
                    .red()
                )
            }
        }

        let mut second_local = SymbolTable::with_outer(rc_ref_cell!(local));

        let second_local_symbols = vec![
            Symbol::new("e".into(), SymbolScope::Local, 0),
            Symbol::new("f".into(), SymbolScope::Local, 1),
        ];

        for expected_symbol in &second_local_symbols {
            let symbol = second_local.define(&expected_symbol.name);
            if &symbol != expected_symbol {
                panic!(
                    "{}",
                    format!(
                        "expected {} to resolve to {:?}, got = {:?}",
                        expected_symbol.name, expected_symbol, symbol
                    )
                    .red()
                )
            }
        }
    }

    #[test]
    fn test_resolve_nested_local() {
        // 创建全局符号表
        let mut global = SymbolTable::new();
        global.define("a");
        global.define("b");

        // 第一层局部符号表（嵌套在全局下）
        let mut first_local = SymbolTable::with_outer(rc_ref_cell!(global));
        first_local.define("c");
        first_local.define("d");

        // 第二层局部符号表（嵌套在第一层下）
        let mut second_local = SymbolTable::with_outer(rc_ref_cell!(first_local.clone()));
        second_local.define("e");
        second_local.define("f");

        // 定义测试用例
        let test_cases = vec![
            TestCase {
                table: first_local,
                expected_symbols: vec![
                    Symbol::new("a".into(), SymbolScope::Global, 0),
                    Symbol::new("b".into(), SymbolScope::Global, 1),
                    Symbol::new("c".into(), SymbolScope::Local, 0),
                    Symbol::new("d".into(), SymbolScope::Local, 1),
                ],
            },
            TestCase {
                table: second_local,
                expected_symbols: vec![
                    Symbol::new("a".into(), SymbolScope::Global, 0),
                    Symbol::new("b".into(), SymbolScope::Global, 1),
                    Symbol::new("e".into(), SymbolScope::Local, 0),
                    Symbol::new("f".into(), SymbolScope::Local, 1),
                ],
            },
        ];

        // 遍历测试用例
        for case in test_cases {
            for expected_symbol in &case.expected_symbols {
                match case.table.resolve(&expected_symbol.name) {
                    Some(actual_symbol) => assert_eq(expected_symbol, &actual_symbol, || {
                        println!(
                            "{}",
                            format!(
                                "expected {} to resolve to {:?}, got = {:?}",
                                expected_symbol.name, expected_symbol, actual_symbol
                            )
                            .red()
                        )
                    }),
                    None => panic!(
                        "{}",
                        format!("name {} not resolvable", expected_symbol.name).red()
                    ),
                }
            }
        }
    }

    #[test]
    fn test_resolve_unresolvable_free() {
        
    }
}
