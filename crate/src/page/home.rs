use crate::{
    generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB, Model, Article,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};
use crate::Visibility::Hidden;

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.container,
            C.w_full,
            C.flex,
            C.flex_wrap,
            C.mx_auto,
            C.px_2,
            C.pt_8,
            C.mt_16,
            // lg__
            C.lg__pt_16,
        ],
        view_menu(model).els(),
        view_content(model).els(),
        view_back_link().els(),
    ]
}

fn view_menu(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_xl,
            C.text_gray_800,
            C.leading_normal,
            // lg__
            C.lg__w_1of5,
            C.lg__px_6,
        ],
        p![
            class![
                C.text_base,
                C.font_bold,
                C.py_2,
                // lg__
                C.lg__pb_6,
                C.text_gray_700,
            ],
            "Menu",
        ],
        view_menu_toggle().els(),
        view_menu_items(model).els(),
    ]
}

fn view_menu_toggle() -> impl View<Msg> {
    div![
        class![
            C.sticky,
            C.inset_0,
            // lg__
            C.lg__hidden,
        ],
        button![
            id!("menu_toggle"),
            class![
                C.flex,
                C.w_full,
                C.justify_end,
                C.px_3,
                C.py_3,
                C.bg_white,
                C.border,
                C.rounded,
                C.border_gray_600,
                C.hover__border_purple_500,
                C.appearance_none,
                C.focus__outline_none,
                // lg__
                C.lg__bg_transparent,
            ],
            svg![
                class![
                    C.fill_current,
                    C.h_3,
                    C.float_right,
                ],
                attrs!{
                    At::ViewBox => "0 0 20 20",
                },
                path![
                    attrs!{
                        At::D => "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z",
                    }
                ]
            ],
        ]
    ]
}

fn view_menu_items(model: &Model) -> impl View<Msg> {
    div![
        id!("menu_items"),
        class![
            C.w_full,
            C.sticky,
            C.inset_0,
            C.hidden => model.menu_visibility == Hidden,
            C.h_64,
            C.overflow_x_hidden,
            C.overflow_y_auto,
            C.mt_0,
            C.border,
            C.border_gray_400,
            C.bg_white,
            C.shadow,
            C.z_20,
            // lg__
            C.lg__h_auto,
            C.lg__overflow_y_hidden,
            C.lg__border_transparent,
            C.lg__shadow_none,
            C.lg__bg_transparent,
            C.lg__block,
        ],
        style! {
            St::Top => em(5),
        },
        ul![
            model.articles.iter().map(|article| view_menu_item(article, false).els())
        ]
    ]
}

fn view_menu_item(article: &Article, active: bool) -> impl View<Msg> {
    li![
        class![
            C.py_2,
            C.hover__bg_purple_100,
            // md__
            C.md__my_0,
            // lg__
            C.lg__hover__bg_transparent,
        ],
        a![
            class![
                C.block,
                C.pl_4,
                C.align_middle,
                C.text_gray_700,
                C.hover__text_purple_500,
                C.border_l_4,
                C.border_transparent,
                C.focus__outline_none,
                // lg__
                C.lg__border_purple_500 => active,
                if active { C.lg__hover__border_purple_500 } else { C.lg__hover__border_purple_400 },
            ],
            attrs! {
                At::Href => "",
            },
            span![
                class![
                    C.block,
                    C.pb_1,
                    C.text_sm,
                    C.text_gray_900 => active,
                    C.font_bold => active,
                    // md__
                    C.md__pb_0,
                ],
                article.menu_title,
            ]
        ]
    ]
}

fn view_content(model: &Model) -> impl View<Msg> {
    let article = &model.articles[0];

    div![
        class![
            C.w_full,
            C.p_8,
            C.mt_6,
            C.text_gray_900,
            C.leading_normal,
            C.bg_white,
            C.border,
            C.border_gray_400,
            C.rounded,
            // lg__
            C.lg__w_4of5,
            C.lg__mt_0,
        ],
        view_content_top_back_link().els(),
        view_content_markdown(article.content).els(),
    ]
}

fn view_content_top_back_link() -> impl View<Msg> {
    div![
        class![
            C.font_sans,
        ],
        span![
            class![
                C.text_base,
                C.text_purple_500,
                C.font_bold,
            ],
            "« ",
            a![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.hover__underline,
                    // md__
                    C.md__text_sm,
                ],
                attrs!{
                    At::Href => "",
                },
                "Back link",
            ]
        ],
    ]
}

fn view_content_markdown(content: &str) -> impl View<Msg> {
    div![
        class![
            "markdown"
        ],
        md!(content)
    ]
}

fn view_back_link() -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_gray_500,
            C.px_4,
            C.py_6,
            // md__
            C.md__text_sm,
            // lg__
            C.lg__w_4of5,
            C.lg__ml_auto,
            C.text_base,
        ],
        span![
            class![
                C.text_base,
                C.text_purple_500,
                C.font_bold,
            ],
            "< ",
            a![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.hover__underline,
                    // md__
                    C.md__text_sm,
                ],
                attrs!{
                    At::Href => "",
                },
                "Back to Help"
            ],
        ]
    ]
}