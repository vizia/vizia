use crate::entity::Entity;
use crate::layout::BoundingBox;
use crate::prelude::Color;
use crate::style::Style;
use cosmic_text::fontdb::Query;
use cosmic_text::{
    fontdb::Database, Attrs, AttrsList, Buffer, CacheKey, Color as FontColor, Edit, Editor,
    FontSystem, Metrics, SubpixelBin, Weight, Wrap,
};
use cosmic_text::{Align, Cursor, FamilyOwned, Shaping};
use femtovg::imgref::{Img, ImgRef};
use femtovg::rgb::RGBA8;
use femtovg::{
    Atlas, Canvas, DrawCommand, ErrorKind, GlyphDrawCommands, ImageFlags, ImageId, ImageSource,
    Quad, Renderer,
};
use fnv::FnvHashMap;
use morphorm::Units;
use std::cmp::Ordering;
use std::collections::HashMap;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::{Format, Vector};
use unicode_segmentation::UnicodeSegmentation;
use vizia_storage::SparseSet;
use vizia_style::{FontStretch, FontStyle, TextAlign};

const GLYPH_PADDING: u32 = 1;
const GLYPH_MARGIN: u32 = 1;
const TEXTURE_SIZE: usize = 512;

#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    pub hint: bool,
    pub subpixel: bool,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self { hint: true, subpixel: false }
    }
}

pub struct TextContext {
    font_system: FontSystem,
    scale_context: ScaleContext,
    rendered_glyphs: FnvHashMap<CacheKey, Option<RenderedGlyph>>,
    glyph_textures: Vec<FontTexture>,
    buffers: HashMap<Entity, Editor>,
    bounds: SparseSet<BoundingBox>,
}

impl TextContext {
    #[allow(dead_code)]
    pub(crate) fn font_system(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    pub(crate) fn clear_buffer(&mut self, entity: Entity) {
        self.buffers.remove(&entity);
    }

    pub(crate) fn has_buffer(&self, entity: Entity) -> bool {
        self.buffers.contains_key(&entity)
    }

    pub(crate) fn set_text(&mut self, entity: Entity, text: &str) {
        self.with_buffer(entity, |fs, buf| {
            buf.set_text(fs, text, Attrs::new(), Shaping::Advanced);
        });
    }

    pub(crate) fn with_editor<O>(
        &mut self,
        entity: Entity,
        f: impl FnOnce(&mut FontSystem, &mut Editor) -> O,
    ) -> O {
        let editor = self.buffers.entry(entity).or_insert_with(|| {
            Editor::new(Buffer::new(&mut self.font_system, Metrics::new(18.0, 20.0)))
        });

        f(&mut self.font_system, editor)
    }

    pub(crate) fn with_buffer<O>(
        &mut self,
        entity: Entity,
        f: impl FnOnce(&mut FontSystem, &mut Buffer) -> O,
    ) -> O {
        self.with_editor(entity, |fs, ed| f(fs, ed.buffer_mut()))
    }

    pub(crate) fn set_bounds(&mut self, entity: Entity, size: BoundingBox) {
        self.bounds.insert(entity, size);
    }

    pub(crate) fn get_bounds(&self, entity: Entity) -> Option<BoundingBox> {
        self.bounds.get(entity).copied()
    }

    /// Sync the style data from vizia with the style attribites stored in cosmic-text buffers.
    pub(crate) fn sync_styles(&mut self, entity: Entity, style: &Style) {
        let (families, font_weight, font_style) = {
            let families = style
                .font_family
                .get(entity)
                .unwrap_or(&style.default_font)
                .iter()
                .map(|x| x.as_family())
                .collect::<Vec<_>>();
            let query = Query {
                families: families.as_slice(),
                weight: Weight(style.font_weight.get(entity).copied().unwrap_or_default().into()),
                stretch: style
                    .font_stretch
                    .get(entity)
                    .map(|stretch| match stretch {
                        FontStretch::UltraCondensed => cosmic_text::Stretch::UltraCondensed,
                        FontStretch::ExtraCondensed => cosmic_text::Stretch::ExtraCondensed,
                        FontStretch::Condensed => cosmic_text::Stretch::Condensed,
                        FontStretch::SemiCondensed => cosmic_text::Stretch::SemiCondensed,
                        FontStretch::Normal => cosmic_text::Stretch::Normal,
                        FontStretch::SemiExpanded => cosmic_text::Stretch::SemiExpanded,
                        FontStretch::Expanded => cosmic_text::Stretch::Expanded,
                        FontStretch::ExtraExpanded => cosmic_text::Stretch::ExtraExpanded,
                        FontStretch::UltraExpanded => cosmic_text::Stretch::UltraExpanded,
                    })
                    .unwrap_or_default(),
                style: style
                    .font_style
                    .get(entity)
                    .map(|style| match style {
                        FontStyle::Italic => cosmic_text::Style::Italic,
                        FontStyle::Normal => cosmic_text::Style::Normal,
                        FontStyle::Oblique => cosmic_text::Style::Oblique,
                    })
                    .unwrap_or_default(),
            };
            let id = self
                .font_system
                .db()
                .query(&query)
                .unwrap_or_else(|| panic!("Failed to find font: {:?}", query)); // TODO worst-case default handling
            let info = self.font_system.db().face(id).unwrap();
            (info.families.clone(), info.weight, info.style)
        };

        let font_color = style.font_color.get(entity).copied().unwrap_or(Color::rgb(0, 0, 0));

        let font_families =
            families.into_iter().map(|(name, _)| FamilyOwned::Name(name)).collect::<Vec<_>>();

        let family = if let Some(font_family) = font_families.first() {
            font_family.as_family()
        } else {
            style.default_font.first().unwrap().as_family()
        };

        let child_left = style.child_left.get(entity).copied().unwrap_or_default();
        let col_between = style.col_between.get(entity).copied().unwrap_or_default();
        let child_right = style.child_right.get(entity).copied().unwrap_or_default();

        let width = style.width.get(entity).copied().unwrap_or_default();

        let mut alignment = match (child_left, col_between, child_right) {
            (Units::Stretch(_), _, Units::Stretch(_)) => Some(Align::Center),

            (Units::Stretch(_), _, _) => Some(Align::Right),

            (_, _, Units::Stretch(_)) => Some(Align::Left),

            (_, Units::Stretch(_), _) => Some(Align::Justified),

            _ => None,
        };

        if let Some(text_align) = style.text_align.get(entity).copied() {
            alignment = match text_align {
                TextAlign::Left => Some(Align::Left),
                TextAlign::Right => Some(Align::Right),
                TextAlign::Center => Some(Align::Center),
                TextAlign::Justify => Some(Align::Justified),
                _ => None,
            };
        }

        if width.is_auto() {
            alignment = None;
        }

        self.with_buffer(entity, |fs, buf| {
            let attrs = Attrs::new().family(family).weight(font_weight).style(font_style).color(
                FontColor::rgba(font_color.r(), font_color.g(), font_color.b(), font_color.a()),
            );

            let wrap = if style.text_wrap.get(entity).copied().unwrap_or(true) {
                Wrap::Word
            } else {
                Wrap::None
            };
            buf.set_wrap(fs, wrap);
            for line in buf.lines.iter_mut() {
                // TODO spans
                line.set_attrs_list(AttrsList::new(attrs));
                line.set_align(alignment);
            }
            let font_size = style.font_size.get(entity).copied().map(|f| f.0).unwrap_or(16.0)
                * style.dpi_factor as f32;
            // TODO configurable line spacing
            buf.set_metrics(fs, Metrics::new(font_size, font_size * 1.25));
            // buf.set_size(fs, 200.0, 200.0);
            // buf.shape_until_scroll(fs);
            buf.shape_until(fs, i32::MAX);
        });
    }

    /// Generate a series of canvas path operations to render the text of a particular entity.
    pub(crate) fn fill_to_cmds<T: Renderer>(
        &mut self,
        canvas: &mut Canvas<T>,
        entity: Entity,
        bounds: BoundingBox,
        justify: (f32, f32),
        config: TextConfig,
    ) -> Result<Vec<(FontColor, GlyphDrawCommands)>, ErrorKind> {
        if !self.has_buffer(entity) {
            return Ok(vec![]);
        }

        let buffer = self.buffers.get_mut(&entity).unwrap().buffer_mut();

        let mut alpha_cmd_map = FnvHashMap::default();
        let mut color_cmd_map = FnvHashMap::default();

        let total_height = buffer.layout_runs().len() as f32 * buffer.metrics().line_height;
        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let mut cache_key = glyph.cache_key;
                let position_x = bounds.x + cache_key.x_bin.as_float();
                let position_y = bounds.y + cache_key.y_bin.as_float();

                let position_y = position_y + bounds.h * justify.1 - total_height * justify.1;

                let (position_x, subpixel_x) = SubpixelBin::new(position_x);
                let (position_y, subpixel_y) = SubpixelBin::new(position_y);
                cache_key.x_bin = subpixel_x;
                cache_key.y_bin = subpixel_y;
                // perform cache lookup for rendered glyph
                let Some(rendered) = self.rendered_glyphs.entry(cache_key).or_insert_with(|| {
                    // ...or insert it

                    // do the actual rasterization
                    let font = self
                        .font_system
                        .get_font(cache_key.font_id)
                        .expect("Somehow shaped a font that doesn't exist");
                    let mut scaler = self
                        .scale_context
                        .builder(font.as_swash())
                        .size(f32::from_bits(cache_key.font_size_bits))
                        .hint(config.hint)
                        .build();
                    let offset =
                        Vector::new(cache_key.x_bin.as_float(), cache_key.y_bin.as_float());
                    let rendered = Render::new(&[
                        Source::ColorOutline(0),
                        Source::ColorBitmap(StrikeWith::BestFit),
                        Source::Outline,
                    ])
                    .format(if config.subpixel { Format::Subpixel } else { Format::Alpha })
                    .offset(offset)
                    .render(&mut scaler, cache_key.glyph_id);

                    // upload it to the GPU
                    rendered.map(|rendered| {
                        // pick an atlas texture for our glyph
                        let content_w = rendered.placement.width as usize;
                        let content_h = rendered.placement.height as usize;
                        let alloc_w = rendered.placement.width + (GLYPH_MARGIN + GLYPH_PADDING) * 2;
                        let alloc_h =
                            rendered.placement.height + (GLYPH_MARGIN + GLYPH_PADDING) * 2;
                        let used_w = rendered.placement.width + GLYPH_PADDING * 2;
                        let used_h = rendered.placement.height + GLYPH_PADDING * 2;
                        let mut found = None;
                        for (texture_index, glyph_atlas) in
                            self.glyph_textures.iter_mut().enumerate()
                        {
                            if let Some((x, y)) =
                                glyph_atlas.atlas.add_rect(alloc_w as usize, alloc_h as usize)
                            {
                                found = Some((texture_index, x, y));
                                break;
                            }
                        }
                        let (texture_index, atlas_alloc_x, atlas_alloc_y) =
                            found.unwrap_or_else(|| {
                                // if no atlas could fit the texture, make a new atlas tyvm
                                // TODO error handling
                                let mut atlas = Atlas::new(TEXTURE_SIZE, TEXTURE_SIZE);
                                let image_id = canvas
                                    .create_image(
                                        Img::new(
                                            vec![
                                                RGBA8::new(0, 0, 0, 0);
                                                TEXTURE_SIZE * TEXTURE_SIZE
                                            ],
                                            TEXTURE_SIZE,
                                            TEXTURE_SIZE,
                                        )
                                        .as_ref(),
                                        ImageFlags::empty(),
                                    )
                                    .unwrap();
                                let texture_index = self.glyph_textures.len();
                                let (x, y) =
                                    atlas.add_rect(alloc_w as usize, alloc_h as usize).unwrap();
                                self.glyph_textures.push(FontTexture { atlas, image_id });
                                (texture_index, x, y)
                            });

                        let atlas_used_x = atlas_alloc_x as u32 + GLYPH_MARGIN;
                        let atlas_used_y = atlas_alloc_y as u32 + GLYPH_MARGIN;
                        let atlas_content_x = atlas_alloc_x as u32 + GLYPH_MARGIN + GLYPH_PADDING;
                        let atlas_content_y = atlas_alloc_y as u32 + GLYPH_MARGIN + GLYPH_PADDING;

                        let mut src_buf = Vec::with_capacity(content_w * content_h);
                        match rendered.content {
                            Content::Mask => {
                                for chunk in rendered.data.chunks_exact(1) {
                                    src_buf.push(RGBA8::new(chunk[0], 0, 0, 0));
                                }
                            }
                            Content::Color | Content::SubpixelMask => {
                                for chunk in rendered.data.chunks_exact(4) {
                                    src_buf
                                        .push(RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]));
                                }
                            }
                        }
                        canvas
                            .update_image::<ImageSource>(
                                self.glyph_textures[texture_index].image_id,
                                ImgRef::new(&src_buf, content_w, content_h).into(),
                                atlas_content_x as usize,
                                atlas_content_y as usize,
                            )
                            .unwrap();
                        RenderedGlyph {
                            texture_index,
                            width: used_w,
                            height: used_h,
                            offset_x: rendered.placement.left,
                            offset_y: rendered.placement.top,
                            atlas_x: atlas_used_x,
                            atlas_y: atlas_used_y,
                            color_glyph: matches!(rendered.content, Content::Color),
                        }
                    })
                }) else {
                    continue;
                };

                let cmd_map = if rendered.color_glyph {
                    &mut color_cmd_map
                } else {
                    alpha_cmd_map
                        .entry(glyph.color_opt.unwrap_or(FontColor::rgb(0, 0, 0)))
                        .or_insert_with(FnvHashMap::default)
                };

                let cmd = cmd_map.entry(rendered.texture_index).or_insert_with(|| DrawCommand {
                    image_id: self.glyph_textures[rendered.texture_index].image_id,
                    quads: Vec::new(),
                });

                let mut q = Quad::default();
                let it = 1.0 / TEXTURE_SIZE as f32;
                q.x0 = (position_x + glyph.x_int + rendered.offset_x - GLYPH_PADDING as i32) as f32;
                q.y0 = (position_y + run.line_y as i32 + glyph.y_int
                    - rendered.offset_y
                    - GLYPH_PADDING as i32) as f32;
                q.x1 = q.x0 + rendered.width as f32;
                q.y1 = q.y0 + rendered.height as f32;

                q.s0 = rendered.atlas_x as f32 * it;
                q.t0 = rendered.atlas_y as f32 * it;
                q.s1 = (rendered.atlas_x + rendered.width) as f32 * it;
                q.t1 = (rendered.atlas_y + rendered.height) as f32 * it;

                cmd.quads.push(q);
            }
        }

        if !alpha_cmd_map.is_empty() {
            Ok(alpha_cmd_map
                .into_iter()
                .map(|(color, map)| {
                    (
                        color,
                        GlyphDrawCommands {
                            alpha_glyphs: map.into_values().collect(),
                            color_glyphs: color_cmd_map.drain().map(|(_, cmd)| cmd).collect(),
                        },
                    )
                })
                .collect())
        } else {
            Ok(vec![(
                FontColor(0),
                GlyphDrawCommands {
                    alpha_glyphs: vec![],
                    color_glyphs: color_cmd_map.drain().map(|(_, cmd)| cmd).collect(),
                },
            )])
        }
    }

    pub(crate) fn layout_selection(
        &mut self,
        entity: Entity,
        bounds: BoundingBox,
        justify: (f32, f32),
    ) -> Vec<(f32, f32, f32, f32)> {
        self.with_editor(entity, |_, buf| {
            let mut result = vec![];
            if let Some(cursor_end) = buf.select_opt() {
                let (cursor_start, cursor_end) = match buf.cursor().cmp(&cursor_end) {
                    Ordering::Less => (buf.cursor(), cursor_end),
                    Ordering::Greater => (cursor_end, buf.cursor()),
                    Ordering::Equal => return result,
                };

                let buffer = buf.buffer();
                let total_height = buffer.layout_runs().len() as f32 * buffer.metrics().line_height;

                for run in buffer.layout_runs() {
                    if let Some((x, w)) = run.highlight(cursor_start, cursor_end) {
                        let y = run.line_y - buffer.metrics().font_size;
                        let x = x + bounds.x;

                        let y = y + bounds.y + bounds.h * justify.1 - total_height * justify.1;
                        result.push((x, y, w, buffer.metrics().line_height));
                    }
                }
            }
            result
        })
    }

    pub(crate) fn layout_caret(
        &mut self,
        entity: Entity,
        bounds: BoundingBox,
        justify: (f32, f32),
        width: f32,
    ) -> Option<(f32, f32, f32, f32)> {
        self.with_editor(entity, |_, buf| {
            let buffer = buf.buffer();
            let total_height = buffer.layout_runs().len() as f32 * buffer.metrics().line_height;

            let position_y = bounds.y + bounds.h * justify.1 - total_height * justify.1;

            let font_size = buffer.metrics().font_size;
            let line_height = buffer.metrics().line_height;

            for run in buffer.layout_runs() {
                let line_i = run.line_i;
                let line_y = run.line_y;

                let position_x = bounds.x;

                let cursor_glyph_opt = |cursor: &Cursor| -> Option<(usize, f32)> {
                    if cursor.line == line_i {
                        for (glyph_i, glyph) in run.glyphs.iter().enumerate() {
                            if cursor.index == glyph.start {
                                return Some((glyph_i, 0.0));
                            } else if cursor.index > glyph.start && cursor.index < glyph.end {
                                // Guess x offset based on characters
                                let mut before = 0;
                                let mut total = 0;

                                let cluster = &run.text[glyph.start..glyph.end];
                                for (i, _) in cluster.grapheme_indices(true) {
                                    if glyph.start + i < cursor.index {
                                        before += 1;
                                    }
                                    total += 1;
                                }

                                let offset = glyph.w * (before as f32) / (total as f32);
                                return Some((glyph_i, offset));
                            }
                        }
                        match run.glyphs.last() {
                            Some(glyph) => {
                                if cursor.index == glyph.end {
                                    return Some((run.glyphs.len(), 0.0));
                                }
                            }
                            None => {
                                return Some((0, 0.0));
                            }
                        }
                    }
                    None
                };

                if let Some((cursor_glyph, cursor_glyph_offset)) = cursor_glyph_opt(&buf.cursor()) {
                    let x = match run.glyphs.get(cursor_glyph) {
                        Some(glyph) => {
                            // Start of detected glyph
                            if glyph.level.is_rtl() {
                                (glyph.x + glyph.w - cursor_glyph_offset) as i32
                            } else {
                                (glyph.x + cursor_glyph_offset) as i32
                            }
                        }
                        None => match run.glyphs.last() {
                            Some(glyph) => {
                                // End of last glyph
                                if glyph.level.is_rtl() {
                                    glyph.x as i32
                                } else {
                                    (glyph.x + glyph.w) as i32
                                }
                            }
                            None => {
                                // Start of empty line
                                0
                            }
                        },
                    };

                    return Some((
                        x as f32 + position_x,
                        (line_y - font_size) + position_y,
                        width,
                        line_height,
                    ));
                }
            }
            None
        })
    }
}

impl TextContext {
    pub(crate) fn new_from_locale_and_db(locale: String, font_db: Database) -> Self {
        Self {
            font_system: FontSystem::new_with_locale_and_db(locale, font_db),
            scale_context: Default::default(),
            rendered_glyphs: FnvHashMap::default(),
            glyph_textures: vec![],
            buffers: HashMap::new(),
            bounds: SparseSet::new(),
        }
    }
}

pub(crate) struct FontTexture {
    atlas: Atlas,
    image_id: ImageId,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct RenderedGlyph {
    texture_index: usize,
    width: u32,
    height: u32,
    offset_x: i32,
    offset_y: i32,
    atlas_x: u32,
    atlas_y: u32,
    color_glyph: bool,
}
