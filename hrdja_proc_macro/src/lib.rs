use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Kiks" => "Err",
        "URedu" => "Ok",
        "ZnakovniNiz" => "String",
        "KartaSažetaka" => "HashMap",
        "Podrazumijevano" => "Default",
        "Greška" => "Error",
        "Neobavezno" => "Option",
        "Neki" => "Some",
        "Nijedan" => "None",
        "Rezultat" => "Result",
        "Suština" => "Self",
        "ispišred" => "println",
        "prekini" => "break",
        "asinhron" => "async",
        "isčekuj" => "await",
        "petlja" => "loop",
        "selidba" => "move",
        "kištra" => "crate",
        "izvor_nedostupan" => "unreachable_code",
        "kao" => "as",
        "postojano" => "const",
        "svojstvo" => "trait",
        "opasno" => "unsafe",
        "u" => "in",
        "iz" => "from",
        "din" => "dyn",
        "odmotaj" => "unwrap",
        "podrazumijevano" => "default",
        "ko_upuć" => "as_ref",
        "ui" => "io", // kao ulaz/izlaz
        "vanjski" => "extern",
        "laž" => "false",
        "fn" => "fn",
        "nad" => "super",
        "ubaci" => "insert",
        "dohvati" => "get",
        "dopusti" => "allow",
        "panika" | "razlaz" | "racija" => "panic",
        "mjera" => "mod",
        "izmjenjiv" => "mut",
        "novi" => "new",
        "gdje" => "where",
        "za" => "for",
        "dohvati_ili_ubaci_uz" => "get_or_insert_with",
        "glavni" => "main",
        "javni" => "pub",
        "kaj" => None?,
        "vrati" => "return",
        "ispuna" => "impl",
        "upuć" => "ref",
        "spari" => "match",
        "ako" => "if",
        "inače" => "else",
        "suština" => "self",
        "neka" => "let",
        "nepokretno" => "static",
        "građa" => "struct",
        "očekuj" => "expect",
        "dok" => "while",
        "koristi" => "use",
        "pretvori" => "into",
        "istina" => "true",
        "nabrajanje" => "enum",
        "Partija" => "Group",
        "Poistovječenje" => "Ident",
        "TokŽetona" => "TokenStream",
        "StabloŽetona" => "TokenTree",
        "u_niz_znakova" => "to_string",
        "kao_nz" => "as_str",
        "raspon" => "span",
        "Vek" => "Vec",
        "tok" => "stream",
        "guraj" => "push",
        "ispruži" => "extend",
        "ograničavać" => "delimiter",
        "Točka" => "Punct",
        "Doslovni" => "Literal",
        "makro_postupak" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn hrdja(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
