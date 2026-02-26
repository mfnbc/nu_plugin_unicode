use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand,
};
use nu_protocol::{Category, Example, Signature, PipelineData, SyntaxShape, Value, Type, LabeledError};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

struct UnicodePlugin;

impl Plugin for UnicodePlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(CharsCommand),
            Box::new(CodepointsCommand),
            Box::new(GraphemesCommand),
            Box::new(NormalizeCommand),
        ]
    }
}

// --- COMMAND: unicode chars ---
struct CharsCommand;
impl PluginCommand for CharsCommand {
    type Plugin = UnicodePlugin;
    fn name(&self) -> &str { "unicode chars" }
    fn signature(&self) -> Signature {
        Signature::build("unicode chars")
            .input_output_types(vec![
                (Type::String, Type::Table(vec![].into()))
            ])
            .category(Category::Strings)
    }
    fn description(&self) -> &str { "Split text into Unicode scalar values (chars) with codepoints" }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let (text, _, _) = input.collect_string_strict(span)?;
        let result: Vec<Value> = text.chars().map(|c| {
            Value::record(
                nu_protocol::record! {
                    "ch" => Value::string(c.to_string(), span),
                    "cp" => Value::int(c as i64, span),
                },
                span
            )
        }).collect();
        Ok(PipelineData::Value(Value::list(result, span), None))
    }
}

// --- COMMAND: unicode codepoints ---
struct CodepointsCommand;
impl PluginCommand for CodepointsCommand {
    type Plugin = UnicodePlugin;
    fn name(&self) -> &str { "unicode codepoints" }
    fn signature(&self) -> Signature {
        Signature::build("unicode codepoints")
            .input_output_types(vec![(
                Type::String,
                Type::List(Box::new(Type::Int))
            )])
            .category(Category::Strings)
    }
    fn description(&self) -> &str { "Returns a flat list of Unicode codepoints (integers)" }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let (text, _, _) = input.collect_string_strict(span)?;
        let result: Vec<Value> = text.chars()
            .map(|c| Value::int(c as i64, span))
            .collect();
        Ok(PipelineData::Value(Value::list(result, span), None))
    }
}

// --- COMMAND: unicode graphemes ---
struct GraphemesCommand;
impl PluginCommand for GraphemesCommand {
    type Plugin = UnicodePlugin;
    fn name(&self) -> &str { "unicode graphemes" }
    fn signature(&self) -> Signature {
        Signature::build("unicode graphemes")
            .input_output_types(vec![(
                Type::String,
                Type::Table(vec![].into())
            )])
            .category(Category::Strings)
    }
    fn description(&self) -> &str { "Split text into grapheme clusters (user-perceived characters)" }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let (text, _, _) = input.collect_string_strict(span)?;
        let result: Vec<Value> = text.graphemes(true).map(|g| {
            let cps: Vec<Value> = g.chars()
                .map(|c| Value::int(c as i64, span))
                .collect();
            Value::record(
                nu_protocol::record! {
                    "grapheme" => Value::string(g, span),
                    "cps" => Value::list(cps, span)
                },
                span
            )
        }).collect();
        Ok(PipelineData::Value(Value::list(result, span), None))
    }
}

// --- COMMAND: unicode normalize ---
struct NormalizeCommand;
impl PluginCommand for NormalizeCommand {
    type Plugin = UnicodePlugin;
    fn name(&self) -> &str { "unicode normalize" }
    fn signature(&self) -> Signature {
        Signature::build("unicode normalize")
            .required("form", SyntaxShape::String, "nfc, nfd, nfkc, nfkd")
            .input_output_types(vec![(
                Type::String,
                Type::String
            )])
            .category(Category::Strings)
    }
    fn description(&self) -> &str { "Normalize Unicode text" }
    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let (text, _, _) = input.collect_string_strict(span)?;
        let form: String = call.req(0)?;
        let normalized = match form.to_lowercase().as_str() {
            "nfc" => text.nfc().collect::<String>(),
            "nfd" => text.nfd().collect::<String>(),
            "nfkc" => text.nfkc().collect::<String>(),
            "nfkd" => text.nfkd().collect::<String>(),
            _ => return Err(LabeledError::new("Invalid form").with_label("Use nfc, nfd, nfkc, or nfkd", span)),
        };
        Ok(PipelineData::Value(Value::string(normalized, span), None))
    }
}

fn main() {
    serve_plugin(&UnicodePlugin, MsgPackSerializer);
}

