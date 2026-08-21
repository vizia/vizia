use vizia::prelude::*;

#[derive(Clone, Copy)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rectangle,
    Button,
    Card,
    Profile,
    List,
    Table,
    Article,
    Media,
}

#[derive(Clone, Copy)]
pub enum SkeletonAnimation {
    Pulse,
    Shimmer,
    Wave,
    Blur,
    Glow,
    Breathe,
}

#[derive(Clone, Copy)]
pub enum SkeletonShape {
    Rectangle,
    Rounded,
    Circle,
}

pub struct Skeleton;
pub struct SkeletonBone;

impl SkeletonBone {
    pub fn new(cx: &mut Context, shape: SkeletonShape) -> Handle<'_, Self> {
        Self.build(cx, |_| {}).class("gallery-skeleton-bone").class(match shape {
            SkeletonShape::Rectangle => "skeleton-shape-rectangle",
            SkeletonShape::Rounded => "skeleton-shape-rounded",
            SkeletonShape::Circle => "skeleton-shape-circle",
        })
    }
}

impl View for SkeletonBone {
    fn element(&self) -> Option<&'static str> {
        Some("skeleton-bone")
    }
}

impl Skeleton {
    pub fn new(cx: &mut Context, variant: SkeletonVariant) -> Handle<'_, Self> {
        Self.build(cx, move |cx| match variant {
            SkeletonVariant::Text => skeleton_text(cx),
            SkeletonVariant::Circle => bone(cx, "gallery-skeleton-shape-circle"),
            SkeletonVariant::Rectangle => bone(cx, "gallery-skeleton-shape-rectangle"),
            SkeletonVariant::Button => bone(cx, "gallery-skeleton-shape-button"),
            SkeletonVariant::Card => skeleton_card(cx),
            SkeletonVariant::Profile => skeleton_profile(cx),
            SkeletonVariant::List => skeleton_list(cx),
            SkeletonVariant::Table => skeleton_table(cx),
            SkeletonVariant::Article => skeleton_article(cx),
            SkeletonVariant::Media => skeleton_media(cx),
        })
        .class("gallery-skeleton")
    }
}

pub trait SkeletonModifiers {
    fn animation(self, animation: SkeletonAnimation) -> Self;
}

impl SkeletonModifiers for Handle<'_, Skeleton> {
    fn animation(self, animation: SkeletonAnimation) -> Self {
        self.class(match animation {
            SkeletonAnimation::Pulse => "skeleton-animation-pulse",
            SkeletonAnimation::Shimmer => "skeleton-animation-shimmer",
            SkeletonAnimation::Wave => "skeleton-animation-wave",
            SkeletonAnimation::Blur => "skeleton-animation-blur",
            SkeletonAnimation::Glow => "skeleton-animation-glow",
            SkeletonAnimation::Breathe => "skeleton-animation-breathe",
        })
    }
}

impl View for Skeleton {
    fn element(&self) -> Option<&'static str> {
        Some("skeleton")
    }
}

fn bone(cx: &mut Context, class: &'static str) {
    Element::new(cx).class("gallery-skeleton-bone").class(class);
}

fn text_lines(cx: &mut Context) {
    VStack::new(cx, |cx| {
        bone(cx, "gallery-skeleton-line-wide");
        bone(cx, "gallery-skeleton-line");
        bone(cx, "gallery-skeleton-line-short");
    })
    .class("gallery-skeleton-lines");
}

fn skeleton_text(cx: &mut Context) {
    text_lines(cx);
}

fn skeleton_card(cx: &mut Context) {
    VStack::new(cx, |cx| {
        bone(cx, "gallery-skeleton-media");
        text_lines(cx);
    })
    .class("gallery-skeleton-card");
}

fn skeleton_profile(cx: &mut Context) {
    HStack::new(cx, |cx| {
        bone(cx, "gallery-skeleton-avatar");
        text_lines(cx);
    })
    .class("gallery-skeleton-profile");
}

fn skeleton_list(cx: &mut Context) {
    VStack::new(cx, |cx| {
        for _ in 0..3 {
            HStack::new(cx, |cx| {
                bone(cx, "gallery-skeleton-avatar-small");
                text_lines(cx);
            })
            .class("gallery-skeleton-row");
        }
    })
    .class("gallery-skeleton-list");
}

fn skeleton_table(cx: &mut Context) {
    VStack::new(cx, |cx| {
        for row in 0..4 {
            HStack::new(cx, |cx| {
                for _ in 0..3 {
                    bone(
                        cx,
                        if row == 0 {
                            "gallery-skeleton-cell-header"
                        } else {
                            "gallery-skeleton-cell"
                        },
                    );
                }
            })
            .class("gallery-skeleton-table-row");
        }
    })
    .class("gallery-skeleton-table");
}

fn skeleton_article(cx: &mut Context) {
    VStack::new(cx, |cx| {
        bone(cx, "gallery-skeleton-title");
        text_lines(cx);
        text_lines(cx);
    })
    .class("gallery-skeleton-article");
}

fn skeleton_media(cx: &mut Context) {
    HStack::new(cx, |cx| {
        bone(cx, "gallery-skeleton-thumbnail");
        VStack::new(cx, |cx| {
            bone(cx, "gallery-skeleton-title-small");
            text_lines(cx);
        })
        .class("gallery-skeleton-lines");
    })
    .class("gallery-skeleton-media-row");
}
