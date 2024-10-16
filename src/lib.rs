pub use style_shared::{
    AlignItems, AutoLengthPercentage, Border, BoxShadow, BoxSizing, Calc, Color, ColumnCount,
    Display, DynamicColor, FlexBasis, FlexDirection, FlexWrap, Font, FontFamily, FontStyle,
    FontWeight, JustifyContent, Length, LengthPercentage, LineStyle, LineWidth, ListStyleType,
    Margin, MarginWidth, MaxWidthHeight, NonemptyCommaList, Padding, PaddingWidth, Percentage,
    Rect, Resize, Shadow, ShadowLength, Style, Styles, TextAlign, Width21, WidthHeight,
};

/// Parse a list of css properties.
///
/// # Examples
///
/// ```
/// # use style::*;
/// // the dummy property will be ignored
/// let styles = styles! {
///     dummy;
///     height: 10px;
///     display: flex;
///     justify-content: space-around;
///     font-family: "Amatic SC", serif;
///     font-weight: 200;
///     padding: 0 1"em";
///     color: #"0ee";
/// };
///
/// // The types are quite verbose - it's much easier to construct them using the `styles!`
/// // macro.
/// assert_eq!(styles, Styles { rules: vec![
///     Style::Height(WidthHeight::LengthPercentage(Calc::Normal(
///         LengthPercentage::Length(Length::Px(10.0))
///     ))),
///     Style::Display(Display::Flex),
///     Style::JustifyContent(JustifyContent::SpaceAround),
///     Style::FontFamily(FontFamily {
///         first: Font::Named("Amatic SC".into()),
///         rest: vec![Font::Serif]
///     }),
///     Style::FontWeight(FontWeight::Number(200.0)),
///     Style::Padding(Padding::VerticalHorizontal(
///         Calc::Normal(LengthPercentage::Length(Length::Zero)),
///         Calc::Normal(LengthPercentage::Length(Length::Em(1.0))),
///     )),
///     Style::Color(DynamicColor::Literal(Color::HexRGB(0, 238, 238)))
/// ] });
/// assert_eq!(
///     styles.to_string(),
///     "height:10px;display:flex;justify-content:space-around;font-family:\"Amatic SC\",serif;\
///         font-weight:200;padding:0 1em;color:#00eeee;".to_string()
/// );
/// ```
pub use style_proc::styles;

/// Parse a list of css properties, and embed the result in the output binary as a static string.
///
/// # Examples
///
/// ```
/// # use style::*;
/// // the dummy property will be ignored
/// let STYLES: &'static str = static_styles! {
///     dummy;
///     height: 10px;
///     display: flex;
///     justify-content: space-around;
///     font-weight: 200;
///     padding: 0 1"em";
///     color: #"0ee";
/// };
///
/// assert_eq!(
///     STYLES,
///     "height:10px;display:flex;justify-content:space-around;\
///         font-weight:200;padding:0 1em;color:#00eeee;"
/// );
/// ```
pub use style_proc::static_styles;

/// Parse a css property.
///
/// # Examples
///
/// ```
/// # use style::property;
/// let prop = property!(display: flex);
/// assert_eq!(prop.to_string(), "display:flex".to_string());
///
/// let prop = property!(dummy); // special property name that means ignore
/// assert_eq!(prop.to_string(), "".to_string());
/// ```
pub use style_proc::property;

/// Parse a css color.
///
/// # Examples
///
/// ```
/// # use style::color;
/// let prop = color!(hsla(60, 50%, 30%, 0.2));
/// assert_eq!(prop.to_string(), "hsla(60, 50%, 30%, 0.2)".to_string());
/// ```
pub use style_proc::color;
