use std::fmt::Display;

impl Display for super::core::DFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Start Status: {}", self.init_status)?;
        writeln!(f, "Line Start Status: {}", self.line_start)?;

        writeln!(f, "TransForm Table")?;
        writeln!(f, "{:^20}|{:^20}|{:^20}", 'S', 'I', 'D')?;
        writeln!(f, "{0:-^20}|{0:-^20}|{0:-^20}", '-')?;
        for ((i, s), d) in &self.table {
            writeln!(f, "{:^20}|{:^20}|{:^20}", s, i, d)?;
        }
        writeln!(f, "{0:-^20}|{0:-^20}|{0:-^20}", '-')?;
        writeln!(f, "Accept Table")?;
        writeln!(f, "S")?;
        for (s, h) in &self.final_status {
            writeln!(f, "{} -> {}", s, h.name())?;
        }

        writeln!(f, "Any Trans Table")?;
        writeln!(f, "S")?;
        for (s, _h) in &self.any_trans {
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Display for super::wraps::Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("Status [ {:2} ]", self.0);
        let c = f.fill();
        if let Some(w) = f.width() {
            if let Some(align) = f.align() {
                match align {
                    std::fmt::Alignment::Left => {
                        write!(f, "{}", s)?;
                        for _ in 0..w - s.len() {
                            write!(f, "{}", c)?;
                        }
                    }
                    std::fmt::Alignment::Right => {
                        for _ in 0..w - s.len() {
                            write!(f, "{}", c)?;
                        }
                        write!(f, "{}", s)?;
                    }
                    std::fmt::Alignment::Center => {
                        let left = (w - s.len()) / 2;
                        for _ in 0..left {
                            write!(f, "{}", c)?;
                        }
                        write!(f, "{}", s)?;
                        for _ in 0..w - left - s.len() {
                            write!(f, "{}", c)?;
                        }
                    }
                };
            };
        } else {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Display for super::wraps::InputChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            super::wraps::InputChar::LineStart => format!("Line Start"),
            super::wraps::InputChar::Char(c) => format!("Char[{:?}]", &c),
            super::wraps::InputChar::Eof => format!("End Of File"),
            super::wraps::InputChar::Any(a) => format!("Any<{}>", &a),
        };
        let c = f.fill();
        if let Some(w) = f.width() {
            if let Some(align) = f.align() {
                match align {
                    std::fmt::Alignment::Left => {
                        write!(f, "{}", s)?;
                        for _ in 0..w - s.len() {
                            write!(f, "{}", c)?;
                        }
                    }
                    std::fmt::Alignment::Right => {
                        for _ in 0..w - s.len() {
                            write!(f, "{}", c)?;
                        }
                        write!(f, "{}", s)?;
                    }
                    std::fmt::Alignment::Center => {
                        let left = (w - s.len()) / 2;
                        for _ in 0..left {
                            write!(f, "{}", c)?;
                        }
                        write!(f, "{}", s)?;
                        for _ in 0..w - left - s.len() {
                            write!(f, "{}", c)?;
                        }
                    }
                };
            };
        } else {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Display for super::wraps::AnyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            super::wraps::AnyType::Any => write!(f, "Any"),
            super::wraps::AnyType::Digit => write!(f, "Digit"),
            super::wraps::AnyType::Alphabet => write!(f, "Alphabet"),
            super::wraps::AnyType::LowerCase => write!(f, "LowerCase"),
            super::wraps::AnyType::UpperCase => write!(f, "UpperCase"),
            super::wraps::AnyType::WhiteSpace => write!(f, "WhiteSpace"),
            super::wraps::AnyType::Numer => write!(f, "Numer"),
            super::wraps::AnyType::Ascii => write!(f, "Ascii"),
            super::wraps::AnyType::Orther(i) => write!(f, "Orther({})", i),
            super::wraps::AnyType::Or(l, r) => write!(f, "( {} | {} )", l, r),
            super::wraps::AnyType::Char(c) => write!(f, "Char [{}]", c),
            super::wraps::AnyType::And(l, r) => write!(f, "( {} & {} )", l, r),
            super::wraps::AnyType::Not(s) => write!(f, "(Not {} )", s),
        }
    }
}
