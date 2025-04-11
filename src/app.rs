use pinyin::ToPinyin;
use wana_kana::IsJapaneseChar;
use web_sys::HtmlTextAreaElement;

use lindera::dictionary::{load_dictionary_from_kind, DictionaryKind};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;

use leptos::{logging, prelude::*, task::spawn_local};
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
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
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
        <Stylesheet id="leptos" href="/pkg/leptos-axum-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=MainComponent/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn MainComponent() -> impl IntoView {
    let submit = ServerAction::<VeryImportantFn>::new();
    let some_value = RwSignal::new("initial value".to_string());
    let server_result = RwSignal::new(None::<String>);

    view! {
      <>
        <ActionForm action=submit>
          <textarea
              prop:value=move || some_value.get()
              on:input:target=move |ev| {
                  let new_val = ev.target().value();
                  some_value.set(new_val.clone());
                  {
                      spawn_local(async move {
                          let res = very_important_fn(new_val).await.unwrap();
                          server_result.set(Some(res));
                      });
                  }
              }
          />
        </ActionForm>
        <p>{move || server_result.get().clone().unwrap_or_default()}</p>
      </>
    }
}
#[server]
async fn very_important_fn(input_text: String) -> Result<String, ServerFnError> {
    println!("Received input: {}", input_text);

    let mut input_text = input_text.clone();
    // Remove whitespace, but keep line breaks
    input_text.retain(|c| c != ' ');
    //input_element.set_value(&input_text);
    //let tokenized: Vec<_> = jieba.cut(&input_text, true);
    //let tokenized: Vec<_> = tokenizer.tokenize(input_text.clone(), &dictionary).unwrap();
    let dictionary = load_dictionary_from_kind(DictionaryKind::CcCedict).unwrap();
    let segmenter = Segmenter::new(
        Mode::Normal,
        dictionary,
        None, // Assuming no user dictionary is provided
    );
    let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
    let tokenized = tokenizer
        .tokenize(&input_text)
        .unwrap()
        .iter()
        .map(|token| token.text.to_string())
        .collect::<Vec<_>>();

    //let tokenized: Vec<_> = input_text.split("").collect();

    let new_tokens: Vec<_> = tokenized
        .into_iter()
        .map(|chin| {
            Token {
                chinese: chin.to_string(),
                pinyin: get_pinyin(&chin),
                english: "test".to_string(), // Placeholder for translation
                meaning: None,               // Placeholder for meaning
            }
        })
        .collect();

    Ok(format!("Processed input: {}", input_text))
}

//#[function_component(App)]
//fn app() -> Html {
//let tokens = use_state(Vec::<Token>::new);
////let jieba = Jieba::new();
//let dictionary = load_dictionary_from_kind(DictionaryKind::CcCedict).unwrap();
//let segmenter = Segmenter::new(
//Mode::Normal,
//dictionary,
//None, // Assuming no user dictionary is provided
//);
//let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
////let tokenizer =
//let on_input = {
//let tokens = tokens.clone();
//Callback::from(move |e: InputEvent| {
//let input_element = e.target_dyn_into::<HtmlTextAreaElement>();
//let input_element = match input_element {
//Some(element) => element,
//None => return,
//};

//tokens.set(new_tokens);
//})
//};
//html! {
//<div style="display: flex; height: 100vh">
//<LeftPane on_input={on_input} />
//<RightPane tokens={(*tokens).clone()} />
//</div>
//}
//}

//#[function_component(LeftPane)]
//fn left_pane(LeftPaneProps { on_input }: &LeftPaneProps) -> Html {
//html! {
//<div style="flex: 1; border-right: 1px solid #ccc; padding: 10px;">
//<textarea style="width: 100%; height: 100%;font-size: 1.5em;" oninput={on_input.clone()}/>
//</div>
//}
//}

//#[derive(Properties, PartialEq)]
//struct AnalyzedTextProps {
//tokens: Vec<Token>,
//}

#[derive(PartialEq, Clone)]
struct Token {
    chinese: String,
    pinyin: String,
    english: String,
    meaning: Option<String>,
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

    #[test]
    fn test_lindera() {
        use lindera::dictionary::{load_dictionary_from_kind, DictionaryKind};
        use lindera::mode::Mode;
        use lindera::segmenter::Segmenter;

        let dictionary = load_dictionary_from_kind(DictionaryKind::CcCedict).unwrap();
        let segmenter = Segmenter::new(
            Mode::Normal,
            dictionary,
            None, // Assuming no user dictionary is provided
        );
        let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
        let input_text =
            "在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南";
        let tokenized = tokenizer
            .tokenize(input_text)
            .unwrap()
            .iter()
            .map(|token| token.text.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            tokenized,
            vec!["都會", "告訴", "爸爸", "在", "校", "發生", "了", "什麼事"]
        );
        // check all tokens
    }

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
