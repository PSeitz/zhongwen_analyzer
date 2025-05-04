//use lindera::token;
use pinyin::ToPinyin;
use wana_kana::IsJapaneseChar;
//use web_sys::HtmlTextAreaElement;

//use lindera::dictionary::{load_dictionary_from_kind, DictionaryKind};
//use lindera::mode::Mode;
//use lindera::segmenter::Segmenter;

use leptos::{logging, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    logging::log!("where do I run?");
    // Provlogging::log!("where do I run?");ides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-axum-test.css" />

        // sets the document title
        <Title text="Zhongwen Analyzer" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=MainComponent />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn MainComponent() -> impl IntoView {
    let submit = ServerAction::<TokenizeTexts>::new();
    let text_area_value = RwSignal::new(None::<String>);
    //let mode = RwSignal::new(DisplayMode::PinyinWithTranslation);

    view! {
        <div style="display: flex">
            <div style="flex: 1; border-right: 1px solid #ccc; padding: 10px;">
                <textarea
                    style="width: 100%; height: 100%; height: 900px; font-size: 1.5em;"
                    prop:value=move || text_area_value.get()
                    on:input:target=move |ev| {
                        let new_val = ev.target().value();
                        logging::log!("SET THE VALUE");
                        text_area_value.set(Some(new_val.clone()));
                    }
                />
            </div>
            <div style="flex: 1; padding: 10px;">
                // <button on:click=move |_| {
                // let current = mode.get();
                // let new_mode = if current == DisplayMode::PinyinWithTranslation {
                // DisplayMode::UniqueWordsWithCounts
                // } else {
                // DisplayMode::PinyinWithTranslation
                // };
                // mode.set(new_mode);
                // }>{move || {
                // if mode.get() == DisplayMode::PinyinWithTranslation {
                // "Show Unique Words with Counts".to_string()
                // } else {
                // "Show Pinyin with Translation".to_string()
                // }
                // }}</button>
                <ShowPinyinWithTranslation current_text=text_area_value />
            </div>
        </div>
    }
}

#[component]
fn ShowPinyinWithTranslation(current_text: RwSignal<Option<String>>) -> impl IntoView {
    let server_result = Resource::new(
        move || current_text.get(),
        move |text| {
            logging::log!("Fetching New {:?}", text);
            tokenize_texts(text.unwrap_or("unset".to_string()))
        },
    );

    view! {
        <>
            <Suspense fallback=|| {
                "Loading tokens..."
            }>
                {move || {
                    let tokens = server_result.get().unwrap().unwrap();
                    tokens
                        .into_iter()
                        .map(|token| {
                            let has_english = token.tokens.is_empty();
                            view! {
                                <div class="token">
                                    <span class="pinyin">{token.pinyin.clone()}</span>
                                    <span class="chinese" class:hasenglish=move || !has_english>
                                        {token.chinese.clone()}
                                    </span>
                                    <Show when=move || { !has_english }>
                                        <Tooltip tokens=token.tokens.clone() />
                                    </Show>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </Suspense>
        </>
    }
}

#[component]
pub fn Tooltip(tokens: Vec<Token>) -> impl IntoView {
    view! {
        <div class="tooltiptext">
            <TooltipContent tokens=tokens />
        </div>
    }
}

#[component]
pub fn TooltipContent(tokens: Vec<Token>) -> impl IntoView {
    tokens
        .into_iter()
        .map(|token| {
            view! {
                <div class="tooltipheader">
                <span class="chinese">
                    { token.chinese }
                </span>
                <span class="tags">
                    {move || {
                        token
                            .tags
                            .clone()
                            .into_iter()
                            .filter(|tag| {
                                tag.to_lowercase().contains("tocfl")
                                    || tag.to_lowercase().contains("hsk")
                                    || tag.to_lowercase().contains("jlpt")
                                    || tag.to_lowercase().contains("wanikani")
                            })
                            .map(|tag| {
                                // whitelist for tags
                                view! { <button class="tag">{tag}</button> }
                            })
                            .collect::<Vec<_>>()
                    }}
                </span>
                </div>

                {move || {
                    let english = token
                        .english.join("; ");
                    view! { <span class="translation">{english}</span> }
                    //token
                        //.english
                        //.clone()
                        //.into_iter()
                        //.map(|transl| {
                            //view! { <span class="translation">{transl}</span> }
                        //})
                        //.collect::<Vec<_>>()
                }}
            }
        })
        .collect::<Vec<_>>()
}

#[server]
async fn tokenize_texts(input_text: String) -> Result<Vec<TokenGroup>, ServerFnError> {
    println!("Received input: {}", input_text);

    let mut input_text = input_text.clone();
    // Remove whitespace, but keep line breaks
    input_text.retain(|c| c != ' ');

    let tokenized = ckip_ws::segment(&input_text).unwrap();
    println!("Tokenized: {:?}", tokenized);

    let index = tantivy_index::open_index("./tantivy_index/dict_index")?;

    let new_tokens: Vec<_> = tokenized
        .into_iter()
        .map(|chin| {
            let mut tokens = chinese_to_tokens(&chin, &index);
            // If there's more than one chinese char, call chinese_to_tokens for every char
            for cha in chin.chars() {
                tokens.extend(chinese_to_tokens(&cha.to_string(), &index));
            }
            TokenGroup {
                chinese: chin.clone(),
                tokens,
                pinyin: get_pinyin(&chin),
            }
        })
        .collect();

    Ok(new_tokens)
}

#[cfg(feature = "ssr")]
fn chinese_to_tokens(chinese: &str, index: &tantivy_index::Index) -> Vec<Token> {
    let pinyin = get_pinyin(chinese);
    let query = format!("traditional:{chinese} AND pinyin_search:\"{pinyin}\"");
    //println!("Query: {}", query);
    let res = tantivy_index::search(&query, index).unwrap_or_default();
    res.iter()
        .map(Token::from_entry)
        //.unwrap_or_else(|| Token {
        //chinese: chinese.to_string(),
        //pinyin: pinyin.clone(),
        //english: Vec::new(),
        //tags: Vec::new(),
        //})
        .collect()
}

#[derive(
    PartialEq, Clone, leptos::server_fn::serde::Serialize, leptos::server_fn::serde::Deserialize,
)]
pub struct TokenGroup {
    chinese: String,
    pinyin: String,
    tokens: Vec<Token>,
}

#[derive(
    PartialEq, Clone, leptos::server_fn::serde::Serialize, leptos::server_fn::serde::Deserialize,
)]
pub struct Token {
    chinese: String,
    pinyin: String,
    english: Vec<String>,
    tags: Vec<String>,
}

impl Token {
    #[cfg(feature = "ssr")]
    pub fn from_entry(entry: &tantivy_index::Entry) -> Self {
        Token {
            chinese: entry.traditional.clone(),
            pinyin: entry.pinyin_pretty.clone(),
            english: entry.meanings.clone(),
            tags: entry.tags.clone(),
        }
    }
}

#[derive(Clone, PartialEq)]
enum DisplayMode {
    PinyinWithTranslation,
    UniqueWordsWithCounts,
}

//#[derive(Clone, PartialEq)]
//enum DisplayMode {
//PinyinWithTranslation,
//UniqueWordsWithCounts,
//}

fn is_chinese_word(text: &str) -> bool {
    text.chars().all(|c| c.is_kanji())
}

//#[function_component(RightPane)]
//fn right_pane(AnalyzedTextProps { tokens }: &AnalyzedTextProps) -> Html {
//let mode = use_state_eq(|| DisplayMode::PinyinWithTranslation);

//let unique_words = {
//let mut word_count = std::collections::HashMap::new();
//for token in tokens
//.iter()
//.filter(|token| is_chinese_word(&token.chinese))
//{
//*word_count.entry(&token.chinese).or_insert(0) += 1;
//}
//// sort by count
//let mut word_count: Vec<_> = word_count.into_iter().collect();
//word_count.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

//word_count
//};

////let on_mode_change = {
////let mode = mode.clone();
////Callback::from(move |e: Event| {
////let select_element = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
////let value = select_element.value();
////let new_mode = match value.as_str() {
////"UniqueWordsWithCounts" => DisplayMode::UniqueWordsWithCounts,
////_ => DisplayMode::PinyinWithTranslation,
////};
////mode.set(new_mode);
////})
////};

//html! {
//<div style="flex: 1; padding: 10px;">
//<select value={match *mode { DisplayMode::PinyinWithTranslation => "PinyinWithTranslation", DisplayMode::UniqueWordsWithCounts => "UniqueWordsWithCounts" }}>
//<option value="PinyinWithTranslation">{"Pinyin with Translation"}</option>
//<option value="UniqueWordsWithCounts">{"Unique Words with Counts"}</option>
//</select>
//{
//match *mode {
//DisplayMode::PinyinWithTranslation => html! {
//{ for tokens.iter().map(|token| {
//html! {
//<div style="display: inline-block; text-align: center; padding-left: 5px" title={token.english.clone()}>
//<span style="display: block; font-size: 0.8em; color: gray;">{token.pinyin.clone()}</span>
//<span>{token.chinese.clone()}</span>
//</div>
//}
//})}
//},
//DisplayMode::UniqueWordsWithCounts => html! {
//{ for unique_words.iter().map(|(word, count)| {
//html! {
//<div style="display: block; text-align: left; padding-left: 5px">
//<span>{format!("{} ({}): {}", word, count, "")}</span>
//</div>
//}
//})}
//},
//}
//}
//</div>
//}
//}
fn get_pinyin(chin: &str) -> String {
    let pinyin: String = chin
        .chars()
        .flat_map(|c| c.to_pinyin())
        .map(|p| p.with_tone())
        .collect();
    pinyin
}

//fn tokenize(form: String, dictionary: &dictionary::Dictionary) -> Vec<String> {
//let jieba = Jieba::new();
//let tokens: Vec<_> = jieba.cut(&form, true);
//let pinyin: Vec<_> = form
//.chars()
//.flat_map(|c| c.to_pinyin())
//.map(|p| p.with_tone())
//.collect();

//let dictionary = load_dictionary_from_kind(DictionaryKind::CcCedict).unwrap();
//let segmenter = Segmenter::new(
//Mode::Normal,
//dictionary,
//None, // Assuming no user dictionary is provided
//);
//let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
//let input_text =
//"在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南";
//let tokenized = tokenizer
//.tokenize(input_text)
//.unwrap()
//.iter()
//.map(|token| token.text.to_string())
//.collect::<Vec<_>>();
//tokenized
//}

#[cfg(test)]
mod test {
    //#[test]
    //fn test_jieba() {
    //let jieba = jieba_rs::Jieba::new();
    //let input_text = "都會告訴爸爸在校發生了什麼事";
    //let tokenized: Vec<_> = jieba.cut(input_text, true);
    //assert_eq!(
    //tokenized,
    //vec!["都", "會", "告訴", "爸爸", "在校", "發生", "了", "什麼", "事"]
    //);
    //let input_text =
    //"在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南";
    //let tokenized: Vec<_> = jieba.cut(input_text, true);
    //assert_eq!(
    //tokenized,
    //vec!["都", "會", "告訴", "爸爸", "在校", "發生", "了", "什麼", "事"]
    //);
    //}

    //#[test]
    //fn test_lindera() {
    //use lindera::dictionary::{load_dictionary_from_kind, DictionaryKind};
    //use lindera::mode::Mode;
    //use lindera::segmenter::Segmenter;

    //let dictionary = load_dictionary_from_kind(DictionaryKind::CcCedict).unwrap();
    //let segmenter = Segmenter::new(
    //Mode::Normal,
    //dictionary,
    //None, // Assuming no user dictionary is provided
    //);
    //let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
    //let input_text =
    //"在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南";
    //let tokenized = tokenizer
    //.tokenize(input_text)
    //.unwrap()
    //.iter()
    //.map(|token| token.text.to_string())
    //.collect::<Vec<_>>();
    //assert_eq!(
    //tokenized,
    //vec!["都會", "告訴", "爸爸", "在", "校", "發生", "了", "什麼事"]
    //);
    //// check all tokens
    //}

    //#[test]
    //pub fn test_ik() {
    //use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};
    //letmut ik = IKSegmenter::new();
    //let text = "都會告訴爸爸在校發生了什麼事";
    //let tokens = ik.tokenize(text, TokenMode::INDEX); // TokenMode::SEARCH
    //let mut token_texts = Vec::new();
    //for token in tokens.iter() {
    //println!("{:?}", token);
    //token_texts.push(token.lexeme_text());
    //}
    //assert_eq!(
    //token_texts,
    //vec![
    //"中华人民共和国",
    //"中华人民",
    //"中华",
    //"华人",
    //"人民共和国",
    //"人民",
    //"共和国",
    //"共和",
    //"国"
    //]
    //)
    //}
}
