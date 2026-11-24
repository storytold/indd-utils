# Attribute lists

Formatting is stored as attribute lists (on page items: chunk 0x6E03).
Decoded in `src/model/attrs.rs`. The IDML name and value kind of each ID
are in the tables of `src/idml/attrs.rs`; `src/idml/kind.rs` converts
each kind.

## Layout

u32 count, then for each attribute:

| Size | Field |
|---|---|
| 4 | Attribute ID |
| 2 | Payload size *n* |
| *n* | Payload: u16 value count, then values |

Each value: u32 type, u16 length, data. The first value is the attribute's
value. Every attribute in the samples has a second value of type 0x6E63
with 6 bytes, meaning unknown.

| Type | Length | Value |
|---|---|---|
| 0x6E68 | 8 | f64 |
| 0x6E67 | 4 | i32 |
| 0x6E65 | 2 | u16 (enumeration or boolean) |
| 0x6E69 | 16 | two f64 (a point) |
| 0x117 | 4 | UID reference |

All 2,730 page item chunks of this layout in the samples parse exactly.

## Page item attributes

The list holds the item's local values. IDML writes an attribute only when
it differs from the applied object style, so a value can be in the INDD
list and absent from the IDML.

| ID | IDML attribute | Evidence: IDML value equals INDD value |
|---|---|---|
| 0x6E68 | `FillColor` (swatch UID) | 1,190 of 1,194 |
| 0x6E69 | `FillTint`; 100 is written as −1 (strokes, below) | 10 of 10 |
| 0x6E64 | `StrokeColor` (swatch UID) | 78 of 82 |
| 0x6E65 | `StrokeWeight` | 171 of 171 |
| 0x6E6D | `MiterLimit` | 778 of 778 |
| 0x6E6F | `CornerOption`: code 0 = `None`, 0x5A15 = `RoundedCorner`, 0x5A16 = `InverseRoundedCorner` (the codes of object styles, `objects.md`) | 20 of 23 for 0x5A15; with all three codes, 1,876 of 1,995 rectangles, 504 of 520 text frames and 489 of 489 page item defaults in the trustworthy pairs |
| 0x6E70 | `CornerRadius` | 213 of 216 |
| 0x6E70, 0x6E94, 0x6E92, 0x6E93 | `TopLeftCornerRadius`, `TopRightCornerRadius`, `BottomLeftCornerRadius`, `BottomRightCornerRadius` (below) | each radius ID against its own corner 4,357, 3,200, 3,276 and 3,325 matches, no mismatch |
| 0x6E6F, 0x6E91, 0x6E8F, 0x6E90 | `TopLeftCornerOption`, `TopRightCornerOption`, `BottomLeftCornerOption`, `BottomRightCornerOption`: codes as `CornerOption` (below) | 11,211 of 11,211 |
| 0x551F | `GradientFillLength` | all non-zero (30) |
| 0x5520 | `GradientFillStart` (point) | all non-zero (30) |
| 0x5525 | `GradientStrokeLength` (assumed; equal to fill in all samples) | 30 |
| 0x5526 | `GradientStrokeStart` (assumed; equal to fill in all samples) | 30 |
| 0x6E6E | `StrokeType` (built-in stroke style code, below) | 51 of 51 items, 576 of 576 object styles; strokes, below |
| 0x6E8C | `StrokeAlignment`: 0 `CenterAlignment`, 1 `InsideAlignment`, 2 `OutsideAlignment` (below) | strokes, below |
| 0x6E66, 0x6E89, 0x6E8A, 0x6E6B, 0x6E6C, 0x6E71, 0x6E72, 0x6E84 | `StrokeTint`, `GapColor`, `GapTint`, `EndCap`, `EndJoin`, `LeftLineEnd`, `RightLineEnd`, `ArrowHeadAlignment` | strokes, below |

**Corners.** The top-left corner uses the same IDs as `CornerRadius`
and `CornerOption`. The corner option codes are 0 `None`, 0x5A15
`RoundedCorner`, 0x5A16 `InverseRoundedCorner`, 0x5A17 `InsetCorner`,
0x5A18 `BevelCorner` and 0x5A19 `FancyCorner`; all six occur. The
evidence is over the items of the trustworthy pairs whose corners
differ: each radius ID matches its own corner with no mismatch, and has
16 to 71 mismatches against each other corner. The same IDs hold the
corners of object styles (chunk 0x1B92B) and of the page item defaults
(class 0x6E07): all four radii and options of `PageItemDefault` match in
495 of 495 documents.

**Values equal to the object style.** The style's effective value is
its own (chunk 0x1B92B), else that of the style it is based on, else the
root `[None]`'s (`StrokeWeight="0"`, `FillTint="-1"`, `FillColor` and
`StrokeColor` `Swatch/None`, `CornerRadius="12"`, `MiterLimit="4"`,
`CornerOption="None"`; `idml-values.md`). Over the 489 trustworthy pairs,
the converter wrote these values where the IDML has none, and every one
of them equals the style's effective value (the stroke attributes of
the strokes section follow the same rule):

| Attribute | Written though the IDML has none | IDML writes it though equal |
|---|---:|---:|
| `StrokeWeight` | 27,989 | 5 |
| `CornerRadius` | 1,608 | 0 |
| `FillTint` | 1,423 | 0 |
| `StrokeColor` | 1,336 | 0 |
| `FillColor` | 312 | 0 |
| `CornerOption` | 87 | 0 |
| `MiterLimit` | 26 | 0 |

Of the `StrokeWeight` values, the 26,969 on items whose IDML element has
the same tag all equal the style (17,501 text frames, 7,070 rectangles,
2,074 polygons, 297 ovals, 27 lines); the others are on shapes written
as another element type. The converter leaves out these attributes, and
the four corner radii and options, when they equal the style's value
(numbers to 6 significant digits).

### Values an item does not store

Every document has one object of class 0x6E02 (606 of 606 trustworthy
pairs). Its chunk 0x6E03 is a page item attribute list with every fill,
stroke and corner attribute. An item whose own list lacks an attribute
has the value of this base list, not that of its object style. IDML
writes that value where it differs from the object style's effective
value, the same rule as for stored values.

The base lists differ in two ways across the trustworthy pairs, always
together: 481 documents have `StrokeWeight` 1 and corner radii 12, 125
have 0.2835 (0.1 mm) and 14.1732 (5 mm). All have `StrokeColor` and
`FillColor` naming the swatch `None`, `FillTint` −1, `MiterLimit` 4,
`StrokeType` `Solid`, `StrokeAlignment` `CenterAlignment` and corner
options `None`.

Evidence: an analysis script looked for the field that tells the
documents whose items have `StrokeWeight="0.2834645669291339"` apart
from those whose items have 1, by testing every f64 of every object
whose class occurs once per document; the `StrokeWeight` of this list
matched 294 of 295 documents. With the base values and the rule above,
the converter reproduces, over the rectangles, ovals, polygons, graphic
lines and text frames of the trustworthy pairs (anchored ones
included), 8,790 of 8,794 `StrokeColor` values, 54,694 of 54,694
`StrokeWeight`, 42,500 of 42,504 `MiterLimit` and 6,383 of 6,387
`CornerRadius`, with no extra value. The values not reproduced in that
measurement were on 4 text frames without an object style (no object
style, below) and on items with stored values not decoded then (77
`FillTint`, stored fills such as pasted smooth shades, the stroke
alignment `OutsideAlignment` and stroke types; strokes, below). Over all pairs, 4 extra `StrokeWeight` values appear,
in 3 stale pairs.

IDML writes no corner attributes for EPS text (class 0x660B; none in
634 elements), so only its other attributes follow the rule.

**Groups.** A group (class 0x401) stores none of these attributes. IDML
gives a group, for each fill, stroke and corner attribute, the value
that all its child page items have in effect: the child's own value
(stored, or from the base list where IDML writes it), else its object
style's effective value; for a child group, its own shared value. EPS
text has no corner values and is left out for the corners. IDML writes
the shared value when it differs from the group's object style value,
when the style turns the category off, when the group has no object
style, and for a fill or stroke tint that every child stores as 100
(written −1, strokes below); it leaves the attribute out when the
children differ.

With this rule the converter reproduces, over the groups of the 1,251
trustworthy pairs of the corpus after 2026-10 (none was written
before):

| Attribute | Values |
|---|---:|
| `StrokeWeight` | 4,741 of 4,741 |
| `FillColor` | 3,813 of 3,813 |
| `MiterLimit` | 3,679 of 3,679 |
| `StrokeColor` | 1,221 of 1,221 |
| `CornerRadius`, `TopLeftCornerRadius` | 1,001 of 1,001 each |
| `StrokeTint` | 664 of 664 |
| `FillTint` | 417 of 417 |
| `BottomLeftCornerRadius`, `BottomRightCornerRadius` | 326 of 326 each |
| `TopRightCornerRadius` | 296 of 296 |
| `GapColor` | 124 of 124 |
| corner options (five attributes) | 92 to 103 of as many |
| `LeftLineEnd`, `RightLineEnd`, `StrokeAlignment`, `EndJoin`, `EndCap` | 40, 9, 15, 7 and 2 of as many |
| `StrokeType` | 15 of 17 |

In 38 of the `FillTint` values every child stores a fill tint of 100,
and the group has −1. The rule adds 27 extra values: 3 documents of
versions 10 to 12 whose groups have `CornerRadius` and
`TopLeftCornerRadius` but not the other three radii, although every
child has all four, and one `StrokeType`. `OverprintFill` and
`OverprintStroke` of groups follow the children with ink only
(`objects.md`, page item overprint).

**Category turned off.** When the item's object style (other than the
root `[None]`) turns a category off, IDML writes the item's values of
that category even where they equal the style's, stored or from the
base list:

| Category (ID in chunk 0x1B92E) | Attributes |
|---|---|
| `EnableFill` (0x1B933) | `FillColor`, `FillTint` |
| `EnableStroke` (0x1B934) | `StrokeColor` |
| `EnableStrokeAndCornerOptions` (0x1B935, 0x1B936) | `StrokeWeight`, `MiterLimit`, `StrokeAlignment`, `EndCap`, `EndJoin`, the line ends, `ArrowHeadAlignment`, corner radii and options |
| both stroke categories | `StrokeType`, `StrokeTint`, `GapColor`, `GapTint` |

Evidence over the trustworthy pairs: 66 items with styles that turn all
three off and 2 with styles that turn off both stroke categories have
every such attribute in IDML, with the stored or base value; 12 items
whose style turns off only `EnableStrokeAndCornerOptions` have
`StrokeWeight`, `MiterLimit`, `StrokeAlignment` and the corners, but
no `StrokeColor` or `StrokeType`. No sample turns off only
`EnableStroke`, so the category of `StrokeType` is not shown; the
converter writes it when both are off. The 68 items also have
`StrokeTint`, `EndCap`, `EndJoin`, `GapColor` and `GapTint`, and the 12
items `EndCap` and `EndJoin` but not `StrokeTint`, `GapColor` or
`GapTint`, which gives the categories of the table (the line ends and
`ArrowHeadAlignment` follow the 12 items in the corpus after 2026-10).

**No object style.** An item without an applied object style has
`AppliedObjectStyle="n"` in IDML, and IDML writes every fill, stroke
and corner attribute of the item, stored or from the base list: 60 of
60 such items (35 rectangles, 25 text frames) of the trustworthy pairs
after 2026-10, every attribute equal.

Transparency attributes (IDs 0x108xx and 0x1EBxx) are described in
`transparency.md`.

### Strokes

Object styles (class 0x1B901) hold a full page item attribute list in
chunk 0x1B92B, with the same record layout as chunk 0x6E03 but a u16
count. The converter reads them (`object style` lists in `indd audit`)
and writes them with the page item table. They also add evidence: the
corpus pairs have 576 object styles whose name matches an IDML
`ObjectStyle`.

**Stroke type (0x6E6E).** The record has three values: a reference (type
0x117), a code (type 0x6E64, 4 bytes) and the usual 6-byte value. With
reference 0 the code names a built-in stroke style. IDML writes
`StrokeType` as `StrokeStyle/$ID/<name>`.

| Code | IDML stroke style | Evidence |
|---|---|---|
| 0x5A29 | `Solid` | 574 object styles in 135 files |
| 0x5A2A | `Dashed` | 2,326 items in 40 files |
| 0x5A37 | `Canned Dashed 4x4` | 52 items in 20 files |
| 0x5A38 | `Canned Dashed 3x2` | 160 items in 45 files |
| 0x5A39 | `Canned Dotted` | 345 items in 30 files |
| 0x5A3A | `Wavy` | 4 items in 1 file |
| 0x5A3B | `Straight Hash` | 109 items in 43 files |
| 0x5A3C | `Right Slant Hash` | 2 items in 1 file |
| 0x5A3E | `White Diamond` | 33 items in 18 files |
| 0x5A3F | `Japanese Dots` | 913 items in 29 files; cells (`tables.md`) |
| 0xB004 | `ThinThin` | 31 items in 3 files |
| 0xB005 | `ThinThick` | 10 items in 3 files |
| 0xB006 | `ThickThin` | 44 items in 12 files; object styles |
| 0xB007 | `ThickThick` | 35 items in 13 files; cells |
| 0xB009 | `ThickThinThick` | 1 item |
| 0xB01A | `Triple_Stroke` | 5 items in 3 files |

Item counts are over the items of the 1,251 trustworthy pairs of the
corpus after 2026-10 with reference 0, IDML value against stored code.
No code has two IDML values. The codes are the same in every list that
names a stroke style (items, object styles, cells, tables, paragraph
rules, underlines and strike-throughs), so the converter uses one table.
Codes 0x5A3D and 0xB008 occur only in files without a trustworthy IDML.

**Custom stroke styles.** A record with code 0x5A42 and a non-zero
reference names a custom stroke style: the reference is the UID of an
object of class 0xB016 (striped) or 0x5A43 (dashed). IDML defines each
in `Resources/Graphic.xml` after the built-in `StrokeStyle` elements and
names it in `StrokeType` as `StripedStrokeStyle/<name>` or
`DashedStrokeStyle/<name>`.

| Class | Chunk | Layout | IDML |
|---|---|---|---|
| 0xB016 | 0xB023 | flag byte, in-object string | `Name` |
| 0xB016 | 0xB002 | u32 *n*, then *n* pairs of f64 (start, width), fractions of the stroke weight | `StripeArray`: each value × 100, in stored order |
| 0x5A43 | 0x5A48 | flag byte, in-object string | `Name` |
| 0x5A43 | 0x5A4F | u32 *n*, *n* f64 dash and gap lengths in points, u16 corner adjustment, u32 end cap | `DashArray` (the values as stored), `StrokeCornerAdjustment` (3 `DashesAndGaps`), `EndCap` (as 0x6E6B) |

IDML writes `Self`, `StripeArray`, `Name` for a striped style and
`Self`, `DashArray`, `StrokeCornerAdjustment`, `EndCap`, `Name` for a
dashed one; `Self` is `StripedStrokeStyle/<name>` or
`DashedStrokeStyle/<name>`. Every object of the two classes is written,
also where nothing uses it, in UID order.

Evidence over the trustworthy pairs of the corpus after 2026-10: the
converter reproduces 21 of 21 `StripedStrokeStyle` elements (one
definition, (0, 0.05), IDML `StripeArray="0 5"`) and 5 of 5
`DashedStrokeStyle` elements (one definition: 24, 0, corner 3, cap 1,
IDML `DashArray="24 0"`, `StrokeCornerAdjustment="DashesAndGaps"`,
`EndCap="RoundEndCap"`), with every attribute, and with them 21 more
object style `StrokeType` values, 5 graphic line values and one of
`PageItemDefault`. Four of the five files with the dashed style use it
nowhere. A chunk whose length is not that of its layout leaves the
style out with a warning. Over the 6,039 distinct INDD and INDT files of
the corpus outside the privately held samples, 78 files have such
styles: all 98 striped and 119 dashed objects have the chunk lengths of
the layout (4 + 16*n* and 4 + 8*n* + 6, either byte order). Striped
styles have 1, 2, 3, 5, 6 or 7 stripes; every dashed style has an even
number of lengths, corner code 3 (117) or 0 (2) and end cap 0 (108) or
1 (11). Only the one-stripe form and corner code 3 occur in a pair.

**Stroke alignment (0x6E8C).** 0 `CenterAlignment`, 1 `InsideAlignment`
(3,238 items in 44 files), 2 `OutsideAlignment` (196 items in 50 files).
Object styles use 0x1BB8 with the same codes.

**More stroke attributes.** Object styles other than `[None]` were
matched by `Self` with their IDML style over the 1,251 trustworthy
pairs (4,305 styles; every one has the IDs below except 0x6E84), and
page items by `Self` with their IDML element:

| ID | IDML attribute | Values | Object styles |
|---|---|---|---|
| 0x6E66 | `StrokeTint` | f64; 100 is written as −1 | 4,305 of 4,305; 69 styles other than −1 (25 to 75) |
| 0x6E89 | `GapColor` | swatch UID | 4,305 of 4,305; 23 `Color/Paper` |
| 0x6E8A | `GapTint` | f64; 100 is written as −1 | two styles with 100 have −1 in IDML |
| 0x6E6B | `EndCap` | 0 `ButtEndCap`, 1 `RoundEndCap`, 2 `ProjectingEndCap` | 4,305 of 4,305; 3 round |
| 0x6E6C | `EndJoin` | 0 `MiterEndJoin`, 1 `RoundEndJoin`, 2 `BevelEndJoin` | 4,305 of 4,305; 5 round |
| 0x6E71 | `LeftLineEnd` | line end code (below) | 4,305 of 4,305; 9 other than `None` |
| 0x6E72 | `RightLineEnd` | line end code | 4,305 of 4,305; 14 other than `None` |
| 0x6E84 | `ArrowHeadAlignment` | 0 `InsidePath`, 1 `OutsidePath` | in the 3,349 styles whose IDML has the attribute, 3,349 of 3,349; one style has 1 |

Line end codes (u32): 0 `None`, 0x5A03 `SimpleArrowHead` (1 item),
0x5A04 `SimpleWideArrowHead` (110 items in 5 files), 0x5A05
`TriangleArrowHead` (44 in 8), 0x5A06 `TriangleWideArrowHead`, 0x5A07
`BarbedArrowHead` (64 in 6), 0x5A08 `CurvedArrowHead`, 0x5A09
`CircleArrowHead`, 0x5A0A `CircleSolidArrowHead`, 0x5A0C
`SquareSolidArrowHead` (the page item defaults of 5 files) and 0x5A0D
`BarArrowHead`. `EndCap` 2 occurs on 4 items in 3 files, `EndJoin` 2 on
6 items in 1 file. One line in the pairs before 2026-10 has 0x6E71 =
0x5A08 and 0x6E72 = 0x5A0D with `LeftLineEnd="CurvedArrowHead"` and
`RightLineEnd="BarArrowHead"`, which gives 0x5A0D. `ArrowHeadAlignment`
code 1 rests on one style; that style's line ends are mapped by their
own IDs, which leaves 0x6E84.

**On page items** IDML writes these attributes, the stroke type and the
stroke alignment by the rule of the fill and stroke colour: the value
the item stores (or that of the base list) where it differs from the
applied object style's effective value, every value where the style
turns the category off, and every value without a style (above). Groups
never store these IDs; their values come from their children (values an
item does not store, groups).

A tint is compared as stored: an item that stores 100 differs from a
style value of −1, and IDML writes it as −1. 4,508 items in 192 files
store `StrokeTint` 100 and have `StrokeTint="-1"` in IDML; no IDML item
has `StrokeTint="100"`. Items that store −1, or the style's own value,
have no attribute; one polygon that stores 100 has none either.
`GapTint` behaves the same (11 items in 2 files store 100), and so does
`FillTint`: over the 1,252 trustworthy pairs, 90 items in 10 files
store a fill tint of 100 (79 polygons, 76 of them in groups, and 11
text frames), and all 90 have `FillTint="-1"` in IDML. No page item or
object style in these IDMLs has `FillTint="100"`.

Over the rectangles, ovals, polygons, graphic lines and text frames of
the trustworthy pairs the converter reproduces these values; the one
extra value is a polygon of a 19.0 document that stores `StrokeTint`
100 and has no attribute in IDML:

| Attribute | Values |
|---|---:|
| `StrokeTint` | 5,874 of 5,874 |
| `EndCap` | 8,538 of 8,538 |
| `EndJoin` | 8,220 of 8,220 |
| `GapColor` | 435 of 435 |
| `GapTint` | 181 of 181 |
| `LeftLineEnd` | 453 of 453 |
| `RightLineEnd` | 435 of 435 |
| `ArrowHeadAlignment` | 139 of 139 |
| `StrokeType` | 4,230 of 4,235 (5 custom stroke styles) |
| `StrokeAlignment` | 3,572 of 3,572 |

The values of `PageItemDefault` come from the page item defaults list
with the same table (`preferences.md`).

**Not identified.** `LeftArrowHeadScale` and `RightArrowHeadScale` are
100 in every object style and in every `PageItemDefault` but one, as are
0x6E95 and 0x6E96, so the two IDs cannot be told apart. The converter
writes them from the value files on object styles, and from the IDs in
their order on `PageItemDefault`, where the values are equal in every
file (`preferences.md`, page item defaults). 0x6E78 is 0 in every style,
0x6E73 in all but 2; 0x6E78 is `Nonprinting` of `PageItemDefault`. 0x6E67, 0x6E6A and 0x6E8B are the
overprint flags (`objects.md`, object style settings).
**Dashes (0x5A35).** Page item attribute 0x5A35 (value type 0x5A36):
8 bytes (0 in every sample), u32 count *n*, *n* f64, u16 corner
adjustment. IDML writes the f64 list as `StrokeDashAndGap` (numbers
separated by spaces) and the code as `StrokeCornerAdjustment`: 3
`DashesAndGaps`, the only code in the samples, which the dashed stroke
style's chunk 0x5A4F also stores for `DashesAndGaps`. IDML has the two
attributes only on items whose stroke type in effect is `Dashed`
(`StrokeStyle/$ID/Dashed`; every such item has them), and on groups by
the group rule: no item with another stroke type has them, though 1,251
documents store the attribute in the page item defaults and many items
of other stroke types store it. Object styles never have them. With
these rules the converter reproduces 2,312 of 2,316 graphic line values
of each attribute, 8 of 9 for polygons and 10 of 24 and 29 for groups,
with no extra value. The values of 9 rectangles and 7 EPS text items
are not written (not examined).

## Text attribute lists

Text formatting uses the same record layout with a **u16** count. It
appears in:

- **Styles:** chunk 0x23F of a style object (u16 count, records).
- **Story runs:** each paragraph-style or character-style run record
  (chunk 0x262, see `objects.md`) continues after the style UID with a u16
  count and records. These are the run's local overrides.

Value types are specific to each attribute (for example 0x1B05 for swatch
references, 0x1B28 for point size), so the converter decodes a text value
by the layout of its attribute. The attributes whose value is not a
single number have their layout listed in `text_layout`
(`src/model/attrs.rs`): string values (`FontStyle`) are a flag byte
followed by an in-object string; tab lists, nested, GREP and line
styles, bullet characters, points, ruby text and cell edge stroke types
are described below. Other values are numbers, decoded by length: 8 bytes f64, 4 bytes
u32, 2 bytes u16. A value that does not fit its attribute's layout is
kept undecoded and not written.

### Mapping and evidence

Found by aligning IDML `ParagraphStyleRange` and `CharacterStyleRange`
elements with INDD runs at the same text offset (1,562 ranges), and IDML
styles with INDD styles of the same name (486 styles), then checked with
`tools/compare.py`. Counts are matches/total in the current comparison
(ranges + paragraph styles + character styles where present).

| ID | IDML attribute | Encoding | Evidence |
|---|---|---|---|
| 0x1B01 | `FillColor` | swatch UID | 354/354 ranges, 226/226 styles |
| 0x1B02 | `FontStyle` | flag + string; empty: below | 474/474 ranges, 202/202 styles |
| 0x1B03 | `PointSize` | f64 | 1,090/1,090 ranges |
| 0x1B06 | `HorizontalScale` | fraction ×100 | 2/2 ranges, 75/75 styles |
| 0x1B07 | `KerningMethod` | code: 15972 Metrics, 79875 Optical, 0x3E65 `$ID/Metrics - Roman Only` (27 styles), 0x221 `$ID/None` (21 styles and text defaults); a run with manual kerning: below | 20/20 ranges, 81/81 styles |
| 0x1B08 | `Ligatures` | 1 = true | 201/201 ranges |
| 0x1B0A | `StrokeWeight` | f64 | 241/241 ranges |
| 0x1B0B | `Tracking` | thousandths of an em ×1000 | 49/49 ranges, 94/94 styles |
| 0x1B0C | `Composer` | code: 0x2001 HL Single, 0x2002 HL Composer, 0x2078 HL Composer Optyca, 0x2079 HL Single Optyca, 0x2010 HL Composer J, 0x2011 HL Single J (runs/styles: 47/14, 0/30, 7/15) | 4/4 ranges, 114/114 styles |
| 0x1B0D | `DropCapCharacters` | u16 | 79/79 styles; see below |
| 0x1B0E | `DropCapLines` | u16 | 79/79 styles; see below |
| 0x1B10 | `BaselineShift` | f64 | 79/79 styles; see below |
| 0x1B11 | `Capitalization` | 0 Normal, 1 SmallCaps, 2 AllCaps, 3 CapToSmallCap, 4 LowerCase | 77/77 styles; codes 1, 3 and 4 below |
| 0x1B12 | `StrokeColor` | swatch UID | 75/75 styles |
| 0x1B13 | `KerningValue` | ems ×1000, rounded to a whole number; 1e8 = none, written `1e+11` | 1e8: 144/144 ranges, 29/29 character styles; rounding and scale (from a sample and its print PDF): below |
| 0x1B15 | `VerticalScale` | fraction ×100 | 79/79 styles; see below |
| 0x1B16 | `LeftIndent` | f64 | 22/22 ranges |
| 0x1B17 | `RightIndent` | f64 | 86/86 styles |
| 0x1B18 | `FirstLineIndent` | f64 | 12/12 ranges |
| 0x1B1A | `AutoLeading` | fraction ×100 | 75/75 styles |
| 0x1B1B | `Leading` (Properties) | f64; negative = Auto | 43/43 ranges, 137/137 styles |
| 0x1B1D | `AppliedLanguage` | language UID; see below | 88/88 styles |
| 0x1B1F | `Hyphenation` | 3 = true, 0 = false | 204/204 ranges |
| 0x1B24 | `NoBreak` | 1 = true | 78/78 styles |
| 0x1B25 | `HyphenationZone` | f64 | 155/155 ranges |
| 0x1B26 | `SpaceBefore` | f64 | 13/13 ranges |
| 0x1B27 | `SpaceAfter` | f64 | 42/42 ranges |
| 0x1B29 | `TabList` (Properties) | list; see below | 110/110 styles |
| 0x1B2A | `Underline` | 1 = true | 81/81 ranges |
| 0x1B2B | `AppliedFont` (Properties) | font family UID; a composite font as an object (below) | 1,071/1,071 ranges, 236/247 styles |
| 0x1B2C | `OTFFigureStyle` | 0 TabularLining (23 runs, 5 styles), 1 ProportionalOldstyle, 2 ProportionalLining, 3 TabularOldstyle (below), 4 Default | 4/4 ranges, 138/138 styles |
| 0x1B2E | `MaximumWordSpacing` | fraction ×100 | 77/77 styles |
| 0x1B2F | `MinimumWordSpacing` | fraction ×100 | 77/77 styles |
| 0x1B31 | `MaximumLetterSpacing` | fraction ×100 | 78/78 styles; see below |
| 0x1B32 | `MinimumLetterSpacing` | fraction ×100 | 78/78 styles; see below |
| 0x1B33 | `DesiredGlyphScaling` | fraction ×100 | 22/22 `ParagraphStyleRange` elements in 10 documents (96 to 99), 1,251 trustworthy pairs; 1.0 in the root style and text defaults of every pair, as IDML's 100. No other text list of the pairs has it |
| 0x1B37 | `StartParagraph` | 0 Anywhere, 1 NextColumn, 2 NextPage, 3 NextFrame, 4 NextOddPage | 78/78 styles; codes below |
| 0x1B3C | `Position` | 0 Normal, 1 Superscript, 2 Subscript, 3 OTSuperscript, 4 OTSubscript, 5 OTNumerator, 6 OTDenominator (runs/styles: 107/23, 4/7, 1/0, 3/0, 21/0 for 1, 2, 3, 4, 6) | 20/20 ranges, 78/78 styles |
| 0x1B40 | `KeepLinesTogether` | 1 = true | 78/78 styles; see below |
| 0x1B42 | `FillTint` | f64 | 24/24 ranges |
| 0x1B46 | `GradientFillAngle` | f64 | 81/81 styles |
| 0x1B48 | `GradientFillLength` | f64 | 2/2 ranges, 81/81 styles |
| 0x1B4A | `GradientFillStart` | two f64 | 2/2 ranges, 81/81 styles |
| 0x1B4D | `RuleAboveLineWeight` | f64 | 155/155 ranges |
| 0x1B4F | `RuleAboveOffset` | f64 | 78/78 styles |
| 0x1B50 | `RuleAboveLeftIndent` | f64 | 91/91 styles |
| 0x1B51 | `RuleAboveRightIndent` | f64 | 91/91 styles |
| 0x1B52 | `RuleAboveWidth` | 1 ColumnWidth, 2 TextWidth | 91/91 styles |
| 0x1B53 | `RuleBelowColor` (Properties) | swatch UID, 0 = "Text Color" | 6/6 ranges, 114/114 styles |
| 0x1B54 | `RuleBelowLineWeight` | f64 | 155/155 ranges |
| 0x1B55 | `RuleBelowTint` | f64 | 78/78 styles |
| 0x1B56 | `RuleBelowOffset` | f64 | 78/78 styles |
| 0x1B5D | `RuleBelow` | 1 = true | 83/83 styles |
| 0x1B6A | `ParagraphBreakType` | 0 Anywhere, 1 NextColumn, 2 NextPage, 3 NextFrame, 4 NextOddPage (below) | 11/11 ranges, 78/78 styles |
| 0x1B6B | `SingleWordJustification` | 0 LeftAlign, 3 FullyJustified | 80/80 styles |
| 0x1B75 | `AllNestedStyles` (Properties) | list; see below | 124/124 paragraph styles, 10/10 text defaults (1,251 trustworthy pairs) |
| 0x1B7E | `Justification` | 0 LeftAlign, 1 CenterAlign, 2 RightAlign, 3 FullyJustified, 4 LeftJustified, 5 CenterJustified, 6 RightJustified, 8 ToBindingSide, 9 AwayFromBindingSide (runs/styles for 3, 6, 8, 9: 34/1, 3/0, 2/1, 7/14) | 70/70 ranges |
| 0x1B80 | `DropcapDetail` | u32 | 5/5 ranges, 103/103 styles |
| 0x1B8C | `OTFContextualAlternate` | 1 = true | 220/220 ranges |
| 0x1B8D | `UnderlineColor` (Properties) | swatch UID, 0 = "Text Color" | 81/81 ranges |
| 0x1B91 | `UnderlineOffset` | f64 | 81/81 ranges |
| 0x1B94 | `UnderlineWeight` | f64 | 81/81 ranges |
| 0x1BB7 | `MiterLimit` | f64 | 161/161 ranges |
| 0x1BB9 | `EndJoin` | 0 MiterEndJoin, 1 RoundEndJoin | 86/86 styles |
| 0x1BBA | `AllGREPStyles` (Properties) | list; see below | 152/152 paragraph styles, 48/48 text defaults, 320/320 text ranges (1,251 trustworthy pairs) |
| 0x1BBB | `AllLineStyles` (Properties) | list; see below | 6/6 paragraph styles, 9/9 text ranges (1,251 trustworthy pairs) |
| 0x1BBD | `SpanColumnType` | 0 SingleColumn, 1 SpanColumns, 2 SplitColumns (69 runs, 10 styles) | 41/41 ranges, 94/94 styles |
| 0x1BBE | `SpanSplitColumnCount` (Properties) | u16; 1 = All | 12/12 ranges, 78/78 styles |
| 0x1BBF | `SplitColumnInsideGutter` | f64 | 155/155 ranges |
| 0x1BC4 | `SpanColumnMinSpaceAfter` | f64 | 84/84 styles |
| 0x1BD2 | `ParagraphShadingColor` (Properties) | swatch UID | 25/25 ranges |
| 0x1BD3 | `ParagraphShadingTint` | f64 | 79/79 styles |
| 0x1BD6 | `ParagraphShadingOn` | 1 = true | 88/88 styles |
| 0x1BDB | `ParagraphShadingTopOffset` | f64 | 91/91 styles |
| 0x1BDC | `ParagraphShadingBottomOffset` | f64 | 90/90 styles |
| 0x1BF6 | `ParagraphBorderColor` (Properties) | swatch UID | 72/72 styles |
| 0x1BFA | `ParagraphBorderGapColor` (Properties) | swatch UID; 0 is `Swatch/None` | present exactly when the list has the ID (344 of 344 styles of the corpus of 2026-10); trustworthy pairs after 2026-10: 1,147 of 1,147 paragraph styles, 918 of 918 text defaults, 2,745 of 2,745 text ranges, 642 + 84 story and cell paragraph ranges |
| 0x1BF9 | `ParagraphBorderOn` | 1 = true | 77/77 styles |
| 0x1DF03 | `ParagraphBorderTopOffset` | f64 | 83/83 styles |
| 0x1DF04 | `ParagraphBorderBottomOffset` | f64 | 83/83 styles |
| 0x1DF21 | `SameParaStyleSpacing` (Properties) | f64; −1 = SetIgnore | 15/15 ranges, 100/100 styles |
| 0x4265 | `GridAlignFirstLineOnly` | 1 = true | 81/81 styles |
| 0x4266 | `GridAlignment` | 0 None, 1 AlignBaseline, 2 AlignEmTop, 3 AlignEmCenter (below) | 86/86 styles |
| 0x425E | `Tatechuyoko` | 1 = true | root styles; 1 from a sample typeset vertically and its print PDF, see below |
| 0x4279 | `ShataiDegreeAngle` | degrees, written ×100 | root styles; see below |
| 0x427A | `ShataiAdjustTsume` | 1 = true | root styles; see below |
| 0x427B | `ShataiAdjustRotation` | 1 = true | root styles and a sample typeset vertically; see below |
| 0x422D | `RubyFlag` | number; written when not 0 | a sample and its print PDF; see below |
| 0x422E | `RubyString` | u32 length, text segments; written when not empty | a sample and its print PDF; see below |
| 0x1A401 | `BulletsAndNumberingListType` | 0 NoList, 1 BulletList, 2 NumberedList (8 runs, 20 styles) | 10/12 ranges |
| 0x1A406 | `BulletChar` (Properties) | u32 type, u32 value; see below | 1/1 ranges, 84/84 styles |
| 0x1A413 | `BulletsFont` (Properties) | font family UID, 0 = `$ID/` | 1/1 ranges, 84/84 styles |
| 0x1A414 | `BulletsFontStyle` (Properties) | flag + string; empty = `Nothing` | 1/1 ranges, 84/84 styles |
| 0x1A417 | `AppliedNumberingList` (Properties) | numbering list UID (`objects.md`, numbering lists) | 519/519 styles |
| 0x1A419 | `NumberingContinue` | 1 = true | 2/2 ranges, 78/78 styles |
| 0x1A41F | `BulletsCharacterStyle` (Properties) | character style UID | 84/84 styles |
| 0x1A420 | `NumberingCharacterStyle` (Properties) | character style UID | 79/79 styles |
| 0x1A423 | `NumberingExpression` | flag + string | 79/79 styles |
| 0x4221 | `Mojikumi` (Properties) | table UID; see below | 936/1,019 styles |
| 0x4224 | `KinsokuSet` (Properties) | table UID; see below | 987/987 styles |
| 0x42C0 | `TreatIdeographicSpaceAsSpace` | 1 = true | 1,082/1,083 styles |
| 0x50F18 | `DiacriticPosition` | 4 OpentypePosition, 5 OpentypePositionFromBaseline | 1,007/1,010 styles |

The counts of these four are paragraph and character styles of all 654
pairs whose IDML style has the attribute and the same name (the root
paragraph style of every pair among them); they also reproduce the
`TextDefault` values of the preferences (`preferences.md`). The kinsoku
and mojikumi value is a UID: 0 is `Nothing`; a built-in table
(`kinsoku and mojikumi tables` in `objects.md`) is written as an
enumeration: `kHardKinsokuName` `HardKinsoku` (51 styles),
`kSoftKinsokuName` `SoftKinsoku` (11), `kKoreanKinsokuName`
`KoreanKinsoku` (12), `kSimpChineseKinsokuName`
`SimplifiedChineseKinsoku` (1), `kMojikumiDefaultName1`
`LineEndAllOneHalfEmEnum` (55) and `kMojikumiDefaultName16`
`SimpChineseDefault` (1); a custom kinsoku table (class 0x4204) as the
object `KinsokuTable/<name>` (38). The corpus of 2026-10 adds three
built-in tables, each from the `TextDefault` of every trustworthy pair
that uses it: `kTradChineseKinsokuName` `TraditionalChineseKinsoku` (90
documents), `kMojikumiDefaultName15` `TradChineseDefault` (80) and
`kMojikumiDefaultName2` `OneEmIndentLineEndUkeOneHalfEmEnum` (1). One
paragraph range of one trustworthy pair (12.1) applies
`kMojikumiDefaultName14`, `LineEndPeriodOneEmEnum`; the base table of
custom tables adds `kMojikumiDefaultName10`, `LineEndAllOneEmEnum`
(`objects.md`, custom mojikumi tables). With
them the converter reproduces 1,251 of 1,251 `TextDefault` `KinsokuSet`
and `Mojikumi` values, 1,430 of 1,430 `KinsokuSet` and 1,532 of 1,532
`Mojikumi` values of paragraph styles. Other built-in tables are not
written.
A custom mojikumi table (class 0x4203, `objects.md`) is the object
`MojikumiTable/<name>`. In the trustworthy pairs of the corpus of
2026-10 this gives the IDML value for 576 paragraph ranges, 105
paragraph styles, 94 ranges in table cells, 19 story ranges and 11 text
defaults, with no value that differs from the IDML.

Paragraph style counts include the 78 root styles `[No paragraph style]`
of the compared pairs. For many attributes above the root holds the only
value that is the same in every pair (for example `RightIndent="0"`), so
the evidence that tells the attribute apart is in the other styles:

| Attribute | Other styles or ranges with the attribute |
|---|---|
| `BaselineShift` | 1 style with 0; 1 range with −16 in a story that `compare.py` does not align |
| `RightIndent` | 8 styles in one pair, two different values |
| `AppliedLanguage` | 10 styles in 2 pairs |
| `NoBreak` | 1 style with 1 (true), in a pair whose IDML is from an older version |
| `TabList` | 33 styles in 5 pairs, left and right stops, with and without leaders |
| `OTFFigureStyle` | 60 styles in one pair (codes 1 and 2), 4 ranges in another (code 4) |
| `Position` | 20 ranges in one pair |
| `GradientFill*` | 3 styles in one pair (angle −90, distinct length and start), 2 ranges in another |
| `RuleAbove*Indent`, `RuleAboveWidth` | 13 styles in one pair (left −1.13, right −2.83) |
| `RuleBelowColor` | 36 styles in 2 pairs, four colours |
| `RuleBelow` | 5 styles in one pair (4 true, 1 false) |
| `ParagraphBreakType` | 19 ranges in one pair |
| `SingleWordJustification` | 2 styles in one pair with code 0 |
| `EndJoin` | 8 styles in one pair |
| `SpanColumnType` | 16 styles in 2 pairs, 46 ranges in one pair, both codes |
| `SpanSplitColumnCount` | 13 ranges in one pair with 2 |
| `SpanColumnMinSpaceAfter` | 6 styles in one pair |
| `ParagraphShading*`, `ParagraphBorder*` | 2 to 14 styles in one pair, several different offsets |
| `SameParaStyleSpacing` | 30 styles and 16 ranges, each in 2 pairs |
| `GridAlignment`, `GridAlignFirstLineOnly` | 8 and 3 styles in 3 pairs |
| `BulletChar`, `BulletsFont`, `BulletsFontStyle`, `BulletsCharacterStyle` | 6 styles in 2 pairs, 1 range in another |
| `NumberingExpression`, `NumberingCharacterStyle` | 1 style |
| `NumberingContinue` | 2 ranges in 2 pairs |
| `VerticalScale` | 1 style; see below |

For codes, the root style gives the default: for example every root style
has 0x1B6B = 3 and IDML `SingleWordJustification="FullyJustified"`, and
0x1BBE = 1 with `SpanSplitColumnCount` `All`.

**`VerticalScale` (0x1B15).** The value is 1 (100%) wherever it appears,
like `HorizontalScale` (0x1B06), so values do not tell them apart. Over
all pairs, including those with an IDML from another version, 0x1B15 is
present in exactly the 5 styles where IDML writes `VerticalScale`; 0x1B06
is identified separately by values other than 100%.

**Languages (0x1B1D).** The value is the UID of an object of class
0x2D07. Its chunk 0x2D0F is a flag byte, then the language name as an
in-object string (`English: USA`), followed by other strings. IDML writes
`$ID/` and that name. Over all pairs, all 62 styles (root styles
included) and 4 ranges with the attribute match, with 5 different
languages.

The language named `Neutral` is the one exception: IDML writes it as
`[No Language]`, without `$ID/`, although its `Language` element is
named `$ID/[No Language]` (`objects.md`, languages). Over the 1,252
trustworthy pairs of the corpus of 2026-10, `AppliedLanguage` has a
value without `$ID/` 514 times in 43 documents, always `[No Language]`;
no IDML writes `$ID/Neutral` or `$ID/[No Language]` there. Each of the
490 values matched with an INDD object (31 text defaults, 54 paragraph
styles, 4 character styles, 401 ranges) names a language object called
`Neutral`.

**Tab stops (0x1B29).** u16 count, then for each stop: f64 position, u16
alignment (0 `LeftAlign`, 1 `CenterAlign`, 2 `RightAlign`, 3
`CharacterAlign`), for code 3 the alignment character as one UTF-16
unit, u16 leader length and the leader in UTF-16 code units. IDML writes
each stop as a `ListItem` with `Alignment`, `AlignmentCharacter`,
`Leader` and `Position`. A stop with another code stores no character,
and IDML writes `.` for it in every sample. An empty list is a count of
0. The converter leaves out a list with another alignment code.

Evidence, corpus of 2026-10, the stops of paragraph styles matched by
`Self` and of story paragraph ranges: 79 `CenterAlign` stops (59 in
trustworthy pairs) and 36 `CharacterAlign` stops (34; 27 with `,`, 9
with `.`) match in alignment, character, leader and position. Before
code 3 was read, a list with such a stop was not read at all: the
character made the record longer than the stops.

**Nested styles (0x1B75).** u32 count, then for each item: u32 character
style UID, u32 length, and a delimiter code of that many characters
stored as text segments (see `objects.md`). The code gives `Delimiter`,
`Inclusive` and `Repetition`:

| Code | IDML | Evidence |
|---|---|---|
| `[c]` | `Delimiter` string *c*, `Inclusive="false"` | 13 items in one pair (`.` and `:`) |
| `(c)` | `Delimiter` string *c*, `Inclusive="true"` | 1 item in the same pair |
| `^c` | `Dropcap`, `Inclusive="true"` | 1 item in the same pair |
| digits after `)` or `]` | `Repetition`; no digits = 1 | a sample and its print PDF |
| `(^w)` | `AnyWord` | a sample and its print PDF |
| `(^?)` | `AnyCharacter` | a sample and its print PDF |
| `(^c)` | `Dropcap` | 54 `Dropcap` records in 18 pairs (with `^c`) |
| `^t` in brackets | `Tabs` | 9 records in 5 pairs (`[^t]`, `(^t)`, `(^t)2` seen) |
| `^L` in brackets | `Repeat`; the digits are `Repetition` | 41 records in 33 pairs (`(^L)2`, `(^L)4` seen) |
| `^p` in brackets | `EndNestedStyle` | 2 records in 1 pair |
| `^9` in brackets | `Digits` | 2 records in 1 pair (`[^9]`, `(^9)`) |
| `^S` in brackets | `Sentence` | 1 record in 1 pair |

The public pair has only single-character literals and repetition 1. The
last three rows rest on a sample and its print PDF:

- A sample has nested items with `(^?)` and with `(^w)` followed by a
  number, the latter applying a character style with Capitalization
  code 3 (`CapToSmallCap`, see below).
- In the PDF, capitals in the first words of paragraphs in those styles
  are set at small-cap height (ink height measured on the rendered
  page), and capitals later in the same lines at full height. Small
  capitals reach at most one word past the number, and capitals a few
  words further on are at full height, so the number is the repetition.
  Of the counting delimiters in the IDML schema, only `AnyWord` gives a stretch of several words: `AnyCharacter`,
  `Letters` and `Digits` would end within the first word, and `Sentence`
  would run past sentence ends, which the PDF does not show.
- A paragraph-level override has `(^?)` followed by a number *n*, then
  `(^w)`. In the PDF the first capital after the first *n* characters of
  that paragraph is a small cap. With a word delimiter, the first *n*
  words would keep full capitals, and that capital lies within them. So
  `^?` counts characters: `AnyCharacter`.

The last six rows come from the 1,251 trustworthy pairs of the corpus
after 2026-10. Their reference IDMLs have 265 nested style records in
72 documents (styles, text defaults and text ranges; the record counts
above are these). The stored items map one to one onto the IDML
records, with the bracket giving `Inclusive` and the digits
`Repetition` as above: the converter reproduces `AllNestedStyles` of
124 of 124 paragraph styles, 10 of 10 text defaults, 274 of 274 text
ranges and 7 of 7 `Story/ParagraphStyleRange` elements. Without these
codes it reproduced 54 paragraph styles and 4 text defaults.

The converter leaves out a list that has any other code.

**GREP styles (0x1BBA).** The layout of nested styles, with a GREP
expression in place of the delimiter code: u32 count, then for each item
u32 character style UID (0 for none), u32 length *n* and the expression
as *n* UTF-16 code units in text segments. IDML writes
`AllGREPStyles` (`type="list"`), one `ListItem type="record"` per item in
stored order, with `AppliedCharacterStyle` (object) and `GrepExpression`
(string). Evidence over the 1,251 trustworthy pairs: every 0x1BBA value
of the paragraph styles parses and ends exactly after its last item; the
reference IDMLs have 494 such lists with 1,064 items, all of these two
fields. The converter reproduces 152 of 152 paragraph styles, 48 of 48
text defaults, 320 of 320 text ranges (`Story/ParagraphStyleRange` 110,
`Cell/ParagraphStyleRange` 94), with no extra value. A style's list is
its own value; IDML does not write an inherited one.

**Line styles (0x1BBB).** u32 count, then 8 bytes per item: u32
character style UID and u32 line count. IDML writes `AllLineStyles`,
one record per item: `AppliedCharacterStyle` (object), `LineCount`
(long) and `RepeatLast` (long). `RepeatLast` is −1 in all 9 list items of
the trustworthy reference IDMLs (3 documents) and no stored field holds
it, so the converter writes −1. Reproduced: 6 of 6 paragraph styles and
9 of 9 text ranges.

**Empty lists.** None of the 1,251 trustworthy reference IDMLs has an
`AllGREPStyles`, `AllLineStyles` or `AllNestedStyles` element without
items. The converter writes no GREP or line style list without items.

**Bullet character (0x1A406).** Two u32: the character type (0
`UnicodeOnly`, 1 `UnicodeWithFont`, 2 `GlyphWithFont`) and the character
value. IDML writes an empty `BulletChar` element with
`BulletCharacterType` and `BulletCharacterValue` attributes. In the
pairs, type 0 occurs in the root styles with value 8226 (•), type 1 in
one range and type 2 in 6 styles, each with its value and a
`BulletsFont`.

**`KeepLinesTogether` (0x1B40).** In every pair the root style has 0, and
IDML writes `KeepLinesTogether="false"`, `KeepAllLinesTogether="false"`,
`KeepFirstLines="2"` and `KeepLastLines="2"`. No public style sets
another value. The rest of the evidence is a sample and its print PDF:

- Some paragraph styles of the sample set 0x1B40 to 1. Apart from
  attributes identified above, they set nothing else.
- In the PDF, paragraphs in those styles that break across pages always
  leave at least two lines on each page. A paragraph of *n* lines broken
  at a random line leaves a single line on one side with probability
  2/(*n* − 1), so this pattern does not come from chance.
- Such paragraphs do break across pages, so the attribute is not
  `KeepAllLinesTogether`. A boolean that keeps at least two lines
  together at both ends, with the defaults above, is `KeepLinesTogether`.

**`Capitalization` codes 1, 3 and 4.**

- Code 1 is `SmallCaps`. In one pair (InDesign 20.2, IDML from the same
  version), 8 of 8 paragraph styles with code 1 have
  `Capitalization="SmallCaps"`. The same pair confirms code 2 (4 of 4
  styles) and code 0.
- Code 3 is `CapToSmallCap`. No pair has it. The evidence is a sample
  and its print PDF. In the PDF, words set in a style with code 3 that
  start with a capital letter show that capital at the height of the
  letter after it, about two thirds of the height of word-initial
  capitals in ordinary text. Heights were measured on the rendered
  pages. So code 3 sets capital letters as small capitals too. Of the values in the IDML schema (`Normal`, `SmallCaps`,
  `AllCaps`, `CapToSmallCap`, `LowerCase`), only `CapToSmallCap` does
  that.
- Code 4 is `LowerCase`. No pair of the corpus has it: no IDML outside
  the privately held samples has `LowerCase`, and no paired INDD stores
  code 4. One privately held sample (DOM 21) has 1 paragraph style with
  code 4, and its IDML has `Capitalization="LowerCase"` (1 of 1).

The converter leaves out other codes.

**Drop caps (0x1B0D, 0x1B0E).** In every pair, the root style
`[No paragraph style]` has 0 for both and IDML writes
`DropCapCharacters="0"` and `DropCapLines="0"`. One pair (InDesign 20.2,
IDML from the same version) also has a style with 0x1B0D = 1 and
0x1B0E = 3; its IDML has `DropCapCharacters="1"` and `DropCapLines="3"`.
The different values tell the two attributes apart.

**Letter spacing (0x1B31, 0x1B32).** In every pair both are 0 in the root
style, as are `MaximumLetterSpacing` and `MinimumLetterSpacing` in the
IDML. That does not tell them apart from other attributes that are 0
there. The rest of the evidence is a sample and its print PDF:

- A paragraph style of the sample sets 0x1B31 to a positive fraction
  and 0x1B32 to a negative one. The style sets no tracking, and its
  other unidentified attributes are not fractions.
- In the PDF, justified lines in that style have a character spacing
  (`Tc` operator) that changes from line to line, both positive and
  negative. So the style lets letter spacing vary in both directions,
  which needs a negative minimum and a positive maximum.
- The values are stored as fractions, like word spacing (0x1B2E,
  0x1B2F), so they are written ×100. The positive one is the maximum.

A layout of the converted sample with these two attributes reproduces
noticeably more of the PDF's line breaks than one without them.

**Ruby (0x422D, 0x422E).** IDML writes neither attribute on the root
style. In every pair the root style stores 0 for 0x422D and a 4-byte 0
for 0x422E. The first evidence was a sample typeset vertically and its
print PDF:

- Character runs of the sample set 0x422D to 1 together with 0x422E,
  whose value is a u32 length in characters followed by text segments
  (as story text, `objects.md`). The text is a short reading of the
  run's characters.
- In the PDF, each such text appears in small type in a narrow column
  right beside the run's characters and centred on them, as ruby is set
  beside vertical text.
- The converter decodes the value of 0x422E by this layout, whatever its
  length.

The converter writes `RubyString` and writes `RubyFlag` with the stored
number (1 in every such run; the schema type is an integer). Pairs
with ruby were added later: in the corpus of 2026-10 the converter
reproduces `RubyFlag` and `RubyString` in 185 of 185 IDML ruby ranges
(15 distinct documents). Per-character ruby stores the readings of the
base characters separated by U+3000, and IDML writes the same string.
The other ruby settings are in "Ruby, kenten and warichu" below.

**`Tatechuyoko` (0x425E).** Every public style that stores it has 0,
and IDML writes `Tatechuyoko="false"` on all 240 root paragraph styles
of the corpus IDML files; no public range has the attribute. The
evidence for 1 is a sample typeset vertically and its print PDF:

- Runs of a few Latin characters (letters or punctuation) set 0x425E
  to 1. Elsewhere in the same vertical stories, Latin text
  without it is drawn turned sideways: the PDF text matrix is a quarter
  turn, and the letters follow each other down the column.
- The glyphs of each run with 1 are drawn upright (identity text
  matrix), side by side on one baseline, and the group is centred on the
  column's centre line to 0.01 pt. That is horizontal-in-vertical
  setting.

**Shatai (0x4278 to 0x427B).** In every public file the root paragraph
style stores 0x4278 = 0, 0x4279 = 45, 0x427A = 1 and 0x427B = 0, and
IDML writes `ShataiMagnification="0"`, `ShataiDegreeAngle="4500"`,
`ShataiAdjustTsume="true"` and `ShataiAdjustRotation="false"` on all 240
root styles; no other public style or range sets them. Only 45 and 4500
are not 0 or a boolean, so 0x4279 is the angle, written ×100. The rest
of the evidence is a sample typeset vertically and its print PDF:

- Runs set 0x4278 = 20, 0x4279 = 60 and 0x427A = 0. Their glyphs have
  the text matrix (0.850, 0.0866, 0.0866, 0.950) × point size. That
  matrix scales by 1 along the direction at 60° and by 0.80 across it:
  a distortion of 20 % at 60°. So 0x4278 is the magnification in
  percent and 0x4279 the angle in degrees.
- A character style sets 0x4278 = 10, 0x4279 = 60 and 0x427B = 1. Its
  glyphs have the matrix (0.9222, 0.0843, 0, 0.9759) × point size. This
  is the 10 % distortion at 60°, turned so that the glyph's vertical
  axis stays vertical (the third value is 0). The runs above, with
  0x427B = 0, are not turned. So 0x427B is `ShataiAdjustRotation`.
- That leaves 0x427A for `ShataiAdjustTsume`, which also matches the root
  values (1 and `true`; 0x427B 0 and `false`). With 0 the glyphs keep
  an advance of one em; no sample has 1 outside root styles.

The converter writes the angle and the two booleans. It leaves out
`ShataiMagnification`: the sample stores the percentage, but the root
value 0 does not show the unit IDML uses, and the angle shows that it
need not be the stored number.

**`KerningValue` (0x1B13).** The value 1e8 means no manual kerning.
IDML writes it `KerningValue="1e+11"` (1e8 × 1000, the scale of other
values) on ranges and character styles:

- In every one of the 1,252 trustworthy pairs, the number of live story
  run records (chunk 0x262 of class 0xCA17) that store 1e8 equals the
  number of reference `CharacterStyleRange` elements with
  `KerningValue="1e+11"`: 144 in 3 documents, 0 elsewhere.
- Of the 1,796 style objects (class 0x205, chunk 0x23F) that store 1e8,
  1,767 are root paragraph styles (one or two per file) and 29 are
  character styles. The reference has `KerningValue="1e+11"` on exactly
  those 29 character styles (21 documents).
- The preferences object (class 0x2202) stores 1e8 in 1,251 files. The
  schema allows no `KerningValue` on paragraph styles or `TextDefault`,
  so the root paragraph styles and text defaults get none.
- The corpus IDMLs have 329 `KerningValue="1e+11"` (169 ranges, 160
  character styles), always with that literal text.

IDML writes every other value as a whole number. Every `KerningValue` in
every corpus IDML is an integer or `1e+11` (6,798 integers, none with a
fraction). Stored values ×1000 such as −231.8, 26.9, 58.6, 59.9,
68.0066 and −29.9945 are −232, 27, 59, 60, 68 and −30 in the reference
(18 ranges of the trustworthy pairs), each the nearest integer. Some runs
store a tiny negative number (about −7e−18); IDML writes `0` (10 ranges
in 4 documents), not `-0`. No sample stores a value exactly halfway
between two integers, so how halves round is open; the converter rounds
them away from zero.

The scale rests on a sample and its print PDF:

- Runs of one character set 0x1B13 to −0.1, −0.06, −0.12 and 0. In the
  PDF, the gap between that character and the next (start of the next
  glyph minus the end of the first glyph's advance width) is −0.1,
  −0.06, −0.12 and 0 times the point size, to 0.01 pt, once the letter
  spacing that every other gap in the same line shows is subtracted.
- The same letter pairs elsewhere, without the attribute, have other
  gaps, which come from the font's kerning. So the stored value replaces
  the font's kerning after that character.
- The value is in ems. The IDML schema types `KerningValue` as a plain
  number. `Tracking` (0x1B0B) is also stored in ems, and IDML writes it
  ×1000 (thousandths of an em, from the pairs); the converter writes
  `KerningValue` the same way. No sample shows this scale directly.
- The schema allows `KerningValue` on character styles and ranges, not
  on paragraph styles, so the converter leaves it out of paragraph
  styles (public samples without IDML have it there).

**Manual kerning method.** When a story run stores both 0x1B07 and
0x1B13, and 0x1B13 is not 1e8, IDML writes `KerningMethod="$ID/Manual"`
if the written `KerningValue` is not 0 and `KerningMethod="$ID/None"` if
it is 0, whatever method code is stored:

- Trustworthy pairs: 37 ranges in 7 documents are `$ID/Manual` (stored
  79875 Optical in 34, 15972 Metrics in 3; values from −1000 to 200). 9
  ranges in 2 documents with value 0 are `$ID/None` (stored Optical in
  8, Metrics in 1). Runs that store a method and 1e8 keep the stored
  method (6 ranges, `$ID/Optical`). No range contradicts the rule. A
  stale pair adds 36 `$ID/Manual` ranges and 3 `$ID/None` ranges (those
  3 store code 0x221 and value 0).
- Runs that store a value but no method get no `KerningMethod` in IDML
  (728 ranges with a non-zero value, 141 with 0).
- The corpus IDMLs have `$ID/Manual` 92 times, only on
  `CharacterStyleRange`, never on a style. No style stores a method
  together with a value other than 1e8, so the converter keeps the
  stored method on styles.
- No sample has a run with code 0x221 and a non-zero value; the rule
  gives `$ID/Manual` there.

Code 0x221 is `$ID/None`. In the trustworthy pairs it occurs 21 times in
7 documents (style lists and text defaults), and each document's count
equals its number of reference styles and defaults with
`KerningMethod="$ID/None"` (11 paragraph styles, 8 character styles, 2
`TextDefault`). Over all little-endian corpus files, 0x221 is the only
0x1B07 code not in the table (26 files in style lists, 12 in runs).

**`StartParagraph` (0x1B37).** In every pair the root style has 0 and
IDML writes `StartParagraph="Anywhere"`; no pair has another value. Code
2 rests on a sample and its print PDF:

- Every paragraph whose style has code 2 is the first text on its page
  in the PDF, and the page before it often ends with room for more
  lines. Those styles set no other attribute that is unidentified.
- Such paragraphs start on both odd and even pages, so code 2 is not
  `NextOddPage` or `NextEvenPage`.
- In the sample, `NextColumn`, `NextFrame` and `NextPage` would give the
  same layout, so the PDF does not tell them apart. The converter writes
  `NextPage`.

Later samples show the other codes in pairs: 1 `NextColumn` (1 style),
3 `NextFrame` (1 run) and 4 `NextOddPage` (1 style), with no run or
style that contradicts them.

The new codes of the table were found by aligning INDD runs with IDML
ranges at the same offset over 17,914 trustworthy stories and styles by
name over all 654 pairs; no run or style contradicts them, and every
value converted with a new code equals the IDML. A run with manual
kerning has `KerningMethod="$ID/Manual"` whatever code it stores (manual
kerning method, above).

**Percentages.** IDML writes a fraction attribute (`HorizontalScale`,
`VerticalScale`, `AutoLeading`, the word and letter spacing and glyph
scaling attributes, `RubyXScale`, `KentenXScale`) as the stored f64
times 100 with every digit of the product, binary noise included:
`112.99999999999999` for a stored 1.13. Over the paragraph and character
styles and story ranges of the trustworthy pairs, 24,055 reference
values of these attributes have more than 9 decimals; the full product
gives 23,910 of them exactly (rounding to 9 decimals gave none), and
the 140,887 exact matches among the 142,524 shorter values stay the
same.

### More paragraph and character attributes

Found by aligning styles by name over all pairs and runs with IDML
ranges at the same offset (17,914 trustworthy stories). On styles other
than the root styles, each ID is present exactly when the IDML style has
the attribute, and every value matches. "Styles" counts the styles that
have both, other than the root styles in brackets.

Encodings: bool = u16, 1 `true`; pct = f64 fraction, written × 100; num
= f64 or u32 as is; swatch = u32 swatch UID, 0 = `Text Color`; stroke
type = 8 bytes, u32 0 (a built-in style) then the stroke style code
(`StrokeStyle/$ID/` and the name of the code, as for page items). A code
not listed leaves the attribute out.

| ID | IDML | Encoding | Styles |
|---|---|---|---|
| 0x1B14 | `HyphenateLadderLimit` | num | 780 (285) |
| 0x1B68 | `HyphenateLastWord` | bool | 760 (265) |
| 0x1B85 | `HyphenateAcrossColumns` | bool | 751 (256) |
| 0x1B22 | `HyphenateCapitalizedWords` | bool | 709 (214) |
| 0x1B34 | `MaximumGlyphScaling` | pct | 709 (214) |
| 0x1B35 | `MinimumGlyphScaling` | pct | 709 (214) |
| 0x1B20 | `HyphenateAfterFirst` | num | 651 (156) |
| 0x1B23 | `HyphenateWordsLongerThan` | num | 636 (141) |
| 0x1B39 | `KeepWithNext` | num | 619 (124) |
| 0x1B2D | `DesiredWordSpacing` | pct | 587 (92) |
| 0x1B21 | `HyphenateBeforeLast` | num | 580 (85) |
| 0x1B30 | `DesiredLetterSpacing` | pct | 559 (64) |
| 0x1B38 | `KeepAllLinesTogether` | bool | 556 (61) |
| 0x1B3E | `CharacterAlignment` | 0 `AlignBaseline`, 1 `AlignEmCenter`, 2 `AlignEmBottom`, 3 `AlignEmTop`, 4 `AlignICFTop`, 5 `AlignICFBottom` (below) | 551 (56) |
| 0x1B4C | `RuleAboveColor` (Properties) | swatch | 546 (51) |
| 0x1B5C | `RuleAbove` | bool | 541 (46) |
| 0x1B83 | `HyphenWeight` | num | 541 (46) |
| 0x1B71 | `RuleAboveType` (Properties) | stroke type | 520 (25) |
| 0x1B95 | `UnderlineType` (Properties) | stroke type | 541 (46) |
| 0x1B72 | `RuleBelowType` (Properties) | stroke type | 505 (10) |
| 0x1B9D | `StrikeThroughType` (Properties) | stroke type | 499 (4) |
| 0x1A40B | `NumberingFormat` (Properties, string) | u32: 0x1A477 `1, 2, 3, 4...`, 0x1A479 `i, ii, iii, iv...`, 0x1A47A `A, B, C, D...`, 0x1A47B `a, b, c, d...`, 0x1A497 `01,02,03...`, 0x1A478 `I, II, III, IV...`, 0x1A4A7 `$ID/Circle Numbering`, 0 `None`; the last two: below | 531 (36) |
| 0x1B73 | `BalanceRaggedLines` (Properties, enumeration) | 0 `NoBalancing`, 1 `VeeShape` | 528 (33) |
| 0x1A422 | `NumberingAlignment` | 0 `LeftAlign`, 2 `RightAlign` | 527 (32) |
| 0x1A421 | `BulletsAlignment` | 0 `LeftAlign`, 1 `CenterAlign`, 2 `RightAlign` | 505 (10) |
| 0x1B4E | `RuleAboveTint` | num (−1 kept) | 526 (31) |
| 0x50F29 | `ParagraphDirection` | 0 `LeftToRightDirection`, 1 `RightToLeftDirection` | 526 (31) |
| 0x50F2A | `ParagraphJustification` | 0 `DefaultJustification`, 6 `NaskhKashidaJustificationFrac` | 495 roots (4 with 6) |
| 0x50F1F | `CharacterDirection` | 0 `DefaultDirection`, 1 `LeftToRightDirection`, 2 `RightToLeftDirection` | 518 (23) |
| 0x50F11 | `DigitsType` | 0 `DefaultDigits`, 2 `HindiDigits` | 499 (4) |
| 0x1A418 | `NumberingLevel` | num | 525 (30) |
| 0x1A424 | `BulletsTextAfter` | flagged string (as `NumberingExpression`) | 512 (17) |
| 0x1A41D | `NumberingApplyRestartPolicy` | bool | 497 (2) |
| 0x1BB8 | `StrokeAlignment` | 0 `CenterAlignment`, 2 `OutsideAlignment` | 523 (28) |
| 0x4297 | `LeadingModel` | 0 `LeadingModelRoman`, 1 `LeadingModelAkiBelow`, 2 `LeadingModelAkiAbove`, 3 `LeadingModelCenter` | 523 (28) |
| 0x1BC1 | `KeepWithPrevious` | bool | 522 (27) |
| 0x1B93 | `UnderlineTint` | num | 519 (24) |
| 0x1B8E | `UnderlineGapColor` (Properties) | swatch | 518 (23) |
| 0x1B77 | `RuleAboveGapColor` (Properties) | swatch | 506 (11) |
| 0x1B96 | `StrikeThroughColor` (Properties) | swatch | 502 (7) |
| 0x426C | `Rensuuji` | bool | 518 (23) |
| 0x4295 | `OTFProportionalMetrics` | bool | 516 (21) |
| 0x1B57 | `RuleBelowLeftIndent` | num | 513 (18) |
| 0x1B58 | `RuleBelowRightIndent` | num | 504 (9) |
| 0x1B59 | `RuleBelowWidth` | 1 `ColumnWidth`, 2 `TextWidth` | 507 (12) |
| 0x1B86 | `KeepRuleAboveInFrame` | bool | 512 (17) |
| 0x1B6F | `OTFTitling` | bool | 508 (13) |
| 0x1B6E | `OTFDiscretionaryLigature` | bool | 501 (6) |
| 0x1BCA | `OTFSwash` | bool | 501 (6) |
| 0x1B89 | `OTFStylisticSets` | num | 501 (6) |
| 0x1B44 | `OverprintFill` | bool | 506 (11) |
| 0x1B92 | `UnderlineOverprint` | bool | 506 (11) |
| 0x1B8F | `UnderlineGapOverprint` | bool | 500 (5) |
| 0x1B3D | `StrikeThru` | bool | 505 (10) |
| 0x1B9A | `StrikeThroughOffset` | num (−9999 kept) | 501 (6) |
| 0x1B9E | `StrikeThroughWeight` | num (−9999 kept) | 499 (4) |
| 0x1B9C | `StrikeThroughTint` | num | 497 (2) |
| 0x1B78 | `RuleAboveGapTint` | num | 496 (1) |
| 0x1B4B | `Skew` | num | 500 (5) |
| 0x1B65 | `LastLineIndent` | num | 497 (2) |
| 0x1B81 | `PositionalForm` | 0 `None`, 1 `Calculate`, 2 `Initial` | 500 (5) |
| 0x1B87 | `IgnoreEdgeAlignment` | bool | 498 (3) |
| 0x42AD | `GlyphForm` | 0 `None`, 2 `ExpertForm`, 5 `MonospacedHalfWidthForm`, 9 `ProportionalWidthForm`, 10 `FullWidthForm` | 501 (6); code 2 below |
| 0x4222 | `LeadingAki` | num (−1 kept) | 502 (7) |
| 0x4223 | `TrailingAki` | num | 504 (9) |
| 0x4225 | `KinsokuType` | 0 `KinsokuPushInFirst`, 2 `KinsokuPushOutOnly`, 3 `KinsokuPrioritizeAdjustmentAmount` | 502 (7) |
| 0x4226 | `KinsokuHangType` | 0 `None`, 1 `KinsokuHangRegular`, 2 `KinsokuHangForce` | 498 (3); code 1 below |
| 0x4227 | `BunriKinshi` | bool | 499 (4) |
| 0x421D | `Tsume` | num | 499 (4) |
| 0x42A6 | `CjkGridTracking` | bool | 497 (2) |
| 0x422C | `RubyOpenTypePro` | bool | 505 (10) |
| 0x422F | `RubyFontSize` | num | 500 (5) |
| 0x4248 | `KentenFontSize` | num | 500 (5) |
| 0x4232 | `RubyType` | 0 `GroupRuby`, 1 `PerCharacterRuby` | 497 (2) |
| 0x4247 | `KentenKind` | 0 `None`, 1 `KentenSesameDot`, 5 `KentenSmallBlackCircle` | 496 (1); code 5 below |
| 0x1DF1F | `MergeConsecutiveParaBorders` | bool | 347 |
| 0x1DF1A | `ParagraphBorderTopLineWeight` | num | 393 (67) |
| 0x1DF1B | `ParagraphBorderBottomLineWeight` | num | 395 (69) |
| 0x1DF1C | `ParagraphBorderLeftLineWeight` | num | 387 (61) |
| 0x1DF1D | `ParagraphBorderRightLineWeight` | num | 387 (61) |
| 0x1DF01 | `ParagraphBorderLeftOffset` | num | 348 (22) |
| 0x1DF02 | `ParagraphBorderRightOffset` | num | 347 (21) |
| 0x1BF7 | `ParagraphBorderTint` | num | 336 (10) |
| 0x1DF16 | `ParagraphBorderStrokeEndCap` | 0 `ButtEndCap`, 1 `RoundEndCap` | 339 (13) |
| 0x1DF17 | `ParagraphBorderWidth` | 0 `ColumnWidth`, 1 `TextWidth` | 328 (2) |
| 0x1DF20 | `ProviderHyphenationStyle` | 0 `HyphAll`, 3 `HyphPreferredAesthetic` | 327 (1) |
| 0x1BD9 | `ParagraphShadingLeftOffset` | num | 406 (26) |
| 0x1BDA | `ParagraphShadingRightOffset` | num | 406 (26) |
| 0x1BD5 | `ParagraphShadingWidth` | 0 `ColumnWidth`, 1 `TextWidth` | 386 (6) |
| 0x1BD7 | `ParagraphShadingClipToFrame` | bool | 388 (8) |

**`NumberingFormat` codes from paragraph runs.** Two codes occur in
story paragraph runs of the trustworthy pairs, each run matching a
reference `ParagraphStyleRange` with that value and no other:

- 0x1A478 is `I, II, III, IV...` (between 0x1A477 `1, 2, 3, 4...` and
  0x1A479 `i, ii, iii, iv...`): 2 runs in one document, the only 2
  ranges with that value in the corpus IDMLs.
- 0x1A4A7 is `$ID/Circle Numbering`: one run in each of 10 documents;
  a stale pair adds an eleventh. The corpus IDMLs have 11 such values.

Other codes occur only in files without IDML and are not decoded:
0xFFFFFFFF (11 files, styles), 0x1A473 (2), 0x1A472 (1), 0x1A4AA (1).

**Found in the corpus of 2026-10.** These IDs are in every text default
list (chunk 0x23F of the preferences, 1,460 of 1,460 files, no
contradiction) and in styles and runs. Counts are the converter's
output over the 1,251 trustworthy pairs, all reproduced, with no extra
value:

| ID | IDML | Encoding | Text defaults; styles; ranges |
|---|---|---|---|
| 0x1B3A | `KeepFirstLines` | num | 1,251; 1,258 paragraph styles |
| 0x1B3B | `KeepLastLines` | num | 1,251; 1,258 paragraph styles |
| 0x42C1 | `AllowArbitraryHyphenation` | bool | 1,251; 1,270; 30 |
| 0x4264 | `GridGyoudori` | num | 1,251; 1,323; 80 |
| 0x4268 | `RotateSingleByteCharacters` | bool | 1,251; 1,277; 329 |
| 0x4261 | `AutoTcy` | num | 1,251; 1,285; 241 |
| 0x42A3 | `ScaleAffectsLineHeight` | bool | 1,251; 1,359 paragraph and character styles; 879 |
| 0x1BD4 | `ParagraphShadingOverprint` | bool | 1,032; 1,033 |
| 0x1BFD | `ParagraphBorderType` (Properties) | stroke type | 918; 919 |
| 0x4233 | `RubyFont` (Properties) | u32 font family UID; 0 is `$ID/` (as `BulletsFont`) | 1,251; 1,255; 23 |
| 0x4257 | `KentenFillColor` (Properties) | swatch | 1,251; 1,262; 23 |

`KeepFirstLines` and `KeepLastLines` are equal in every list (2 in all
but one file, 1 in that one), also in every paragraph style that has
them, so which ID is which is assumed from their order. 0x1BD4 is told
from 0x1BD8 by one document whose shading is overprinted (0x1BD4 1,
0x1BD8 0). 0x4233 and 0x4257 are told apart in the one document that
sets them by the class of the stored UID: a font family (class 0x3E03)
and a colour. `ParagraphBorderType` uses the stroke type codes of page
items: 0x5A29 `Solid` in 1,042 files, 0x5A3F `Japanese Dots` in one.
0x1BD4 and 0x1BFD are absent exactly where IDML has no attribute.

`MergeConsecutiveParaBorders` is in IDML from version 13.1 (in none of
the 3 trustworthy pairs of version 13.0, in all 46 of 13.1); the
converter writes it when the header or the last session of the save
history is 13.1 or later (`objects.md`, save history).

Evidence over the converter's output (trustworthy pairs): 121,196
values of these attributes in paragraph and character styles, text
defaults and text ranges; 120,409 reproduced, 3 wrong (`TextDefault`
`DesiredLetterSpacing`, `DesiredWordSpacing` and `UnderlineType` in one
document each), the rest in stories whose text differs.

**More codes, corpus of 2026-10.** Over the 1,251 trustworthy pairs,
the converter's output with these codes reproduces the IDML values below
and adds no extra value:

| ID | Code | IDML | Evidence (converter's output, trustworthy pairs) |
|---|---|---|---|
| 0x4266 | 2 | `GridAlignment="AlignEmTop"` | with code 3: 11,024 of 11,024 range values (2,333 more than before), 1,826 of 1,826 paragraph style values |
| 0x4266 | 3 | `GridAlignment="AlignEmCenter"` | |
| 0x1B3E | 3 | `CharacterAlignment="AlignEmTop"` | with codes 4 and 5: 9,603 of 9,605 range values (1,699 more) |
| 0x1B3E | 4, 5 | `AlignICFTop`, `AlignICFBottom` | one run each, one document |
| 0xB676 (cells) | 3 | `FirstBaselineOffset="EmboxHeight"` | 6,668 of 6,668 cell values (2,143 more) |

**Page number type (0x1B09).** A u16 enumeration: 0 `AutoPageNumber`, 2
`NextPageNumber`, 3 `TextVariable`, written as the range's
`PageNumberType`. The root paragraph style has 0, the `AutoPageNumber`
of IDML's root style. In the trustworthy pairs the ranges with
`TextVariable` are those that hold a text variable instance; the
converter reproduces 278 of 278 values with no extra value.

**Custom glyph (0x1B5E).** Value type 0x1B26: u16 length *n*, *n* bytes
of a glyph name, u32 glyph ID. IDML writes `Properties/CustomGlyph`
(type `string`) as `$ID/` and the name, for example `$ID/ampersand.smcp`.
The converter leaves out records with an empty name. It reproduces 1,499
of 1,499 values in the trustworthy pairs, with no extra value. The glyph
ID is not used.

**Alternate glyphs (0x42AE).** A glyph chosen from the alternates of an
OpenType feature (for example in the Glyphs panel) is stored on its run
as attribute 0x42AE, value type 0x42DF: u16 count *n*, then per feature
four bytes of the feature tag in reading order and a u32 number, in the
file's byte order. IDML writes `Properties/OpenTypeFeatures` (type
`list`) on the character range, after `AppliedFont`: one `ListItem` of
type `list` per feature, holding a `ListItem` of type `string` with
`$ID/` and the tag and a `ListItem` of type `long` with the number:

```xml
<OpenTypeFeatures type="list">
  <ListItem type="list">
    <ListItem type="string">$ID/aalt</ListItem>
    <ListItem type="long">1</ListItem>
  </ListItem>
</OpenTypeFeatures>
```

Evidence, in the corpus of 2026-10: 127 of 127 runs in the trustworthy
pairs (27 files) have the IDML value at the same story offset, with 11
tags (`ss01` to `ss07`, `aalt`, `ccmp`, `liga`, `nalt`) and numbers 1,
2 and 8; no IDML range has `OpenTypeFeatures` without the attribute.
Every run in the corpus stores one feature (*n* = 1). The tag order in
both byte orders comes from 7 big-endian files (29 runs, no IDML): the
count and number are big-endian there, the tag bytes are not reversed.
Styles store the attribute with *n* = 0 (2,230 styles of the paired
files); no IDML style has `OpenTypeFeatures`, and the converter writes
nothing for them. The IDML form of several features is not shown, so
the converter leaves out values with *n* other than 1, and values whose
tag is not printable ASCII or that do not fill the value exactly.

### Ruby, kenten and warichu

These attributes are in story runs (character and paragraph runs, chunk
0x262) and in style chunk 0x23F. They were found in the corpus of
2026-10 (803 pairs, 606 trustworthy) by aligning IDML character and
paragraph ranges with the INDD run at the same text offset, and styles
by kind and name. The evidence:

- 24 character ranges in 4 pairs store a full block of CJK attributes
  (150 to 152 attributes each), all with default values, and their IDML
  ranges write every ruby, kenten and warichu attribute. Values that
  occur only once in the block identify the field: 0.66 ↔ 66, 0.5 ↔ 50,
  4 ↔ `RubyJIS`, 7 ↔ `Auto`.
- One paragraph style (a pair of version 13) sets eight ruby attributes
  to other values, and one ruby range (another pair of version 13) sets
  five others.
- 326 ranges and 9 styles in 6 pairs set `RubyAutoAlign="false"` and
  `WarichuAlignment="LeftAlign"`.
- 205 trustworthy ranges in 13 documents have `RubyXScale` and
  `KentenXScale`, 44 have `RubyAutoScaling`, and 573 ranges and styles
  have an empty ruby and kenten font style.

Over all these items, each ID below is present exactly when the IDML
element has the attribute, and every value matches. Encodings: bool =
u16, 1 `true`; pct = f64 fraction, written ×100; code = u16, by the
table; string = flag byte and in-object string.

| ID | IDML | Encoding | Values seen (items) |
|---|---|---|---|
| 0x4231 | `RubyAlignment` | code: 1 `RubyCenter`, 2 `RubyRight`, 4 `RubyJIS` | 4: 24 ranges; 2: 1 range; 1: 1 style |
| 0x4235 | `RubyParentSpacing` | code: 0 `RubyParentNoAdjustment`, 1 `RubyParentBothSides`, 2 `RubyParent121Aki` | 2: 24; 0: 1; 1: 1 |
| 0x423C | `RubyParentOverhangAmount` | code: 0 `None`, 1 `RubyOverhangOneRuby`, 5 `RubyOverhangNoLimit` | 1: 24; 5: 1; 0: 1 |
| 0x423A | `RubyPosition` | code: 0 `AboveRight`, 1 `BelowLeft` | 0: 24; 1: 1 |
| 0x4239 | `RubyYOffset` | f64, points | 0: 24; −1.4173228: 1 |
| 0x423F | `RubyParentScalingPercent` | pct | 0.66: 24; 0.4: 1 |
| 0x423E | `RubyAutoScaling` | bool | true: 21 ranges and 1 style; false: 24 |
| 0x423B | `RubyAutoAlign` | bool | false: 326 ranges and 9 styles; true: 24 |
| 0x42B1 | `RubyAutoTcyDigits` | u16 number | 0: 24; 2: 3 styles; 5: 1 |
| 0x42B2 | `RubyAutoTcyIncludeRoman` | bool | false: 24; true: 1 |
| 0x4236 | `RubyXScale` | pct | 222 ranges and 1 style, 80.9 to 113.1 |
| 0x424C | `KentenXScale` | pct | the same items; always equal to 0x4236 |
| 0x4234 | `RubyFontStyle` (Properties) | string; empty = enumeration `Nothing` | 512 ranges and 61 styles, all empty |
| 0x424B | `KentenFontStyle` (Properties) | as 0x4234 | the same items |
| 0x4281 | `WarichuAlignment` | code: 0 `LeftAlign`, 7 `Auto` | 0: 326 ranges and 9 styles; 7: 24 |
| 0x427D | `Warichu` | bool | false: 24 ranges; true: print PDF, below |
| 0x427E | `WarichuLines` | u16 number | 2: 24; see below |
| 0x427F | `WarichuSize` | pct | 0.5 ↔ 50: 24; print PDF, below |
| 0x4280 | `WarichuLineSpacing` | f64, points | 0: 24; print PDF, below |

`KentenKind` (0x4247) code 5 is `KentenSmallBlackCircle` (1 range).
`KinsokuHangType` (0x4226) code 1 is `KinsokuHangRegular`: 31 styles and
120 ranges over all pairs (5 styles, 8 paragraph ranges and 6 story
ranges in trustworthy pairs), and no other value goes with code 1.
`GlyphForm` (0x42AD) code 2 is `ExpertForm`. Over the pairs of the
corpus of 2026-10 that have the value, the converted output of each
pair has `GlyphForm="ExpertForm"` on as many elements of each kind as
its IDML: 41 character ranges in 13 pairs, 45 paragraph styles in 31 pairs and 4 text defaults
(34 pairs with it in styles or text defaults). Codes 1, 3, 4, 11 and 12
occur only in samples without IDML; the converter leaves them out.
With both codes, `compare.py` reports no wrong, missing or extra
`GlyphForm` or `OTFFigureStyle` value in the pairs of the corpus of
2026-10, in styles, text defaults or ranges.

**How IDs with equal values were told apart.**

- `RubyAutoAlign` and `WarichuAlignment` are always present together,
  with (1, 7) for (`true`, `Auto`) and (0, 0) for (`false`,
  `LeftAlign`). 0x423B has value type 0x1B03, which 17 attributes use
  (booleans and `RubyType`), and over all styles the values of that
  type are 0 or 1 (9,186 of 9,186). 0x4281 has its own value type and
  holds 7. So 0x423B is the boolean.
- `RubyYOffset`, `RubyPosition` and `RubyParentScalingPercent` are in
  the same 25 items. The style's values −1.4173 (f64), 1 (u16) and 0.4
  (fraction) fit only one attribute each.
- `RubyAlignment`, `RubyParentSpacing`, `RubyParentOverhangAmount`,
  `RubyAutoTcyDigits` and `RubyAutoTcyIncludeRoman`: the range sets (2,
  0, 5, 5, 1) and the style (1, 1, 0, none, none). 0x423C and 0x42B1
  are both 5 in the range; the 24 default ranges (1 ↔
  `RubyOverhangOneRuby`, 0 ↔ `RubyAutoTcyDigits="0"`) and the style
  (0x423C = 0 ↔ `None`, no 0x42B1) separate them.
- Ruby or kenten for 0x4234/0x424B and 0x4236/0x424C: the two values are
  equal in every pair item. The ruby strand (`objects.md`) decides it:
  its records hold only ruby attributes, including 0x4234 (5,145 runs)
  and 0x4236 (5,297 runs), and never 0x424B, 0x424C or a kenten ID
  (0x4247, 0x4248). So 0x4234 and 0x4236 are the ruby attributes.

**Values without an IDML pair.** 0x423C = 2 (6 styles), 0x4281 = 2 (5
styles), 0x423F = 0.75 (5 runs) and non-empty font styles (`Regular` in
104 runs) occur only in files without IDML. The converter leaves them
out.

**Warichu.** No pair has warichu switched on. The evidence for
`Warichu`, `WarichuSize` and `WarichuLineSpacing` is a third-party
sample typeset vertically (version 21, no IDML) and its print PDF:

- A run of 7 characters sets only 0x427D = 1. In the PDF the paragraph
  is 8.504 pt and the run is set at 4.252 pt (50 %) in two rows of 3 and
  4 characters, with baselines 4.252 pt apart (row pitch = size + 0),
  between full-size characters. The PDFs of three saved revisions show
  the same.
- A run of 36 characters sets 0x427D = 1, 0x427F = 0.6, 0x4280 =
  0.708661, 0x4281 = 0, and 0x428E = 0x428F = 1. In the PDF the
  paragraph is 6.378 pt and the run is 3.827 pt (0.600 × 6.378) in two
  rows that start at the same position, with baselines 4.535 pt apart
  (3.827 + 0.708), and it continues on the next line in two rows again.

So 0x427D switches warichu on, 0x427F is the size as a fraction of the
point size, and 0x4280 is the extra space between the rows in points.
Both runs have two rows, the second with 0x428E and 0x428F set to 1, and
the root style stores 0x427E = 2. So neither 0x428E nor 0x428F is the
number of rows. 0x427E is `WarichuLines`: value type 0x1B10 is used
only by 0x427E, 0x428E and 0x428F, and 0x427E is the remaining one.

**Other IDs of the block.** The 24 default ranges also hold IDs for the
ruby and kenten fonts, colours, tints, overprint, weights, Y scales and
offsets, kenten placement and characters, and two warichu IDs (0x428E,
0x428F; 2 and 2 in every IDML range). Each holds the default value in
every pair, and several IDs hold the same value, so their values cannot
tell them apart. Their order in IDML places them (attributes placed by
order, below).

### Attributes placed by order

IDML writes a style's text attributes in ascending order of their
attribute ID: the XML attributes in that order, and the `Properties`
children in that order among themselves. The style's own attributes
(`Self`, `Name`, `Imported`, `NextStyle`, `KeyboardShortcut` and the
like) are not part of this order. Evidence: the 11,983 paragraph and
character styles of the trustworthy pairs that match an INDD style by
kind and name (2,504 root styles, 6,671 other paragraph styles and
2,808 other character styles). Every one of their 0x23F lists is in
ascending ID order, and the IDs of the other tables in this section
appear in IDML in the same order:

| Sequence | Adjacent pairs of IDs | Out of order |
|---|---:|---:|
| XML attributes | 270,429 | 0 |
| `Properties` children | 44,683 | 0 |
| XML attributes, with the table below | 358,891 | 0 |
| `Properties` children, with the table below | 52,244 | 0 |

The IDs below were placed by this order. For each IDML attribute that
no known ID explains, the candidates are the stored IDs without a
mapping that lie between the nearest known IDs before and after it.
Intersected over every trustworthy style that has the attribute, and
assigned in order within each gap, this gives one assignment in every
gap but four. The payload type has to fit: a `true` or `false` value
needs a 2-byte value of type 0x1B03, 0x1B04, 0x1B09 or 0x1B1F, a number
an f64 equal to the IDML value (or to it divided by 100 for a
percentage), and a colour or font in `Properties` a UID (type 0x1B05).
In the four gaps the type decides:

- 0x1B79–0x1B7D, five IDs for four attributes: 0x1B7A has type 0x1B0D
  (0 in 1,252 of 1,252 roots), not a boolean type, so
  `RuleAboveGapOverprint` is 0x1B79. 0x1B7B is the only UID:
  `RuleBelowGapColor`.
- 0x1BFB, 0x1BFC and 0x1BFE for `ParagraphBorderGapTint` (`-1`) and
  `ParagraphBorderGapOverprint` (`false`): 0x1BFE is an f64 of 1.0 in
  919 of the 919 roots that store it, which fits neither.
- the ruby IDs 0x4237–0x4246, ten IDs for eight XML attributes and two
  `Properties` children: 0x4240 and 0x4243 are the only UIDs.
- the kenten, tatechuyoko, warichu and OpenType IDs 0x424D–0x42B3, 23
  IDs for 22 XML attributes and one `Properties` child: 0x425A is the
  only UID.

The table counts the trustworthy pairs: styles (the root paragraph
style and others) and character or paragraph ranges whose stored value
the converter writes equal to the reference IDML's, and those it writes
with another value or where the IDML has none (Differ). Every value of
these attributes that the IDML has on a matched style or range is
written. Encodings: bool =
u16, 0 `false`, 1 `true`; pct = f64 fraction, written ×100; colour =
swatch UID, 0 `Text Color`; code = u16 or u32, by the listed codes. The
second part of the encoding column is the value type.

"Order only" marks an ID whose value is the same in every trustworthy
item, so only the order and the payload type place it. The four
OpenType alternates vary, but always together, so the order alone
separates them too.

| ID | IDML | Encoding, type | Equal: root styles | other styles | ranges | Differ | Values seen | Placed by |
|---|---|---|---:|---:|---:|---:|---|---|
| 0x1B41 | `StrokeTint` | f64, 0x1BB1 | 1,252 | 9 | 23 | 0 | `-1` (1,282), `100` (2) | order, values |
| 0x1B43 | `OverprintStroke` | bool, 0x1B04 | 1,252 | 12 | 23 | 0 | `false` (1,287) | order only |
| 0x1B45 | `GradientStrokeAngle` | f64, 0x1B2C | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x1B47 | `GradientStrokeLength` | f64, 0x1B13 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x1B49 | `GradientStrokeStart` | two f64, 0x1B22 | 1,252 | 7 | 23 | 0 | `0 0` (1,282) | order only |
| 0x1B5A | `RuleAboveOverprint` | bool, 0x1B04 | 1,252 | – | – | 0 | `false` (1,252) | order only |
| 0x1B5B | `RuleBelowOverprint` | bool, 0x1B04 | 1,252 | – | – | 0 | `false` (1,252) | order only |
| 0x1B6C | `OTFOrdinal` | bool, 0x1B04 | 1,252 | 8 | 23 | 0 | `false` (1,283) | order only |
| 0x1B6D | `OTFFraction` | bool, 0x1B04 | 1,252 | 8 | 56 | 0 | `false` (1,283), `true` (33) | order, values |
| 0x1B79 | `RuleAboveGapOverprint` | bool, 0x1B04 | 1,252 | – | – | 0 | `false` (1,252) | order only |
| 0x1B7B | `RuleBelowGapColor` (Properties) | colour, 0x1B05 | 1,252 | 10 | 357 | 0 | `Swatch/None` (1,252), `Text Color` (367) | order, values |
| 0x1B7C | `RuleBelowGapTint` | f64, 0x1BB1 | 1,252 | – | – | 0 | `-1` (1,252) | order only |
| 0x1B7D | `RuleBelowGapOverprint` | bool, 0x1B04 | 1,252 | – | – | 0 | `false` (1,252) | order only |
| 0x1B82 | `OTFMark` | bool, 0x1B03 | 1,252 | – | 23 | 0 | `true` (1,275) | order only |
| 0x1B84 | `OTFLocale` | bool, 0x1B03 | 1,252 | – | 23 | 0 | `true` (1,275) | order only |
| 0x1B88 | `OTFSlashedZero` | bool, 0x1B04 | 1,252 | 7 | 32 | 0 | `false` (1,282), `true` (9) | order, values |
| 0x1B8A | `OTFHistorical` | bool, 0x1B04 | 1,252 | 7 | 23 | 0 | `false` (1,282) | order only |
| 0x1B90 | `UnderlineGapTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x1B97 | `StrikeThroughGapColor` (Properties) | colour, 0x1B05 | 1,252 | 17 | 729 | 0 | `Swatch/None` (1,282), `Text Color` (716) | order, values |
| 0x1B98 | `StrikeThroughGapOverprint` | bool, 0x1B04 | 1,252 | 7 | 23 | 0 | `false` (1,282) | order only |
| 0x1B99 | `StrikeThroughGapTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x1B9B | `StrikeThroughOverprint` | bool, 0x1B04 | 1,252 | 7 | 23 | 0 | `false` (1,282) | order only |
| 0x1BC0 | `SplitColumnOutsideGutter` | f64, 0x1BA8 | 1,252 | – | – | 0 | `0` (1,252) | order only |
| 0x1BC3 | `SpanColumnMinSpaceBefore` | f64, 0x1BA8 | 1,252 | – | – | 0 | `0` (1,252) | order only |
| 0x1BD8 | `ParagraphShadingSuppressPrinting` | bool, 0x1B04 | 1,033 | – | – | 0 | `false` (1,033) | order only |
| 0x1BF8 | `ParagraphBorderOverprint` | bool, 0x1B04 | 919 | – | – | 0 | `false` (919) | order only |
| 0x1BFB | `ParagraphBorderGapTint` | f64, 0x1BB1 | 919 | – | – | 0 | `-1` (919) | order only |
| 0x1BFC | `ParagraphBorderGapOverprint` | bool, 0x1B04 | 919 | – | – | 0 | `false` (919) | order only |
| 0x4237 | `RubyYScale` | pct, 0x1B16 | 1,252 | 7 | 23 | 0 | `100` (1,282) | order only |
| 0x4238 | `RubyXOffset` | f64, 0x1B14 | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x423D | `RubyOverhang` | bool, 0x1B04 | 1,252 | 7 | 23 | 0 | `false` (1,282) | order only |
| 0x4240 | `RubyFill` (Properties) | colour, 0x1B05 | 1,252 | 7 | 23 | 0 | `Text Color` (1,282) | order only |
| 0x4241 | `RubyTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x4242 | `RubyOverprintFill` | u16 code 0xFFFF `Auto`; 0x1BBB | 1,252 | – | 23 | 0 | `Auto` (1,275) | order only |
| 0x4243 | `RubyStroke` (Properties) | colour, 0x1B05 | 1,252 | 7 | 23 | 0 | `Text Color` (1,282) | order only |
| 0x4244 | `RubyStrokeTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x4245 | `RubyOverprintStroke` | u16 code 0xFFFF `Auto`; 0x1BBB | 1,252 | – | 23 | 0 | `Auto` (1,275) | order only |
| 0x4246 | `RubyWeight` | f64, 0x1BB6 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x424A | `KentenFont` (Properties) | font family UID, 0 `$ID/`; 0x1B05 | 1,252 | 1 | 23 | 0 | `$ID/` (1,275), a composite font (1) | order, values |
| 0x424D | `KentenYScale` | pct, 0x1B16 | 1,252 | 7 | 23 | 0 | `100` (1,282) | order only |
| 0x424F | `KentenPlacement` | f64, 0x1B14 | 1,252 | 10 | 23 | 0 | `0` (1,285) | order only |
| 0x4250 | `KentenAlignment` | u16 code 1 `AlignKentenCenter`; 0x4238 | 1,252 | 10 | 23 | 0 | `AlignKentenCenter` (1,285) | order only |
| 0x4251 | `KentenPosition` | u16 code 0 `AboveRight`; 0x1B04 | 1,252 | 10 | 23 | 0 | `AboveRight` (1,285) | order only |
| 0x4252 | `KentenCustomCharacter` | u16 code 0 empty; 0x1B0D | 1,252 | 7 | 23 | 0 | empty (1,282) | order only |
| 0x4253 | `KentenCharacterSet` | u16 code 0 `CharacterInput`; 0x1B0D | 1,252 | 7 | 23 | 0 | `CharacterInput` (1,282) | order only |
| 0x4258 | `KentenTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x4259 | `KentenOverprintFill` | u16 code 0xFFFF `Auto`; 0x1BBB | 1,252 | – | 23 | 0 | `Auto` (1,275) | order only |
| 0x425A | `KentenStrokeColor` (Properties) | colour, 0x1B05 | 1,252 | 7 | 23 | 0 | `Text Color` (1,282) | order only |
| 0x425B | `KentenStrokeTint` | f64, 0x1BB1 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x425C | `KentenOverprintStroke` | u16 code 0xFFFF `Auto`; 0x1BBB | 1,252 | – | 23 | 0 | `Auto` (1,275) | order only |
| 0x425D | `KentenWeight` | f64, 0x1BB6 | 1,252 | 7 | 23 | 0 | `-1` (1,282) | order only |
| 0x425F | `TatechuyokoXOffset` | f64, 0x1B14 | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x4260 | `TatechuyokoYOffset` | f64, 0x1B14 | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x4262 | `AutoTcyIncludeRoman` | bool, 0x1B04 | 1,252 | – | – | 0 | `false` (1,252) | order only |
| 0x4263 | `Jidori` | u16, 0x1B0D | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x4267 | `CharacterRotation` | f64, 0x1B14 | 1,252 | 7 | 25 | 0 | `0` (1,282), `3` (2) | order, values |
| 0x4278 | `ShataiMagnification` | f64, 0x42A4 | 1,252 | 7 | 23 | 0 | `0` (1,282) | order only |
| 0x428E | `WarichuCharsBeforeBreak` | u16, 0x1B10 | 1,252 | 7 | 23 | 0 | `2` (1,282) | order only |
| 0x428F | `WarichuCharsAfterBreak` | u16, 0x1B10 | 1,252 | 7 | 23 | 0 | `2` (1,282) | order only |
| 0x4294 | `OTFHVKana` | bool, 0x1B04 | 1,252 | 8 | 23 | 0 | `false` (1,283) | order only |
| 0x4296 | `OTFRomanItalics` | bool, 0x1B04 | 1,252 | 8 | 23 | 0 | `false` (1,283) | order only |
| 0x42A4 | `ParagraphGyoudori` | bool, 0x1B04 | 1,252 | 12 | 1 | 0 | `false` (1,252), `true` (13) | order, values |
| 0x42B3 | `RubyAutoTcyAutoScale` | bool, 0x1B03 | 1,252 | 7 | 23 | 0 | `true` (1,282) | order only |
| 0x1A407 | `NumberingStartAt` | u32, 0x1B00 | 1,252 | – | 7 | 0 | `1` (1,252), `2` (5), `3` (2) | order, values |
| 0x1A41E | `NumberingRestartPolicies` (Properties) | three u32 (below), 0x1A4A6 | 1,252 | 1 | – | 0 | `AnyPreviousLevel` 0 0 (1,252), `AfterSpecificLevel` 1 1 (1) | order, values |
| 0x1DF05 | `ParagraphBorderStrokeEndJoin` | u32 code 0 `MiterEndJoin`; 0x1DF3B | 919 | – | – | 0 | `MiterEndJoin` (919) | order only |
| 0x1DF06 | `ParagraphBorderTopLeftCornerOption` | u32 code 0 `None`; 0x1DF51 | 919 | – | – | 0 | `None` (919) | order only |
| 0x1DF07 | `ParagraphBorderTopRightCornerOption` | u32 code 0 `None`; 0x1DF51 | 919 | – | – | 0 | `None` (919) | order only |
| 0x1DF08 | `ParagraphBorderBottomLeftCornerOption` | u32 code 0 `None`; 0x1DF51 | 919 | – | – | 0 | `None` (919) | order only |
| 0x1DF09 | `ParagraphBorderBottomRightCornerOption` | u32 code 0 `None`; 0x1DF51 | 919 | – | – | 0 | `None` (919) | order only |
| 0x1DF1E | `ParagraphBorderDisplayIfSplits` | bool, 0x1B04 | 919 | – | – | 0 | `false` (919) | order only |
| 0x50F15 | `Kashidas` | u32 code 0 `DefaultKashidas`; 0x50F1B | 1,252 | – | 23 | 0 | `DefaultKashidas` (1,275) | order only |
| 0x50F2B | `ParagraphKashidaWidth` | f64, 0x1DF21 | 1,243 | – | – | 0 | `2` (1,243) | order only |
| 0x50F2F | `XOffsetDiacritic` | f64, 0x1B14 | 1,252 | – | – | 0 | `0` (1,252) | order only |
| 0x50F30 | `YOffsetDiacritic` | f64, 0x1B14 | 1,252 | – | – | 0 | `0` (1,252) | order only |
| 0x50F3C | `OTFOverlapSwash` | bool, 0x1B04 | 1,252 | – | 25 | 0 | `false` (1,275), `true` (2) | order only (the four vary together) |
| 0x50F3D | `OTFStylisticAlternate` | bool, 0x1B04 | 1,252 | – | 25 | 0 | `false` (1,275), `true` (2) | order only (the four vary together) |
| 0x50F3E | `OTFJustificationAlternate` | bool, 0x1B04 | 1,252 | – | 25 | 0 | `false` (1,275), `true` (2) | order only (the four vary together) |
| 0x50F3F | `OTFStretchedAlternate` | bool, 0x1B04 | 1,252 | – | 25 | 0 | `false` (1,275), `true` (2) | order only (the four vary together) |
| 0x50F40 | `KeyboardDirection` | u32 code 0 `DefaultDirection`, 1 `LeftToRightDirection`; 0x50F2E | 1,252 | – | 34 | 0 | `DefaultDirection` (1,252), `LeftToRightDirection` (34) | order, values |

Notes:

- Tints, weights and lengths are written as stored (`-1`, and `100` for
  the 2 `StrokeTint` values that store 100), not by the page item tint
  rule.
- `KentenPosition` has a boolean value type but an enumeration value
  (0 = `AboveRight`).
- The enumerations list only the codes seen. Other codes are left out
  and reported by `indd audit`. Each has one code in the trustworthy
  pairs, except `KeyboardDirection`: code 1 is `LeftToRightDirection` in
  34 of 34 character ranges of one pair. It has the value type of
  `CharacterDirection` (0x50F2E), whose code 1 is the same value.
- `NumberingRestartPolicies` is a `Properties` child with three XML
  attributes and no text. Its value is three u32: (0, 0, 0) in the
  1,252 roots is `RestartPolicy="AnyPreviousLevel" LowerLevel="0"
  UpperLevel="0"`, and (1, 1, 1) in one other paragraph style is
  `RestartPolicy="AfterSpecificLevel" LowerLevel="1" UpperLevel="1"`.
  The second triple does not show which word is the lower level and
  which the upper, so the converter writes these two triples only.

**Stored but never written.** The root paragraph styles also store IDs
that no IDML attribute matches: after this table, every text attribute
of the trustworthy IDML styles is explained by a stored ID except
`Label` (6 styles) and `HyphenQuality` (7 styles). Such IDs, with the
number of the 1,252 trustworthy roots that store each: 0x1B36, 0x1B3F,
0x1B5F–0x1B63, 0x1B7A, 0x4228, 0x4230, 0x4249, 0x4269–0x426B, 0x4290,
0x429F, 0x1300F, 0x13516, 0x13549 and 0x20714 (1,252 each), 0x1A405 and
0x1A415 (1,228), 0x1BFE (919), and in later versions 0xBA429, 0xBA42A
and eight IDs from 0x16923C to 0x16924A (24 each) and 18 IDs from
0x5351D to 0x58820 (2 each). The converter does not write them.

### Root styles and text defaults

The root paragraph style `[No paragraph style]` and the document's text
defaults (`TextDefault` in `Resources/Preferences.xml`) take their
values from attribute lists like other styles, but IDML leaves some of
them out. Over the 489 trustworthy pairs (`measurement.md`):

| Value | IDML |
|---|---|
| `NextStyle` of the root paragraph style | never (0 of 489) |
| `AllNestedStyles` of the root paragraph style | never (0 of 489) |
| `AllNestedStyles` of `TextDefault` | only when the list has items: absent in 485, written in the 4 with items |
| `AllNestedStyles` of other paragraph styles | only when the list has items: written on 115 of 115 trustworthy styles with items, on 0 of the 2 that store a count of 0 |
| `AllGREPStyles` and `AllLineStyles` of the root paragraph style | never (0 of 1,251 trustworthy pairs after 2026-10); the root style stores a count of 0 in all 1,251 |
| `AllGREPStyles` of `TextDefault` | only when the list has items (48 of 48 such lists written, 1,251 trustworthy pairs) |
| Empty `TabList` of the root paragraph style | DOM 8 on: 482 of 482; DOM 7: 0 of 7 |
| `TabList` of `TextDefault` | DOM 8 on: in 482 of 482 (459 empty); DOM 7: 0 of 7 |

The converter leaves out these values accordingly; it writes a
non-empty `TabList` in any version.

### Paragraph borders and shading: corners and origins

| ID | IDML | Styles with both (other than the root) |
|---|---|---|
| 0x1DF12 | `ParagraphShadingTopLeftCornerRadius` | 452 (126) |
| 0x1DF14 | `ParagraphShadingBottomLeftCornerRadius` | 452 (126) |

On every style other than the root the ID is present exactly when the
IDML style has the attribute, and every value matches. The two are told
apart by 8 styles (4.5 against 0).

**Tied groups.** The IDs of each group below have the same value in
every style, so which ID belongs to which attribute is not known. The
converter writes a group only when all its IDs are present and equal:

| IDs | IDML | Styles |
|---|---|---|
| 0x1DF0A–0x1DF0D | the four `ParagraphBorder…CornerRadius` | 446 (120) |
| 0x1DF13, 0x1DF15 | `ParagraphShadingTopRightCornerRadius`, `ParagraphShadingBottomRightCornerRadius` | 444 (118) |
| 0x1DF0E, 0x1DF10 | `ParagraphShadingTopLeftCornerOption`, `ParagraphShadingBottomLeftCornerOption` | 339 (13) |
| 0x1DF0F, 0x1DF11 | `ParagraphShadingTopRightCornerOption`, `ParagraphShadingBottomRightCornerOption` | 331 (5) |

The shading corner option codes are 0 `None`, 0x5A15 `RoundedCorner`
and 0x5A18 `BevelCorner`.

**Origins.** The two IDs of each pair always change together; the
converter writes a pair only for these combinations of codes:

| IDs | IDML | Codes → values | Styles |
|---|---|---|---|
| 0x1BDD, 0x1BDE | `ParagraphShadingTopOrigin`, `ParagraphShadingBottomOrigin` | 0, 0 → `AscentTopOrigin`, `DescentBottomOrigin`; 3, 2 → `EmBoxTopOrigin`, `EmBoxBottomOrigin`; 1, 1 → `BaselineTopOrigin`, `BaselineBottomOrigin` | 378, 14, 4 |
| 0x1DF18, 0x1DF19 | `ParagraphBorderTopOrigin`, `ParagraphBorderBottomOrigin` | 0, 0 → `AscentTopOrigin`, `DescentBottomOrigin`; 3, 2 → `EmBoxTopOrigin`, `EmBoxBottomOrigin` | 325, 13 |

Attributes whose value never varies in the corpus cannot be located this
way. The converter writes the root styles' values for them from IDML
observation (`idml-values.md`); styles and ranges inherit them. Many
others are placed by their order in IDML (attributes placed by order,
above). The attributes left include
`KeepFirstLines`, `KeepLastLines`, `KeepAllLinesTogether`,
`KeepWithNext`, `KeepWithPrevious`, `Skew`, `StrikeThru`,
`LastLineIndent`, `RuleAbove`, `RuleAboveColor`, the `RuleBelow` indents
and width, and the hyphenation and word-spacing settings other than those
in the table. Some attributes vary but cannot be told apart from another
attribute, because both always have the same value or appear in the same
styles:

- left and right shading offsets, left and right border offsets, and the
  four border line weights (equal in every sample);
- `KeepRuleAboveInFrame` and `RuleAboveType` (always set together, with
  other attributes, in one pair);
- `TreatIdeographicSpaceAsSpace` and `DiacriticPosition`.

`OTFFigureStyle` code 3 is `TabularOldstyle`. One pair has it, on one
character range, and its IDML has `TabularOldstyle` on that range.
The value occurs in 3 more IDMLs of the corpus of 2026-10, none with an
INDD.

**`ParagraphBreakType` (0x1B6A) codes.** The codes are those of
`StartParagraph`. The evidence is the 112 pairs of the corpus of 2026-10
whose IDML has a `CharacterStyleRange` with a value other than
`Anywhere` (no paragraph range has one). In each story the runs with a
code other than 0 were aligned in order with those ranges; every story
of these pairs aligns:

| Code | IDML value | Trustworthy pairs | Stale pairs |
|---|---|---|---|
| 1 | `NextColumn` | 642 ranges in 74 documents | 80 in 11 |
| 2 | `NextPage` | 159 in 19 | 16 in 8 |
| 3 | `NextFrame` | 76 in 19 | 0 |
| 4 | `NextOddPage` | 5 in 1 (version 20.1) | 0 |

`NextEvenPage` occurs in no IDML. Its code is not known, and the
converter leaves other codes out.

**Empty font styles.** A list can store an empty `FontStyle` (0x1B02).
IDML then writes the style of a font of the list's `AppliedFont` family:
the font whose style is `Regular`, or the family's first font if none
is. Over the 1,251 trustworthy pairs, writing the stored empty string
gave 294 wrong `FontStyle` values (ParagraphStyle 69, CharacterStyle 35,
TextDefault 11, text ranges 179); with this rule 243 of them match and
no value that matched before changes. 20 of the 243 come from the
first-font case (a family of `Light` and `LightItalic`, IDML `Light`).
So every empty value that IDML also writes empty is in a list without
an `AppliedFont`, or in a family without a `Regular` font whose first
font has an empty style. Not explained: 22 empty values that IDML writes as
`Nothing`, and lists without an `AppliedFont`, which keep the empty
string.

**Font family names.** `AppliedFont` and `BulletsFont` name a font family
(class 0x3E03), and IDML writes the family's name. An `AppliedFont`
whose family stands for a composite font is written as a reference to
the composite font, except in character styles. See `fonts.md`,
composite fonts as applied fonts.

**Alignment in InDesign 2.0.** Files from InDesign 2.0 have no 0x1B7E.
Paragraph styles and paragraph-style records have two attributes with
the value type of 0x1B7E (0x1B20, a u16 code; 0x1B7E has this type in
a 9.2 fixture): 0x1B05, the alignment (0 left, 1 center, 2 right, 3
justified), and 0x1B04, the alignment of the last line. The root
paragraph style has both (0 and 0); other styles and records have one or
both, and inherit the other through `BasedOn`. Over the 82 records of
three privately held 2.0 documents that set either (`objects.md`, the
story list of InDesign 2.0), the pairs in effect (0x1B04, 0x1B05)
are (0, 0) 16, (1, 1) 26, (2, 2) 11, (0, 3) 13, (1, 3) 6 and (2, 3) 10.
They are mapped as (0, 0) `LeftAlign`, (1, 1) `CenterAlign`, (2, 2)
`RightAlign`, (0, 3) `LeftJustified`, (1, 3) `CenterJustified` and (2,
3) `RightJustified`. A seventh pair, (3, 3), occurs once, in a later
2.0 document: a one-word paragraph whose word the PDF spreads over the
frame's full width (letters from 86 to 514 pt in a frame from 86.1 to
513.6), so its last line is justified too; it is mapped as
`FullyJustified`. Whether 0x1B04 is the alignment of the last line or
of the whole paragraph cannot be told from these pairs (they agree
where 0x1B05 is below 3), so no other pair is mapped: the converter
leaves it out with a warning. Evidence from the print PDFs, over the
paragraphs of three lines or more whose PDF lines can be told apart:
65 `LeftJustified` paragraphs are flush on both edges, 4 `CenterAlign`
centred and 2 `LeftAlign` flush left; 17 more read otherwise only where
the PDF text joins lines of two columns, or (one paragraph) where the
left edge steps around a wrapped image. Short paragraphs agree: bylines
with (2, 3) end at the frame's right edge, a short paragraph with (1,
3) is centred in its column. Before this, 0x1B04 and 0x1B05 were not read and
every paragraph of these documents was left aligned.
