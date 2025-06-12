use font_kit::source::SystemSource;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let arg = &std::env::args().collect::<Vec<String>>();
    let wanted_font_name;

    if arg.len() != 2 {
        println!("用法: {} <font-name>", arg[0]);
        return Ok(());
    } else {
        wanted_font_name = arg[1].clone();
    }

    // 创建系统字体源
    let source = SystemSource::new();

    // 预保存字体全名数据
    let mut uniq_font_full_names = HashSet::new();

    // 遍历所有可能的字体源并保存字体全名
    if let Ok(sources) = source.all_fonts() {
        for handle in sources {
            match handle.load() {
                Ok(font) => {
                    uniq_font_full_names.insert(font.full_name());
                }
                Err(_) => continue, // 跳过加载失败的字体
            }
        }
    }

    let mut result_font_names = Vec::new();

    for font_full_name in uniq_font_full_names {
        if matched(&wanted_font_name, &font_full_name) {
            result_font_names.push(font_full_name);
        }
    }

    if result_font_names.is_empty() {
        println!("未找到匹配的字体！");
    } else {
        println!("找到以下匹配的字体字重全名：");
        for font_full_name in result_font_names {
            println!("{}", font_full_name);
        }
    }

    Ok(())
}

fn matched(wanted_text: &str, full_text: &str) -> bool {
    // 直接使用 Rust 的字符迭代器进行比较
    let mut full_chars = full_text.chars();

    // 检查是否匹配目标文本的每个字符
    for want_char in wanted_text.chars() {
        match full_chars.next() {
            Some(c) if c == want_char => continue,
            _ => return false,
        }
    }

    // 检查匹配后的边界情况
    match full_chars.next() {
        None => true,                                 // 后面没有字符了
        Some(next_char) => next_char.is_whitespace(), // 下一个字符必须是空格
    }
}