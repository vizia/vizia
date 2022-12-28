use crate::cache::CachedData;
use crate::entity::Entity;
use crate::style::Style;
use cosmic_text::{
    Attrs, AttrsList, Buffer, CacheKey, Database, Edit, Editor, Family, FamilyOwned, Font,
    FontSystem, Metrics, Wrap,
};
use femtovg::imgref::ImgRef;
use femtovg::rgb::RGBA8;
use femtovg::{
    Atlas, Canvas, DrawCmd, ErrorKind, GlyphDrawCommands, ImageFlags, ImageId, ImageSource,
    PixelFormat, Quad, Renderer,
};
use fnv::FnvHashMap;
use ouroboros::self_referencing;
use std::collections::HashMap;
use std::sync::Arc;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::{Format, Vector};

const GLYPH_PADDING: u32 = 1;
const GLYPH_MARGIN: u32 = 1;
const TEXTURE_SIZE: usize = 512;

#[self_referencing]
pub struct CosmicContext {
    font_system: FontSystem,
    #[borrows(font_system)]
    #[covariant]
    int: CosmicContextInternal<'this>,
}

struct CosmicContextInternal<'a> {
    font_system: &'a FontSystem,

    scale_context: ScaleContext,
    rendered_glyphs: FnvHashMap<CacheKey, Option<RenderedGlyph>>,
    glyph_textures: Vec<FontTexture>,
    buffers: HashMap<Entity, Editor<'a>>,
}

impl CosmicContext {
    pub fn clear_buffer(&mut self, entity: Entity) {
        self.with_int_mut(move |int: &mut CosmicContextInternal| {
            int.buffers.remove(&entity);
        });
    }

    pub fn has_buffer(&self, entity: Entity) -> bool {
        self.with_int(move |int: &CosmicContextInternal| int.buffers.contains_key(&entity))
    }

    pub fn with_editor<O>(&mut self, entity: Entity, f: impl FnOnce(&mut Editor) -> O) -> O {
        self.with_int_mut(move |int: &mut CosmicContextInternal| {
            f(int.buffers.entry(entity).or_insert_with(|| {
                Editor::new(Buffer::new(&int.font_system, Metrics::new(18, 20)))
            }))
        })
    }

    pub fn with_buffer<O>(&mut self, entity: Entity, f: impl FnOnce(&mut Buffer) -> O) -> O {
        self.with_editor(entity, |ed| f(ed.buffer_mut()))
    }

    pub fn sync_styles(&mut self, entity: Entity, style: &Style, cache: &CachedData) {
        self.with_buffer(entity, |buf| {
            let family = style
                .font
                .get(entity)
                .unwrap_or(style.default_font.as_ref().unwrap_or(&FamilyOwned::SansSerif));
            let attrs = Attrs::new().family(family.as_family());
            let wrap = if style.text_wrap.get(entity).copied().unwrap_or_default() {
                Wrap::Word
            } else {
                Wrap::None
            };
            for line in buf.lines.iter_mut() {
                // TODO spans
                line.set_attrs_list(AttrsList::new(attrs));
                line.set_wrap(wrap);
            }
            if let Some(size) = cache.size.get(entity) {
                buf.set_size(size.width as i32, size.height as i32);
            }
            let font_size =
                style.font_size.get(entity).copied().unwrap_or(16.0) * style.dpi_factor as f32;
            buf.set_metrics(Metrics::new(font_size as i32, (font_size * 1.1) as i32));
        });
    }

    pub(crate) fn fill_to_cmds<T: Renderer>(
        &mut self,
        canvas: &mut Canvas<T>,
        entity: Entity,
    ) -> Result<GlyphDrawCommands, ErrorKind> {
        self.with_int_mut(move |int: &mut CosmicContextInternal| {
            let buffer = int.buffers.get_mut(&entity).unwrap().buffer_mut();

            println!("Size: {:?}", buffer.visible_lines());

            for line in buffer.lines.iter() {
                println!("line: {}", line.text())
            }

            let mut alpha_cmd_map = FnvHashMap::default();
            let mut color_cmd_map = FnvHashMap::default();

            for run in buffer.layout_runs() {
                println!("run: {:?}", run.text);
                for glyph in run.glyphs.iter() {
                    println!("glyph: {:?}", glyph);
                    // perform cache lookup for rendered glyph
                    let Some(rendered) = int.rendered_glyphs.entry(glyph.cache_key).or_insert_with(|| {
                        // ...or insert it

                        // do the actual rasterization
                        let font = int.font_system.get_font(glyph.cache_key.font_id).expect("Somehow shaped a font that doesn't exist");
                        let mut scaler = int.scale_context.builder(font.as_swash())
                            .size(glyph.cache_key.font_size as f32)
                            .hint(true)
                            .build();
                        let offset = Vector::new(glyph.cache_key.x_bin.as_float(), glyph.cache_key.y_bin.as_float());
                        let rendered = Render::new(&[
                            Source::ColorOutline(0),
                            Source::ColorBitmap(StrikeWith::BestFit),
                            Source::Outline,
                        ])
                            .format(Format::Alpha)
                            .offset(offset)
                            .render(&mut scaler, glyph.cache_key.glyph_id);

                        // upload it to the GPU
                        rendered.map(|rendered| {
                            // pick an atlas texture for our glyph
                            let content_w = rendered.placement.width as usize;
                            let content_h = rendered.placement.height as usize;
                            let alloc_w = rendered.placement.width + (GLYPH_MARGIN + GLYPH_PADDING) * 2;
                            let alloc_h = rendered.placement.height + (GLYPH_MARGIN + GLYPH_PADDING) * 2;
                            let used_w = rendered.placement.width + GLYPH_PADDING * 2;
                            let used_h = rendered.placement.height + GLYPH_PADDING * 2;
                            let mut found = None;
                            for (texture_index, glyph_atlas) in int.glyph_textures.iter_mut().enumerate() {
                                if let Some((x, y)) = glyph_atlas.atlas.add_rect(alloc_w as usize, alloc_h as usize) {
                                    found = Some((texture_index, x, y));
                                    break;
                                }
                            }
                            let (texture_index, atlas_alloc_x, atlas_alloc_y) = found.unwrap_or_else(|| {
                                // if no atlas could fit the texture, make a new atlas tyvm
                                // TODO error handling
                                let mut atlas = Atlas::new(TEXTURE_SIZE, TEXTURE_SIZE);
                                let image_id = canvas.create_image_empty(TEXTURE_SIZE, TEXTURE_SIZE, PixelFormat::Rgba8, ImageFlags::empty()).unwrap();
                                let texture_index = int.glyph_textures.len();
                                let (x, y) = atlas.add_rect(alloc_w as usize, alloc_h as usize).unwrap();
                                int.glyph_textures.push(FontTexture {
                                    atlas,
                                    image_id,
                                });
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
                                Content::Color => {
                                    for chunk in rendered.data.chunks_exact(4) {
                                        src_buf.push(RGBA8::new(chunk[0], chunk[1], chunk[2], chunk[3]));
                                    }
                                }
                                Content::SubpixelMask => unreachable!(),
                            }
                            canvas.update_image::<ImageSource>(int.glyph_textures[texture_index].image_id, ImgRef::new(&src_buf, content_w, content_h).into(), atlas_content_x as usize, atlas_content_y as usize).unwrap();


                            RenderedGlyph {
                                texture_index,
                                width: used_w,
                                height: used_h,
                                offset_x: rendered.placement.left,
                                offset_y: rendered.placement.top,
                                atlas_x: atlas_used_x as u32,
                                atlas_y: atlas_used_y as u32,
                                color_glyph: matches!(rendered.content, Content::Color),
                            }
                        })
                    }) else { continue };

                    let cmd_map = if rendered.color_glyph {
                        &mut color_cmd_map
                    } else {
                        &mut alpha_cmd_map
                    };

                    let cmd = cmd_map.entry(rendered.texture_index).or_insert_with(|| DrawCmd {
                        image_id: int.glyph_textures[rendered.texture_index].image_id,
                        quads: Vec::new(),
                    });

                    let mut q = Quad::default();
                    let it = 1.0 / TEXTURE_SIZE as f32;

                    q.x0 = (glyph.x_int + rendered.offset_x - GLYPH_PADDING as i32) as f32;
                    q.y0 = (run.line_y + glyph.y_int - rendered.offset_y - GLYPH_PADDING as i32) as f32;
                    q.x1 = q.x0 + rendered.width as f32;
                    q.y1 = q.y0 + rendered.height as f32;

                    q.s0 = rendered.atlas_x as f32 * it;
                    q.t0 = rendered.atlas_y as f32 * it;
                    q.s1 = (rendered.atlas_x + rendered.width) as f32 * it;
                    q.t1 = (rendered.atlas_y + rendered.height) as f32 * it;

                    cmd.quads.push(q);
                }
            }

            Ok(GlyphDrawCommands {
                alpha_glyphs: alpha_cmd_map.drain().map(|(_, cmd)| cmd).collect(),
                color_glyphs: color_cmd_map.drain().map(|(_, cmd)| cmd).collect(),
            })
        })
    }

    pub(crate) fn take_buffers(&mut self) -> HashMap<Entity, Vec<String>> {
        self.with_int_mut(move |int: &mut CosmicContextInternal| {
            // TODO no clone please
            int.buffers
                .drain()
                .map(|(k, mut v)| {
                    (k, v.buffer_mut().lines.drain(..).map(|l| l.text().to_owned()).collect())
                })
                .collect()
        })
    }

    pub(crate) fn into_font_system(self) -> FontSystem {
        self.into_heads().font_system
    }
}

impl CosmicContext {
    pub fn new_from_locale_and_db(locale: String, font_db: Database) -> Self {
        CosmicContextBuilder {
            font_system: FontSystem::new_with_locale_and_db(locale, font_db),
            int_builder: |font_system| CosmicContextInternal {
                font_system,
                scale_context: Default::default(),
                rendered_glyphs: FnvHashMap::default(),
                glyph_textures: vec![],
                buffers: HashMap::new(),
            },
        }
        .build()
    }
}

pub struct FontTexture {
    atlas: Atlas,
    image_id: ImageId,
}

#[derive(Copy, Clone, Debug)]
pub struct RenderedGlyph {
    texture_index: usize,
    width: u32,
    height: u32,
    offset_x: i32,
    offset_y: i32,
    atlas_x: u32,
    atlas_y: u32,
    color_glyph: bool,
}

//#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
//enum RenderMode {
//    Fill,
//    Stroke(u32),
//}
