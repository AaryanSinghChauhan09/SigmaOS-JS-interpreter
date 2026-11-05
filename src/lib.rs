#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsValue {
    Undefined,
    Boolean(bool),
    Number(i64),
    String(String),
}

pub struct JsInterpreterHost;

impl JsInterpreterHost {
    pub fn eval_simple_expression(expr: &str) -> JsValue {
        let trimmed = expr.trim();
        if trimmed == "true" {
            JsValue::Boolean(true)
        } else if trimmed == "false" {
            JsValue::Boolean(false)
        } else if let Ok(n) = trimmed.parse::<i64>() {
            JsValue::Number(n)
        } else if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            JsValue::String(trimmed[1..trimmed.len()-1].to_string())
        } else {
            JsValue::Undefined
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_js_interpreter_eval_aaa() {
        // Arrange & Act
        let v1 = JsInterpreterHost::eval_simple_expression("true");
        let v2 = JsInterpreterHost::eval_simple_expression("42");
        let v3 = JsInterpreterHost::eval_simple_expression(r#""SigmaOS""#);
        // Assert
        assert_eq!(v1, JsValue::Boolean(true));
        assert_eq!(v2, JsValue::Number(42));
        assert_eq!(v3, JsValue::String("SigmaOS".into()));
    }
}
