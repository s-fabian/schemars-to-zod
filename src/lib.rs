//! ## schemars-to-zod
//!
//! A library for converting schemars's `Schema`s
//! into zod schemas
//!
//! Example:
//!
//! ```
//! # #[allow(unused)]
//! # fn main() {
//! use schemars::{JsonSchema, schema_for, schema::Schema};
//! use schemars_to_zod::Parser;
//!
//! #[derive(JsonSchema)]
//! struct MyStruct {
//!     name: String,
//!     age: u8,
//! }
//!
//! let schema = schema_for!(MyStruct);
//! let schema = Schema::Object(schema.schema);
//!
//! let parser = Parser::default();
//! // with the feature pretty
//! let result = parser.parse_pretty_default(&schema).unwrap();
//! // without the feature pretty
//! let result = parser.parse(&schema).unwrap();
//!
//! println!("{result}");
//! # }

#![warn(missing_docs)]

use std::fmt::{Display, Formatter};

use schemars::schema::Schema;

mod parsers;

#[cfg(feature = "pretty")]
pub(crate) use dprint_plugin_typescript::configuration::Configuration as PrettyConfig;

#[cfg(feature = "pretty")]
use crate::pretty::{default_pretty_conf, format_js};

#[derive(Debug)]
/// The errors which my be returned from the
/// `Parser::parse` functions
pub enum Error {
    /// An invalid schema was given
    SchemaError(&'static str),
    /// Parts of the schema are not supported yet
    Unimplemented(&'static str),
    /// A parse function was called on an
    /// unsupported schema.
    ///
    /// This may get returned when using the
    /// `inner` feature
    ForgotCheck(&'static str),
    /// serde_json::to_string returned an error
    JsonError(serde_json::Error),
    /// Formatting the code went wrong
    #[cfg(feature = "pretty")]
    PrettifyError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SchemaError(err) => write!(f, "Invalid schema: {err}"),
            Error::Unimplemented(err) => write!(f, "Unimplemented: {err}"),
            Error::ForgotCheck(err) => write!(f, "Forgot a check: {err}"),
            Error::JsonError(err) => write!(f, "Serde error: {err}"),
            #[cfg(feature = "pretty")]
            Error::PrettifyError => write!(f, "Error when prettifying"),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self { Error::JsonError(value) }
}

pub(crate) type ParserResult = Result<String, Error>;

#[cfg(feature = "pretty")]
pub mod pretty {
    //! Helper functions and dprint exports

    use std::path::Path;

    pub use dprint_core::configuration::NewLineKind;
    pub use dprint_plugin_typescript::configuration::*;
    use dprint_plugin_typescript::format_text;

    use super::PrettyConfig;

    /// Format the given js/ts/jsx/tsx code with
    /// the given config
    pub fn format_js(
        text: &str,
        extension: &str,
        config: &PrettyConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = if extension.contains('.') {
            extension.to_string()
        } else {
            format!(".{extension}")
        };

        Ok(format_text(Path::new(&path), text, config)?
            .ok_or(String::from("Error: could not format js"))?)
    }

    /// Get the default `PrettyConfig`
    pub fn default_pretty_conf() -> PrettyConfig {
        use crate::pretty::*;

        PrettyConfig {
            // not important
            jsx_quote_style: JsxQuoteStyle::PreferDouble,
            jsx_multi_line_parens: JsxMultiLineParens::Never,
            jsx_force_new_lines_surrounding_content: false,
            jsx_opening_element_bracket_position: SameOrNextLinePosition::Maintain,
            jsx_self_closing_element_bracket_position: SameOrNextLinePosition::Maintain,
            jsx_attributes_prefer_single_line: false,
            jsx_element_prefer_single_line: false,
            // not important
            module_sort_import_declarations: SortOrder::Maintain,
            module_sort_export_declarations: SortOrder::Maintain,
            import_declaration_sort_named_imports: SortOrder::Maintain,
            export_declaration_sort_named_exports: SortOrder::Maintain,
            import_declaration_force_single_line: false,
            export_declaration_force_single_line: false,
            export_declaration_force_multi_line: false,
            import_declaration_force_multi_line: false,
            ignore_node_comment_text: "".to_string(),
            ignore_file_comment_text: "".to_string(),
            // important
            indent_width: 2,
            line_width: 60,
            use_tabs: false,
            new_line_kind: NewLineKind::Auto,
            quote_style: QuoteStyle::AlwaysSingle,
            quote_props: QuoteProps::AsNeeded,
            semi_colons: SemiColons::Always,
            arrow_function_use_parentheses: UseParentheses::PreferNone,
            binary_expression_line_per_expression: false,
            conditional_expression_line_per_expression: false,
            member_expression_line_per_expression: false,
            type_literal_separator_kind_single_line: SemiColonOrComma::SemiColon,
            type_literal_separator_kind_multi_line: SemiColonOrComma::SemiColon,
            arrow_function_brace_position: BracePosition::Maintain,
            class_declaration_brace_position: BracePosition::Maintain,
            class_expression_brace_position: BracePosition::Maintain,
            constructor_brace_position: BracePosition::Maintain,
            do_while_statement_brace_position: BracePosition::Maintain,
            enum_declaration_brace_position: BracePosition::Maintain,
            get_accessor_brace_position: BracePosition::Maintain,
            if_statement_brace_position: BracePosition::Maintain,
            interface_declaration_brace_position: BracePosition::Maintain,
            for_statement_brace_position: BracePosition::Maintain,
            for_in_statement_brace_position: BracePosition::Maintain,
            for_of_statement_brace_position: BracePosition::Maintain,
            function_declaration_brace_position: BracePosition::Maintain,
            function_expression_brace_position: BracePosition::Maintain,
            method_brace_position: BracePosition::Maintain,
            module_declaration_brace_position: BracePosition::Maintain,
            set_accessor_brace_position: BracePosition::Maintain,
            static_block_brace_position: BracePosition::Maintain,
            switch_case_brace_position: BracePosition::Maintain,
            switch_statement_brace_position: BracePosition::Maintain,
            try_statement_brace_position: BracePosition::Maintain,
            while_statement_brace_position: BracePosition::Maintain,
            arguments_prefer_hanging: PreferHanging::Never,
            array_expression_prefer_hanging: PreferHanging::Never,
            array_pattern_prefer_hanging: false,
            do_while_statement_prefer_hanging: false,
            export_declaration_prefer_hanging: false,
            extends_clause_prefer_hanging: false,
            for_statement_prefer_hanging: false,
            for_in_statement_prefer_hanging: false,
            for_of_statement_prefer_hanging: false,
            if_statement_prefer_hanging: false,
            implements_clause_prefer_hanging: false,
            import_declaration_prefer_hanging: false,
            jsx_attributes_prefer_hanging: false,
            object_expression_prefer_hanging: false,
            object_pattern_prefer_hanging: false,
            parameters_prefer_hanging: PreferHanging::Never,
            sequence_expression_prefer_hanging: false,
            switch_statement_prefer_hanging: false,
            tuple_type_prefer_hanging: PreferHanging::Never,
            type_literal_prefer_hanging: false,
            type_parameters_prefer_hanging: PreferHanging::Never,
            union_and_intersection_type_prefer_hanging: false,
            variable_statement_prefer_hanging: false,
            while_statement_prefer_hanging: false,
            enum_declaration_member_spacing: MemberSpacing::Maintain,
            if_statement_next_control_flow_position: NextControlFlowPosition::Maintain,
            try_statement_next_control_flow_position: NextControlFlowPosition::Maintain,
            do_while_statement_next_control_flow_position:
                NextControlFlowPosition::Maintain,
            binary_expression_operator_position: OperatorPosition::Maintain,
            conditional_expression_operator_position: OperatorPosition::Maintain,
            conditional_type_operator_position: OperatorPosition::Maintain,
            if_statement_single_body_position: SameOrNextLinePosition::Maintain,
            for_statement_single_body_position: SameOrNextLinePosition::Maintain,
            for_in_statement_single_body_position: SameOrNextLinePosition::Maintain,
            for_of_statement_single_body_position: SameOrNextLinePosition::Maintain,
            while_statement_single_body_position: SameOrNextLinePosition::Maintain,
            arguments_trailing_commas: TrailingCommas::OnlyMultiLine,
            parameters_trailing_commas: TrailingCommas::OnlyMultiLine,
            array_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
            array_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
            enum_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
            export_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
            import_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
            object_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
            object_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
            tuple_type_trailing_commas: TrailingCommas::Never,
            type_literal_trailing_commas: TrailingCommas::Never,
            type_parameters_trailing_commas: TrailingCommas::Never,
            if_statement_use_braces: UseBraces::Maintain,
            for_statement_use_braces: UseBraces::Maintain,
            for_of_statement_use_braces: UseBraces::Maintain,
            for_in_statement_use_braces: UseBraces::Maintain,
            while_statement_use_braces: UseBraces::Maintain,
            array_expression_prefer_single_line: false,
            array_pattern_prefer_single_line: false,
            arguments_prefer_single_line: false,
            binary_expression_prefer_single_line: false,
            computed_prefer_single_line: false,
            conditional_expression_prefer_single_line: false,
            conditional_type_prefer_single_line: false,
            decorators_prefer_single_line: false,
            export_declaration_prefer_single_line: false,
            for_statement_prefer_single_line: false,
            import_declaration_prefer_single_line: false,
            mapped_type_prefer_single_line: false,
            member_expression_prefer_single_line: false,
            object_expression_prefer_single_line: false,
            object_pattern_prefer_single_line: false,
            parameters_prefer_single_line: false,
            parentheses_prefer_single_line: false,
            tuple_type_prefer_single_line: false,
            type_literal_prefer_single_line: false,
            type_parameters_prefer_single_line: false,
            union_and_intersection_type_prefer_single_line: false,
            variable_statement_prefer_single_line: false,
            binary_expression_space_surrounding_bitwise_and_arithmetic_operator: true,
            comment_line_force_space_after_slashes: true,
            construct_signature_space_after_new_keyword: false,
            constructor_space_before_parentheses: true,
            constructor_type_space_after_new_keyword: true,
            do_while_statement_space_after_while_keyword: true,
            export_declaration_space_surrounding_named_exports: false,
            for_statement_space_after_for_keyword: true,
            for_statement_space_after_semi_colons: true,
            for_in_statement_space_after_for_keyword: true,
            for_of_statement_space_after_for_keyword: true,
            function_declaration_space_before_parentheses: false,
            function_expression_space_before_parentheses: false,
            function_expression_space_after_function_keyword: false,
            get_accessor_space_before_parentheses: false,
            if_statement_space_after_if_keyword: true,
            import_declaration_space_surrounding_named_imports: false,
            jsx_expression_container_space_surrounding_expression: false,
            jsx_self_closing_element_space_before_slash: false,
            method_space_before_parentheses: false,
            object_expression_space_surrounding_properties: true,
            object_pattern_space_surrounding_properties: true,
            set_accessor_space_before_parentheses: false,
            space_surrounding_properties: false,
            tagged_template_space_before_literal: false,
            type_annotation_space_before_colon: false,
            type_assertion_space_before_expression: false,
            type_literal_space_surrounding_properties: false,
            while_statement_space_after_while_keyword: true,
            arguments_space_around: false,
            array_expression_space_around: false,
            array_pattern_space_around: false,
            do_while_statement_space_around: false,
            for_in_statement_space_around: false,
            for_of_statement_space_around: false,
            for_statement_space_around: false,
            if_statement_space_around: false,
            parameters_space_around: false,
            switch_statement_space_around: false,
            tuple_type_space_around: false,
            while_statement_space_around: false,
        }
    }
}

/// Configuration for the parser
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Output `z.coerce.date()` instead of
    /// `z.date()`
    pub use_coerce_date: bool,
    /// Output `z.array(z.any())` instead of
    /// `z.any().array()`
    pub array_wrapper: bool,
    /// Output `z.number().gte(10)` instead of
    /// `z.number().min(10)`
    pub explicit_min_max: bool,
    /// Add `.describe("description")` to schemas
    /// with a description
    pub add_descriptions: bool,
    /// Output `z.union([]).and(z.object({}))`
    /// instead of
    /// `z.object({}).and(z.union([]))`
    pub union_first: bool,
    /// Add `.default(123)` to schemas with a
    /// default value inside of objects
    pub add_default: bool,
}

/// The inside of the parser, which contains all
/// helper methods
#[cfg(feature = "inner")]
pub struct ParserInner {
    config: Config,
}
#[cfg(not(feature = "inner"))]
pub(crate) struct ParserInner {
    config: Config,
}

/// The main parser which can convert schemars's
/// `Schema` into a zod schema.
///
/// Example:
///
/// ```
/// # #[allow(unused)]
/// # fn main() {
/// use schemars::{schema::Schema, schema_for, JsonSchema};
/// use schemars_to_zod::Parser;
///
/// #[derive(JsonSchema)]
/// struct MyStruct {
///     name: String,
///     age: u8,
/// }
///
/// let schema = schema_for!(MyStruct);
/// let schema = Schema::Object(schema.schema);
///
/// let parser = Parser::default();
/// // with the feature pretty
/// let result = parser.parse_pretty_default(&schema).unwrap();
/// // without the feature pretty
/// let result = parser.parse(&schema).unwrap();
///
/// println!("{result}");
/// # }
/// ```
#[repr(transparent)]
pub struct Parser(
    #[cfg(feature = "inner")] pub ParserInner,
    #[cfg(not(feature = "inner"))] pub(crate) ParserInner,
);

impl Default for Parser {
    fn default() -> Self {
        Self(ParserInner {
            config: Default::default(),
        })
    }
}

impl Parser {
    /// Create a new parser
    pub fn new(config: Config) -> Self { Parser(ParserInner { config }) }

    /// Parse a schema
    pub fn parse(&self, schema: &Schema) -> ParserResult { self.0.parse_schema(schema) }

    /// Parse a schema and format it with the
    /// given config
    #[cfg(feature = "pretty")]
    pub fn parse_pretty(&self, schema: &Schema, config: &PrettyConfig) -> ParserResult {
        let parsed = self.0.parse_schema(schema)?;

        format_js(&parsed, ".js", config)
            .ok()
            .ok_or(Error::PrettifyError)
    }

    /// Parse a schema and format it with the
    /// default config
    #[cfg(feature = "pretty")]
    pub fn parse_pretty_default(&self, schema: &Schema) -> ParserResult {
        let parsed = self.0.parse_schema(schema)?;

        format_js(&parsed, ".js", &default_pretty_conf())
            .ok()
            .ok_or(Error::PrettifyError)
    }
}

#[cfg(feature = "inner")]
impl std::ops::Deref for Parser {
    type Target = ParserInner;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(test)]
mod test_helpers {
    use schemars::gen::{SchemaGenerator, SchemaSettings};

    pub(crate) fn generator() -> SchemaGenerator {
        let mut settings = SchemaSettings::default();
        settings.inline_subschemas = true;
        settings.meta_schema =
            Some("http://json-schema.org/draft-03/hyper-schema".to_string());
        SchemaGenerator::new(settings)
    }
}
