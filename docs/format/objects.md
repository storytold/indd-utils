# INDD objects

How the byte stream of a database object (see `database.md`) is
structured, and what the classes and chunks used by the converter mean.
Implemented in `src/object.rs` and `src/model/`.

## How this was learned

IDML exported by InDesign names most objects `Self="u<hex>"`, and that hex
number is the object's UID in the INDD file. So each corpus pair gives, for
every such IDML element, the INDD object, its class, and the attribute
values its chunks must encode. Facts below were found by comparing the two,
and are checked by `tools/compare.py`, which converts every pair (78 unique
pairs, InDesign 7.5–21) and compares the output with the reference IDML
attribute by attribute.

## Chunks

Most objects (99.9 % of 773,753 in the little-endian corpus) are a sequence
of chunks:

| Size | Field |
|---|---|
| 4 | Chunk ID |
| 4 | Data length *n* |
| *n* | Data |

The rest are plain byte streams: objects of class 0x129 (raw data, below)
and the XMP packet (UID 0x80000001). In the 245 distinct little-endian
files, all 1,171 class 0x129 objects are plain byte streams. Chunk IDs are in the same number space as
class IDs; a chunk often lists objects of the class with the same number
(for example chunk 0x501 of the document lists spreads, class 0x501).

## Primitive encodings

Object data is in the byte order of the header's flag (`header.md`).
All samples except two are little-endian, and the examples here are
little-endian. Big-endian data and the layouts of InDesign 3.0 and 4.0
are in `big-endian.md`.

- **UID lists:** u32 count, then that many u32 UIDs.
- **Matrices:** six f64, in IDML `ItemTransform` order (a b c d tx ty).
- **Text segments:** a u16 header, flags in bits 14–15 and a count in bits
  0–13. 0x4000: *count* single-byte characters follow. 0x8000: *count*
  UTF-16 code units follow. A text of *n* code units is a sequence of
  segments adding up to *n*. Example: "WOMEN’S\r" is
  `05 40 "WOMEN" 01 80 19 20 02 40 "S\r"`.
- **In-object strings:** u8 2, u8 (usually 0; a script code in some
  strings, below), u16 length in code units, then segments. 4,135 occurrences in three sample files.
  In files from InDesign 2.0 the first byte is 1: in the four distinct 2.0
  files, the pattern tag, 0, length *n*, 0, segment header 0x4000 + *n*
  occurs 154 to 353 times per file with tag 1 and never with tag 2. In
  the 83 distinct 3.0, 4.0 and 5.0 little-endian files the same pattern
  has tag 2 (76 to 4,296 times per file); tag 1 matches once in each of
  two of them.

## Document (class 0xE01, always UID 1)

| Chunk | Contents |
|---|---|
| 0x501 | UID list: spreads, in order |
| 0x1401 | UID list: master spreads |
| 0x301 | UID list: layers (the first is an internal layer that holds pages) |
| 0x313 | u32: active layer (IDML `ActiveLayer`) |
| 0x222 | Two UID lists: the stories, then the XML backing story; IDML `StoryList` is both (`xml.md`). InDesign 2.0: one list of 6-byte records (below) |
| 0x4C01 | UID list: sections |
| 0xA443 | Document users: u32 count, then per user a flag byte, the name as an in-object string, u32 colour |
| 0x1D8 | Save history: u32 count, then one record per session (below) |
| 0x1630B | Label: u16 0x7B7B, u32 count *n*, then *n* pairs of key and value, each a flag byte and an in-object string |
| 0x21B03 | UID list: articles, in order (below, articles) |

**Document label.** IDML writes `Document/Properties/Label` with one
`KeyValuePair` (`Key`, `Value`) per stored pair, in stored order, with
the strings as stored. Without the chunk there is no `Label`.
Evidence: 436 of the 438 trustworthy pairs whose IDML has a document
label match (one differs, one has no chunk). The flag byte is 1 on 4
keys in 4 pairs, all others 0; the 3 of these pairs whose IDML has a
document label write the key without the `$ID/` prefix of built-in
names.

**The story list of InDesign 2.0.** In files from InDesign 2.0, chunk
0x222 is a u32 count *n* and then *n* records of 6 bytes: a u32 UID and
a u16 flag. Flag 2 marks the XML backing story (the story chunk 0xBF14
names); the others have flag 1. There is no second list. Evidence: three
privately held InDesign 2.0 documents with their print PDFs, which have
no IDML (`CLEANROOM.md`, points a reviewer should know): the chunk is
4 + 6*n* bytes in 3 of 3 (19, 30 and 37 records), every record names a
story object (86 of 86) and the records name every story object of the
file, flag 2 occurs once per file and names the story of chunk 0xBF14
in 3 of 3. Read as a UID list, the chunk gives 10 of 30 stories in one
of them, and the text of the others is missing from the output; read as
records, 98 to 99 % of the words of each PDF are in the converted
stories. A later file's two lists can have the same length (four
stories and one backing story take 28 bytes, as four records do), so
the converter reads records only in files from 2.0.

**Script byte of the user names.** The second byte of each user name's
in-object string (the byte after the string tag) is a script code: 0
in documents in Latin script, 1 in all 9 trustworthy pairs with
Japanese user or assignment names, 7 in a Russian, 25 in a Simplified
Chinese and 29 in a Czech document. The same codes are font writing
scripts (1 for Japanese fonts, 25 for Simplified Chinese fonts). All
users of a document have the same byte, except in one pair with users
of bytes 0 and 7. The converter uses the byte of the last user to
choose font family names (`fonts.md`, native family names).

Other in-object strings carry the same kind of byte, so a search for a
string must accept any second byte. In the 149 pairs typeset in
Japanese it is 1 for most user-made names, and 3, 5, 7, 0x19 and 0x1D
also occur, in style names, style GUIDs, layer names and ruby font
styles. Layer names and style GUIDs (below) rely on this.

**Document users.** IDML lists one `DocumentUser` per user in chunk
0xA443, `Self` being `dDocumentUser` and the index in hexadecimal (491 of
495 trustworthy pairs have as many users as the chunk; the other 4 are
DOM 7 and 16 files with one user more in the IDML). A user with flag 0
has its stored name as `UserName` (1,724 of 1,724). Flag 2 marks the
placeholder for an unknown user, stored with a name such as
`Unknown User Name` or the same words in the language of the computer
that saved it; IDML writes `$ID/Unknown User Name` for 495 of the 586
such users, and the stored name for the other 91. The rule is the
saving edition's name for an unknown user (saving edition, below):
IDML writes `$ID/Unknown User Name` for every user, of either flag,
whose stored name is that edition's, and the stored name for every
other user, also a flag 2 user named in another language (for example
`Unknown User Name` in a document last saved by a Korean edition).
Without a black name the converter writes `$ID/Unknown User Name` for
every flag 2 user (the schema requires `UserName`). The colour UID names an interface colour, but the IDML
`UserColor` does not follow from it (users stored with 0.6, 0.4, 0 are
`BrickRed` in 4 files and `Gold` in 1), so it is left out of the
conversion and of the measurement (`measurement.md`).

**Save history (chunk 0x1D8).** A u32 count, then one record per
session that saved the document, oldest first:

| Field | Contents |
|---|---|
| u16 | kind (not used) |
| u16 | platform: 0 or 3 (style shortcuts, below) |
| u16, u16 | not identified |
| u16 | language code (below) |
| flag byte, in-object string | application version, such as `16.1.0.20` |
| u16 | build (not used) |
| u32, u32 | FILETIME of the session, high part first (not used) |

Evidence: the layout parses every record of 4,431 of the 4,439
distinct valid corpus files outside the privately held samples (4,272
little-endian, 159 big-endian), from InDesign 3.0 to 21. Of the other
8, six are InDesign 2.x files whose strings have another tag
(in-object strings, above) and two (one 3.x, one 13.x) have no chunk. All 803
pairs parse.

**Language code.** The u16 of the last record is 0x0100 in 4,018 of
those files, 0x0101 in 335, 0x0103 in 53 and 0x0400 in 25. It follows
the language edition of the application of that session:

- All 149 pairs typeset in Japanese that the corpus gained in 2026-10
  have 0x0101; so do 11 earlier pairs, among them the 7 whose default
  story tag name (preferences chunk 0xBF4F) is Japanese or Chinese.
  No document with a tag name in another language (Korean, German,
  French, Russian, …) has 0x0101 (654 pairs before 2026-10).
- The values IDML writes in the exporting InDesign's language follow
  it (below): in 0x0101 documents the flattener resolution, the index
  title and separator and the endnote separator are those of a
  Japanese edition.
- 0x0103 occurs in documents in Latin and Hebrew script; it gives the
  same values as 0x0100 in all 12 pairs that have it.

The converter reads the code of the last record only, and only uses
whether it is 0x0101.

**Application version.** The version string of the last record can be
later than the version in the header: InDesign 13.1 saved files whose
header says 13.0. IDML writes some values only from 13.1, and follows
the application, not the header. Over the trustworthy pairs of version
12.0 to 14.0 (corpus after 2026-10), the document's
`TextFramePreference` has the `Footnotes…` values:

| Header | Last session | IDML `DOMVersion` | Pairs | With the values |
|---|---|---|---:|---:|
| 12.0, 12.1 | same | same | 72 | 0 |
| 13.0 | 13.0 | 13.0 | 42 | 0 |
| 13.0 | 13.0 | 13.1 | 16 | 16 |
| 13.0 | 13.1 | 13.1 | 93 | 93 |
| 13.1 | 13.1 | 13.1 | 148 | 148 |
| 14.0 | 14.0 or 14.3 | 14.0 | 43 | 43 |

The 16 were exported by InDesign 13.1 without saving, which the INDD
cannot show. The converter takes a 13.1 boundary as reached when the
header or the last session is at least 13.1: for the `Footnotes…`
values of the document, of text frames and of object styles, and for
`MergeConsecutiveParaBorders`. This adds 93 to 253 values for each of
these keys and no wrong or extra value. The record count is checked against the chunk
size; a chunk that does not parse gives no code.

**Values that follow the language of the last session.** Over all 803
pairs (160 with code 0x0101):

| Value | 0x0101 | Other codes |
|---|---|---|
| Spread `FlattenerPreference` resolutions, no chunk 0x10833 | 400, 400 | 300, 150 |
| `IndexOptions` `BetweenEntriesSeparator`, no chunk 0x13010 (`preferences.md`) | `、` in 157 of 159 | `; ` in 632 of 632 |
| `IndexOptions` `Title`, no chunk 0x13010 | `索引` in 158 of 159 | `Index` in 624 of 632, and `Indice`, `Rejstřík`, `Índice`, `Указатель` |
| `EndnoteOption` `EndnoteSeparatorText` and `EndnoteMarkerPositioning`, no chunk 0x2261E (`footnotes.md`) | U+3000 and `RubyMarker` in 17 of 18 | tab and `SuperscriptMarker` in 123 of 123 |
| `EndnoteOption` `EndnoteTitle`, no chunk 0x2261E | `文末脚注` (15, DOM 13), `後注` (2, DOM 19 and 20), `Endnotes` (1) | `Endnotes` in 93 of 123, and 4 other titles |
| `Assignment` `Name` | `$ID/UnassignedInCopy` in 157 of 160 | `$ID/UnassignedInCopy` in 612 of 643 |

One trustworthy pair from version 16.1 has code 0x0101 but the values
of another language in every row (flattener 300 and 150 on its 9
spreads, `Index`, `; `, tab, `Endnotes`). Its IDML is dated 55.5 hours
after the last session in the INDD, while the IDMLs of 9 other 0x0101
documents are dated 0 to 40 seconds after it (plus their time zone).
So that IDML was exported in a session the INDD does not record, by an
application of another language.

The converter uses the code where it decides the value: the flattener
resolutions, the index separator and the endnote separator. The index
title, the endnote title and marker position and the unknown user
follow the saving edition (below). It writes no assignment `Name`:
names that are not `$ID/UnassignedInCopy` occur with every code.

**Saving edition (the XMP black name).** The XMP packet of the INDD
lists the document's colours in `xmpTPg:Colorants`, each with
`xmpG:swatchName`, `xmpG:mode`, `xmpG:type` and its values (`cyan` to
`black` in percent, `red`, `green`, `blue` 0 to 255). Tints have an
`xmpG:tint` and keep the stored name of their base colour. The black
swatch is listed with the name the InDesign that saved the document
gives it: `Black`, `검정`, `黒`, `黑色`, `Schwarz`, `Noir`, `Nero`,
`Zwart`, `Černá`, `Preto`, `Черный`. The converter takes as the black
name the `swatchName` of the one colorant that is not a tint, is a
process colour, has the mode and values of the document's black colour
(`Color/Black`, as IDML writes its values) and whose name no other
colour of the document has. Over the 1,251 trustworthy pairs of the
corpus after 2026-10 there is exactly one such colorant in each: 502
`검정`, 431 `Black`, 126 `黒`, 93 `黑色`, 45 `Schwarz`, 44 `Noir`, 4
`Nero`, 3 `Zwart` and one each of `Černá`, `Preto` and `Черный`.

This is a rule over stored data: the packet records the edition of the
last save, and IDML follows the edition that exported it. Where a
document was exported by another installation than the one that saved
it last, the IDML has the other edition's values; the exceptions below
are such documents.

IDML writes these values in the language of that edition:

| Black name | Tint of black | Unknown user | Endnote title (no chunk 0x2261E) | Index title (no chunk 0x13010) |
|---|---|---|---|---|
| `Black` | `[Black] 40%` | `Unknown User Name` | `Endnotes` | `Index` |
| `검정` | `[검정] 40%` | `알 수 없는 사용자 이름` | `미주` | `색인` |
| `黒` | | `不明なユーザー名` | `文末脚注` (DOM 13), `後注` (DOM 19, 20) | `索引` |
| `黑色` | `[黑色] 40%` | `不明的使用者名稱`, `未知用户名` | | `索引` |
| `Schwarz` | `[Schwarz] 40%` | `Unbekannter Benutzername` | `Endnoten` | `Index` |
| `Noir` | `[Noir] 40%` | `Nom d'utilisateur inconnu` | `Notes de fin` | `Index` |
| `Nero` | | `Nome ut. sconosciuto` | `Note di chiusura` | `Indice` |
| `Zwart` | | `Onbekende gebruikersnaam` | `Eindnoten` | `Index` |
| `Černá` | | `Neznámé jméno uživatele` | | `Rejstřík` |
| `Preto` | | `Nome de usuário desconhecido` | | `Índice` |
| `Черный` | | `Неизвестное имя пользователя` | | `Указатель` |

An empty cell has no sample. The endnote marker is `RubyMarker` for
`黒` and `SuperscriptMarker` for every other black name.

With these rules the converter reproduces, over the trustworthy pairs:

| Value | Reproduced | Before (assignment name and code rules) |
|---|---|---|
| `Tint` elements and their names (tints of black) | 204 of 204 tints, no extra tint | 171, and 409 extra values in 27 documents |
| `DocumentUser` `UserName` | 3,208 of 3,212 | 3,000 |
| `EndnoteOption` `EndnoteTitle` | 917 of 918 | 692 (no title without the chunk) |
| `EndnoteOption` `EndnoteMarkerPositioning` | 916 of 918 | 876 |
| `IndexOptions` `Title` | 1,250 of 1,251 | 1,162 |

The remaining differences are the 16.1 document above (exported by
another edition: three users, its endnote title and its index title),
one 20.0 document whose black name is `Black` but whose IDML writes the
German name of the unknown user as `$ID/Unknown User Name` (two users),
and DOM 7 documents whose IDML lists one user more than the INDD. In 8
stale pairs the rule loses 1 to 3 values each: their IDML was exported
by another edition than the one that saved the INDD later (for example
a Dutch endnote title in a document whose last save has `Black`).

Not explained by the black name: `BaselineFrameGridColor`,
`ColumnGuideColor`, the grid and gutter defaults and the print marks of
the Korean documents with code 0x0101 whose IDML has the values of a
Roman edition (`preferences.md`, values of the exporting edition), the
language quotes and the trimmed language lists.

## Spreads (0x501) and master spreads (0x1401)

| Chunk | Contents |
|---|---|
| 0x503 | u32 self, u32 0, UID list of spread layers |
| 0x56E | Matrix: `ItemTransform` |
| 0x1B8 | u32: `BindingLocation` |
| 0x1A8 | u16: 0 = `AllowPageShuffle="true"`, 1 = `false` |
| 0x10833 | Flattener settings (below) |
| 0x14580 | Tab orders: u32 count, then per page u32 page UID and a UID list (the page's `TabOrder`) |
| 0x140D | Master spreads: u16, 0 = `ShowMasterItems="false"` |
| 0x140A | Master spreads: u32 story of the primary text frame (below) |
| 0x2050C | Page transition (12 bytes): u32 type code, u32 40, u32 direction code (below) |
| 0x2056D | Timing lists (animation and timing, below) |

Evidence, over the 5,129 spreads and 914 master spreads of the 495
trustworthy pairs:

- `AllowPageShuffle`: 0x1A8 is 0 in 5,064 spreads, all `true`, and 1 in
  53, all `false`. The 12 spreads without the chunk (in 7 documents) are
  all `false`.
- `ShowMasterItems` of master spreads: 913 have no chunk 0x140D and are
  `true`; the one with the chunk (0) is `false`.
- `PrimaryTextFrame` of master spreads, over all pairs of the corpus of
  2026-10: IDML writes `n` when chunk 0x140A is absent (1,471 master
  spreads) or holds 0 (6). Otherwise it names the text
  frame on that master spread whose `ParentStory` is the chunk's story
  and whose `PreviousTextFrame` is `n` (50 of 50; in 40 of them the
  story has a second frame on the spread, which is not the primary
  one). IDML of DOM 7 has no `PrimaryTextFrame` (9 master spreads). The
  attribute is the last one of `MasterSpread`. The converter writes it
  from DOM 8 and leaves it out when the spread has no such frame: 1,521
  of 1,521 values over all pairs with the same major version.
- `TabOrder` of pages: the list for the page in its spread's chunk 0x14580
  equals the IDML list for 9,367 of 9,367 pages (8 with a tab order);
  pages not in the chunk have an empty `TabOrder`.

**Spreads without a transform.** No spread of three privately held
InDesign 2.0 documents (19 spreads; `objects.md`, the story list of
InDesign 2.0) has chunk 0x56E, so every spread would be written at the
origin. IDML exported by InDesign stacks the spreads vertically: each
spread's `ItemTransform` moves it down from the previous one by half of
both spreads' page heights, plus 180 pt, plus the top and bottom bleed.
Evidence: the two fixtures that store spread transforms, 6 of 6 steps
(a 9.2 file with 1,000 pt pages and no bleed, step 1,180; a 7.5 file
with 666 pt pages and a 9 pt bleed, step 846). When no spread of a
document has chunk 0x56E, the converter places the spreads by this
rule. Not known: the effect of a slug, or of a pasteboard other than
the default (72 pt above and below).

**Page transitions.** Chunk 0x2050C gives `PageTransitionType` and
`PageTransitionDirection`. Over the 11,676 spreads of the distinct
little-endian pairs with the same major version (corpus after 2026-10):

| u32 at 0 | `PageTransitionType` | u32 at 8 | `PageTransitionDirection` | Spreads (documents) |
|---|---|---|---|---|
| no chunk | `None` | | `NotApplicable` | 11,646 (1,453) |
| 0 | `None` | 0 | `NotApplicable` | 12 (2) |
| 0x205C8 | `BlindsTransition` | 0xB | `Horizontal` | 8 (4) |
| 0x205CA | `CombTransition` | 0xB | `Horizontal` | 2 (2) |
| 0x205CC | `DissolveTransition` | 0 | `NotApplicable` | 4 (4) |
| 0x205CF | `PushTransition` | 1 | `Down` | 2 (2) |
| 0x205D0 | `SplitTransition` | 0xD | `HorizontalIn` | 2 (2) |

The u32 at 4 is 40 in every chunk, and every spread has
`PageTransitionDuration="Medium"` (`idml-values.md`), so the field is
not mapped. Master spreads have neither the chunk nor the attributes
(2,395 of 2,395). The converter writes the two attributes on
spreads for these codes, and leaves out an attribute whose code is not
in the table.

**Flattener settings.** Every IDML spread has a `FlattenerPreference`
child. Chunk 0x10833 (52 bytes) is in 20 spreads of 5 trustworthy
documents, always with the same bytes: f64 0.5 at offset 8, f64 400 at
offsets 20 and 28, 2 at 36 and 800 at 44. Their IDML has
`LineArtAndTextResolution` and `GradientAndMeshResolution` 400 and
`RasterVectorBalance` 50. In the corpus of 2026-10, 182 spreads of 20
trustworthy documents store 150 at 20 and 300 at 28, and their IDML has
`LineArtAndTextResolution="300"` and `GradientAndMeshResolution="150"`
(182 of 182): the f64 at 28 is `LineArtAndTextResolution` and the f64 at
20 `GradientAndMeshResolution`. Without the chunk, the IDML has 400 and 400 when the
last session of the save history (document, above) has code 0x0101,
and 300 and 150 otherwise: 7,481 of the 7,490 spreads of all 803 pairs
match, the other 9 being the spreads of the pair exported in a
session the INDD does not record. No stored resolution decides it: the
bytes of 400 as f64 occur in 1 of the 10 such documents of the 654
pairs before 2026-10, and no byte of the spread chunks separates the
two groups. The converter writes the resolutions by this rule, and
none when the history cannot be read. The other four values are the
same in every IDML (`idml-values.md`).

**Spread layers (0x301)** hold the items of one document layer on one
spread. Chunk 0x302: u32 document layer UID, u16 1 for the layer's guide
part. Chunk 0x303: u32 spread, u32 spread, UID list of children. Pages are
children of the spread layer for the internal pages layer.

## Guides (0x3301)

Ruler guides are objects of class 0x3301. They are children of a spread
layer (listed in its chunk 0x303), like page items, and their chunk 0x15B
names the spread and the spread layer. The spread layer's document layer
is IDML `ItemLayer`. IDML writes a guide as a `Guide` element inside a
`Page` element. Chunk 0x3308 (52 bytes):

| Offset | Contents | IDML |
|---|---|---|
| 0 | f64 position in spread coordinates: y for a horizontal guide, x for a vertical one | `Location` (below) |
| 8 | u32 owner: a page of the spread, or the spread itself | the `Page` element the guide is in |
| 12 | u16 orientation: 1 horizontal, 0 vertical | `Orientation` |
| 14 | f64, 0.05 in every sample | `ViewThreshold="5"` in every sample |
| 22 | u32, 6 in every sample | `GuideColor` `Cyan` in every sample |
| 26 | u16 fit to page: 1 true, 0 false | `FitToPage` |
| 28 | f64, not identified | |
| 36 | u32, not identified (24, 25, 30 or 31) | |
| 40 | u32 guide type: 0 ruler, 1 liquid | `GuideType` (`Ruler`, `Liquid`) |
| 44 | f64, 1 in every sample | |

**Evidence.** The 137 distinct little-endian files hold 847 guides; 815
have this 52-byte record. The other 32 are in one InDesign 7.0 file with
no IDML and have a 40-byte record, the first 40 bytes of the layout
above; the InDesign 3.0 and 4.0 files have it too (`big-endian.md`). The
converter reads it and leaves out `GuideType`. In the corpus pairs, 71 IDML guides in 33 distinct files have an
INDD object with the same UID. Two more pairs have the same number of
guides on both sides but different UIDs (the IDML was exported from
another save) and are not used. For the 71:

- `Orientation`: 71 of 71 (50 horizontal, 21 vertical).
- `FitToPage`: 71 of 71 (49 true, 22 false).
- `GuideType`: 71 of 71 (59 ruler, 12 liquid).
- `ItemLayer`: 71 of 71.
- Owner: the 49 guides owned by a page are in that page's element, the
  22 owned by the spread are in the spread's first page (71 of 71). All
  22 spread guides have `FitToPage="false"` and all 49 page guides
  `FitToPage="true"`.
- The view threshold and colour have one value each in the INDD and in
  IDML, so the fields are not proven. The converter writes
  `ViewThreshold="5"` only for 0.05 and `GuideColor` `Cyan` only for 6.

**Location.** IDML measures `Location` from the ruler origin
(`ViewPreference` `RulerOrigin`, chunk 0x808 of the preferences,
`preferences.md`). It is the stored position minus the origin's y
(horizontal guides) or x (vertical guides). A page's corner is its
`GeometricBounds` corner mapped by its `ItemTransform`. The origin is:

- `SpreadOrigin`: the top-left corner of the spread's pages (the
  smallest left and top edge of any page).
- `PageOrigin`: the top-left corner of the guide's own page; for a
  guide of the spread, the corner of the spread's pages as with
  `SpreadOrigin`.
- `SpineOrigin`: for vertical guides on a spread of two pages, the left
  edge of the right page (x = 0 in the samples); otherwise as with
  `SpreadOrigin`.

IDML computes the difference after adding the spread's translation
(`ItemTransform` of the spread) to both values:
`(position + t) − (origin + t)`. This only changes the last bits, but
it decides whether a guide on the origin gets 0 or a remainder such as
−1.1e-13. Over the 1,252 trustworthy pairs it gives the IDML number
exactly for 8,580 of the 9,127 guides in `SpreadOrigin` documents
(7,782 without the translation); the others differ only in the last
digits.

**Evidence.** All 553 guides of the 23 trustworthy pairs with
`PageOrigin` follow the rule: 435 page guides (304 horizontal, 131
vertical, on spreads of one, two and five pages) and 118 spread guides
(116 horizontal, 2 vertical on one-page spreads). The 9,127 guides of
`SpreadOrigin` documents follow it but for 25 (below), including pages
whose bounds do not start at 0. The one `SpineOrigin` file (a stale
pair, InDesign 15.0) has three vertical guides: on a two-page master
spread at stored −552.76 and 42.52 with IDML −552.76 and 42.52 (from
x = 0), on a one-page master spread at stored 0 with IDML 297.64 (from
the page's left edge at −297.64); its horizontal guides are measured
from the top of the pages. No spread of more than two pages with
`SpineOrigin` was seen.

**Guides at the spread origin.** In 5 trustworthy pairs (InDesign
17.0, one collection), 5 guides each that belong to a page with
`FitToPage="true"` have `Location` values that put them at 0 in spread
coordinates: vertical guides stored at −457.14 and −58.47 get the
spine's position, a horizontal guide stored at −9.92 gets y = 0. The
stored positions lie inside their pages. No field of the guide record
or the guide's other chunks separates them from the other guides of
the same documents (`unknowns.md`). The converter writes the stored
position.

**Spread guides outside the pages.** IDML leaves out a guide owned by
the spread when its position lies outside the bounding box of the
spread's pages (each page's corners mapped by its `ItemTransform`; the
bleed does not count): for a horizontal guide when its y is above the
top or below the bottom, for a vertical one the same with x. Over the
489 trustworthy pairs before 2026-10: 84 of 85 such guides are left out
(positions such as −15, −66 or 1215 on pages 1200 wide), all 1,198
spread guides inside are written, and all 5,740 page guides are written
wherever they are. With the bleed added, 30 of the left-out guides
would be inside. The exception is a horizontal guide at −9 on a master
spread, which IDML writes; its object has chunks 0x1424 and 0x1425,
which no left-out guide has. The converter leaves out such guides
(with a tolerance of 1e-6).

**Other settings.** A guide has chunks 0x2C2D (`Locked`) and 0x1424
(`OverriddenPageItemProps`) as page items do (`Page item settings`),
and the f64 at offset 44 of the 52-byte record is `GuideZone`. Over the
7,003 guides of the trustworthy pairs: `Locked` 7,003 of 7,003 (3
locked), `GuideZone` 6,988 of the 6,988 with a 52-byte record (464 with
0), `OverriddenPageItemProps` 6,988 of 6,988; the 15 guides with a
40-byte record have neither attribute in IDML.

`PageIndex` counts from the spine of the spread: 0 for a guide of the
spread itself; for a page guide, the pages left of the spine are −1,
−2, … (counting outwards) and those right of it 1, 2, …, the spine being
after `BindingLocation` pages; on master spreads it is at the left edge.
This gives 6,998 of 7,003 (1,215 spread guides 0; 2,266 of 2,269 page
guides on document spreads, 473 of them −1 and 12 −2;
3,517 of 3,519 on master spreads).

## Layers (0x302)

Chunk 0x304:

| Offset | Contents | IDML |
|---|---|---|
| 0 | u16 | `Locked` (1 = true) |
| 2 | u16 | `Visible` |
| 4 | u16 | `Printable` |
| 6 | u16 | `LockGuides` |
| 8 | u16, 1 in every sample | |
| 10 | u32 UID of an interface colour | `LayerColor` (below) |
| 14 | u16 | `UI` |
| 16 | u16, 1 in every sample | |
| 18 | u8 flag, in-object string | `Name` |
| after the name | u16 | `IgnoreWrap` |

The internal layer is the first UID of the document's layer list
(chunk 0x301 of UID 1) and has no IDML element: in 489 of 489
trustworthy pairs that layer is not written. It is usually named
`Internal_pages_layer_name` (467), but also `Pages` (20) or has an empty
name (2), so the name does not identify it. In all 1,053 layers of the trustworthy pairs the name starts at
offset 19, and each field above equals the IDML value for 1,053 of 1,053
layers (127 locked, 55 hidden, 59 not printable, 4 with locked guides,
1 without `UI`, 2 ignoring text wrap). In the corpus of 2026-10, the
string at offset 19 gives the IDML `Name` of 1,683 of 1,683 layers in
all pairs, whatever its second byte (in the trustworthy pairs, 188
layers have a byte other than 0, all with the IDML name). The converter reads the name at offset 19 when byte
19 is the string tag, and otherwise searches for a string from offset
18. `ShowGuides` and `Expendable` are
true in every layer; the two fields that are 1 in every sample may hold
them. The converter reads the fields only when the name starts at offset
19 (in all little-endian corpus pairs, 1,465 layers).

## Pasted smooth shades (0x5533)

Objects of class 0x5533 hold shadings. Chunk 0x5531: a flag byte (1 =
built-in key) and the name, then a u32. Chunk 0x5532 holds the shading;
a constant shade has 92 bytes, with u32 28 (the length of what follows)
at offset 60, then a u32 and three f64. IDML writes it as a
`PastedSmoothShade` with `ContentsType="ConstantShade"` and `Contents`
the base64 of the same u32 and three f64 in big-endian order
(`AAAAAUBv4AAA…` for 1 and 255, 0, 0).

Every trustworthy pair has one constant shade in IDML, and it is the
class 0x5533 object of lowest UID with a constant shade (495 of 495);
in 84 pairs the INDD has two more that IDML leaves out. The converter
writes that one shade, and matches the IDML on all values in 495 of
495 (name `$ID/` from flag 1 and an empty name; two contents values,
255 and 1).

**Axial shades.** Chunk 0x5532 in general: u32 0 at 0; six f64 at 4 (a
matrix); u32 0 at 52; u32 shading type at 56; u32 *n* at 60; *n* bytes
of shading data from 64, to the end of the chunk. A constant shade has
type 0 and *n* = 28. Type 5 is an axial shade, which IDML writes as
`ContentsType="AxialShade"` with `ContentsMatrix` the six f64 and
`Contents` the base64 of the shading data with every field converted to
big-endian. The data is a sequence of fields:

```
shading:
  f64 ×6                      coordinates
  u16 ×4                      (0 or 1)
  u32 colour space            (2 in every sample)
  u32 function count F
  function ×F
  u32                         (4 in every sample)
  f64 ×6
  u32                         (2 in every sample)
  u16 ×2                      (0 or 1)
function:
  u32 function type           (0 in every sample)
  u32 inputs, u32 outputs     (1 and 1 in every sample)
  u16, u32 c, f64 ×2c         domain
  u16, u32 c, f64 ×2c         range
  u32, u32 c, f64 ×2c         encode
  u32 c, f64 ×2c              decode
  u16
  u32, u32 bits per sample (8 in every sample), u32 byte count B
  u8 ×B                       samples, not swapped
```

Over all pairs of the corpus of 2026-10 the INDD files hold 3,725 axial
shades (3,704 in 37 trustworthy pairs); the grammar consumes every chunk
to its end. IDML has 900 axial shades (891 in 12 trustworthy pairs), and
the converter's `Contents`, `ContentsMatrix` and other attributes equal
IDML's for all 900. The name chunk is that of constant shades (`$ID/`).
The other attributes are those every IDML has on an axial shade
(`idml-values.md`), and `SwatchColorGroupReference` from DOM 12 as for
other swatches. Type 6 also occurs (43 objects in trustworthy pairs);
IDML writes none of them, so its layout is not needed. A shading with
another value where the grammar names the value of every sample, or with
bytes left over, is left out with a warning.

**Which axial shades IDML writes.** The rule of unnamed colours (colours,
below): a shade is written when a value in the package refers to it, or
when the page item defaults name it (chunk 0x6E07 or entry 0 of chunk
0x6E06, by class 0x5533). In the trustworthy pairs IDML writes 891 of
the 3,704 axial shades, all 891 that the rule selects and none of the
other 2,813. Of the 891, 884 are referred to by a page item's
`FillColor` (attribute 0x6E68 of the item's chunk 0x6E03 holds the
shade's UID; 744 rectangles of spreads and 140 of master spreads) and 7
are named by the defaults only. All references are rectangle fills.

**Order.** In `Graphic.xml` the shades follow the inks in UID order (the
constant shade has the lowest UID), before `Swatch/None`: 12 of 12
trustworthy pairs with axial shades.

## Index sort options

Chunk 0x1307E of the preferences object (class 0x2202) holds the index
groups. A "string" is a flag byte and an in-object string; a "u32
string" is a u32 length in UTF-16 units and text segments.

```
u32 group count
group:
  string     group name (kIndexGroup_Symbol, kWRIndexGroup_GreekAlphabet, ...)
  u8         include
  u8         (not identified)
  u16        header variant
  u16        (not identified)
  u32        variant count
  variant:
    string      internal name
    string      UI string
    u32 string  document string
    u8          1 = visible
    u8          (0)
    u32         section count
    section:
      u32 string  sorting string
      u32 string  document header string
      string      UI header string
      u16         language
```

Every chunk of the 654 pairs parses to its end with this layout. The
variants and sections have the same fields as the list that IDML writes
under `IndexHeaderSetting` (`preferences.md`), which the converter takes
from chunk 0x1300E. The converter uses the name, include byte and header
variant of each group, and the internal name of the variant that the
header variant indexes in the group's variant list.

IDML writes an `IndexingSortOption` per group, in the stored order:
`Self` `dIndexingSortOptionn` and the name, `Name` `$ID/` and the name,
`Include`, `Priority` (the position, from 0) and `HeaderType`, which
follows the internal name of the chosen variant:

| Variant name | `HeaderType` |
|---|---|
| `IDX_Basic` | `BasicLatin` |
| `IDX_Spanish` | `Spanish` |
| `IDX_Czech` | `Czech` |
| `IDX_Russian` | `Russian` |
| `IDX_AllHira` | `HiraganaAll` |
| `IDX_ChinesePinyin` | `ChinesePinyin` |
| `IDX_ChineseStroke` | `ChineseStrokeCount` |
| `IDX_KoreanConsonant` | `KoreanConsonant` |
| the group's own name (symbol, numeric, Greek, Arabic and Hebrew groups) | `Nothing` |

The converter leaves `HeaderType` out for other names. Evidence: the
495 trustworthy pairs before 2026-10 have 11 different lists (8 or 10
groups, in different orders, with different groups included). Over the
1,251 trustworthy pairs of the corpus after 2026-10 the converter
reproduces all 12,472 `HeaderType` values. The variant index decides
where the list order differs: the Chinese group has variant 0 in every
file, but in 83 documents its list starts with `IDX_ChineseStroke`, and
their IDML has `ChineseStrokeCount` (83 of 83); a table by group and
variant number, used before, gave `ChinesePinyin` there. Over all 654
pairs before 2026-10 the Alphabet variant is 0 in 652 files, 3 and 5 in
one each, and the Cyrillic variant is 2 in all.

## Numbering lists (0x1A483)

Chunk 0x1A49B of the preferences object: u32 UID of the default list,
u16 count *n*, *n* list UIDs, i32 −1. Each list has chunk 0x1A49A: a
flag byte (1 for a built-in key; 0 and 2 plain) and the name, then u8
`ContinueNumbersAcrossStories` and, two bytes after the name, u8
`ContinueNumbersAcrossDocuments` (1 `true`); the rest is not identified.

IDML writes one `NumberingList` per list in `designmap.xml`, the default
(`$ID/[Default]`) first, then the *n* lists in stored order, with
`Self` `NumberingList/` and the name. The paragraph attribute 0x1A417
(`AppliedNumberingList`, a `Properties` child of type `object`) is the
UID of a list. Evidence: 705 of 705 lists of the trustworthy pairs
(953 of 953 in all pairs), names, both flags and order;
`AppliedNumberingList` 519 of 519 paragraph styles and 489 of 489 text
defaults.

## Assignments (0x1BE01)

Every document has one object of class 0x1BE01, listed in chunk 0x1BE13
of the document; IDML writes it as an `Assignment` with `Self` its UID
(495 of 495 trustworthy pairs). Chunk 0x1BE1B starts with a flag byte
and the name in the language of the edition the document was made with
(`Unassigned InCopy Content`, `Contenu InCopy non affecté`, …; the
converter uses it for values of that edition, `preferences.md`), then two empty
strings and fields that are the same in every pair. IDML names it
`$ID/UnassignedInCopy` in 476 pairs and gives the stored name in 19, in
files of several languages and versions, so the name follows the
computer that exported the IDML; the converter leaves `Name` out (it is
optional in the schema) and the measurement too. The language code of
the save history does not decide it either (document, above). The other
attributes are the same in every IDML (`idml-values.md`).

## Articles (0x21B01)

**Article list.** Chunk 0x21B03 of the document object: u32 count, then
that many article UIDs. IDML writes one `Article` element in
`designmap.xml` per listed UID, in list order (not UID order).
Evidence: the 19 pairs whose IDML has `Article` elements or
`ArticleChildrenList` (DOM 8.0, 10.1, 13.x, 20.5): the list order equals
the IDML order for all 31 articles; the chunk is in all 19 (count 0 in
the 6 with no articles), and its length is exactly 4 + 4 × count. In
all 66 corpus INDD files that have article objects (INDD 7.5 to 21.2,
203 articles), the list holds exactly the document's 0x21B01 objects.

**Article object.**

| Chunk | Layout | IDML |
|---|---|---|
| 0x13C | flag byte, in-object string | `Name` |
| 0x21B02 | u32 count, then that many page item UIDs | one `ArticleMember` per UID, in order, `ItemRef="u<hex>"` |
| 0x1A8 | u16 | not mapped (below) |

- `Self` is `u<hex UID>`: 31 of 31 in pairs.
- Name: the flag byte is 2 in all 203 articles of the corpus, and the
  string ends at the chunk's end in all 203. 31 of 31 names match in
  pairs (Latin, Hangul and Han text; segments with 0x40 and 0x80
  encodings).
- Members: 31 of 31 lists match the IDML `ArticleMember` `ItemRef`s in
  order. Without chunk 0x21B02 (2 articles) or with count 0 (2 in
  pairs), the IDML `Article` has no children. In the corpus, members
  are page items (class 0x6201) or groups (0x401); no other class
  appears (201 lists).
- `ArticleMember` `Self` is the article's `Self`, `ArticleMember`, and
  the member's index from 0: `u281ArticleMember0`, `u281ArticleMember1`.
  44 of 44 members in pairs. No corpus IDML has an article with more
  than 10 members, so whether the index is decimal or hexadecimal is not
  shown; the converter writes it in decimal.
- `ArticleExportStatus` is `true` on every `Article` of every corpus
  IDML (31 in pairs; no other value in 3,557 IDML files). No stored
  field is identified; the converter writes `true` (`idml-values.md`,
  defaults).
- Chunk 0x1A8 (u16) is in 50 of 203 articles: 1 in 47, 0 in 3. The one
  in a pair has 1 and `ArticleExportStatus="true"`, as do the articles
  without the chunk; it is not mapped.
- No corpus IDML has an `Article` `Properties/Label`, so an article's
  label chunk is not shown.

**Page items.** Chunk 0x21B04 of a page item or group: u32 count and the
UIDs of the articles the item is a member of. 44 of 44 members in pairs
have count 1 and their article's UID; 331 other items have count 0. IDML
has no attribute for it; the converter does not read it.

**`ArticleChildrenList` on groups.** Chunk 0x21B05 of a group (class
0x401): u32 count, then UIDs. IDML `Group` has
`ArticleChildrenList="u<hex> u<hex> …"` with these UIDs in order,
exactly when the group has the chunk: 68 of 68 groups in 12 pairs; no
group has the attribute without the chunk or the chunk without the
attribute. The list is the group's children in reverse order of chunk
0x15B in 68 of 68 groups in pairs and 259 of 259 in the files with
articles. Groups with the chunk also occur in documents without
articles (4 pairs of 13.x). IDML writes the attribute directly after
`Self` (68 of 68).

**Element order.** In `designmap.xml` the `Article` elements come last,
after `Assignment` (all 13 pairs with articles). The schema puts
`Article` in the final interleaved group with `Assignment` and others,
so writing them after the assignments is valid. `Article` allows
`Self`, `Name`, `ArticleExportStatus`, `Properties/Label`, then
`ArticleMember*` (`Self` and `ItemRef`, both required).

**Versions.** Article objects occur in INDD 7.5 to 21.2. Pairs show DOM
8.0, 10.x and 20.x; no 7.x INDD with articles has an IDML. The
converter writes articles for every version.

**Damaged input.** The converter writes each listed article once and
skips listed UIDs that are not articles. An article without a readable
name or member list is left out with a warning. A member that is not
written as a page item element is left out with a warning, and so is an
`ArticleChildrenList` UID that is not a child of the group.

## Named grids (0xCD12)

Chunk 0xCD28: u32, a flag byte (1 = built-in key) and the name
(`[Page Grid]`, IDML `$ID/[Page Grid]`). IDML writes a `NamedGrid` per
object, in UID order, with a `GridDataInformation`:

- A grid made by the user has its own chunk 0xCD02, with the layout of
  the page chunk (layout grid, below), and IDML writes those settings.
- The built-in `[Page Grid]` has only chunk 0xCD28. Its
  `GridDataInformation` equals the document's layout grid preference
  (`LayoutGridDataInformation`, chunk 0xCD2F, `preferences.md`) in every
  attribute that preference has: 803 of 803 pairs of the corpus of
  2026-10 (606 trustworthy). It does not always equal the pages' grid:
  4 of the 574 trustworthy pairs whose pages all have the same grid
  differ.

The converter writes each named grid with its own settings, or with
the layout grid preference. Over all pairs: 876 of 876 named grids, and
every attribute of their `GridDataInformation` (`AppliedFont` 875: one
family that IDML names in another form, `fonts.md`); 38 pairs have
grids made by the user (2 or 3 named grids).

**Applied named grid (chunk 0xCD32).** A story (class 0x201) or object
style (class 0x1B901) that uses a named grid has chunk 0xCD32: u16 1,
then the UID of the named grid object (6 bytes). An object style can
also have u16 0 (2 bytes). IDML writes `AppliedNamedGrid` as the grid's
`Self` (`NamedGrid/<name>`, `NamedGrid/$ID/[Page Grid]` for the
built-in grid), and `n` for u16 0 or without the chunk. Evidence over
all pairs of the corpus of 2026-10, compared by `compare.py`:

- Stories: 26,805 of 26,805 (19,424 of 19,424 in the trustworthy
  pairs). 106 IDML stories name a grid made by the user (two grids, in
  13 pairs of version 12.1), and their INDD stories have the chunk with
  that grid's UID.
- Object styles: 3,515 of 3,515 (2,661 of 2,661 trustworthy). 55 name
  `[Page Grid]` (u16 1); the others have u16 0 or no chunk and `n`,
  among them all root styles `[None]` but one, which names
  `[Page Grid]`.

Over the whole little-endian corpus, every story chunk 0xCD32 has 6
bytes, u16 1 and the UID of a named grid object. The converter writes
`n` for u16 0 or no chunk, and the grid's `Self` for u16 1 with the UID
of a named grid of the document; otherwise it leaves the attribute out.

## Table of contents styles (0x11605)

Chunk 0x11605: a flag byte (1 = built-in key) and the name, a u16 (0 in
every style), a u16 `IncludeBookDocuments` and a u32 `CreateBookmarks`
(1 = true), the u32 UID of the title's paragraph style, a flag byte and the
title, a flag byte and a string not identified, then u16 fields: at 0
`SetStoryDirection` (0 `Horizontal`, 1 `Vertical`), at 2
`NumberedParagraphs` (0 `IncludeFullParagraph`, 2 `ExcludeNumbers`), at
4 `MakeAnchor` and at 6 `RemoveForcedLineBreak` (1 = true; the chunk of
older files ends before the fields IDML does not have yet), then the
entries. IDML writes one `TOCStyle` per object in `Styles.xml`.

A story made by a table of contents has chunk 0x8C40, the UID of an
object of class 0x8C20 whose chunk 0x11613 is the TOC style; IDML writes
that style as the story's `AppliedTOCStyle`, and `n` for other stories.

Evidence over the trustworthy pairs: the 511 TOC styles the converter
writes match on `Name`, `Title` and `TitleStyle` (511 of 511),
`NumberedParagraphs` (511, one `ExcludeNumbers`), `SetStoryDirection`
(620 of 620 in the corpus of 2026-10; over all its pairs 823 of 824, 2
`Vertical` in two pairs of version 18), `MakeAnchor` (445 of
445 from DOM 9 on, 4 true) and `RemoveForcedLineBreak` (340 of 340 from
DOM 13 on, 4 true); `AppliedTOCStyle` matches for 18,923 of 18,923
stories (21 made by a table of contents). `IncludeBookDocuments` and
`CreateBookmarks` match all 1,520 TOC styles of the corpus of 2026-10
whose IDML style has the same name (1,304 trustworthy; 15 with
`IncludeBookDocuments="true"`, one with `CreateBookmarks="false"`, in a
stale pair). The flagged string after the title is the path of a book
file in some of these styles, but it does not decide
`IncludeBookDocuments`. 

**Entries.** After the u16 fields (two before version 9, three before
13, four from 13) the chunk holds a u16 entry count and the entries:

| Field | IDML |
|---|---|
| flag byte, in-object string | `Name`: the paragraph style's name, with `$ID/` first for flag 1; the group separator U+E00B is written `:` |
| flag byte, in-object string | a document path; not in IDML |
| u32 | `FormatStyle`: paragraph style UID |
| u16 | `Level` |
| u16 | 0 in every entry of the pairs; 1 in some entries of files without a pair (not mapped) |
| u16 | `PageNumberPosition`: 0 `AfterEntry`, 1 `BeforeEntry`, 2 `None` |
| u32 | `PageNumberStyle`: character style UID |
| u32 *n*, segments | `Separator` |
| u32 | `SeparatorStyle`: character style UID |
| u16 | 0 in every entry |
| u32 *n*, segments | `; `, `;` or `、`; not in IDML |
| u16 | 1 in every entry |

IDML writes each entry as a `TOCStyleEntry` child of its `TOCStyle`:
`Self` is `u`, the TOC style's UID in hexadecimal, `TOCStyleEntry` and
the entry's index (0 to 5 in the samples, so whether the index is
hexadecimal is not shown), then
`Name`, `Level`, `PageNumberPosition`, `Separator`,
`SortAlphabet="false"` (every entry), and `Properties` with
`FormatStyle`, `PageNumberStyle` and `SeparatorStyle`: a user style as
`type="object"` and its `Self`; the root style as `type="string"`
`$ID/[No paragraph style]` or `$ID/[No character style]`; UID 0 as
`type="string"`, `$ID/kSameStyleWithBracket` for `FormatStyle` and
`$ID/[Same Style]` for the other two. `Separator` is `$ID/` and the text,
with tab written `^t`, U+0008 `^y`, line feed `^n`, U+2003 `^m` and
U+2014 `^_`.

Evidence: every TOC style with entries in the trustworthy pairs
(versions 8.0 to 21.3) parses to the end of the chunk with this layout,
and the converter reproduces all 255 `TOCStyleEntry` elements with all
their values: `Name`, `Level`, `PageNumberPosition`, `Separator`,
`SortAlphabet` and the three style properties, 255 of 255 each.
`SortAlphabet` is `false` in all; neither constant u16 can be shown to
hold it. A chunk whose entries do not parse to its end keeps its style
and loses its entries, with a warning.

Two TOC styles of the corpus pairs have another value at the
`SetStoryDirection` field: 0x40 with no `SetStoryDirection` in the IDML
(version 15.0) and 0xF6FC with `Horizontal` (a stale pair of version
12.0). The converter writes the attribute for 0 and 1 only.

## Languages (0x2D07)

Chunk 0x2D0F: a flag byte and the name (`English: USA`), a flag byte and
the primary name (`English`), a flag byte and the secondary name (`USA`,
often empty), u16 `Id`, then two vendors, each a flag byte, a u32 and a
string: first the spelling vendor, then the hyphenation vendor
(`Hunspell`, `Proximity`, `Duden`, `WinSoft`); last the locale
(`en_US`).

IDML lists the language objects in UID order as `Language` elements at
the start of `designmap.xml`: `Name`, `PrimaryLanguageName` and
`SublanguageName` are `$ID/` and the stored names, except that the
language named `Neutral` is `$ID/[No Language]` in all three. Evidence
over the trustworthy pairs: in 488 of 495 the IDML list equals the INDD
objects in that order; in the other 7 the INDD has 4 to 61 language
objects and the IDML lists only some of them (1 or 2), for no reason
found. For the 1,249 IDML languages the names and `Id` are equal in
1,249. `HyphenationVendor` is the second vendor and `SpellingVendor` the
first when their flag is 1 (1,146 and 1,166 of 1,249); otherwise the
stored string is empty or names a vendor the IDML does not show, and
IDML gives `$ID/`, `$ID/InDihyph` or a vendor such as `Hunspell` that
depends on the computer that exported it.

**Vendors without a stored name.** Over the 3,139 language objects of
the corpus of 2026-10 matched with an IDML `Language` by name, an empty
vendor (flag 0, u32 0x01000000, empty string) is written `$ID/` for every
language whose vendor of that kind is never stored with flag 1 anywhere
in the corpus: hyphenation 914 of 914 (847 trustworthy), spelling 894
of 895 (the other, in a stale pair, has `Hunspell`). For the languages
that have a stored vendor in some document, an empty vendor is written
as the exporting computer's dictionary (`Hunspell` 14 and 19, `Duden` 1
and 1, `$ID/` 1 and 1), which no stored value shows; the converter
leaves those out. The two lists of languages without a stored vendor:

- Hyphenation (33): `Albanian`, `Arabic`, `Byelorussian`, `Chinese: Hong
  Kong`, `Chinese: Simplified`, `Chinese: Taiwan`, `Chinese:
  Traditional`, `Icelandic`, `Japanese`, `Korean`, `Neutral`,
  `Vietnamese`, `cy_GB`, `en_US+Medical`, `eu_ES`, `fo_FO`, `fy_NL`,
  `ga_IE`, `gl_ES`, `gv_GB`, `ko_KR`, `la_VA`, `lb_LU`, `mk_MK`, `mn_MN`,
  `ms_MY`, `ne_NE`, `oc_FR`, `sr_RS`, `sr_RS-Cyr`, `sr_RS-Lat`, `tk_TM`,
  `uz_UZ`.
- Spelling (29): the same without `Arabic`, `Vietnamese`,
  `en_US+Medical` and `ko_KR`.

Two other hyphenation vendor flags occur: flag 3 with the text
`DefaultLinguistic` is written `$ID/` (24 of 24, Korean and Chinese:
Traditional), flag 10 with `InDihyph` is written `$ID/InDihyph` (2 of
2). Two spelling vendors with flag 1 and the text `InDitect` are
`Hunspell` in IDML; the converter writes the stored text for flag 1.

**Quotes (chunk 0x2D26).** 34 language objects of the corpus have a
second chunk 0x2D26: a flag byte, the language name as an in-object
string, then four UTF-16 units, the two single quotes and the two double
quotes. IDML writes them as `SingleQuotes` and `DoubleQuotes`: 34 of 34
(Japanese `‘’` `“”`, Romanian `‘’` `»«`, Hebrew `’’` `””`, Korean). Without
the chunk, the quotes come from observation (`idml-values.md`, language
quotes).

## Interface colours (0x1F11)

Layers, pages and XML tags name their colour by the UID of an object of
class 0x1F11. Chunk 0x1F01 of that object: u32 5 (RGB), u16 3, three f64
fractions. IDML writes the colour as one of its named colours, or, for
other colours, as a list of three numbers (0 to 255):

| Red, green, blue | IDML | Red, green, blue | IDML |
|---|---|---|---|
| 0.31, 0.6, 1 | `LightBlue` | 1, 0.6, 0 | `Gold` |
| 1, 0, 0 | `Red` | 1, 0.4, 0 | `Orange` |
| 0.31, 1, 0.31 | `Green` | 0, 0.33, 0 | `DarkGreen` |
| 0, 0, 1 | `Blue` | 0.6, 0.6, 1 | `Lavender` |
| 1, 1, 0.31 | `Yellow` | 0.67, 0.64, 0.71 | `Charcoal` |
| 1, 0.31, 1 | `Magenta` | 0.6, 0.2, 1 | `Violet` |
| 0, 1, 1 | `Cyan` | 1, 1, 1 | `White` |
| 0.5, 0.5, 0.5 | `Gray` | 1, 0.6, 0.8 | `Pink` |
| 0, 0, 0 | `Black` | 0.61, 0.87, 0.61 | `GridGreen` |
| 0.6, 0, 0 | `BrickRed` | 0, 0, 0.53 | `DarkBlue` |
| 0.6, 0.8, 0 | `GrassGreen` | 0.81, 0.51, 0.71 | `Lipstick` |
| 1, 0.71, 0.42 | `GridOrange` | 0.97, 0.35, 0.42 | `Fiesta` |
| 0, 0.6, 0.6 | `Teal` | 0.73, 0.73, 0.73 | `LightGray` |
| 0.48, 0.73, 0.85 | `GridBlue` | 1, 0.6, 0.6 | `Peach` |
| 0.6, 0, 0.2 | `Burgundy` | | |

Evidence: in the layers and pages of the trustworthy pairs every colour
object with these components has this name (1,107 layers and pages; from
491 `LightBlue` down to 1 `Teal`). The last four are from the guide,
grid and pasteboard colours of the preferences (`preferences.md`), where
every colour with these components has this name. Two other colours occur, 1, 0.4863,
0.651 (3 layers) and 0.9412 three times (1 layer); IDML writes them as
the lists 255, 124, 166 and 240, 240, 240, the components times 255.
The converter writes such a list for a colour whose components are whole
multiples of 1/255 and that is not in the table, and leaves the colour
out otherwise. XML tags use the table but no list, as before.

## Pages (0x50F)

| Chunk | Contents |
|---|---|
| 0x140F | u32 applied master spread, u16 (unknown), matrix `MasterPageTransform` |
| 0x5CC | Matrix: `ItemTransform` |
| 0x5DD | Four f64: left, top, right, bottom. IDML `GeometricBounds` is top, left, bottom, right. |

Files from InDesign 3.0 and 4.0 store the page transform and bounds in
chunks 0x151 and 0x154, and chunk 0x140F has no matrix
(`big-endian.md`).

**Page settings.**

| Chunk | Contents | IDML | Without the chunk |
|---|---|---|---|
| 0x1404 | u32 count; if not 0, a UID list of master page items and a UID list of their overrides | `OverrideList`: each item and its override (`n` for 0) | empty |
| 0xCD04 | 6 bytes: u32 at 0, then u16 | `GridStartingPoint` (u32: 0 `TopOutside`, 4 `CenterVertical`; below), `UseMasterGrid` (u16, 1 = true) | |
| 0x563 | u32, u32 code: 1 `Recenter`, 2 `ObjectBased`, 3 `Scale`, 4 `GuideBased`, 5 `UseMaster` | `LayoutRule` | `Off` |
| 0x5FF | u32: 0 `Nothing`, 1 `UseMasterColor`, else an interface colour | `PageColor` | `UseMasterColor` |

Evidence over the 9,366 pages (document and master pages) of the
trustworthy pairs that the converter writes: `OverrideList`,
`UseMasterGrid` and `PageColor` 9,366 of 9,366 each, `LayoutRule` 9,347
of the 9,347 pages that have it. Of the 9,367 IDML pages, 1,352 have
overrides and 2,445 `UseMasterGrid="false"`; 5,111 have no chunk 0x563
and `Off`, 4,117 `UseMaster`, 92 `ObjectBased`, 24 `GuideBased`, 3
`Recenter` and 1 `Scale`; 9,260 have no chunk 0x5FF, 44 code 1, 5 code 0
and 58 an interface colour. `LayoutRule` is in IDML from DOM 8 on (no
page of the DOM 7 files has it).

**Layout grid starting point.** The u32 at 0 of chunk 0xCD04 is
`GridStartingPoint`. Over all pairs of the corpus of 2026-10, 13,458 of
13,458 pages that the converter writes match (10,080 of 10,080 in the
trustworthy pairs): 13,430 have 0 and `TopOutside`, 28 have 4 and
`CenterVertical` (master pages in 14 pairs of version 18.4). Every page
in the corpus has the chunk. Codes 1, 5 and 6 occur only in files
without an IDML (versions 3, 6, 19 and 21), so their values are not
shown; the converter leaves the attribute out for them.
| 0x51A | Margins: four f64 (left, top, right, bottom), u16 own-margins flag |
| 0x528 | Columns: u32 count *n*, *n* f64 column edges, f64 gutter, u16 own-columns flag, u16, u16 column direction (1 vertical, 0 horizontal) |
| 0xCD02 | Layout grid (`GridDataInformation`, below) |

**Master spread colour.** IDML gives each `MasterSpread` a `PageColor`
property equal to the `PageColor` of its pages, which is the same for all
pages of a master in all 1,359 master spreads of the 654 pairs
(`UseMasterColor` in 1,326, a named colour or `Nothing` in the others).
The converter writes the pages' colour. Over all pairs this reproduces
1,352 of 1,352 compared values.

**Margins and columns.** IDML writes them as `MarginPreference`. The
column edges are `ColumnsPositions` (left and right edge of each column,
so `ColumnCount` is *n*/2), the gutter is `ColumnGutter`. A flag of 1
means the page has its own values. With 0, or without the chunk, the
page shows the values in effect on its master page: the page at the same
position, counted from the left by `ItemTransform`, in the applied
master spread, or that spread's only page. That master page can itself
take them from its own master. The stored values of such a page are
often stale.

Evidence: all 325 IDML pages of the same-version pairs that have an INDD
page with the same UID (document and master pages):

- Margins: 156 pages with flag 1 match their own values; 169 take them
  from a master page (146 directly, 23 through two or three masters) and
  match. 325 of 325. Without the rule, 72 pages with flag 0 differ.
- Columns, gutter and positions: 156 with flag 1, 169 from a master page,
  325 of 325. Without the rule, 83 differ.
- The order of the margins: the four values differ in many pages (for
  example 56.69, 42.52, 56.69, 56.69 with IDML `Top="42.51968503937008"`).

In the 250 distinct little-endian files, 2,246 pages hold 0x51A in 2,160
(34 bytes each, flag 0 or 1) and 0x528 in 2,211 (always 18 bytes after
the positions). The last u16 of 0x528 is the column direction: IDML
`ColumnDirection` is `Vertical` where it is 1 and `Horizontal` where it
is 0, taking it from the master page with the columns (above). Evidence:
9,274 of 9,274 pages of the trustworthy pairs (14 vertical, in 3
documents), 12,224 of 12,224 over all pairs. A page whose flag is 0
takes the direction from its master page: 44 such pages store 1 but have
horizontal columns in IDML, like their master pages.

**Layout grid (chunk 0xCD02).** u32 font family UID, a flag byte (1 =
built-in key, `$ID/`), the font style as an in-object string, five f64,
four u32. IDML writes `GridDataInformation`:

| Field | IDML | Evidence |
|---|---|---|
| Font family | `AppliedFont` (Properties, the family name) | 283 of 325 (below) |
| Font style | `FontStyle` | 325 of 325 (`Regular`, `Roman`) |
| f64 1 to 5 | `PointSize`, `CharacterAki`, `LineAki`, `HorizontalScale` (×100), `VerticalScale` (×100) | see below |
| u32 1 to 4 | `LineAlignment`, `GridAlignment`, `CharacterAlignment`; one not identified | see below |

In the first corpus the numbers were 12, 0, 9, 1, 1 and the codes 3, 0,
3, 1 in all 2,246 pages of the 250 distinct little-endian files, and all
832 `GridDataInformation` elements of the corpus IDML files had
`PointSize="12"`, `CharacterAki="0"`, `LineAki="9"`, scales of 100,
`LineAlignment="LeftOrTopLineJustify"` and `AlignEmCenter` for both
alignments. The numbers 12 and 9 occur once, so those fields were
located. The corpus of 2026-10 confirms them: pages with other sizes
(such as 9.2126 and 6.9094) are in many pairs typeset in Japanese, and
`PointSize` and `LineAki` match in 13,458 of 13,458 pages of all pairs.
Named grids made by the user (above) locate `CharacterAki`: 37 of them
over all pairs have a second f64 other than 0 (such as −0.354), and
their IDML has the same value in 37 of 37. With the
flag, `FontStyle` matches in all 13,458 pages (44 trustworthy pages have
flag 1 and IDML `$ID/` before the style). The scales and the codes
still have one value only: the converter writes the scales only when
they are 1, and the three alignments only when the codes are 3, 0, 3,
1. The other 42 pages are in one pair whose IDML
names the family `Minion Pro (OTF)` where the INDD family name is
`Minion Pro`; its `FontFamily` element has the same difference
(`fonts.md`).

**Applied master of a master page.** Pages of master spreads use the same
chunk 0x140F. When it is absent or names UID 0, IDML writes
`AppliedMaster="n"` and `MasterPageTransform="1 0 0 1 0 0"` (220 of 220
master pages in the corpus pairs; 2 of them have the chunk with UID 0 and
an identity matrix). In 147 master pages of 73 distinct INDD files without
an IDML, chunk 0x140F names another master spread of the same document,
never the page's own spread. No pair shows the IDML for this case; the
converter writes it as `AppliedMaster`, as for document pages.

## Page items

Frames, shapes and lines are all class 0x6201; groups are 0x401.

| Chunk | Contents |
|---|---|
| 0x151 | Matrix: `ItemTransform` |
| 0x40D | Groups without chunk 0x151: the matrix (`ItemTransform`) |
| 0x162B | Path geometry (below) |
| 0x15B | Hierarchy: u32 spread, u32 parent, UID list of children |
| 0x6E03 | Attribute list: local formatting (see `attributes.md`) |
| 0x1B916 | u32: applied object style |

**Content type.** A rectangle, oval or polygon with a placed graphic is
`ContentType="GraphicType"`. Without one, chunk 0x1623 (u16) decides: 1
is `GraphicType` (an empty graphic frame), 0 or no chunk `Unassigned`.
In 200 of the trustworthy pairs, the 757 empty shapes with
`GraphicType` all have 1, and the 15,768 with `Unassigned` have 0
(13,226) or no chunk.

Groups store their matrix in chunk 0x40D when they have no chunk 0x151:
with it, the `ItemTransform` of all 5,670 groups of the trustworthy
pairs (8,788 of 8,789 over all pairs) matches; without it, 5,277 of them were
written as the identity, though they have a translation in IDML.

**Path geometry:** u32 path count; per path: u32 point count, points, u16
1 if the path is open. Each point is a u32 type, then f64 values: type 2 =
anchor (x, y) only; types 0 and 1 = left direction, anchor, right
direction. All 3,191 path chunks in the samples parse exactly, and the
points of every page item path match the IDML `PathPointType` values.

**IDML element type.** A frame is a `TextFrame` if a child of class
0x263 exists. Otherwise chunk 0x6204 (u16, then u32) gives a shape code:
1 `GraphicLine`, 2 or 3 `Rectangle`, 4 or 5 `Oval`, 0 and 6 to 8
`Polygon`; code 9 means none. With code 9 or without the chunk the
converter uses the path: two points and open → `GraphicLine`; four
corner points on two distinct x and y values → `Rectangle`; four points
with direction points, all of point type 0 → `Oval`; anything else →
`Polygon`. Path points of type 1 have direction points too, and a closed
four-point path with one or more of them is a `Polygon` in IDML (1,672
shapes in the trustworthy pairs).

Evidence over the 62,908 rectangles, ovals, polygons and lines of the
trustworthy pairs: the code gives the IDML element for 22,579 of 22,587
shapes that have one (7 polygons have code 2 or 3, 1 rectangle code 4);
the path for 40,070 of 40,321 without one. Most of the others (215) are
polygons whose four points of type 0 form an ellipse, as ovals do.

**Text frames:** the frame's child of class 0x263 (multi-column frame) has
a child of class 0x227 (frame column). Chunk 0x220 of the column holds the
frame list (class 0x228). Its chunk 0x205: u32 story, then a UID list of
the columns of all threaded frames in order, which gives
`PreviousTextFrame` and `NextTextFrame`.

**Page item labels.** A page item's `Properties/Label` has the layout and
rule of the document label: u16 0x7B7B, u32 count, then flagged key and
value strings, written in stored order as stored. It is stored in:

| Item | Chunk | Offset |
|---|---|---|
| Frames and shapes (class 0x6201) | 0x6204, after the shape code | 6 |
| Groups (0x401) | 0x2C0B | 0 |
| Buttons (0x1450C) and multi-state objects (0x145D8) | 0x14582 | 0 |

Without these bytes the item has no `Label`. In `Properties` the `Label`
comes first. Evidence: all 278 labelled items of these kinds in the
IDMLs of all pairs (258 in 27 trustworthy pairs: 108 rectangles, 44 text
frames, 40 lines, 36 groups, 18 buttons, 9 multi-state objects, 3
polygons) have the bytes and match key for key; no item has the bytes
without a `Label` in IDML. Flag 1 appears on labelled frames whose IDML
keys and values have no `$ID/` prefix.

## Page item settings

Every page item element in IDML (`TextFrame`, `Rectangle`, `Oval`,
`Polygon`, `GraphicLine`, `Group`) has the attributes below. Each is
stored in a chunk of the item (class 0x6201 or 0x401), or in its
attribute list (chunk 0x6E03, `attributes.md`); without the chunk the
IDML value is the default in the table.

| Chunk | Contents | IDML | Without the chunk |
|---|---|---|---|
| 0x2C10 (groups: 0x418) | u8 1 if a built-in key, then an in-object string | `Name`: the string, or `$ID/` and the key | `$ID/` |
| 0x2C32 | u16, 0 = hidden | `Visible` | `true` |
| 0x2C2D | u32, 1 = locked | `Locked` | `false` |
| 0x142D | u32, 1 = overrides not allowed | `AllowOverrides` on items of master spreads (the spread in chunk 0x15B is a master spread): `false` for 1, else `true`; no item of a document spread and no placed graphic has it (11,791 of 11,791 trustworthy items) | `true` on master spreads |
| 0x21D4E | u32 count *n*, *n* pairs of u32 | `ParentInterfaceChangeCount`: the numbers | empty |
| 0x21D50 | same | `TargetInterfaceChangeCount` | empty |
| 0x21D53 | same | `LastUpdatedInterfaceChangeCount` | empty |
| 0x1424 | u32 master page item, UID list | `OverriddenPageItemProps`: the list in decimal; empty if the master item is 0 | empty |
| 0x22228 | u8 flags | `HorizontalLayoutConstraints` (bits 4–6), `VerticalLayoutConstraints` (bits 0–2): per bit `FixedDimension` if set, else `FlexibleDimension` | as 0x22 |
| 0x324 | u32 at 0, a layer or spread layer UID | `BeforeGroupingLayerReference`, from 21.4 (below) | none |
| 0x324 | i32 at 4 | `BeforeGroupingLayerPosition`, after `ItemTransform`, from 21.4 (also on placed graphics) | `-1` |

| Attribute | IDML | Without the attribute |
|---|---|---|
| 0x5520, 0x551F, 0x551E | `GradientFillStart`, `GradientFillLength`, `GradientFillAngle` | `0 0`, `0`, `0` |
| 0x5526, 0x5525, 0x5524 | `GradientStrokeStart`, `GradientStrokeLength`, `GradientStrokeAngle` | same |
| 0x5522, 0x5523 | `GradientFillHiliteLength`, `GradientFillHiliteAngle` | `0` |
| 0x5528, 0x5529 | `GradientStrokeHiliteLength`, `GradientStrokeHiliteAngle` | `0` |

Rules the IDML follows, over the 92,409 page item elements of the 495
trustworthy pairs:

- `Locked` is on the 36,817 items that are children of a spread or
  anchored in text, and on none of the 55,592 items inside a group or
  another page item.
- `ItemLayer` is on the 35,276 items that are children of a `Spread`
  or `MasterSpread` (35,276 of 35,276), and on none of the items inside
  a group (48,289 of 48,291), inside a rectangle, oval or polygon (2,925
  of 2,925) or anchored in text (1,999 of 1,999). The layer of a nested
  item always equals its parent's. Counted over the 489 trustworthy pairs
  of the corpus without the privately held samples.
- The change counts, `OverriddenPageItemProps` and the layout
  constraints are in IDML from DOM 8 on: every page item of the
  distinct corpus IDML files from DOM 8 on has them, none of the DOM 7
  files does (145 text frames, 335 rectangles, 61 groups).
- A group has no gradient attributes of its own (no group in the pairs
  has 0x551E to 0x5529). IDML gives it, for each gradient attribute, the
  value all its child items have, and leaves the attribute out when they
  differ: 58,068 of the 58,080 gradient values of the 5,808 groups.

Evidence: the INDD values, decoded as above, against the IDML of the
same item, over the page items of the trustworthy pairs that have an
INDD object:

| IDML attribute | Equal |
|---|---|
| `Name` | 92,264 of 92,402 |
| `Visible` | 92,402 of 92,402 |
| `Locked` | 36,810 of 36,810 |
| The three change counts and `OverriddenPageItemProps` | 91,812 of 91,812 each |
| Both layout constraints | 91,812 of 91,812 each |
| Gradient start and length, fill and stroke (four attributes), items other than groups | 86,594 of 86,594 each |
| Gradient angles and highlights (six attributes) | all, 92,396 to 92,402 per attribute (groups by the rule above) |

The 138 names that differ belong to groups that are the states of form
fields; IDML names them by their state type (form fields).

Values other than the default: 206 items with a name given by the
user, 176 hidden, 1,241 with change counts, 1,113 with overridden
properties. Flags 0x22 are in 66,820 items, 0x55 (`FixedDimension
FlexibleDimension FixedDimension` both ways) in 643 and 0x02
(`FlexibleDimension` three times horizontally) in 13; with another bit
set the converter leaves both attributes out. In 14,918 items with
chunk 0x1424 the master item is 0; in 197 of them the list still holds
IDs, and IDML writes the attribute empty.

**Layer position before grouping.** From 21.4 every page item and
placed graphic in IDML has `BeforeGroupingLayerPosition`; no IDML before
21.4 has it (13 distinct corpus IDML files from 21.4, none of the 1,673
earlier ones). Its value is the i32 at 4 of chunk 0x324, or −1 without the
chunk. The converter writes it for files saved by 21.4 or later: 1,793 of 1,793
values over the trustworthy pairs (1,308 polygons, 241 text frames, 131
rectangles, 53 groups, 38 images, 16 EPS, 3 graphic lines and 3 SVG).

Chunk 0x324 of page items and placed graphics:

| Offset | Contents | IDML |
|---|---|---|
| 0 | u32 UID: 0, a layer (class 0x302) or a spread layer (class 0x301) | `BeforeGroupingLayerReference` |
| 4 | i32 | `BeforeGroupingLayerPosition` |
| 8 | u8 0, 1 or 2, then bytes not decoded (in some items an in-object string with a layer name) | |

`BeforeGroupingLayerReference` is the layer the UID gives: the UID itself
for a layer, the u32 at 0 of the spread layer's chunk 0x302 for a spread
layer. 0, no chunk or another class give no attribute. Over the 1,831
page items and placed graphics of the 12 DOM 21.4–21.5 pairs:

| Chunk 0x324 | IDML | Items |
|---|---|---|
| u32 at 0 is 0 | no `BeforeGroupingLayerReference` | 1,667 of 1,667 |
| a spread layer | that spread layer's layer | 36 of 36 |
| a layer | that layer | 14 of 15 |
| no chunk | no reference, position −1 | 113 of 113 |

The exception is one rectangle on a master spread whose chunk names a
layer and position 1; its IDML has position −1 and no reference. No
field that sets it apart was found. The converter's output has 50 of
the 50 references of the pairs (19 polygons, 15 rectangles, 10 text
frames, 6 groups) and no other.

**Stroke weight without an attribute.** An item whose attribute list has
no stroke weight (attribute 0x6E65) takes it from the document's base
list (class 0x6E02, `attributes.md`, values an item does not store):
1 in most documents, 0.2835 in others. IDML writes it when it differs
from the object style's, so items with `[None]` (stroke weight 0) have
it. Groups vary (1,166 without the attribute, 625 with 1 and 30 with
other values, in 200 of the pairs) and get none.

**Overprint.** The attribute list of a page item holds the overprint
flags of the fill (0x6E6A) and the stroke (0x6E67), u16 1 `true`, as in
object styles (object style settings). IDML writes `OverprintFill` on a
rectangle, oval, polygon, graphic line, text frame or EPS text when its
fill puts ink on the page, by the object style rule: the fill colour
(the item's 0x6E68, else its object style's) is not `None`, `Paper` or a
white process colour, and the tint is not 0. The value is the item's
flag; an item without the flag has `false`, which IDML writes only when
the object style turns the fill category off (as for the fill colour,
attributes.md, values an item does not store). `OverprintStroke`
follows the same rule with the stroke colour (0x6E64), tint (0x6E66)
and category. Trustworthy pairs: the rule reproduces all 3,315
`Polygon`, 64 `Rectangle`, 33 `TextFrame`, 33 `EPSText` and 3 `Oval`
`OverprintFill` values and all 68, 30, 20, 2 and 1 `OverprintStroke`
values of polygons, graphic lines, rectangles, ovals and text frames,
with no extra value. Of the items that store 0x6E6A, 27 have a `Paper`
fill and no `OverprintFill`.

A group stores no flag. IDML gives it the flag that its children share,
counting only children whose fill (for `OverprintFill`) or stroke (for
`OverprintStroke`) puts ink on the page by the rule above: the child's
own flag, `false` without one; for a child group, its shared flag. A
group whose children with ink differ, or that has none, gets no
attribute, and neither does a group whose shared flag equals its object
style's value (`false` where the style has none). In documents saved by
21.4 or later, where every object style has the overprint attributes
(object style settings, overprint), children without ink count too,
with their own flag, else their style's value. Over all 1,462 pairs this
gives 320 of 320 `OverprintFill` values of groups (313 in 82
trustworthy pairs) and 2 of 2 `OverprintStroke` values, all `true`,
with no extra value; 4 more are written in a stale pair on groups that
its IDML does not have. 75 of the 320 groups have a child that
has no attribute in IDML. The 21.4 case rests on one group of one
trustworthy pair, whose child without ink has the style value `false`
and whose IDML has no attribute; before 21.4 a child whose stroke is
`Paper` under a style with `OverprintStroke="false"` does not count
(one group).

## Object export options

Page items (class 0x6201 and groups, 0x401) may have chunk 0x1E206, the
alternative text and tagging settings that IDML writes in
`ObjectExportOption`:

| Size | Contents | IDML `ObjectExportOption` |
|---|---|---|
| 4 | u32 alternative text source: 0 `SourceCustom`, 5 `SourceXMLStructure`, 8 `SourceDecorativeImage` | `AltTextSourceType` |
| flagged string | custom alternative text | `CustomAltText` |
| flagged string | | `Properties/AltMetadataProperty NamespacePrefix` |
| flagged string | | `Properties/AltMetadataProperty PropertyPath` |
| 4 | u32 actual text source: 0 `SourceCustom`, 5 `SourceXMLStructure`, 6 `SourceXMPAltText` | `ActualTextSourceType` |
| flagged string | custom actual text | `CustomActualText` |
| flagged string ×2 | | `Properties/ActualMetadataProperty` (as above) |
| 4 | u32 tagging: 0 `TagFromStructure`, 1 `TagArtifact` | `ApplyTagType` |
| rest | 64 bytes (DOM 8–9), 83 (10–20), 88 (21) | not decoded |

A flagged string is a u8 (1: a built-in key, written `$ID/` and the
string; 2: a plain string) and an in-object string. An empty key is
`$ID/`.

Evidence: 69,814 page items with the chunk in the trustworthy pairs; all
eight values match in 69,814 of 69,814 (other than the default: 30
custom and 199 decorative alternative text sources, 197 custom and 14
XMP actual text sources, 205 artifacts, 31 custom alternative texts, 197
custom actual texts). The 22,005 items without the chunk all have the
values `SourceXMLStructure`, `$ID/` and `TagFromStructure`. The chunk
parses in all 363,145 page items of the distinct little-endian corpus
files. The converter leaves out an attribute whose code is not in the
table. Which other attributes `ObjectExportOption` has depends on the
version (`idml-values.md`, export options of page items).

**Object styles** (class 0x1B901) may have chunk 0x1E206 with the same
layout, and IDML writes its first eight values in the style's
`ObjectExportOption` the same way. In the 1,503 pairs outside the
privately held samples, 3,643 object styles have the chunk: 20 store
tagging 1 and have `ApplyTagType="TagArtifact"` (20 of 20, in 2 DOM 20
pairs); all others store the default values and have them in IDML.
One privately held sample adds 1 object style with source 0 and a
custom alternative text: `AltTextSourceType="SourceCustom"` and the text
in `CustomAltText` (1 of 1). Styles without the chunk have the
defaults.

**Export size (chunk 0x1E22B).** A list of keys and values: u32 count;
per entry a u32 length and the key in text segments, a flag byte, two
bytes, a u16 value count and the values, each a u8 length, a u8 and that
many bytes of text. The keys are `UseOrigImage`, `CustomSizeType`,
`CustomSizeValue` and `CustomContentConversion`. `CustomSizeType` is IDML
`SizeType`: `1` `DefaultSize`, `2` `FixedSize`, `3` `RelativeToTextFlow`,
`4` `RelativeToTextSize`; an item without the chunk has `DefaultSize`.

**Generated alternative text.** From 21.4 page items have three more
chunks:

| Chunk | Contents | IDML `ObjectExportOption` |
|---|---|---|
| 0x1E260 | u16: 1 `true`, 0 `false` | `AltTextGenerationError` |
| 0x1E261 | four f64 | `AltTextCropSyncRect`: empty when all four are 0, else the four numbers (below) |
| 0x1E262 | u16: 1 `true`, 0 `false` | `AIGeneratedAltText` |

Without the chunk the values are `false`, `false` and empty; the
converter also writes these for another u16 or a chunk too short for
its values. Evidence, the 1,701 page items other than groups of the 12
DOM 21.4–21.5 pairs: `AltTextGenerationError` 1,701 of 1,701 (22
`true`), `AIGeneratedAltText` 1,701 of 1,701 (3 `true`),
`AltTextCropSyncRect` 1,701 of 1,701 (8 not empty, two distinct
rectangles). Items with a `true` flag have `AltTextSourceType="SourceCustom"`
and the generated or error text in `CustomAltText` (chunk 0x1E206); the
tail of chunk 0x1E206 is the same in all of them and does not hold the
flags. Object styles have `AltTextCropSyncRect=""` (49 of 49).

`AltTextCropSyncRect` writes each f64 with exactly 8 decimals, cut, not
rounded, from the shortest decimal form, joined by commas:
`415.60850393700787` gives `415.60850393` (rounding would give `…394`),
`306.7741732283466` gives `306.77417322`, `370.32` gives `370.32000000`.

**Groups** have no `AIGeneratedAltText` and `AltTextGenerationError` in
their `ObjectExportOption`: none of the 53 groups of the 7 trustworthy
documents from 21.4 has them. The converter leaves them out on groups,
which removes 106 extra values.

## Text wrap

Page items (class 0x6201 and groups, 0x401) and placed graphics have
chunk 0x3703 (44 bytes):

| Offset | Contents |
|---|---|
| 0 | u32 wrap mode: 0 `None`, 1 `JumpObjectTextWrap`, 3 `BoundingBoxTextWrap`, 6 `Contour` |
| 4 | u32 UID of the wrap path object (class 0x3702), or 0 |
| 8 | Four f64 offsets: left, top, right, bottom (IDML `TextWrapOffset`) |
| 40 | u16 master page flag: 1 `ApplyToMasterPageOnly="false"`, 0 `true` |
| 42 | u16 `TextWrapSide`: 0 `BothSides`, 1 `LeftSide`, 2 `RightSide`, 3 `SideTowardsSpine`, 4 `SideAwayFromSpine`, 5 `LargestArea` |

**Text wrap of InDesign 2.0.** In files from InDesign 2.0 the chunk
has 40 bytes: the fields up to offset 40 above, without the master
page flag and the side. The converter reads them and writes
`ApplyToMasterPageOnly="false"` and `BothSides`, the values of codes 1
and 0. Evidence: three privately held 2.0 documents with their print
PDFs (above, the story list of InDesign 2.0): 129 of 129 chunks are
40 bytes (115 page items, 13 images, 1 group); the modes are 0 (89),
3 (17) and 6 (23); each of the 40 items with mode 3 or 6 names a wrap
path object (class 0x3702) whose chunk 0x3709 has 10 bytes and custom
path flag 0 (40 of 40), so no path is written. In the PDFs, text keeps
clear of these items: on the pages with mode 6 images and mode 3
frames, the text beside them is set around them, and a justified
paragraph's left edge steps right beside an image while its right edge
stays flush. The converter took the 40-byte chunk for no chunk, and
wrote every item of these documents with `TextWrapMode="None"`.

**Wrap path.** Every item with wrap mode 1, 2, 3 or 6 (and the codes
0x10003 and 0x10006) has a wrap path object; mode 0 items have none
(9,061 objects in 735 files of the corpus of 2026-10). The u32 at 4
equals the object's UID in 8,824 of the 8,824 objects whose owner was
found. Its chunks:

| Chunk | Contents |
|---|---|
| 0x162B | Path geometry, the layout of page items. Parses in 8,846 of the 8,846 little-endian objects that have it; 18 have none |
| 0x3709 | 10 bytes: u32 UID of the item (9,061 of 9,061), u32 0 (9,061 of 9,061), u16 custom path flag |
| 0x2C0B, 0x2C2D, 0x377A | On 4,813 of 9,061; not read |

IDML writes the path as `TextWrapPreference/Properties/PathGeometry`,
before `TextWrapOffset`, only when the custom path flag is 1. Over the
2,342 IDML wrap elements that have a wrap path object (1,412 in
trustworthy pairs):

| u16 at 8 of 0x3709 | IDML `PathGeometry` | All pairs | Trustworthy |
|---|---|---|---|
| 1 | present | 35 of 35 | 32 of 32 |
| 0 | absent | 2,307 of 2,307 | 1,380 of 1,380 |

The 35 are 28 polygons (22 `BoundingBoxTextWrap`, 6 `Contour`), one
rectangle, one group, one text frame (`JumpObjectTextWrap`) and 4 images
(`Contour`). In all 35 the IDML paths equal the 0x162B paths: number of
paths (1 to 3), point count, `Anchor`, `LeftDirection` and
`RightDirection` from the point's anchor, left and right, and `PathOpen`
from the u16 after the points (0 `false`). The numbers are the same
(tolerance 1e-6), so no transform applies: the path is in the item's
inner coordinates, those of its own `PathGeometry` or, for a placed
graphic, of its `GraphicBounds` (before the graphic's `ItemTransform`).
It already includes the wrap offsets: the path of an image with a 8.5 pt
top offset and a scale of 0.40885 extends 20.79 (8.5 / 0.40885) beyond
the graphic bounds on each side.

With flag 0 the object still stores a path: for frames and groups the
item's path bounding box grown by the offsets (1,538 of 1,538 cases
compared; in about 500 more the box is also grown by 0.25 to 1.5 pt on
each side), for contour wraps of images the computed contour. IDML omits
these, and the converter writes the path only for flag 1.

Placed graphics also have chunk 0x373D, the contour (IDML
`ContourOption`). See "Contour" below.

**Evidence from the pairs.** For every element with a `Self` in the
corpus pairs, the IDML `TextWrapPreference` was compared with chunk
0x3703 of the INDD object with that UID. Same-version pairs first, then
the pairs whose IDML is from an older version:

| INDD | IDML | Same version | Older IDML |
|---|---|---|---|
| Mode 0 | `TextWrapMode="None"` | 2,625 of 2,625 | 615 of 615 |
| No chunk 0x3703 | `TextWrapMode="None"` | 1,468 of 1,468 | 873 of 873 |
| Mode 1 | `TextWrapMode="JumpObjectTextWrap"` | 2 of 2 (one file) | 1, matched by geometry (below) |
| Mode 3 | `TextWrapMode="BoundingBoxTextWrap"` | 1 of 1 (an image) | none |
| Mode 6 | `TextWrapMode="Contour"` | 3 of 3 (one file) | 1 of 1 (an image) |
| Offsets as left, top, right, bottom | `TextWrapOffset` | 2,631 of 2,631 | 616 of 616 |
| u32 at 40 = 1 (little-endian) | `Inverse="false"`, `ApplyToMasterPageOnly="false"`, `TextWrapSide="BothSides"` | 2,630 of 2,630 | 616 of 616 |
| 0x373D type 5 | `ContourType="SameAsClipping"` | 224 of 225 (1 has no `ContourOption`) | 75 of 75 |

**Contour.** Chunk 0x373D of placed graphics (29 bytes) holds the
contour that IDML writes as `ContourOption`. The preferences chunk 0x373F
(`preferences.md`) and the object style chunk 0x3777 (object style
settings, below) have the same layout.

| Offset | Contents |
|---|---|
| 0 | u32 contour type: 0 `BoundingBox`, 1 `DetectEdges`, 2 `AlphaChannel`, 3 `PhotoshopPath`, 4 `GraphicFrame`, 5 `SameAsClipping`, 6 `SelectSubject` |
| 4 | f64: 5 in every placed graphic and the preferences, 5 or 8 in object styles |
| 12 | u8: 25 everywhere |
| 13 | `00 00 FF FF` everywhere |
| 17 | u8 `IncludeInsideEdges`: 0 `false`, 1 `true` |
| 18 | 11 zero bytes |

Evidence, over the placed graphics of the 1,251 trustworthy pairs (corpus
of 2026-10) whose IDML has a `ContourOption`:

| INDD | IDML | Graphics |
|---|---|---|
| Type 5 | `ContourType="SameAsClipping"` | 2,693 of 2,698 (5 have no `ContourType`, below) |
| Type 2 | `AlphaChannel` | 13 of 13 |
| Type 1 | `DetectEdges` | 8 of 8 |
| Type 0 | `BoundingBox` | 4 of 4 |
| Type 6 | `SelectSubject` | 1 of 1 |
| Byte 17 = 1 | `IncludeInsideEdges="true"` | 1 of 1 (type 2) |
| Byte 17 = 0 | `IncludeInsideEdges="false"` | every other graphic with `ContourType` |

Types 3 and 4 occur in no pair: no IDML in the corpus has either code.
The IDML schema's `ContourOptionsTypes` list leaves `PhotoshopPath` and
`GraphicFrame` as the two names without a code, and the INDD shows which
is which:

- The graphic's chunk 0x1635 holds path data in the layout of chunk
  0x162B (an outline of the picture content). All 5 type 3 graphics of
  v7.0 and later files have it (3 images, 2 EPS, 5 files); none of the
  12 type 4 graphics (3 files) does. Types 1, 2 and 6 have it too (118
  of 118 sampled), types 0 and 5 do not (56 of 56 sampled).
- The wrap outline (chunk 0x162B of the 0x3702 object) of a type 4
  graphic is the frame outline grown by the Top offset on every side.
  Two graphics have a frame that differs from the image bounds (one
  crops the image, one is wider than the placed bounds); both outlines
  follow the frame, not the image. The wrap outline of a type 3 graphic
  is its 0x1635 path, simplified and grown by the Top offset (3 of 3 with
  an outline; one is 2.1 units short at the bottom), not the image box.

So type 3 is drawn from a path of the picture (`PhotoshopPath`) and type 4
from the frame (`GraphicFrame`). The converter leaves the
`ContourOption` of other codes out.

**Contour parameters.** From DOM 21.4 every `ContourOption` has
`ContourTolerance="5"`, `ContourThreshold="25"`,
`ContourAlphaIndex="0"` and `ContourPathIndex="-1"`, after
`ContourPathName`: 211 of 211 elements in the 12 corpus IDML files of
DOM 21.4 and 21.5 (graphics, frames, object styles and preferences), all
with these values. No `ContourOption` of an earlier DOM has them (32,227
elements in the 1,444 IDML files of DOM 7 to 21.3). Every `ContourOption` of DOM 21.4 and
later also has `ContourType`. The contour chunks (graphic 0x373D, object
style 0x3777, preferences 0x373F) may hold them as the f64 at 4, the u8
at 12 and the i16 at 13 and 15, in the layout of the clipping chunk
0x2C1A (f64 tolerance, u8 threshold, i16 index): the 102 contour chunks
of the DOM 21.4–21.5 pairs hold 5, 25, 0 and −1 there. Over all 1,462
pairs, the only other value is f64 8 in 72 object style chunks (24
files of DOM 10.0, 10.2 and 17.0), whose IDML has no `ContourTolerance`,
so the assignment is not shown. The converter writes the four values on every `ContourOption`
with a `ContourType` from DOM 21.4. A `ContourOption` with only
`ContourPathName` has no sample from DOM 21.4; the converter writes it
without them.

`ContourPathName` is not in the chunk. Every `ContourOption` of the
corpus IDMLs has it (25,551 of 25,551 in 1,693 IDML). It is `$ID/` in
25,525; the other 26 name an alpha channel of an image: 13 images with
`ContourType="AlphaChannel"` and their 13 frames, in five documents.
The name comes from image chunk 0x177C:

| Offset | Contents |
|---|---|
| 0 | u32 channel count *n* (0 in most images, which then have 4 bytes) |
| 4 | per channel: a flag byte (3 or 0; meaning unknown), an in-object string (the channel name), then bytes not decoded |

For example `01 00 00 00 | 03 | 02 03 03 00 | 03 80` and 3 UTF-16
units. Over the images of the pairs that have a `ContourOption`:

| Image | `ContourPathName` | Images |
|---|---|---|
| Type 2, *n* ≥ 1 | the first channel name | 13 of 13 |
| Type 2, *n* = 0 or no chunk | `$ID/` | 10 of 10 |
| Any other type | `$ID/` | all (8,629), including 1,075 images with 1 to 7 channels |

No chunk of the type 3 graphics holds a path name (strings searched in
all chunks), so the converter writes `$ID/` for type 3 as for the other
types.

Which channel a type 2 contour names when *n* > 1 is not shown (no
sample); the converter takes the first. Image chunk 0x1714 is not the
source: its alpha channel name is `$ID/` in all 22 type 2 images.

A graphic without chunk 0x373D has the contour of the preferences chunk
0x373F, and without that chunk, type 5 and `IncludeInsideEdges="false"`.
Of the 9,535 graphics (images, EPS, PDF, SVG and imported pages) whose
`ContourOption` the converter writes and the trustworthy IDMLs have,
`ContourType` and `IncludeInsideEdges` are equal in all; this includes
the 319 images without the chunk that have `DetectEdges` because their
document's chunk 0x373F has type 1.

A frame with one placed graphic (`Rectangle`, `Oval`, `Polygon`) has the
contour of its graphic, unless its object style turns the text wrap
category on (all of 0x1B942, 0x37C8 and 0x37C9 in chunk 0x1B92E, IDML
`EnableTextWrapAndOthers="true"`). Then its `ContourOption` has only
`ContourPathName`: 517 of 518 such frames in the trustworthy pairs. In
all corpus pairs, the value is `$ID/` in all 568 `ContourOption`
elements with only that attribute (554 frames and the 14 graphics
below); the converter writes the name of the graphic's contour, which
is `$ID/` unless the graphic has type 2 and a channel (no sample of
that in such a frame). The 8,965 other frames with `ContourType` take
their graphic's type and `IncludeInsideEdges` in all cases, and the
frames of the 13 named images their `ContourPathName`. Frames without a
graphic have no `ContourOption` (all but one).

The IDML has no `ContourType` and `IncludeInsideEdges` on 14 graphics:
5 with type 5 in chunk 0x373D and 9 without the chunk. 12 of them are in
frames whose object style turns the text wrap category on, but 495
graphics in such frames have both attributes. No INDD field is known
that separates them, and the converter writes the type for all 14.

**Master page flag and side.** The 4 bytes at 40 are two u16 in the
file's byte order. The big-endian pair shows it: its 261 page items
store `00 01 00 00` (251, IDML `ApplyToMasterPageOnly="false"`) or
`00 00 00 00` (10, `true`), all with `TextWrapSide="BothSides"`. In
little-endian files the u32 1 is therefore the u16 1 at 40 and the u16 0
at 42. With this layout the converter reproduces, over the 1,251
trustworthy pairs of the corpus after 2026-10, all 256,651
`ApplyToMasterPageOnly` and `TextWrapSide` values of page items, form
fields and placed graphics, where the u32 reading had left 6,322 of
each out (items storing 0 at 40 or a side code). Side codes 1 to 5
occur on few items (14, 4, 9, 1 and 32 in the trustworthy pairs).
Items without the chunk have `false` and `BothSides`.

`Inverse` is `false` on every item of the corpus IDML files (256,651 of
256,651 in the trustworthy pairs); no INDD field is known for it, and
the converter writes `false`.

**Order of the offsets.** Most offsets in the pairs are 0, so the order
rests on a few elements:

- Offset 24 (third) is `Right`: the bounding-box image has 2.83 there
  and 0 elsewhere; its IDML has `Right="2.834645669291339"` and 0 for
  the other sides.
- Offset 32 (fourth) is `Bottom`: both jump-object frames have 14.17
  there and 0 elsewhere; their IDML has `Bottom="14.173228346456694"`
  and 0 for the other sides.
- Offset 16 (second) is `Top`. In a pair whose IDML is from an older
  version (IDML 17.0, INDD 18.1), the INDD has one jump-object frame with
  11.34 at offset 16 and 5.67 at offset 32. Its UID differs from the IDML
  element's, but its path and `ItemTransform` are identical to those of
  the IDML's only jump-object frame, which has `Top="11.338582677165356"`
  and `Bottom="5.669291338582678"`. A sample and its print PDF agree; see
  below.
- Offset 8 (first) is therefore `Left`. It is 0 in every sample that
  has a reference.
- The two contour frames have the same value at all four offsets, as in
  their IDML.

**Evidence from a sample and its print PDF.** The sample has frames
with mode 1. Offset 8 is 0 in all of them; offset 16 is 0, negative or
positive. With jump-object wrap, no text is set beside a frame, and text
above the frame must end above the top of the wrap area. Text line
positions in the PDF were taken from pdftotext word boxes and compared
with the frame bounds:

- Where offset 16 is 0, the body line above a frame ends at most 1.5 pt
  below the frame's top edge.
- Where offset 16 is negative, the body line above may end inside the
  frame, never further below its top edge than the offset's size plus
  1.5 pt. Where the next line position would end further down than
  that, it is left empty.
- A line that lies wholly between a frame's top and bottom edges falls
  inside the band that a large negative offset opens.

So the value at offset 16 moves the top edge of the wrap area, and a
negative value lets text into the frame from above.

**Other values.** One image in the same-version pairs has 0x40001 at
offset 40 and `TextWrapSide="SideAwayFromSpine"`; no other value is
shown, so the converter writes the side and inverse settings only when
the u32 at 40 is 1. Modes other than 0, 1, 3 and 6 have no IDML
evidence; the converter leaves out `TextWrapPreference` for them.

## Frame fitting

The frame fitting settings are attributes in the page item attribute list
(chunk 0x6E03, `attributes.md`). Object styles hold all of them in chunk
0x1B956 (a u16 count, then records of the same layout).

| ID | IDML `FrameFittingOption` attribute | Encoding |
|---|---|---|
| 0x6E83 | `AutoFit` | u32: 0 `false`, 1 `true` |
| 0x6E7E | `LeftCrop` | f64 |
| 0x6E7F | `TopCrop` | f64 |
| 0x6E80 | `RightCrop` | f64 |
| 0x6E81 | `BottomCrop` | f64 |
| 0x6E7C | `FittingOnEmptyFrame` | u32: 0 `None`, 1 `ContentToFrame`, 2 `Proportionally`, 3 `FillProportionally` |
| 0x6E7D | `FittingAlignment` | u32: 0 `TopLeftAnchor`, 1 `TopCenterAnchor`, 3 `LeftCenterAnchor`, 4 `CenterAnchor` |

Evidence, from the same-version pairs:

- **Object styles.** All 337 object styles paired by name hold all seven
  IDs in chunk 0x1B956, and all seven match the style's IDML
  `FrameFittingOption` in 337 of 337 (`FittingAlignment` 4 in 313
  styles, 0 in 24).
- **Crops.** Each crop ID is told apart by non-zero values: wherever IDML
  writes the attribute, the frame's own value, or its style's if the
  frame has none, matches: left 429, top 397, right 433, bottom 451, no
  mismatch. Non-zero values: left 235, top 204, right 239, bottom 258.
- **Fitting on an empty frame.** 301 of 301; codes 0, 1, 2 and 3 occur
  (1,154, 9, 77 and 16 values).
- **Alignment.** 200 of 205. The other 5 are frames in one file with the
  style `[None]`, no local value and a style value of 0, where IDML says
  `CenterAnchor`; not explained. Code 0 is shown in 9 frames, code 4 in
  object styles. Other codes are left out.
- **AutoFit.** 0 and `false` wherever IDML writes it (200 of 200); other
  values are left out.

Codes 1 and 3 of `FittingAlignment` and code 1 of `AutoFit` come from
the corpus after 2026-10, over all pairs (item and object style lists
against their IDML): `AutoFit` 1 `true` in 927 items and styles, 0
`false` in 2,548; `FittingAlignment` 1 `TopCenterAnchor` in 21 object
styles of one 14.0 document, 3 `LeftCenterAnchor` in 1 item. Codes 2
and 5 to 8 do not occur; the converter leaves them out.

**Where IDML writes it.** Only on rectangles, ovals and polygons, never
on text frames (1,154) or lines (136), even when their list has these
attributes. The rule depends on the applied object style's category
list (chunk 0x1B92E, object style settings):

1. **Fitting category off.** When the list lacks 0x1B960
   (`EnableFrameFittingOptions`), IDML writes all seven attributes:
   the item's value where its list has the ID, the style's otherwise.
   This holds for the root `[None]` too, although its IDML element has
   no `Enable…` attributes. In the trustworthy pairs of the corpus after
   2026-10, the root style's list lacks 0x1B960 in 22 documents, and all
   9,560 items there that use the root style have all seven attributes;
   in the other 883 documents with such items, no item that uses the
   root style has all seven.
2. **Otherwise**, let D be the item's local attributes whose value
   differs from its object style's. IDML writes D; with D empty it
   writes no element. An object style without a category list (older
   files) gives all seven of its values when D is empty, unless it is
   the root `[None]`.

Over the trustworthy pairs, the converter reproduces 33,274 of 33,702
`Rectangle/FrameFittingOption` elements, 13,131 of 13,648 for polygons
and 6,929 of 7,207 for ovals, with every attribute of the written
elements except 131 `FittingAlignment` values and 7 values of each of
three crops. Of the elements not reproduced, 585 belong to items the
converter does not write with that tag, and 632 are on items that use
the root `[None]` (style value 0) and do not store 0x6E7D, where IDML
writes only `FittingAlignment="CenterAnchor"` (609 of them in one 20.3
document); 6 rectangles remain. The 131 differing values are of the same kind,
on items of other styles. No stored value tells these items apart from
the items where IDML writes the style's value; the converter writes the
style's value.

## Clipping path settings

Images, PDF and EPS graphics may have chunk 0x2C1A (37 bytes):

| Offset | Contents | IDML `ClippingPathSettings` |
|---|---|---|
| 0 | u32: 0 `None`, 1 `DetectEdges`, 2 `AlphaChannel`, 3 `PhotoshopPath` | `ClippingType` |
| 4 | f64 | `Tolerance` |
| 12 | f64 | `InsetFrame` |
| 20 | u8 | `Threshold` |
| 23 | i16 | `Index` (not for type 2, below) |
| 25 | u8 flags: 0x02 `UseHighResolutionImage`; 0x01 and 0x08 `InvertPath` and `RestrictToFrame` | |
| 26 | 11 bytes, 0 in every sample | |

Evidence: 75 of 235 images in the same-version pairs have the chunk, and
all 75 match on the tolerance, inset, threshold and index (tolerance 2
or 0, threshold 25 or 0). PDF (153 of 206) and EPS (6 of 18) graphics
have the chunk with the same values as images that use the defaults.
Graphics without the chunk (160 images, 53 PDF, 12 EPS) have
`UseHighResolutionImage="true"`, `Threshold="25"`, `Tolerance="2"`,
`InsetFrame="0"` and `Index="-1"` in IDML, and the converter writes
those values for them.

Over the trustworthy pairs of the corpus after 2026-10, every graphic
with the chunk was compared with its IDML element (2,686 graphics):

| INDD | IDML | Graphics |
|---|---|---|
| Type 0 | `ClippingType="None"` | 2,655 of 2,655 |
| Type 1 | `DetectEdges` | 2 of 2 (2 pairs) |
| Type 2 | `AlphaChannel` | 1 of 1 |
| Type 3 | `PhotoshopPath` | 28 of 28 (12 pairs) |
| Flags 0x02 | `UseHighResolutionImage="true"`, `InvertPath="false"`, `RestrictToFrame="false"` | 2,680 of 2,680 |
| Flags 0x00 | `UseHighResolutionImage="false"`, `InvertPath="false"`, `RestrictToFrame="false"` | 5 of 5 |
| Flags 0x09 | `UseHighResolutionImage="false"`, `InvertPath="true"`, `RestrictToFrame="true"` | 1 of 1 (type 1, threshold 111, tolerance 3.9) |
| i16 at 23, types 0, 1 and 3 | `Index` (-1, or 0 in 9 type 3 images) | all |

Bits 0x01 and 0x08 are set together in the one sample, so which is
`InvertPath` and which `RestrictToFrame` is not shown. The converter
writes both when the two bits are equal and leaves both out otherwise.
Other type codes have no sample; the converter leaves out the element
for them.

The one type 2 image stores -1 at 23, and its IDML has `Index="0"`. No
other field of the chunk differs from the type 1 and 3 images, so the
index of the alpha channel is not in this chunk; the converter leaves
`Index` out for type 2.

`IncludeInsideEdges="false"` has no identified field: all 11,823
`ClippingPathSettings` elements in the IDML files of the 1,462 corpus
pairs have it, and the converter writes it.

**Path name.** `AppliedPathName` names the image's path or channel:

| Type | `AppliedPathName` | Graphics |
|---|---|---|
| 0 | `$ID/` | all 12,280 `None` elements of the corpus IDML files |
| 1 | `$ID/` | 2 of 2 |
| 2 | the first channel name of image chunk 0x177C (text wrap, contour) | 1 of 1 |
| 3 | the name in key 0x7D of image chunk 0x1708 | 17 of 17 with an ASCII name |

Key 0x7D of the image properties (13 bytes in all samples) holds a u8
length *n*, *n* bytes of name, then 6 bytes (`00 00 33 33 00 01` or
`00 00 00 00 00 01`). The name bytes are not Unicode. The other 11 type
3 images name their path in a legacy code page: in 5 (Korean documents)
key 0x7D holds bytes that decode in code page 949 to the IDML name; the
other 6 (Japanese documents) have no key 0x7D, and key 0x80 of the same
chunk holds the name as bytes that decode in Shift-JIS. None of these
INDD files has the name as UTF-16 or UTF-8 text, except two where it is
part of a longer text in another record. The file does not record the
code page, so the converter writes the name only when every byte is
printable ASCII (0x20 to 0x7E) and leaves `AppliedPathName` out
otherwise.

**Geometry.** IDML writes `Properties/ClippingPathGeometry` for every
graphic of type 1, 2 or 3 (31 of 31 images, 13 pairs), and for no
graphic of type 0. The paths are those of the graphic's own chunk
0x162B, in the layout of page item paths: the converter's output equals
the IDML element exactly, number for number, in all 31 (1 to 8 paths of
4 to 179 points). The 2,655 graphics of type 0 with chunk 0x162B (1,797
images, 329 PDF, 528 EPS, 1 WMF graphic) have no `ClippingPathGeometry`.
IDML puts it before `GraphicBounds`.

**Placed pages.** IDML has `ClippingPathSettings` on every
`ImportedPage` (42 in 8 pairs, 38 in 7 trustworthy pairs). No placed
page has chunk 0x2C1A, and all 42 have the values of graphics without
the chunk; the converter writes those values for them.

**ImageIOPreference.** Chunk 0x1714 of an image:

| Offset | Size | IDML `ImageIOPreference` |
|---|---|---|
| 0 | u32 | 72 in all 1,300 images with the chunk |
| 4 | u16 | `ApplyPhotoshopClippingPath`: 1 `true`, 0 `false` |
| 6 | u16 | 1 in all (`AllowAutoEmbedding="true"` in all IDML) |
| 8 | flagged string | `AlphaChannelName` (an empty key is `$ID/`) |

Without the chunk, IDML has `ApplyPhotoshopClippingPath="true"` and
`AlphaChannelName="$ID/"`. Evidence: 3,607 of 3,607 images of the
trustworthy pairs for both attributes (14 `false`, 92 `$ID/kNoneName`,
2 plain names). Chunk 0x8C39 (u16 at 0), which looked like the clipping
flag in fewer samples, gives the wrong value in 6 images and is missing
in 549. `AllowAutoEmbedding="true"` is written from observation.

## Placed graphics

Images (0x1702), PDF (0x2501), EPS (0x6601) and SVG (0x6639) are children
of a frame (in its 0x15B list) and share these chunks:

| Chunk | Contents |
|---|---|
| 0x151 | Matrix: `ItemTransform` (437/437 match) |
| 0x1633 | Four f64: `GraphicBounds` left, top, right, bottom |
| 0x8CBC | link list (story links and placed text, link lists): the link is the first UID, at offset 8 |

**Links (0x8C42).** Links of linked stories and linked page items
have another class with the same chunks (story links and placed text).
`LinkClassID` is the class of the link object in decimal: 2,386 of
2,386 IDML links in the 158 little-endian documents whose UID is a link
object in the INDD (19 more UIDs hold other objects, all in stale
pairs). Chunk 0x8C9B:

| Offset | Size | Contents | IDML `Link` |
|---|---|---|---|
| 0 | 4 | 0 in all links | |
| 4 | 4 | 0x101 in all links | (`LinkClientID="257"`) |
| 8 | 4 | UID of the link resource (class 0x8C41) | |
| 12 | 4 | 1 or 0 | `LinkResourceModified`: 1 `false`, 0 `true` |
| 16 | 4 | UID of the graphic (or story) | |
| 20 | 2 | 1 or 0 | `ShowInUI`: 1 `true`, 0 `false` |
| 22 | 6 | three u16, 1 in all links | |
| 28 | 4 | 0 in all links | |
| 32 | 4 + text | import stamp: u32 length, then text segments | `LinkImportStamp` |
| then | 8 | modification time of the file (FILETIME) | `LinkImportModificationTime` |
| then | 8 | time the file was placed or updated (FILETIME) | `LinkImportTime` |

A FILETIME here is the count of 100 ns intervals since 1601-01-01 UTC,
stored as two u32: the high half first, then the low half (each in the
file's byte order). Evidence: the stamp text is `file <n> <size>`, and
`<n>` equals the first time value in 4,686 of 4,686 links with a stamp.
A link with an empty stamp (length 0) has neither `LinkImportStamp` nor
`LinkImportModificationTime` in IDML, but has `LinkImportTime` (13 of
13).

Chunk 0x1B6 of a link (u32, on 59 links) is `PDFIdentifier`, which IDML
writes from DOM 21 on; links without the chunk have `PDFIdentifier="0"`
(81 of 81 DOM 21 links).

**Link resources (0x8C41)**: chunk 0x8C92 is a flag byte, u32 length,
then the URI as bytes (`LinkResourceURI`).

After the URI, chunk 0x8C92 continues:

| Offset after the URI | Field |
|---|---|
| 0 | u8 1, u8 1, u16 0 (all resources) |
| 4 | u32 0 or 1, not identified |
| 8 | u32 stored state: 0 `Normal`, 2 `Embedded`, 3 `Contained` (`StoredState`, below) |
| 12 | u32 UID of a raw data object (class 0x129), or 0 |
| 16 | stamp: u32 length, text segments (as in the link) |
| then | modification time (FILETIME, as in the link) |
| then | file size: u32 high half, u32 low half; `LinkResourceSize` is `<high>~<low>` in lower-case hexadecimal without leading zeros (`0~42ebd`) |
| then | u8, 1 in all resources |
| then | in-object string: the format name; `LinkResourceFormat` is `$ID/` and the name, and the graphic's `ImageTypeName` is the same |

Evidence over the 4,699 links of the trustworthy pairs (4,697 with a
resource object), with the converter's output compared with the
reference:

| IDML | Equal |
|---|---|
| `LinkResourceModified` | all 4,699 (5 with 0 = `true`) |
| `ShowInUI` | all 4,699 (2 with 0 = `false`) |
| `LinkImportStamp` | 4,105 of 4,105 links with the same URI in the INDD and the IDML |
| `LinkResourceSize`, `LinkResourceFormat` | 4,118 of 4,118 links with the same URI |
| `PDFIdentifier` | 81 of 81 |

The resource's stamp differs from the link's in 184 links; IDML then has
the link's stamp and the resource's size. All 63,470 link chunks and
their resource chunks in the distinct little-endian corpus files parse
with this layout.

**Links resolved to another file at export.** In 19 trustworthy
documents, 529 links differ from the INDD: the IDML names the same file
in another folder (344) or a newer file (185). InDesign wrote the
location and state of the file it found on the exporting computer; the
INDD keeps the state of its last save. These values cannot come from the
INDD, and `compare.py` leaves such links out (`docs/measurement.md`).

**Link times.** IDML writes `LinkImportTime` and
`LinkImportModificationTime` as `YYYY-MM-DDTHH:MM:SS` (seconds
truncated) in the local time of the computer that exported the IDML:

- IDML equals the INDD's UTC time plus a whole number of hours in 9,009
  of 9,385 time values; the other 376 belong to links resolved to
  another file at export or have no stamp.
- The offset differs between documents (−8 to +13 hours) and, in 71 of
  284 documents, between links of the same document in step with the
  season (for example 1 and 2 hours, −5 and −4, +10 and +11), with the
  larger offset on summer dates of that hemisphere: the daylight-saving
  rule of the exporting computer's time zone, which the INDD does not
  store.

The converter estimates the offset from the dates of the XMP packet that
have a UTC offset (`xmp:CreateDate`, `xmp:MetadataDate`,
`xmp:ModifyDate`, every `stEvt:when`): it takes the offset of the date
nearest to the link time in day of the year (difference of the days of
the year, not wrapping at the year end; ties: the latest date), and UTC
when there is no such date. Over the trustworthy pairs this gives
`LinkImportTime` for 3,886 of 4,446 links and
`LinkImportModificationTime` for 3,316 of 4,434 (links with the same URI:
3,748 of 4,118 and 3,019 of 4,105). Other rules give less: UTC about
155 values of 9,385, the offset of `xmp:ModifyDate` 6,488, the same rule
with the day difference wrapping at the year end 3,840 import times.
`compare.py` leaves both times out of the value coverage
(`docs/measurement.md`).

**Embedded links.** A link is embedded (`StoredState="Embedded"`) when its
resource names a raw data object at offset 12. That object holds the
linked file's bytes, which IDML writes as the graphic's `Contents`.

Evidence:

- In the 245 distinct little-endian files there are 883 link resources.
  The field at offset 8 is 2 in 123 of them, and exactly those 123 name a
  raw data object at offset 12. The other 760 have 0 at offset 12 (742
  with 0 at offset 8, 18 with 3).
- In the corpus pairs, all 10 resources of embedded links (12 links) name
  a raw data object, and none of the 369 resources of normal links do.
- For all 12 embedded links, the raw data object's bytes equal the
  base64-decoded `Contents` of the IDML graphic (JPEG, PNG, PDF, EPS and
  SVG files, 2 KB to 11 MB).

**Contained links.** The field at offset 8 is 3 on exactly the 50 links
IDML marks `StoredState="Contained"`: 30 linked stories, 17 stories
placed from Word and 3 linked page items. It is 0 on 2,237 `Normal`
links and 2 on 99 `Embedded` links (the 158 little-endian documents
whose IDML has a story link, a link source ID, style mappings or import
options; 19 more `Normal` links have a UID that holds another object, in
stale pairs).

**Settings shared with page items.** Images, PDF, EPS and SVG graphics
carry the page item setting chunks (page item settings, above): 0x2C10
name, 0x2C32 visibility, 0x21D4E, 0x21D50 and 0x21D53 change counts,
0x1424 overridden properties, 0x22228 layout constraints, and 0x1B916
object style (`AppliedObjectStyle`). Many graphics (2,396 images) lack
0x2C10 and 0x2C32; the defaults of the page item table then apply.
Graphics never have `Locked`. Evidence over the 5,131 graphics of the
trustworthy pairs: `Visible`, `Name` and `AppliedObjectStyle` 5,131 of
5,131; the change counts, `OverriddenPageItemProps` and both layout
constraints 5,079 of 5,079 from DOM 8 on, and absent in all 52 DOM 7
graphics. Images also have the five fill gradient attributes
(`GradientFillStart`, `…Length`, `…Angle`, `…HiliteLength`,
`…HiliteAngle`) from their attribute list (chunk 0x6E03) with the
defaults of page items, and `FillColor` when the list has attribute
0x6E68 (77 of 77 images; no image without it has `FillColor`). PDF, EPS
and SVG graphics have no gradient attributes.

**`ImageTypeName`** is `$ID/` and the format name of the link resource
(links, above), on graphics with a link only: 4,696 of 4,699 linked
graphics; no graphic without a link has it (308 images, 111 PDF, 12
EPS).

**Image properties (chunk 0x1708 of an image).** u32 count, then per
record u32 key, u32 data length *n*, a u8 (meaning unknown), *n* bytes;
then 4 zero bytes:

| Key | Data | Meaning |
|---|---|---|
| 0x64 | u32 | width in pixels |
| 0x65 | u32 | height in pixels |
| 0x67 | u32 | colour space: 1 grey, 2 RGB, 4 CMYK |
| 0x6C | u32, 16.16 fixed point | horizontal resolution (ppi) |
| 0x6D | u32, 16.16 fixed point | vertical resolution |
| 0x6F | colour table | present only for indexed colour |

In all 3,607 images of the trustworthy pairs, `GraphicBounds` width and
height equal pixels × 72 / resolution. All 29,055 image property chunks
of the distinct little-endian corpus files parse. IDML writes these
values only on images with a link (3,299 of 3,299; no image without a
link has them):

| IDML | Rule | Equal |
|---|---|---|
| `Space` | 1 `$ID/#Links_Grayscale`, 2 `$ID/#Links_RGB`, 2 with key 0x6F `$ID/#Links_Indexed RGB`, 4 `$ID/#Links_CMYK` | 3,299 of 3,299 |
| `ActualPpi` | both resolutions rounded half up to integers (72.009 → 72) | 3,299 of 3,299 |
| `EffectivePpi` | each `ActualPpi` value divided by the image's scale on the spread, rounded half up | 3,299 of 3,299 |

The scale on the spread: multiply the `ItemTransform` of the image with
those of all its ancestors (frames, groups, the spread; for an item
anchored in text, its own chain only) to get *a b c d tx ty*. The
horizontal scale is √(a² + b²); the vertical scale is |ad − bc| divided
by the horizontal scale. Using √(c² + d²) for the vertical scale fails on
87 skewed images; rounding half to even fails on one image whose value
is exactly 76.5 (IDML: 77).

**Profile** (`Properties/Profile` of `Image`, chunk 0x7C0F): u32 at 0 is
3 → `$ID/Embedded`, 1 → `$ID/Use Document Default`; 0 or no chunk →
`$ID/None`. A profile name follows as an in-object string. 3,607 of
3,607 images (751, 1,652 and 1,204).

**Vector colour policies of PDF and EPS graphics (chunk 0x7C42).** 16
bytes, four u32. Offset 4 is `RGBVectorPolicy`, offset 12
`CMYKVectorPolicy`: 1 `IgnoreAll`, 3 `HonorAllProfiles`. Offset 0 is 1
and offset 8 is 3 in all 334 graphics with the chunk. Without the chunk
the policies follow the document's colour policies (chunk 0x7C44 of the
preferences, `preferences.md`): `RGBVectorPolicy` is `IgnoreAll` when
`RGBPolicy` is `ColorPolicyOff`, else `HonorAllProfiles`;
`CMYKVectorPolicy` is `IgnoreAll` for `ColorPolicyOff` and
`CombinationOfPreserveAndSafeCmyk`, `HonorAllProfiles` for
`PreserveEmbeddedProfiles` and `ConvertToWorkingSpace`. Evidence: with
the chunk 334 of 334 graphics; without it 1,005 of 1,005 graphics in 128
documents (5 combinations of document policies).

**PDF placement (chunk 0x251B of a PDF).**

| Offset | Size | IDML `PDFAttribute` |
|---|---|---|
| 0 | u32 | `PageNumber` |
| 6 | u8 | `TransparentBackground`: 1 `true`, 0 `false` |
| 8 | u32 | `PDFCrop`: 0 `CropContentVisibleLayers`, 2 `CropArt`, 3 `CropPDF`, 4 `CropTrim`, 6 `CropMedia`, 7 `CropContentAllLayers` |

Evidence: 492 of 492 PDFs with the chunk (28 bytes in 486, 64 in 6),
all three fields. Without the chunk, IDML has `PageNumber="1"`,
`TransparentBackground="true"` and `PDFCrop="CropContentVisibleLayers"`
in 507 of 509 PDFs; the other 2 (one document) have `CropArt`, not
explained. Every PDF has `PDFAttribute` (1,001 of 1,001).

**Graphic layers (chunk 0x177A of images, PDFs and imported pages).**

| Offset | Size | Contents |
|---|---|---|
| 0 | 2 | 1 if an image has a `GraphicLayerOption` |
| 2 | 8 | unknown |
| 10 | 4 | layer count *n* |
| 14 | | *n* layer records, in IDML order |

A layer record is a flagged string (`Name`; a key is written with
`$ID/`), u32 layer ID (`Id`), u32 original and u32 current visibility (1
= visible: `OriginalVisibility`, `CurrentVisibility`), i32 ID of the
parent layer (−1 for a top-level layer), and u32 flags: 0x01
`SeparatorLayer`, 0x04 `FXLayer`, 0x08 `Locked`. A layer whose parent
ID is *p* is a child element of the `GraphicLayer` with `Id` *p*.
`Self` is the graphic's `Self`, `GraphicLayerOption1`, and `i` and the
layer ID in hexadecimal for each layer from the top level down
(`u5dbcGraphicLayerOption1i92i8e` is layer 0x8E inside layer 0x92).

Evidence: all 1,064 `GraphicLayer` elements of the trustworthy pairs
(679 PDF, 291 image and 94 imported page layers, 262 of them nested):
name, `Id`, `Self`, both visibilities and nesting 1,064 of 1,064; flag
0x04 ↔ `FXLayer="true"` 11 of 11; flag 0x08 ↔ `Locked="true"` 37 of 37;
flag 0x01 ↔ `SeparatorLayer="true"` 1 of 1. Flags 0x400 (10 group
layers) and 0x800 (5) have no IDML counterpart. `AdjustmentLayer`,
`HasViewState`, `ViewState`, `HasExportState`, `ExportState`,
`HasPrintState` and `PrintState` are `false` in all 1,064.
`GraphicLayerOption` with `UpdateLinkOption="KeepOverrides"` is on every
PDF and imported page (1,039 of 1,039, also when the layer count is 0)
and on the images whose chunk has 1 at offset 0 (11 of 11; the 1,212
images with 0 there have no element). All 38,087 layer chunks of the
distinct little-endian corpus files from InDesign 4.0 on parse; 172 in
InDesign 3.0 files do not, and the converter leaves their layers out
with a warning.

**Layer comps.** Images with a `GraphicLayerOption` also have
`LayerCompOption AppliedLayerComp`: chunk 0x9209 of the image, i32 at
offset 4 (−1, −2 or a comp number). 11 of 11.

## Movies and sounds (classes 0x14801 and 0x14F01)

Evidence: 21 `Movie` and 8 `Sound` elements in 17 trustworthy pairs
(DOM 8.0: 2 pairs, 10.2: 11, 11.0: 1, 13.1: 1, 18.0: 2), and 128 movies
in 42 files and 42 sounds in 12 files of the corpus (distinct,
little-endian, InDesign 3.0 to 18.2; no big-endian sample).

**Where they are.** A movie or sound is listed in the child list (chunk
0x15B) of a spline item (class 0x6201) and is that frame's only child
(170 of 170 objects). Its own chunk 0x15B names the frame as parent (29
of 29 paired) and lists its poster, if any. IDML writes it as a `Movie`
or `Sound` element, the last child of the frame's `Rectangle` (29 of
29), with `Self` the object's UID. The frame is written as a frame
without a graphic: `ContentType="GraphicType"` (its chunk 0x1623 is 1)
and no `ContourOption` in its text wrap (29 of 29 frames).

**Chunks.**

| Chunk | Contents | IDML | Evidence |
|---|---|---|---|
| 0x151 | matrix | `ItemTransform`; identity without the chunk | 29 of 29 (one sound has no chunk and `1 0 0 1 0 0`) |
| 0x162B | path geometry, as page items | `Properties/PathGeometry` | 29 of 29 |
| 0x1633 | four f64 | `Properties/GraphicBounds` (left, top, right, bottom) | 29 of 29 |
| 0x8CBC | link list, as placed graphics | `Link` | 29 of 29 |
| 0x15B | hierarchy: spread, parent frame, children (the poster) | | 29 of 29 |
| 0x14004 | playback settings (below) | `Name`, `Description`, flags, `ControllerSkin` | 29 of 29 |
| 0x1480C (movie), 0x14F03 (sound) | label at offset 0, as group labels (u16 0x7B7B, u32 count, flagged key and value strings) | `Properties/Label` | 2 movies with one pair match; 6 with count 0 and 21 without the chunk have no `Label` |
| 0x1B916 | object style | `AppliedObjectStyle` | 29 of 29 |
| 0x21D4E, 0x21D50, 0x21D53, 0x1424, 0x22228, 0x2C32 | page item settings | change counts, `OverriddenPageItemProps`, layout constraints, `Visible` | 29 of 29 each |
| 0x3703 | text wrap | `TextWrapPreference` | 29 of 29 (7 with mode 0; 22 without the chunk, all `None` with offsets 0) |
| 0x6E03 | attribute list | `TransparencySetting` | 1 movie with a gradient feather |
| 0x2C10 | name chunk of page items | not used: flag 1 and an empty string where present (7 objects); `Name` comes from chunk 0x14004 | |

Not read: 0x14022 (below), 0x8C39, 0x8C24, 0x1623, 0x2C5D, 0x14009,
0x6E20, 0x6EDD and the other page item chunks that 7 of the 29 paired
objects carry (0x1E206, 0x1E22B, 0xB30A, 0x2056D, 0x2055F, 0x3723).
IDML has no `ObjectExportOption`, `AnimationSetting` or `TimingSetting`
on any of the 29 (all 0x2056D counts are 0 and all 0x2055F are `00
00`).

**Playback settings (chunk 0x14004).** A sequence of lists, each a u32
count and that many entries of u32 key and a value whose type depends
on the list: list 1 u16, list 2 u32, lists 3 to 6 unknown (count 0 in
all 168 chunks that parse), list 7 a flag byte and an in-object string.
The chunk ends after list 7 in 167 of 168; one 10.1 movie has one more
list (count 1, key 0x14009, 16 bytes) with no IDML counterpart. A 3.0
movie uses keys 0x14800 to 0x14804 and another layout. Keys within a
list are in no fixed order. Every u16 value is 1: IDML has `true` when
the key is present and `false` when it is not.

| Key | List | `Movie` | `Sound` | Evidence (paired) |
|---|---|---|---|---|
| 0x14000 | string | `Name`, as stored | `Name` | 21 of 21, 8 of 8 |
| 0x14002 | string | `Description`; `""` without the key | `Description` (no sample with the key) | 1 with the key, 28 without |
| 0x14008 | string | `ControllerSkin`; `""` without the key | | 19 of 19 (11 `SkinOverAll`, 1 empty, 7 without the key) |
| 0x14003 | u16 | `CustomPoster` | | 21 of 21 (14 `true`) |
| 0x14006 | u16 | `PlayOnPageTurn` | `PlayOnPageTurn` | 21 of 21 (3 `true`), 8 of 8 (7 `true`) |
| 0x1400C | u16 | `ShowController` | | 19 of 19 (5 `true`) |
| 0x1400E | u16 | `MovieLoop="true"` and `PlayMode="RepeatPlay"`; without it `false` and `Once` | `SoundLoop` | 21 of 21 (2 with the key), 8 of 8 (5 `true`) |
| 0x14004 | u16 | `EmbedInPDF` (never stored on a movie: `false` in 21 of 21) | `EmbedInPDF` | 8 of 8 sounds have it and `true` |
| 0x1400D | u16 | | `StopOnPageTurn` | 8 of 8 (2 `true`) |
| 0x140FF | u16 | not identified (1 movie; no attribute differs) | | |
| 0x14007 | u32 | not identified (0 in 139, 5485 in one unpaired movie) | | |
| 0x140FE | u32 | not identified (5 in all 170) | | |

The 14 movies with `CustomPoster="true"` have a poster without a link,
the 7 with `false` a linked poster; the converter takes the key. A chunk
whose lists 3 to 6 are not empty gives the flags without the strings
and a warning; a damaged chunk gives the defaults and a warning.

**Values without a stored field.** These are the same in every IDML of
the corpus with these elements (19 files, paired or not: DOM 8.0, 10.1,
10.2, 11.0, 13.1, 18.0). No stored field was found for them; the
converter writes them as defaults.

| Element | Attribute | Value | Samples |
|---|---|---|---|
| `Movie` | `FloatingWindow` | `false` | 27 of 27 |
| `Movie` | `FloatingWindowPosition` | `Center` | 27 of 27 |
| `Movie` | `FloatingWindowSize` | `Full` | 27 of 27 |
| `Movie` | `ShowControls` | `false` | 26 of 26 (DOM 8.0 to 13.1) |
| `Sound` | `DoNotPrintPoster` | `false` | 13 of 13 |
| both | `LocalDisplaySetting` | `Default` | 40 of 40 |

`ShowControls`, `ControllerSkin` and `ShowController` are on all 26
movies of DOM 8.0 to 13.1, also on the 7 whose chunk has neither key
0x14008 nor 0x1400C, and on none in the one DOM 18.0 file. They follow
the version, not the stored keys. No IDML of DOM 14 to 17 has a movie;
the converter writes them below DOM 18.

**`FilePath`** is the `LinkResourceURI` of the media's own link as a
native path: without `file:`, percent-decoded as UTF-8, and with `\`
for `/` in a path that starts with a drive letter (`C:/`). From the
IDML's own link URI this gives `FilePath` in 29 of 29 (25 Windows
paths, 4 Mac paths). From the INDD's link it gives 8 of 29: the other
21 links were resolved to another file on the exporting computer, and
the IDML has that file's path in both `Link` and `FilePath`. A media
object without a link has no `FilePath`.

**Children.** IDML order (29 of 29): `Properties` (`PathGeometry`,
`GraphicBounds`, then `Label`), `TransparencySetting`,
`TextWrapPreference`, the poster `Image`, `Link`.

- Text wrap: as for placed graphics (chunk 0x3703; without it mode
  `None` and offsets 0), 29 of 29. `ContourOption` is in the wrap of
  every movie and sound with a poster (26 of 26) and of none without (3
  of 3), as a frame has one only with a graphic. All 26 are
  `SameAsClipping`, `false`, `$ID/`; no media object has chunk 0x373D,
  and the poster's contour and the preferences' contour agree in all
  26, so either can be the source. The converter uses the poster's.
- Poster: 125 of 128 movies and 9 of 42 sounds list one child, an image
  (class 0x1702). IDML writes it as an `Image` child (26 of 26 paired),
  like any image: 12 linked, 14 with their file in the document
  (`Properties/Contents`). Its `EffectivePpi` uses the transforms of the
  image, the movie or sound, the frame and their ancestors (12 of 12
  linked posters).
- `Link`: the layout of image links (chunks 0x8C9B and 0x8C92). For the
  8 links whose URI is the same in INDD and IDML, `LinkResourceFormat`
  (`$ID/Flash H264 Video`, `$ID/MP3`, `$ID/Flash Video`, `$ID/SWF
  (Shockwave Flash)`), `LinkResourceSize` and `LinkImportStamp` match 8
  of 8.

**Media file properties (chunk 0x14022).** The same kind of typed list,
with list types u16, u32, u64, f64, four f64 and flagged string (29 of
29 paired chunks end after the sixth list). Keys seen: 0x14002 and
0x14004 (u16); 0x140FE, 0x14F00 (sounds: 44100, the sample rate) and
0x14F02 (sounds: 128) (u32); 0x14000 (u64); 0x14006 (f64, movies: the
duration in seconds); 0x14001 (four f64, movies: the frame size, for
example 0 0 640 640); 0x14003 and 0x14005 (strings; 0x14005 of sounds
is an older import path that matches neither link URI). IDML writes
none of these: no IDML in the corpus has `IntrinsicBounds`,
`CanChoosePosters` or `PosterAvailable`, a `NavigationPoint` or a
`Movie` `URL`.

**References.** `MovieBehavior/@MovieItem` (6 in 5 pairs) and
`SoundBehavior/@SoundItem` (2 in 1 pair) name a `Movie` or `Sound` of
the same IDML (8 of 8); see button behaviours. `TimingTarget` elements
with `TargetRole` 132354 (3 movies) and 132356 (7 sounds), in 9 pairs,
name a written `Movie` or `Sound` in 10 of 10; see timing lists.

## Text on a path (class 0xB320)

A shape or text frame with text on its path names a text path object
in chunk 0xB30A (600, 392 or 336 bytes). Items without text on their
path can have the chunk too, with 0 in the first u32.

| Offset in 0xB30A | Contents | IDML `TextPath` |
|---|---|---|
| 0 | u32 text path UID (class 0xB320), 0 for none | `Self` |
| 4 | u32, the same UID | |
| 8 | u16 1, u16 4, u32 3 in all 41 | `PathAlignment="CenterPathAlignment"`, `TextAlignment="BaselineTextAlignment"`, `PathEffect="RainbowPathEffect"`, `PathSpacing="0"` (which field is which is not known) |
| 16 | u8: 1 `Flipped`, 0 `NotFlipped` | `FlipPathEffect` |
| 18 | f64 | `StartBracket` |
| 26 | f64 | `EndBracket` |

The text path object has chunk 0xB334: u32 (its own UID), u32 0, then a
UID list with one multi-column frame (class 0x263). That frame's 0x15B
child list holds a column of class 0xB318, which gives the story and
the thread as the column of a text frame does (text frames, below):
`ParentStory`, `PreviousTextFrame` and `NextTextFrame` (`n` in every
sample).

IDML writes `TextPath` as a child of its shape or frame, directly after
`Properties`, with the attributes in the order of the table (then the
two thread attributes). The converter writes the four constant values
only when the bytes at 8 are those of every sample.

Evidence: 42 `TextPath` elements with an INDD object in the pairs (36
in trustworthy pairs; parents: polygons, an oval and text frames), all
values equal in 42 of 42, both `FlipPathEffect` values occurring.

## EPS text (class 0x660B)

Text kept from a placed EPS or PDF graphic as text that cannot be
edited. IDML writes it as an `EPSText` page item. It is listed in the
0x15B child list of a spread layer or a group (483 of 634 are in
groups), and has the usual page item chunks (0x151 transform, 0x6E03
attributes, 0x1B916 object style, 0x3703 text wrap, 0x2C10 name, …) but
no path.

| Chunk | Contents | IDML |
|---|---|---|
| 0x151 | Matrix | `ItemTransform` |
| 0x154 | Four f64: left, top, right, bottom | `Properties/PathBoundingBox` `Left`, `Top`, `Right`, `Bottom` |
| 0x6612 | 60 bytes: four f64 in the same order, then f64 72 and zeros (not identified) | `Properties/EPSTextAttributeBounds` `Top`, `Left`, `Bottom`, `Right`, from the four f64 |
| 0x6611 | Text record (below) | `Properties/EPSTextData` |

**Text record (chunk 0x6611).** u32 version (0x00010002 in all 634), a
flagged in-object string (the font name), nine f64 (a matrix of font
size, 0, 0, font size, x, y, then three numbers), a flagged in-object
string (the text) and a tail of 210 bytes: u32 at 0 (16 in all), zeros
to 72, u32 at 72 (varies), u16 at 76 (0 or 1), u32 at 78 (14 in all),
zeros to 210.

IDML `EPSTextData` is this record in big-endian byte order, without the
u32 at 72 and at 78 of the tail, in base64 in lines of 76 characters:
each number big-endian, each string as in big-endian data (tag 2, flag,
the byte after the tag, u16 length, each segment header big-endian,
byte segments copied and UTF-16 code units big-endian), and the zero
areas copied. So `EPSTextData` is 8 bytes shorter than the chunk.

IDML writes `Properties` (`PathBoundingBox`, `EPSTextData`,
`EPSTextAttributeBounds`) and then `TextWrapPreference`; `EPSText` has
no `ContentType`, no path and no export options. Its other attributes
are those of other page items (page item settings, above); the values
that every `EPSText` has come from `element_values.xml`.

Evidence: 634 `EPSText` elements in 14 pairs (633 in 13 trustworthy
pairs), versions 8.1 to 20.5. The transform, both bounds, the text data
and the text wrap equal the IDML in 634 of 634. `StrokeType` (634),
`OverprintFill` (33) and `StrokeDashAndGap` (7) are not reproduced yet:
IDML writes `StrokeType="StrokeStyle/$ID/Solid"` on 627 items whose
attribute list has no stroke type.

## Form fields and multi-state objects

Seven classes are page items that IDML writes as interactive elements:

| Class | IDML | Elements (documents) |
|---|---|---|
| 0x1450C | `Button` | 284 (43) |
| 0x1450D | `CheckBox` | 72 (5) |
| 0x1450E | `RadioButton` | 11 (4) |
| 0x1450F | `TextBox` | 27 (4) |
| 0x14511 | `ComboBox` | 1 (1) |
| 0x14512 | `SignatureField` | 2 (2) |
| 0x145D8 | `MultiStateObject` | 18 (16) |

Counts are over the pairs of the corpus of 2026-10: 48 documents, all
trustworthy, saved by InDesign 8 to 20. Each element's `Self` is the
object's UID (415 of 415). A field is listed like any page item, in the
child list of its spread, group or anchor, and its fields and states
nest (a button inside a group inside a multi-state object's state).

**Fields with states** (button, check box, radio button, multi-state
object) have no chunk 0x15B and no 0x151:

- Chunk 0x14521: u32 active state ID, u32 count, then per state u32
  state ID, u16 enabled (1, or 0 for `Enabled="false"`), u16 key, u16
  (0). IDML writes one `State` per entry, in order (705 of 705), with
  `Self` the field's `Self`, `i` and the state ID in hexadecimal,
  `Active="true"` for the active state ID and `Enabled` from the u16
  (705 of 705; 4 not enabled).
- Chunk 0x14526: u32 spread, u32 parent, u32 count, then per key u32
  key, u32 *k*, *k* UIDs: the items each state shows, by its key (one
  group per state in 705 of 705). The chunks parse to their end in all
  385 fields. The spread decides `AllowOverrides` as chunk 0x15B does
  for other items.
- Chunk 0x14527: six f64, the transform (`ItemTransform`, 385 of 385).

**State types.** `Properties/Statetype` follows from the state ID. For
a multi-state object it is the ID itself (`type="long"`, IDs 9 to 15;
72 of 72 states). For the others it is an enumeration (633 of 633):

| ID | `Statetype` | States | Name of the state's group without a name of its own |
|---:|---|---:|---|
| 0 | `Up` | 284 | `$ID/$$$/StateType/Normal` |
| 1 | `Rollover` | 109 | `$ID/$$$/StateType/Over` |
| 2 | `Down` | 61 | `$ID/$$$/StateType/Down` |
| 3 | `UpOn` | 83 | `$ID/$$$/StateType/NormalOn` |
| 4 | `UpOff` | 83 | `$ID/$$$/StateType/NormalOff` |
| 6 | `RolloverOff` | 7 | `$ID/$$$/StateType/OverOff` |
| 7 | `DownOn` | 3 | `$ID/$$$/StateType/DownOn` |
| 8 | `DownOff` | 3 | `$ID/$$$/StateType/DownOff` |

The key is not the state type: a button's second state has key 2
whether it is `Rollover` (ID 1) or `Down` (ID 2). ID 5 has no sample; a
state with an ID not in the table is left out with a warning. A state's
group has no name chunk (0x418), or one with flag 1 and an empty name;
IDML names it by the state type, as in the table (633 of 633 groups of
buttons, check boxes and radio buttons). The groups of multi-state
objects have names of their own (72 of 72).

**Fields with child items** (text box, combo box, signature field) have
the usual chunks 0x151 (transform) and 0x15B, whose children IDML writes
inside the element: a `Rectangle` or `TextFrame` (30 of 30). Text boxes
can be anchored in text, with the anchor's `AnchoredObjectSetting`.

**`Properties/PathBoundingBox`.** The bounds of the paths of the items
the field shows, in the field's own coordinates: the curves through the
anchor and direction points (not only the anchors) of the items of every
state, or of the child items, each through its transform and those of
the groups and fields between it and the field. Items inside a frame
(an oval inside an oval) do not count. The converter takes each item's
box in its own coordinates and the box around its corners through its
transform, from the innermost item out: 415 of 415 equal to 6
significant digits, 379 of them as text (the others differ in the last
digit or two). The field has no `PathGeometry`.

**Attributes from the attribute list (chunk 0x6E03).** Strings have value
type 0x1451F or 0x1B02 (a flag byte and an in-object string), numbers
0x6E68 (f64), flags 0x6E65 (u16).

| ID | IDML | On | Evidence |
|---|---|---|---|
| 0x14534 | `Name` | all | 415 of 415, 237 distinct |
| 0x14535 | `Description` | all but multi-state objects | 397 of 397 (23 distinct); 3 multi-state objects have `Description=""` without it |
| 0x1453D | `Required` (1 `true`) | check box, radio button, text box, combo box, signature field | 113 of 113 (6 `true`); buttons have the attribute but IDML writes no `Required` |
| 0x14550 | `ExportValue` | check box, radio button | 83 of 83 (9 distinct) |
| 0x14552 | `Multiline` | text box | 27 of 27 (8 `true`) |
| 0x14553 | `Scrollable` | text box | 27 of 27 (9 `true`) |
| 0x14540 | `FontSize` | text box, combo box | 28 of 28 (4 distinct); signature fields have it, IDML writes none |
| 0x1455F | `AppliedFont` | text box | 24 of 24 (2 distinct); the 3 text boxes without it have neither `AppliedFont` nor `FontStyle` |
| 0x14542 | `FontStyle` | text box with 0x1455F | 24 of 24 |

The flags `ReadOnly`, `PrintableInPdf`, `CheckedByDefault`, `Password`,
`RightToLeftField`, `SortChoices`, `HiddenUntilTriggered`,
`InitiallyHidden` and `VisibilityInPdf` have one value in every sample,
and so do the candidate u16 attributes (0x14539 to 0x1453C, 0x14545,
0x14549, 0x14551, 0x14554 to 0x14558, 0x145E1); which holds which cannot
be shown. Where every IDML has the value, it comes from the value files
(`idml-values.md`, form fields). `ChoiceList` of the combo box (0x1455D,
one sample) is not decoded.

**Values of the page item.** `Visible`, `Locked`, the change counts,
the layout constraints and `ItemLayer` follow the page item rules
(page item settings). The gradient attributes are, as for a group,
the values all the items the field shows directly have (the state
groups or the child items), and IDML leaves an attribute out where they
differ: the six gradient attributes are equal in 400 of 415 fields (4
of them without the start and length values). The fill, stroke and corner values in
the field's attribute list are not IDML's: IDML's `FillColor` and
corner values of buttons are not in the list, and other stored values
differ, so they are not written. Fields with states have no object
style chunk (0x1B916); IDML names `[None]` for 363 of their 385 and
another style for 22, and where that is stored is not known, so
`AppliedObjectStyle` is left out for them, and so are the anchor
settings of such a field anchored in text, which IDML compares with the
style (12 anchored check boxes and radio buttons with a style other
than `[None]` have no `AnchoredObjectSetting`). A field has no
`ObjectExportOption`.

Timing lists of buttons and animations: animation and timing, below.

Behaviours of buttons: button behaviours, below.

### Button behaviours

Evidence: 206 behaviour elements in 193 buttons of 26 pairs (all
trustworthy, versions 8.0 to 18.0), and 547 behaviour objects in 78
documents of the corpus (InDesign 3.0 to 18.2, both byte orders). The
converter writes every behaviour type that has a paired sample, and
`compare.py` finds all their values in its output.

**Behaviour list (chunk 0x15601).** A form field lists its behaviours
by event:

```
u32 groups
  per group: u32 event code, u32 count k, k behaviour UIDs
```

- The chunk parses to its end in 712 of 712 fields (all field classes;
  271 have 0 groups). Only buttons have groups (441 buttons).
- Every behaviour object is listed in exactly one group of one field
  (547 of 547). No group is empty.
- IDML writes the behaviours as children of the button, in group order
  and, within a group, in stored order (193 of 193 buttons). In the five
  buttons with two groups the stored order is also ascending event
  code, so these two orders cannot be told apart.
- They are the last children of the button, after every `State` (193 of
  193). The schema allows them in any order there.

| Event code | `BehaviorEvent` | Paired behaviours |
|---|---|---:|
| 0x15602 | `MouseDown` | 32 |
| 0x15603 | `MouseUp` | 145 |
| 0x15604 | `MouseEnter` | 29 |

Unpaired groups also have 0x15605 (31) and 0x15607 (1). Their events
are not known: the behaviour is written without `BehaviorEvent`, with a
warning.

**Behaviour objects.** Each behaviour type is a class. IDML `Self` is
the object's UID (206 of 206); timing targets of role 132353 reference
these UIDs (53 of 53 resolve in the converter's output). The object has
one data chunk, and some also chunk 0x156E1, a label with the layout of
the document label. Chunk 0x156E1 has count 0 in all 112 occurrences,
and no IDML behaviour has `Properties/Label`; a label with pairs would be
written there (no sample).

| Class | IDML element | Data chunk | Type code | Objects (paired elements) |
|---|---|---|---|---:|
| 0x15613 | `GotoFirstPageBehavior` | 0x15614 | 0x15601 | 10 (7) |
| 0x15614 | (go to last page, no pair) | 0x15619 | 0x15602 | 2 (0) |
| 0x15615 | `GotoNextPageBehavior` | 0x1561A | 0x15603 | 88 (21) |
| 0x15616 | `GotoPreviousPageBehavior` | 0x1561B | 0x15604 | 37 (24) |
| 0x1561B | `GotoURLBehavior` | 0x15622 | 0x15605 | 45 (11) |
| 0x1562C | `MovieBehavior` | 0x1562C | 0x15606 | 99 (6) |
| 0x15636 | `ShowHideFieldsBehavior` | 0x15634 | 0x15607 | 29 (26) |
| 0x15664 | `ClearFormBehavior` | 0x15650 | 0x15609 | 3 (1) |
| 0x1566E | `SubmitFormBehavior` | 0x1565A | 0x1560A | 5 (1) |
| 0x15678 | `OpenFileBehavior` | 0x15664 | 0x1560B | 1 (1) |
| 0x156D3 | `GotoAnchorBehavior` | 0x156F0 | 0x15617 | 27 (18) |
| 0x156DD | `GotoNextViewBehavior` | 0x15703 | 0x15619 | 12 (12) |
| 0x156E8 | (view zoom, no pair) | 0x1570A | 0x1561A | 5 (0) |
| 0x156F4 | `SoundBehavior` | 0x1571C | 0x1561B | 7 (2) |
| 0x15625 | `GotoPageBehavior` | 0x156DE | 0x1561C | 54 (30) |
| 0x1562F | (animation, no pair) | 0x1566C | 0x1561D | 16 (0) |
| 0x1563F | `GotoStateBehavior` | 0x1566D | 0x1561E | 80 (37) |
| 0x15643 | `GotoNextStateBehavior` | 0x1567D | 0x1561F | 17 (4) |
| 0x15660 | `GotoPreviousStateBehavior` | 0x15680 | 0x15620 | 8 (4) |
| 0x156F9 | `PrintFormBehavior` | 0x15721 | 0x15621 | 2 (1) |

The class and the element match in 206 of 206 paired behaviours. The
type code is constant per class in 547 of 547 objects. The three
classes without a pair are named by their built-in name keys
(`$$$/Forms Action/GoTo Last Page`, `…/View Zoom`, `…/Animation`); the
converter leaves them out with a warning.

**Data chunk.** All data chunks start with a common header:

| Field | Contents | Evidence |
|---|---|---|
| u32 | UID of the field that owns the behaviour | 206 of 206 equal to the IDML parent button |
| u32 | type code (table above) | constant per class, 547 of 547 |
| flag byte, in-object string | the name: flag 1 and a built-in key `$$$/Forms Action/…` | flag 1 in 547 of 547 |
| u16 | `EnableBehavior` (1 `true`) | 1 in 547 of 547; all 206 paired are `true`. That 0 is `false` is assumed |

The type's fields follow (all u32 unless stated otherwise); the chunk
ends after them in 546 of 547 objects. The exception is a 3.0 movie
behaviour that ends after the operation. Big-endian objects (2 files of
8.0, 3 files of 4.0) parse to their end with the flag and string tag
swapped (`big-endian.md`).

| Type | Fields | IDML | Evidence |
|---|---|---|---|
| first, last, next, previous page, next view | navigation kind (0 first, 1 last, 2 previous, 3 next, 5 next view), zoom | `ZoomSetting` from the zoom | kind constant per class, 203 of 203 objects; zoom 64 of 64 |
| go to page | navigation kind 6, page index, zoom | `PageNumber` = index + 1, `ZoomSetting` | 30 of 30 (pages 1 to 6) |
| go to anchor | flag byte (0 in 27 of 27), in-object string, anchor u32, zoom | `FilePath`, `ZoomSetting`, `Properties/AnchorItem` (`type="long"`) | 18 of 18 |
| go to URL | flag byte, in-object string | `URL` | 11 of 11, plain and `navto://` URLs |
| open file | flag byte, in-object string | `FilePath` | 1 of 1 |
| submit form | u32 (0 in 5 of 5), flag byte, in-object string | `URL` | 1 of 1 |
| clear form, print form | none | | 1 and 1 |
| show/hide fields | count n, n × (field UID, action: 0 hide, 1 show) | `FieldsToHide`, `FieldsToShow` | 26 of 26 |
| go to state, next state, previous state | multi-state object UID, kind (0, 1, 2; constant per class, 105 of 105), u16 (0 in 105), u16 (1 in 45 paired, 0 in 3 unpaired), i32 state ID (-1 for next and previous) | `AssociatedMultiStateObject`; go to state `Properties/Statetype` (`type="long"`) = state ID | 45 of 45; state IDs 37 of 37 |
| movie | movie UID, operation (0 `Play`), i32 navigation point | `Operation`, `NavigationPointID` | 6 of 6 (all -1) |
| sound | sound UID, operation (0 `Play`, 1 `Stop`), f64 (1.0 in 5, 0.0 in 2 of 7), u32 (0 in 7) | `Operation` | 2 of 2 |
| view zoom (no pair) | u32 (0 in 5) | | |

- Zoom codes: 0 `InheritZoom` (every page navigation behaviour and 8
  anchors), 2 `FitWindow` (10 anchors). No other code occurs. Movie
  operation 3 (2 unpaired objects) and sound operation 2 (1) are not
  known. An unknown code leaves the attribute out, with a warning.
- `FieldsToHide` lists the UIDs with action 0 and `FieldsToShow` those
  with action 1, each in stored order, as references separated by
  spaces. With no such UID the attribute is written empty. The action
  is 0 or 1 in all 149 corpus entries.
- `AnchorItem` is the stored u32 when `FilePath` is not empty (5 of 5,
  anchors in another file). When `FilePath` is empty it is 0 (13 of
  13): the stored u32 is 0 in 8, and in 5 it is a UID with no object.
- In the converter's output the button (206 of 206) and every
  `AssociatedMultiStateObject` (45 of 45), `FieldsToHide` UID (124 of
  124) and `FieldsToShow` UID (25 of 25) exist as `Self`.
- `MovieItem` and `SoundItem` are the stored movie and sound UIDs: each
  names a `Movie` or `Sound` of the same IDML (6 and 2 of 6 and 2;
  movies and sounds). A UID that is not a written movie or sound is
  left out.
- The sound's f64 and last u32 and the submit u32 have no IDML
  attribute.

**Values with no stored field.** These are the same in every sample,
and no stored field was found for them. The converter writes them as
defaults:

| IDML | Value | Samples |
|---|---|---|
| `GotoAnchorBehavior` `AnchorName` | `$ID/` | 18 of 18 |
| `GotoStateBehavior` `GoBackOnRollOff` | `false` | 37 of 37 |
| `GotoNextStateBehavior`, `GotoPreviousStateBehavior` `LoopsToNextOrPrevious` | `true` | 8 of 8 |

The second u16 of a state behaviour is 1 in all paired samples, which is
consistent with `LoopsToNextOrPrevious="true"` but does not prove it.

**`Name`.** No behaviour has a name of its own: the name is a built-in
key with flag 1 in all 547 objects. IDML writes the key's name in the
language of the exporting InDesign, which follows the saving edition
(the XMP black name, saving edition): in all 26 pairs the edition
matches the language of every behaviour name (1 English, 7 Korean and
18 Chinese documents). The converter writes the names below and leaves
`Name` out for a key or edition without a sample. A name with flag 0
(no sample) would be written as stored.

| Key (`$$$/Forms Action/…`) | English | Korean | Chinese |
|---|---|---|---|
| `GoTo Anchor` | `Go To Destination` (10) | `대상으로 이동` (8) | |
| `GoTo First Page` | | `첫 페이지로 이동` (3) | `跳至第一頁` (4) |
| `GoTo Next Page` | | `다음 페이지로 이동` (1) | `跳至下一頁` (20) |
| `GoTo Previous Page` | | `이전 페이지로 이동` (1) | `跳至上一頁` (23) |
| `GoTo Next View` | | `다음 보기로 이동` (12) | |
| `GoTo Page` | | | `跳至頁面` (30) |
| `GoTo URL` | | | `前往 URL` (11) |
| `SwitchToMSOState` | | `상태로 이동` (13) | `跳至狀態` (24) |
| `SwitchToNextMSOState` | | `다음 상태로 이동` (1) | `跳至下一個狀態` (3) |
| `SwitchToPrevMSOState` | | `이전 상태로 이동` (1) | `跳至上一個狀態` (3) |
| `Movie` | | | `影片` (6) |
| `Sound` | | | `聲音` (2) |
| `Show/Hide Fields` | | | `顯示/隱藏欄位` (26) |
| `Open File` | | | `開啟檔案` (1) |
| `Submit Form` | | | `送出表格` (1) |
| `Reset Form` | | | `清除表格` (1) |
| `Print Form` | | | `列印表格` (1) |

The Chinese names are Traditional Chinese. Of the 93 trustworthy pairs
with the black name `黑色`, 92 name their paper swatch `紙張`
(Traditional Chinese) and one has no paper swatch; the corpus has no
Simplified Chinese saving edition.

**Attribute order.** The type's attributes first, in schema order, then
`Name`, `EnableBehavior` and `BehaviorEvent`. Example:
`<GotoPageBehavior Self="u762" ZoomSetting="InheritZoom" PageNumber="1" Name="跳至頁面" EnableBehavior="true" BehaviorEvent="MouseUp" />`.

**Not shown.** Behaviours of other field types (the schema allows most
types on check boxes, radio buttons, text boxes, combo boxes, list boxes
and signature fields; the corpus has none, and the converter leaves them
out with a warning); `AnchorItem` of a live anchor in the same document;
URLs stored with flag 1 (6 unpaired objects, key
`$$$/Dialog/Behaviors/GoToView/DefaultURL`, and one empty submit URL),
which the converter leaves out.

## Animation and timing

Spreads and buttons keep timing lists, page items an animation, and the
document a list of motion presets. Evidence: the 43 pairs of the corpus
whose IDML has any of these elements (little-endian, versions 8.0 to
18.0; all trustworthy). The layouts are the same in all versions. A
sweep of the first occurrence of each chunk in the spreads, buttons,
spline items, groups and preferences of all 1,251 trustworthy pairs
found non-empty timing lists, preset lists or animations only in these
43 documents.
The converter writes all of them by the rules below, and `compare.py`
finds all 3,783 of their values in its output for these 43 pairs.

Strings below:

- **Text string**: u32 length in UTF-16 code units, then text segments.
- **Name**: a flag byte (1 = built-in key, `$ID/` in IDML) and an
  in-object string.

### Timing lists (chunks 0x2056D and 0x2056E)

Spreads keep their timing lists in chunk 0x2056D, buttons (class
0x1450C) in chunk 0x2056E. Most page items also have chunk 0x2056D,
with a count of 0.

```
u32 lists
  per list:   u32 trigger, u32 groups
    per group:  u32 targets
      per target: u32 UID, u32 role, u32 action, u32 (0 in all 163), f64 delay
    then:       u32 plays, u16 (0 in all 159)
```

The chunk parses to its end in 96 of 96 owners. IDML writes a
`TimingSetting` exactly when the list count is not 0: 43 of 43 spreads
and 53 of 53 buttons with lists have one; 20 spreads and 119 buttons
with count 0 have none.

| IDML | Source | Evidence |
|---|---|---|
| `TimingList` `Self` | owner `Self` + `TimingSetting1TimingList` + list index (from 0) | 96 of 96 |
| `TriggerEvent` | trigger code: 1 `OnPageLoad`, 5 `OnClick`, 6 `OnRelease`, 7 `OnRollover` | 43 / 2 / 45 / 6; spreads all 1 |
| `TimingGroup` `Self` | list `Self` + `TimingGroup` + group index (from 0) | 159 of 159 |
| `Plays` | u32 plays | 159 of 159 (all 1) |
| `TimingTarget` `Self` | group `Self` + `i` + target UID in hex | 163 of 163 |
| `DynamicTarget` | `u` + target UID in hex | 163 of 163 |
| `DelaySeconds` | f64 delay | 163 of 163 (0 and 0.25) |
| `TargetRole` | u32 role, in decimal | 163 of 163 |
| `TargetAction` | u32 action, in decimal | 163 of 163 (all 132353) |
| `Placement` | target index in its group (from 0) | 163 of 163 |

Roles by target class: 132353 button behaviours (classes 0x1562C,
0x1563F, 0x15643, 0x15660, 0x156F4; 53 targets), 132354 movies (3),
132355 spline items, groups and multi-state objects (100), 132356
sounds (7). The stored value is written as is.

Every target of the samples names an element the converter writes:
a page item, a button behaviour, or a movie or sound (7 sound and 3
movie targets of 163; movies and sounds). The converter leaves out a
target whose element it does not write, with a warning, and numbers
`Placement` over the targets kept. A group or list left empty is left out too: no sample has
an empty one (159 groups, 96 lists).

Example, a spread with one list, one group and one target:
`01 00 00 00 | 01 00 00 00 01 00 00 00 | 01 00 00 00 | 63 04 00 00 03 05 02 00 01 05 02 00 00 00 00 00 | 00 00 00 00 00 00 00 00 | 01 00 00 00 00 00`.

IDML writes the spread's `TimingSetting` after `FlattenerPreference`
and before the pages, and the button's after `TextWrapPreference` and
before the `State` elements (53 of 53). Multi-state objects have chunk
0x2056D with count 0 in all samples; where their timing lists would be
stored is not shown.

### Motion presets (class 0x20517)

Chunk 0x2053E of the preferences object (class 0x2202): u32 *n*, then
*n* preset UIDs. IDML writes one `MotionPreset` in designmap.xml per
UID, in this order, after the `NamedGrid` elements (63 presets in 30
documents; 5 more documents have the chunk with *n* = 0 and no
`MotionPreset`).

Chunk 0x20532 of a preset:

| Field | IDML |
|---|---|
| text string | `Properties/Contents`, as is (63 of 63; IDML writes it as CDATA) |
| 4 bytes | not mapped; they vary without pattern (`00 00 00 00`, `5C 00 50 00`, …) |
| name | empty, flag 1 in all 63 |
| name | `Name`: `$ID/` and the string for flag 1 (63 of 63, all flag 1) |

The chunk parses to its end in 63 of 63. The stored name is not always
the one inside the preset's XML (`appear` stored, `Appear` in the XML's
`<name>`); IDML takes the stored name (2 of 2).

### Animation settings (chunk 0x2055F)

Every spline item (class 0x6201), group (0x401), button (0x1450C) and
multi-state object (0x145D8) has chunk 0x2055F, and so do placed
graphics. Two bytes `00 00` mean no animation: IDML writes no
`AnimationSetting` (1,996 of 1,996 objects with this chunk in the 43
documents, items and graphics). A longer chunk means an animation, and
IDML writes `AnimationSetting` (100 of 100: 57 rectangles, 22 text
frames, 13 ovals, 6 multi-state objects, 2 groups). Longer chunks also
occur in 12 objects that are not written as page items (classes 0x129,
0x1D6, 0x301, 0x3E03, 0x8C41, 0x8C4B, 0xCA8B).

| Offset | Type | IDML | Evidence |
|---:|---|---|---|
| 0 | u16 | not mapped | 1 in 100 of 100 |
| 2 | u16 | `HasCustomSettings`: 1 `true`, 0 `false` | 100 of 100 (20 `true`) |
| 4 | u32 | `Properties/Preset type="object"`: the preset UID | 100 of 100 |
| 8 | f64 | `Duration`, in seconds | 100 of 100 (1, 0.5833…, 1.0416…) |
| 16 | u32 | not in IDML: 24 (frames per second) | 100 of 100 |
| 20 | u32 | not in IDML: frames, `Duration` × 24 | 100 of 100 |
| 24 | u32 | `DesignOption`: 0 `FromCurrentAppearance`, 2 `ToCurrentLocation` | 44 / 56 |
| 28 | u32 | not mapped: 0 in all | 100 of 100 |
| 32 | u32 | `Plays` | 100 of 100 (87 × 1, 13 × 2) |
| 36 | u16 | not mapped: 0 in all | 100 of 100 |
| 38 | u32 | `EaseType`: 0 `NoEase`, 4 `CustomEase` | 3 / 97 |
| 42 | 2 × f64 | `TransformOffsets`, each rounded to 6 significant digits | 100 of 100 (`0.499924 0.499886`) |
| 58 | | motion path key frames, motion path points, properties (below) | |

From offset 58:

```
u32 n             key frames of the motion path
  per key frame:  u32 frame, u32 (2 in all 155), f64 x, f64 y
u32 m             points of a motion path without key frames
  per point:      u32 (2 in all 4), f64 x, f64 y
u32 p             animated properties
  per property:   text string (name), text string (the same name again),
                  u32 k keys, per key: u32 frame, u32 (6 in all 528), f64 value
```

The chunk parses to its end in 100 of 100.

- `Properties/MotionPath`: one `AnimationDataPathKeyFrameType
  KeyFrame="frame"` per key frame, with `PathPoint Anchor="x y"
  LeftDirection="x y" RightDirection="x y"` (98 of 98 with *n* > 0;
  left and right direction equal the anchor in all 155 points). No
  `MotionPath` when *n* = 0 (2 of 2).
- `Properties/MotionPathPoints`: when *m* > 0, a `PathPointArray` with
  one `PathPointType` per point, anchor and both directions `x y` (2 of
  2; both have *n* = 0).
- Properties by name: `Alpha_Amount` → `OpacityArray`, `Rotation_Z` →
  `RotationArray`, `Scale_X` → `ScaleXArray`, `Scale_Y` → `ScaleYArray`;
  each key is `AnimationDataKeyFrameType KeyFrame="frame" Value="value"`
  (all 396 arrays). A property with no keys has no element (1 of 1),
  and neither has a property that is not stored (4 items have no
  `Alpha_Amount`, and no `OpacityArray`). The second name equals the
  first in 396 of 396.
- `Properties` children are in the order `Preset`, `MotionPath` or
  `MotionPathPoints`, `OpacityArray`, `RotationArray`, `ScaleXArray`,
  `ScaleYArray` (100 of 100).

Numbers other than `TransformOffsets` are in the shortest form
(`764.0000000000001`, `-552.6140386840932`, `0.5833333333333334`; all
100 elements). IDML writes `AnimationSetting` after
`ObjectExportOption` and before child items and placed graphics, and
after the `State` elements of a multi-state object (100 of 100).

### Values without a stored field

These attributes have one value in every sample, and no stored field
was found for them. The converter writes that value:

| Attribute | Value | Samples |
|---|---|---:|
| `TimingGroup/@PlaysLoop` | `false` | 159 |
| `TimingTarget/@ReverseAnimation` | `false` | 163 |
| `AnimationSetting/@PlaysLoop` | `false` | 100 |
| `AnimationSetting/@InitiallyHidden` | `false` | 100 |
| `AnimationSetting/@HiddenAfter` | `false` | 100 |
| `MotionPreset/@EditLocked`, `@DeleteLocked`, `@NameLocked` | `false` | 63 each |
| `MotionPathPoints/@PathOpen` | `true` | 2 |

Candidate fields are 0 in every sample: in a timing target the u32
after the action (`ReverseAnimation`?), after a group's plays the u16
(`PlaysLoop`?), in chunk 0x2055F the u32 at 28 (`InitiallyHidden` and
`HiddenAfter`?) and the u16 at 36 (`PlaysLoop`?). Mapping them needs a
sample with a `true` value.

### Not shown

- Codes without samples: `DesignOption` 1 (the schema's third value is
  `ToCurrentAppearance`), `EaseType` 1 to 3, trigger codes other than
  1, 5, 6 and 7. The converter leaves such an attribute out with a
  warning.
- Curved motion paths: every path point has u32 2 before its
  coordinates and left and right directions equal to the anchor.
- Chunk 0x2056C of spreads (a UID list), the 4 bytes after a preset's
  contents and the frame fields at 16 and 20 of chunk 0x2055F are not
  needed for IDML.
- Big-endian files: no samples.

## Placed InDesign pages (class 0x6607)

A placed page of another InDesign document is a child of a frame, like
an image, and has the chunks of placed graphics (transform 0x151,
bounds 0x1633, link 0x8CBC, text wrap). IDML writes it as
`ImportedPage`. Chunk 0x2505 (68 bytes) holds the page: its u32 at 0
plus 1 is `PageNumber`.

The converter writes `ImportedPage` with `PageNumber`, the attributes
of placed graphics (`ImageTypeName` from the link format, `$ID/InDesign
Format Name`), `Properties/GraphicBounds`, `TextWrapPreference` and
`Link`. Not decoded, and left out: `ImportedPageCrop` (41 of 42 are
`CropContent`, 1 `CropBleed`; the byte at 18 of 0x2505 is 0 in 28 and
1 in 13 `CropContent` items, so it is not that field),
`MetadataPacketPreference` (an XMP packet of an object of class 0x12609
that chunk 0x12629 names), `GraphicLayerOption` and `PDFAttribute`.
`ClippingPathSettings` is described under clipping path settings.

Evidence: 42 `ImportedPage` elements in 8 pairs (38 in 7 trustworthy
pairs). `ItemTransform`, `GraphicBounds` and `PageNumber` equal the
IDML in 42 of 42.

## Graphics pasted without a link

A graphic with no link (its chunk 0x8CBC names no link) can hold its file
itself:

| Class | Chunk | Contents |
|---|---|---|
| Image (0x1702), EPS (0x6601) | 0x8C23 | u32 raw data object (0 = none) |
| PDF (0x2501) | 0x2521 | u32 raw data object (0 = none) |

EPS graphics (class 0x6601) without a link name their raw data object in
chunk 0x8C23 too: over the trustworthy pairs of the corpus after
2026-10 the converter reproduces 497 of the 515 EPS `Contents` (483
were missing before; 18 differ and are not examined). Linked graphics
can have the chunk as well; their IDML has no `Contents`, and the
converter does not use it for them.

Evidence from the corpus pairs: the raw data object equals the IDML
`Contents` for 124 of 124 PDFs and 2 of 2 images without a link. In the
whole corpus, two linked and embedded PDFs also have a 0x2521 object,
different from the link resource's; with no pair to compare, the
converter uses the link resource's.

### Images that keep their file in their preview

An image (class 0x1702) without a link whose chunk 0x8C23 is 0 or
absent keeps its file in its preview (next section): chunk 0x170D names
an object of class 0x1708, whose chunk 0x119 names a raw data object.
IDML writes that object's bytes as `Contents`. The converter does so
only for images, and not for an image whose chunk 0x8C1B names an
object (see the end of this section). Linked graphics, EPS and PDF
graphics do not use their preview as `Contents`.

Evidence, over the 9,608 graphics (`Image`, `EPS`, `PDF`, `SVG`, `WMF`,
`ImportedPage`) in the reference IDMLs of the 1,252 trustworthy pairs,
each paired with its INDD object by `Self`:

| Images | Count | IDML `Contents` | Preview data equals `Contents` |
|---|---:|---:|---:|
| No link, 0x8C23 = 0 | 359 | 359 | 359 |
| No link, no chunk 0x8C23 | 17 | 17 | 17 |
| No link, 0x8C23 names a raw data object | 77 | 77 | 4 (the 0x8C23 object equals it in 77) |
| Linked, embedded | 101 | 101 | 38 |
| Linked, not embedded | 6,886 | 0 | – |

- The 376 images of the first two rows are in 31 pairs, InDesign 8.0 to
  18.x. None of them lacks `Contents` in its IDML. Of the 362 that the
  converter writes, 357 have chunk 0x8C1B set to 0 and 5 have no chunk
  0x8C1B.
- Over the whole little-endian corpus (5,734 distinct files), 2,456
  images without a link have 0x8C23 = 0 or no 0x8C23. All of them have
  the complete chain: 0x170D names a class 0x1708 object whose chunk
  0x119 names a raw data object. No image without a link has a 0x8C23
  that names anything other than 0 or a raw data object. 1,773 of them,
  in 150 files (InDesign 3.0 to 21.x), have 0x8C1B = 0 or no 0x8C1B;
  the converter writes their preview data as `Contents`. 15 more are in
  3 big-endian files (InDesign 8.0 and 11.x).
- The 17 images with no chunk 0x8C23 have no chunk 0x8CBC either; 5 of
  them (InDesign 8.0 and 11.x) have only 11 or 12 chunks.
- Every EPS (483) and PDF (111) graphic without a link names a raw data
  object, so the case occurs only for images. For EPS graphics without a
  link the preview never equals `Contents` (0 of 483).
- Formats of the 376 `Contents`: 357 in the `EPSImage` form below, 17
  JPEG, 2 TIFF. Of the 2,456 in the whole corpus: `EPSImage` 1,539,
  JPEG 514, TIFF 391, PNG 12.
- The converter reproduces 362 of 362 such `Contents` that it writes in
  the trustworthy pairs, in 20 documents, with no `Contents` where the
  reference has none. The other 14 images belong to structures the
  converter leaves out.

**The `EPSImage` form.** 357 of the 376 `Contents` start with the 8
ASCII bytes `EPSImage`, then five u32 (little-endian): bytes per row,
width, height, bits per channel, channels. Pixel data follows, height ×
bytes per row bytes, with nothing after it: the length is 28 + height ×
width × channels in 357 of 357 (8 bits per channel in all; 3 channels in
182, 4 in 175). All 357 have 0x8C23 = 0. The converter copies the bytes
and does not read this form.

**Images that name an old link object.** 762 images without a link
and with 0x8C23 = 0 or no 0x8C23, in 79 files of InDesign 2.0 to 5.0
(683 images in 53 little-endian files, 79 in 26 big-endian files), have
no chunk 0x8CBC, and their chunk 0x8C1B (u32) names an object of class
0x8C01. That object's chunk 0x8C2A names the image back in 759 of 762.
No file of these versions has an IDML pair, so it is not known whether
IDML would write the preview as `Contents`. The converter treats these
images as linked graphics: no `Contents`, and the preview goes to
`Conversion::previews`. It reads their link in files from 2.0 only
(below); in files from 3.0 to 5.0 the link is not read. In files of
later versions, 0x8C1B of such an image is 0 or absent in every case.

**Links of InDesign 2.0.** In files from InDesign 2.0 the old link
object (class 0x8C01) has chunks 0x8C04, 0x8C1B and 0x8C6B. Chunk
0x8C04 holds the linked file's path as a flag byte and an in-object
string (tag 1) at offset 24; chunk 0x8C1B names the object the link
belongs to, an image or a story (placed text). Evidence: three
privately held 2.0 documents (above, the story list of InDesign 2.0),
51 of 51 old link objects: the same three chunks in 51, the string at
24 parses in 51, every path starts with a drive letter (`C:\`). Of the
51, 28 name an image in chunk 0x8C1B and 23 a story; each of the 28
images names its link back in its own 0x8C1B (28 of 28), and the
documents have no other images. The 16 bytes before the path are not
identified. No IDML exists for these files; the converter writes the
link of each image as IDML writes a graphic link (class 0x8C42,
`StoredState="Normal"`), with the path as `LinkResourceURI` by the
reverse of the `FilePath` rule (`file:`, `\` as `/`, percent-encoded;
above, movies and sounds), and leaves `LinkResourceFormat`, the import
stamp and the times out. Links of stories are not written. Not known:
the form of a path saved on a Mac.

Not known: why these images keep their file in the preview rather than
in a raw data object named by 0x8C23. None of the 77 images whose 0x8C23
names a raw data object has an `EPSImage` `Contents`. The one WMF
graphic without a link in the pairs has a `Contents` different from its
preview; it is not examined.

## Graphic previews

Placed graphics also have chunk 0x170D, the UID of an object of class
0x1708, whose chunk 0x119 names a raw data object. In the 245 distinct
little-endian files, these objects are TIFF (613), JPEG (411), PNG (307),
GIF (27) and one other. They equal the IDML `Contents` of embedded links
in 38 of 101 and of EPS graphics without a link in 0 of 483, so they are
most likely screen previews. The exception is the images of the
previous section, whose preview is their file. The package does not
include the other previews: IDML has no place for a preview.
`indd::convert` returns the previews of graphics without `Contents` in
`Conversion::previews`, so an application can show one when the linked
file is missing. An image whose `Contents` is its preview's data is not
in that list: the package holds its file.

## Embedded data in IDML

The reference IDML files write `Contents` inside the graphic's
`Properties` as base64 (with `=` padding) in lines of 76 characters joined
by a line feed, with no line feed at the end. The text is split into
CDATA sections of 262,144 characters. The converter writes the same form.

## Stories (0x201)

Chunk 0x223: u32 length, u16, u32 first strand, UID list of more strands,
then other fields. Each strand has chunk 0x261: u16 count, then (u32
length, u32 data object UID) pairs. Each data object has chunk 0x262:

| Field | Contents |
|---|---|
| u32 | Kind: 0x202 text, 0x203 character style runs, 0x204 paragraph style runs |
| u32 | Owning strand |
| u16 | Run count |
| runs | Each: u32 record size, then the record |
| u32 | Total length |

Each record starts with a u32 run length in UTF-16 code units. Text
records continue with text segments. Style records continue with the
style UID.

**Paragraph-style records and paragraph ranges.** Counts are over the
INDD files of the 1,252 trustworthy pairs and the 37,156 stories that
both IDMLs have.

- Every paragraph-style record ends at a paragraph end (U+000D):
  87,496 of 87,496 records.
- After its text attribute list (`attributes.md`), a paragraph-style
  record has a u16 count and that many u32 values. In 82,842 of the
  87,496 records they are the lengths of the record's paragraphs, in
  order, and add up to the run length. In the other 4,654 some values
  have high bits set (for example 0x10000F74). The converter does not
  read these values.
- IDML starts a `ParagraphStyleRange` at every paragraph-style record,
  also between neighbouring records with the same style and the same
  decoded attributes. There are 2,496 such boundaries in 459 stories:
  913 between two data objects of the strand and 1,583 within one. 153
  of them are between records whose bytes after the run length are
  identical. Other rules split a record's range further (XML elements,
  `xml.md`; tracked changes, below).
- With one range per record, the text lengths of the non-empty
  paragraph ranges equal the reference in 37,133 of 37,156 stories,
  including 455 of the 459 stories with equal neighbours. No story
  has more ranges than the reference. Merging neighbours with equal
  formatting matches 36,810 stories. Of the 23 other stories, 3 have
  different text and 20 have ranges that the reference splits inside
  one record (`unknowns.md`, resources and text).
- Character-style records with the same style and decoded attributes
  are neighbours only across a data-object boundary: 3,288 boundaries
  in 533 stories, none within one data object. IDML splits there too.
  The converter writes a `CharacterStyleRange` per character-style
  record (and further cuts), and the character range lengths equal the
  reference in 525 of these 533 stories and in 37,112 of all 37,156.

**Stories outside the document's list.** The document's story list does
not name the stories of text frames anchored in the XML backing story.
IDML writes them like the others and lists them at the end of
`StoryList`: one trustworthy document (in two pairs) has 9 such stories,
whose frames are anchored in its backing story; its IDML has all 9
after the listed stories. The converter adds the stories of the text
frames anchored in the backing story, and of frames anchored in those
stories, after the listed ones, in text order. The order IDML gives
them is not that order (it starts with a later one), so `StoryList`
stays different there.

**Story settings.** Three chunks of the story object (trustworthy
pairs, 18,610 stories):

| Chunk | Layout | IDML | Without the chunk | Evidence |
|---|---|---|---|---|
| 0xA44C | flag byte (1 for a built-in key), in-object string | `Story` `StoryTitle` (`$ID/` and the key for flag 1) | `$ID/` | 18,610 of 18,610 (30 with a name) |
| 0x50F96 | u16: 1 | `StoryPreference` `StoryDirection="RightToLeftDirection"` | `LeftToRightDirection` | 18,610 of 18,610 (780 right to left, 10 documents) |
| 0x2EE (16 bytes) | u16 at 0; f64 at 2; u16 at 10 (not identified); u16 at 12 (1 `true`); u16 at 14 | `StoryPreference` `StoryOrientation` (u16 at 0: 0 `Horizontal`, 1 `Vertical`), `OpticalMarginSize`, `OpticalMarginAlignment`, `FrameType` (u16 at 14: 0 `TextFrameType`, 1 `FrameGridType`) | 12 (13 Q where text sizes are in Q, below), `false`, `Horizontal`, `TextFrameType` | alignment 18,610 of 18,610; size below; orientation and frame type below |

`OpticalMarginSize` over the 37,156 stories of the 1,252 trustworthy
pairs of 2026-10 (DOM 8.1 to 21):

| Story | Stories | IDML `OpticalMarginSize` |
|---|---:|---|
| with chunk 0x2EE | 3,805 | the f64 at 2 (all) |
| no chunk, `TextSizeMeasurementUnits` `Points` | 32,979 | 12 (all) |
| no chunk, `TextSizeMeasurementUnits` `Q` | 372 (76 pairs) | `9.2125984251969` (13 Q, all) |

The Q documents are DOM 12.1, 13, 16.2, 18 and 20.3. The text size
unit is the u32 at 12 of preferences chunk 0x1202 (`preferences.md`,
view). The size of new frames in the preferences is not the
source: in 3 pairs it differs from the stories without the chunk. Two
DOM 8.1 pairs in points have 13 Q in the preferences and 12 in their
13 stories; one DOM 16.2 pair in Q has 12 in the preferences and 13 Q
in its 3 stories. The text size unit gives the story value in all
three. Another value of the direction or alignment u16 leaves the
attribute out.

Orientation and frame type, over the
28,669 stories of all corpus pairs (stale pairs included), with chunk
0x2EE as (u16 at 0, u16 at 14):

| Chunk | Stories | IDML `FrameType`, `StoryOrientation` |
|---|---:|---|
| none | 25,206 | `TextFrameType`, `Horizontal` (all) |
| (0, 0) | 3,359 | `TextFrameType`, `Horizontal` (all) |
| (0, 1) | 4 | `FrameGridType`, `Horizontal` (all) |
| (1, 0) | 47 | `TextFrameType`, `Vertical` (all) |
| (1, 1) | 53 | `FrameGridType`, `Vertical` (all) |

The vertical and frame grid stories are in documents typeset in
Japanese (DOM 12, 13 and 18). Another frame type value leaves
`FrameType` out. For another orientation value the converter uses the
orientation of the story's frames (text orientation, below).

**Frame grids.** Over all pairs of the corpus of 2026-10:

- Chunk 0xCD41 of a text frame's multi-column frame object (class
  0x263, the child that the frame's chunk 0x15B lists) is `00 00` for a
  plain frame (23,537 of 23,537) and starts with u16 1 for a frame grid
  (248 of 248; 100, 146, 192 or 238 bytes). IDML writes an empty
  `GridDataInformation` in the `TextFrame` for exactly these 248 frames,
  after `Properties`, with `GridView="GridViewEnum"`,
  `CharacterCountLocation="BottomAlign"` and `CharacterCountSize` equal
  to the `CharacterCountSize` of the document's story grid preference
  (chunk 0xCD2E, `preferences.md`): 248 of 248 in 39 documents. The rest
  of the chunk is not decoded.
- The stories of these frames are the frame grid stories of chunk 0x2EE
  (above): 57 of 57. They have chunk 0xCD02 in the story object, with
  the layout of the page chunk (layout grid, below), and IDML writes a
  `GridDataInformation` in the `Story`, after `StoryPreference` and
  before `InCopyExportOption`, with the same values: 57 of 57 for every
  attribute. 223 other stories have the chunk but are not frame grid
  stories, and IDML writes no `GridDataInformation` for them. One frame
  grid story in the trustworthy pairs has the IDML element but no
  chunk.

The converter writes both elements by these rules.

**Owned items (run kind 0x209).** Each record: u32 run length, u16
count, then that many (u32 class, u32 UID) pairs, the objects owned by
the text position at the start of the run. An item anchored in text has
the character U+FFFC at that position and is owned through an object of
class 0x262, whose chunk 0x2C8 (u32, u32, UID list) lists the anchored page
item. IDML writes the page item element in place of the U+FFFC.
A text variable instance (class 0xCA64) is owned by a U+0018 at its
position; see `text-variables.md`. Footnotes (class 0x24F) and endnotes (class
0x2805) are owned by a U+0004 (`footnotes.md`), text and paragraph
destinations through class 0x1353C by a U+FEFF (`hyperlinks.md`).

**Text owners.** The text-owner strand (run kind 0x2A4, `tables.md`)
gives each stretch of a story's text an owner. In the 495 trustworthy
pairs of the research run the owners were the story itself (19,418
stretches), a table (class 0xB608, 11,844), a deleted text (class 0xA40A,
1,889), a footnote (class 0x24F, 502) and a note (class 0xA429, 186). The
cell field is 1 for footnotes and 0 for the story, deleted texts and
notes. Each footnote, note and deleted-text stretch ends with U+000D
(2,577 of 2,577), which IDML does not write, as for a table cell.

**Ruby strand (run kind 0x420D).** A story can list a strand of class
0x420C (chunk 0x4216: u32 data object, u32 story) whose data objects
(class 0x42B9) have chunk 0x262 with run kind 0x420D. The runs cover the
whole story text. After the run length a record has u16 1 if attributes
follow (0 otherwise), a u16 count and text attribute records
(`attributes.md`). Over the little-endian corpus: 911 stories, 11,514
runs, 7,442 with attributes. The records hold only ruby attributes, a
copy of those on the character runs. IDML does not use the copy: all
185 IDML ruby ranges of the pairs have the character run's values, and
where the two differ (5 ranges, whose strand string lacks the U+3000
separators; 16 ranges with 0x423E only on the character run) IDML
follows the character run. IDs found only in the strand, such as 0x4232
in 6,176 records, are not in IDML. The converter skips this strand.

**Notes (class 0xA429).** A note is owned by a U+FEFF (186 of 186), and
IDML writes a `Note` element in its place, inside the character range;
the `Note` holds the note's text stretch as paragraph and character
ranges. Chunk 0xA412: u32 length *n* and *n* UTF-16 units of text
segments (the user name), then two FILETIME values (u32 high part, u32
low part; 100 ns since 1601-01-01 UTC), creation and modification, then
u16 (0 in all 186). Attributes, in this order:

| Attribute | Value |
|---|---|
| `Collapsed` | `false` in all; the trailing u16 may hold it, but it never varies |
| `CreationDate`, `ModificationDate` | the two times as local time, by the link time rule below |
| `UserName` | the user name; `$ID/Unknown User Name` when the first document user of that name has flag 2 |
| `AppliedDocumentUser` | `dDocumentUser<index hex>` of the first document user (document chunk 0xA443) with that name; `n` if none has it |

Check: over all pairs the converter's notes equal the IDML notes on these
five attributes in 211 of 211 (compared as multisets per pair, since
`Note` has no `Self`).

**Tracked changes.** Deleted text is an object of class 0xA40A, owned by
the character that follows the deletion (any character; 1,696 of 1,889
in the research run were U+F035). Its text is the stretch the text-owner
strand gives to its UID, without the final U+000D. IDML writes it as
`<Change ChangeType="DeletedText">` inside the character range of the
owning character, before that character and before any element that
replaces it, holding the deleted text as paragraph and character ranges
(one `Change` per object: 1,889 of 1,889 in 21 pairs).

A story strand of class 0xA465 has runs of kind 0xA466 that cover the
whole story text (positions count characters). Each record: u32 run
length, u16 entry count, then per entry a flag byte (0), a string (the
user name, in-object form), u16 kind, FILETIME (u32 high part, u32 low
part) and u16 (0). Kind 1 is a deletion at the start of the run, with the
user and time of that deleted text; kind 2 marks the run's text as
inserted, which IDML writes as `<Change ChangeType="InsertedText">`
around it (2,073 of 2,073 entries, one element each). Records of
deleted-text stretches have no entries. `Change` attributes, in this
order: `Date`, `ChangeType`, `UserName`, `AppliedDocumentUser`, the last
two by the rule for notes.

**Where IDML puts an inserted-text `Change`.** IDML writes it at the
lowest level that holds its whole extent, as for hyperlink sources
(`hyperlinks.md`, extent and placement). The 3,962 `Change` elements of
the 21 trustworthy pairs with tracked changes sit as follows:

| Parent | Content | Elements |
|---|---|---:|
| `CharacterStyleRange` | inserted text within one range, also across a `Br` where the range goes on, and empty changes | 1,989 |
| `CharacterStyleRange` | deleted text | 1,887 |
| `CharacterStyleRange` | a `HyperlinkTextSource` with the change's extent | 43 |
| character-level `HyperlinkTextSource` | inserted text from inside the source to its end | 13 |
| `ParagraphStyleRange` (story or cell) | `CharacterStyleRange` children, and paragraph-level sources inside the extent | 23 |
| paragraph-level `HyperlinkTextSource` | `CharacterStyleRange` children | 4 |
| `Cell` | deleted text at the cell's end | 2 |
| `Story` | `ParagraphStyleRange` children | 1 |

So a change over several character ranges of one paragraph range is a
child of the `ParagraphStyleRange`, one over several paragraph ranges
a child of the `Story`, and one over several character ranges inside a
paragraph-level source a child of the source. A change with the extent
of a source holds it (`hyperlinks.md`, sources in tracked changes). The
ranges are split at the ends of such a change, as at a source.

The converter writes deleted text before the owning character. It
writes inserted text that lies in one text run in a `Change` inside the
character range. Inserted text over several runs (once the runs are
split at the paragraph-level sources) is written above them: as a child
of the paragraph range, or of the story where the runs belong to more
than one paragraph range, which then ends and starts again at the
change's ends. Inside a lifted change the character ranges have no
`Change` of their own. A table, an anchored item (the schema allows no
page item in a `Change`) or an XML marker in the extent keeps the
change inside each character range, opened again after the element; so
does a change that does not nest with a paragraph-level source.

Over the 21 trustworthy pairs the output has 3,969 `Change` elements,
and 3,960 of the 3,962 reference elements have an equal one (`Date`,
`ChangeType`, `UserName`, compared as multisets per pair). The
differences are the 4 changes of the export defect below and a few
changes still split. compare.py gains
13 range starts at the ends of lifted changes. Where a hyperlink source
starts with inserted text, IDML writes an empty `Change` before the
source (`hyperlinks.md`, sources at inserted text).

**Export defect.** In one trustworthy pair, 4 inserted-text changes
start inside a paragraph-level source and end after it. IDML writes each
inside the source, holding the text past the source's end, and writes
text after the source that the INDD stores once. The converter writes
the INDD's text once, so this story counts as "story text differs" in
compare.py.

A deletion owned by the U+000D that ends a table cell, which no
character range holds, is the last child of the `Cell`, after its last
`ParagraphStyleRange` (2 in one trustworthy pair, 1 in a stale pair;
every other deletion of the corpus is owned by a character inside a
range). With it the converter writes all 1,889 deleted-text `Change`
elements of the trustworthy pairs.

**Dates of notes and changes.** IDML writes `CreationDate`,
`ModificationDate` and `Date` as local time of the exporting computer,
like link times: the converter applies the link time rule below (the
offset of the XMP date nearest in day of the year). With it, 211 of 211
notes and 6,024 of 6,027 changes have an equal element in the output;
the other 3 are deleted texts, not examined further. The 21 pairs with tracked changes come from at most two
computers in one time zone, so the rule is checked on one time zone
only.

**Index markers (class 0x13006).** An index marker is owned by a U+FEFF,
which is not text in IDML: IDML writes a `PageReference` in its place.
Its layout and attributes are in `index.md`.

**Anchored object settings (chunk 0x2800).** The anchor object (class
0x262), object styles and the preferences have a 62-byte chunk 0x2800:

| Offset | Contents | IDML `AnchoredObjectSetting` |
|---|---|---|
| 0 | f64 | `AnchorYoffset` |
| 16 | f64 | `AnchorXoffset` |
| 32 | f64, the negative of the value at 0 in every sample | |
| 40 | u16: 0 `InlinePosition`, 1 `Anchored`, 2 `AboveLine` | `AnchoredPosition` |
| 42 | u16: 1 `TextFrame`, 2 `PageMargins`, 3 `PageEdge`, 4 `AnchorLocation` | `HorizontalReferencePoint` |
| 44 | u16: 2 `PageMargins`, 3 `PageEdge`, 4 `LineBaseline`, 6 `Capheight` | `VerticalReferencePoint` |
| 46, 50 | u16, u16 (below) | `AnchorPoint` |
| 48 | u16: 0 `RightAlign`, 1 `CenterAlign`, 2 `LeftAlign` | `HorizontalAlignment` |
| 52 | u16: 0 `TopAlign`, 1 `CenterAlign`, 2 `BottomAlign` | `VerticalAlignment` |
| 54 | u16: 0 `false`, 1 `true` | `SpineRelative` |
| 56 | u16: 0 `false`, 1 `true` | `PinPosition` |

`AnchorPoint` from the u16 at 46 and at 50: (0, 2) `BottomRightAnchor`,
(2, 2) `BottomLeftAnchor`, (2, 0) `TopLeftAnchor`, (0, 0)
`TopRightAnchor`, (2, 1) `LeftCenterAnchor`. The field at 46 is 0 for a
right and 2 for a left point, the one at 50 is 0 for top, 1 for centre
and 2 for bottom, as the alignment codes; the converter writes only
these five combinations.

Evidence, corpus of 2026-10: the 6,363 object styles whose name matches
one IDML style (5,457 in trustworthy pairs), the 1,076 anchored items
matched by `Self` with their IDML element (944 trustworthy; where the
item element lacks an attribute, its object style's value), and the
1,295 preference chunks (`preferences.md`). Every field above matches
in every one. Codes seen: `AnchoredPosition` `Anchored` in 41 and
`AboveLine` in 37 styles, in 116 and 2 items; `SpineRelative` `true` in
2 styles (the only ones with 1 at 54); `HorizontalAlignment`
`RightAlign` in 24 and `CenterAlign` in 37 styles. On the items, 42 and
44 are (4, 6) in 4 items, (2, 2) in one, (3, 3) in one and (1, 4) in
the others. The two fields change together in every sample, so
which one is the horizontal point is assumed from the order of the IDML
attributes; codes 2 and 3 mean the same in both.

IDML writes an anchored item's `AnchoredObjectSetting` by the category
rule of other object style settings: every value when the item's object
style turns the category off (`EnableAnchoredObjectOptions`, ID 0xCA2F
missing from the style's category list; 670 items, all with every
value), and otherwise the values that differ from its object style (406
items, of `[None]` or of styles with the category on). Where it writes
every value, it also writes `LockPosition="false"` and
`AnchorSpaceAbove="0"`, the values of every such element in the corpus
(`idml-values.md`, anchored object settings of page items).

**Items inside an anchored item.** The children of an anchored group
(and the items inside them) have an `AnchoredObjectSetting` too. IDML
applies the rule above to each of them with the anchor's chunk 0x2800
values and the child's own object style: every value where the child's
style turns the category off, else the values that differ from that
style. With this rule the converter writes 584 more `AnchoredObjectSetting`
elements over the trustworthy pairs of the corpus after 2026-10 (344 on
text frames, 114 ovals, 61 rectangles, 30 polygons, 22 graphic lines, 7
groups, 6 text boxes), with every value equal and no extra value; 6,503
values of these elements were missing before, and 388 in 2 or 3
documents per element still are (not examined).

**Positions count characters.** A text record's run length counts
UTF-16 code units, but every other position counts characters, a
surrogate pair being one: the lengths in a strand's chunk 0x261 (also
for the text strand), and the run lengths of style runs, owned items and
text owners. Evidence: the 6 stories of the little-endian corpus whose
text has a character outside the Basic Multilingual Plane (emoji, all in
one pair). In all 6 the text records add up to the number of UTF-16
units, and the other strands and all chunk 0x261 lengths to the number
of characters; with that reading, each emoji gets the character style
range IDML gives it (6 of 6). The converter applies the same reading to
hyperlink range trees, for which no sample with such a character exists.

U+FEFF characters can be XML markers, which IDML does not write
(`xml.md`).

INDD stores a forced line break as U+000A; IDML writes it as U+2028. The
last paragraph return of a story is not written to IDML.

## Story links and placed text

Stories placed from another story (linked stories), from a Word or
Excel file, and the text frames that linked page items copy, carry the
elements below. Evidence base: the 159 distinct corpus pairs whose IDML
has any of them (158 little-endian), and for the presence rules all
45,640 stories of the distinct little-endian pairs. No story has an
element below without its stored data, and none has the data without
the element.

### Link lists (chunk 0x8CBC)

Chunk 0x8CBC is a list of link UIDs: 4 bytes (`01 01 00 00`), u32
count *n*, then *n* UIDs. It occurs on stories (0x201), spline items
(0x6201), groups (0x401), graphics, link resources and more; the empty
list (8 bytes) is the common case (28,569 spline items). Graphics and
movies read the UID at offset 8, the first entry. No object in the
corpus has more than one entry.

### Story links

A story with a non-empty 0x8CBC list has a `Link` child in IDML, and
`Link Self` is `u` and the listed UID in hexadecimal: 47 of 47 stories
whose UID is in both files; no story with a link lacks the IDML `Link`,
and none the other way round, over the 45,640 stories.

The link object has the layout of a graphic link (placed graphics,
links), with the story's UID at offset 16 of chunk 0x8C9B (47 of 47),
and its resource (class 0x8C41) the same chunk 0x8C92:

| IDML | Equal |
|---|---|
| `LinkResourceURI` | 45 of 47 (2 links to another document, resolved to another path at export) |
| `LinkResourceFormat` (`$ID/` and the format name) | 47 of 47: `Internal Linked Story` 28, `External Linked Story` 2, `Microsoft Office Word 2007 Import Filter` 17 |
| `LinkImportStamp` | 47 of 47 (a plain number such as `1653409666` for linked stories, `file …` for Word files) |
| `LinkResourceSize` | 47 of 47 (`0~0` for linked stories, the file size for Word files) |
| `LinkResourceModified`, `ShowInUI` | 47 of 47 each |
| `StoredState="Contained"` | 47 of 47 (resource field 3, placed graphics, link resources) |

`RenditionData` and `PDFIdentifier` follow the rules of graphic links:
present on the 17 version-14 Word links and the 49 version-21.0 links,
absent on the 30 version-10 links.

Link object classes (`LinkClassID` is the class in decimal):

| Class | `LinkClassID` | Links | URI |
|---|---|---|---|
| 0x8C42 | `35906` | all graphic links; 17 stories placed from Word | `file:` |
| 0x21D03 | `138499` | 30 linked stories | `idinternalcontent:<id>`, `idexternalcontent:<path>?<id>` |
| 0x21D32 | `138546` | 3 linked page items (text frame, group) | `idinternalSharedArticle:`, `idexternalSharedArticle:` |

The converter writes story links. Links of page items (class 0x21D32)
are not written: their page item's `LinkedPageItemOption` was not
studied.

### Link source IDs (`LinkResourceId`)

`idinternalcontent:<id>` names the story a link copies by an ID, and
that story has `LinkResourceId="<id>"`. A text frame can have one too
(the source of a linked page item). The IDs are in chunk 0x21D06 of the
document object (UID 1):

| Offset | Type | Contents |
|---|---|---|
| 0 | u32 | not identified (non-zero even when the list is empty) |
| 4 | u32 | count *n* |
| 8 | *n* × (u32, u32) | ID, UID of a story or spline item |

IDML writes `LinkResourceId="<ID>"` (decimal) on the element whose
`Self` is that UID. In 36 of 36 distinct pairs whose IDML has any
`LinkResourceId`, the set of (UID, ID) equals the chunk's list exactly
(195 values: 189 `Story`, 6 `TextFrame`; 31 trustworthy documents). In
the other 1,500 pairs the list is empty. Listed UIDs are stories and
spline items, all `TextFrame` in IDML. IDs are below 32,768. The
converter writes the attribute on stories and text frames; a listed UID
that is not written is ignored.

### Linked story options (chunk 0x21D12)

Chunk 0x21D12 (10 bytes) on a story. IDML writes `Story/LinkedStoryOption`
when the story has the chunk and a link (30 of 30). Two stories with the
chunk and no link have no `LinkedStoryOption`; stories placed from Word
have no chunk.

| Offset | Seen | IDML |
|---|---|---|
| 0–5 | `00 00 01 00 00 00` in all 30 | `UpdateWhileSaving="false"`, `WarnOnUpdateOfEditedStory="true"`, `RemoveForcedLineBreaks="false"` in all 30 (which byte holds which is not shown; the converter writes these values) |
| 6 | u32 | `ApplyStyleMappings`: 1 `true` (28), 0 `false` (2) |

The preferences object (0x2202) has the same chunk (`00 00 01 00 00 00 00
00 00 00` in every document); the document's `LinkedStoryOption` is
written from observed values (`idml-values.md`).

### Style mappings (chunk 0x21D30)

Chunk 0x21D30 holds five lists. Each list: u32 count, then per entry a
kind byte (2 or 3), an in-object string (source name), a kind byte (2),
an in-object string (destination name) and a u32 rule.

| List | IDML |
|---|---|
| 1 | `ParaStyleMapping` |
| 2 | `CharStyleMapping` |
| 3–5 | empty in every sample (the schema has `TableStyleMapping` and `CellStyleMapping`; which list is which is not shown) |

| Rule | `MappingRuleType` | IDML name | Seen |
|---|---|---|---|
| 0 | `StyleMappingRule` | as stored | 4 |
| 1 | `GroupMappingRule` | `:*` after source and destination | 60 |
| 2 | `StyleToGroupMappingRule` | `:*` after the destination | 30 |

The kind byte does not add `$ID/` (kind 3 is on `[Basic Paragraph]`
under a localized name). Group names may contain `/`
(`<group>/<subgroup>`), written as stored. Attributes: `Self` is the
owner's `Self`, then `ParaStyleMapping` or `CharStyleMapping` and the
index in its list from 0; `SourceStyleName`, `DestinationStyleName`,
`MappingRuleType`.

- **Story**: a story chunk longer than 20 bytes gives the story's
  elements (32 of 32 stories; 107 elements equal, `Self` included). The
  empty form is 20 zero bytes (75 stories), and no story with it has
  mappings.
- **Document**: chunk 0x21D30 of the preferences object (class 0x2202),
  in `designmap.xml` with `Self="dParaStyleMapping<i>"`. Non-empty in 1
  of 1,553 documents, the only one with `ParaStyleMapping` in
  `designmap.xml` (2 `StyleMappingRule` entries, equal). 1,431 documents
  have the empty chunk, 121 none.

Rule code 3 and entries in lists 3 to 5 are not seen; the converter
leaves them out with a warning.

### Import options of placed text (chunk 0x12202)

Story chunk 0x12202 is 8 bytes: u32 UID of an import options object,
u32 0 (in all). The class of that object decides the `Properties`
child:

| Class | IDML | Stories |
|---|---|---|
| 0x12210 | `WordRTFImportPreferences` | 39 of 39 |
| 0x12213 | `ExcelImportPreferences` | 9 of 9 |
| UID 0 | none | 20 |

**Word (class 0x12210).** Chunk 0x12210: 19 u32 (*s0* … *s18*). IDML
writes a list of 19 items for versions 8 to 12 and 20 for 14 to 20 (no
version-13 sample). This mapping reproduces all 39 stories (8 with 19
items, 31 with 20; 10 distinct option sets):

| Item (20-item list) | Type | Value |
|---|---|---|
| 0, 1 | boolean | *s0* ≠ 0, *s1* ≠ 0 |
| 2 | enumeration | *s5*: 0 `PageBreak`, 2 `None` (1 sample) |
| 3 | boolean | `true` (no stored field) |
| 4 | enumeration | `UnformattedTable` (no stored field) |
| 5, 6 | boolean | *s8* ≠ 0, *s9* ≠ 0 |
| 7 | boolean | `false` (no stored field) |
| 8, 9 | enumeration | *s11*, *s12*: 0 `ResolveClashUseExisting`, 2 `ResolveClashAutoRename` (1 sample each) |
| 10 to 13 | boolean | *s14*, *s15*, *s16*, *s17* ≠ 0 |
| 14 | boolean | `false`, only in the 20-item list (no stored field) |
| 15, 16 | boolean | `true` (no stored field) |
| 17 | long | *s4* (1 or 2) |
| 18, 19 | boolean | `false` (no stored field) |

The 19-item list is the same without item 14. In every sample *s0* =
*s1*, *s14* = *s15* and *s16* = *s17*, so the order within each pair is
not shown, and items 0/1, 10/11 and 12 are told apart only by assuming
the same stored layout in both list lengths. Code 1 of *s5*, *s11* and
*s12* is not seen; the converter leaves the property out with a warning
for a code not in the table.

**Excel (class 0x12213).** Chunk 0x12213:

| Offset | Type | Seen | IDML item |
|---|---|---|---|
| 0 | u32 | 1 | |
| 4 | u32 | 0 | |
| 8 | u32 | 3 | |
| 12 | u32 | 0 | |
| 16 | u32 | 0 or 1 | 3: 0 `ExcelFormattedTable` (5), 1 `ExcelUnformattedTable` (4) |
| 20 | u32 | 1 | |
| 24 | flag byte, in-object string | flag 1, `Sheet1` | 7: `$ID/Sheet1` |
| then | flag byte, in-object string | flag 0, the range (4 distinct values) | 8: the range |
| then | flag byte, in-object string | flag 1, empty | 6: `$ID/` |
| then | u32 | 1 | |

The strings are written as names (`$ID/` and the string for flag 1).
Every sample has the list `Spreadsheet` (enumeration), `3` (long),
`false`, the table option, `true`, `true`, item 6, item 7, item 8, `1`
(long). Items 0, 1, 2, 4, 5 and 9 have no identified field: the longs
`3` and `1` equal the u32 at 8 and the last u32 in every sample, which
does not show that they come from them.

### Style mapping preferences (chunk 0x12216)

Story chunk 0x12216: u32 *p*, u32 *c*, then 2·*p* names (paragraph
styles: source, destination, …) and 2·*c* names (character styles),
each a flag byte (0 to 3) and an in-object string. When *p* + *c* > 0,
IDML writes `Properties/StyleMappingPreferences`: a list of two lists of
strings, the paragraph names and the character names (empty when *c* =
0), as stored (flag 1 does not add `$ID/`). 67 of 67 stories with the
chunk (29 non-empty; *p* and *c* up to 13); no story has
`StyleMappingPreferences` without the chunk. The import options come
before it in `Properties`; no story has Excel options together with it.

### Element order

In all 107 stories with these elements InDesign writes: `Properties`
(import options, then `StyleMappingPreferences`), `StoryPreference`,
`GridDataInformation`, `LinkedStoryOption`, `InCopyExportOption`,
`ParaStyleMapping`, `CharStyleMapping`, `Link`, then the text. The
schema allows this order (`Story.rnc`). In `designmap.xml` the document
mappings follow the `idPkg:Story` references.

### Plugin data

Stories of 39 version-14.0 documents also carry data of two third-party
plugins (`DocsFlowImportSettings`, the `WordsFlow…` properties, the
`Story` attributes `Observers` … `ComponentId`). The published IDML
schema rejects it, and it follows the plugins of the exporting computer;
the converter does not write it (`unknowns.md`, plugin data;
`docs/measurement.md`).

## Styles (0x205)

Paragraph and character styles share the class. Chunk 0x230: u32 next
style (0 = itself), u32 based-on style, fields not yet identified, u16 1
for paragraph styles or 0 for character styles, u16 `Imported` (1 =
true), u8 1 if the name is a built-in key (`$ID/` in IDML), the name as
an in-object string, then, in newer files, a GUID string
(`StyleUniqueId`). The name's offset varies (23–26
bytes), so the converter locates it as a flag byte followed by a valid
in-object string.
All 486 style names and 291 `NextStyle` values in the pairs match.

**Style names in InDesign 2.0.** The name starts at offset 20: u16 kind
at 16, u16 at 18, flag byte at 20, then an in-object string with tag 1
(above, in-object strings) that ends the chunk. Evidence: three
privately held 2.0 documents (above, the story list of InDesign 2.0),
22 of 22 styles; the kind is 1 in 14 and 0 in 8, and the flag is 1 on
the two root styles of each document (`[No paragraph style]`,
`[No character style]`). The search for a flag byte and a string must
accept tag 1 in these files. In the 6 root styles, bytes 15 to 19 (0, 1,
0, 0, 0) also read as a flag byte and an empty tag-1 string; the search
skips empty strings in 2.0 files, as a style name is never empty. Before
this, every style of these documents was left out ("no name"), the
paragraphs named styles that the IDML did not define, and the root
style's font (the default of most text) was lost.

**Imported and unique ID.** Over the 4,694 paragraph and character
styles of the trustworthy pairs whose IDML style has the same name, the
u16 after the kind equals `Imported` in 4,694 (180 imported). The GUID
is a 36-character in-object string after the name's string; the byte
after its string tag is not always 0. Every `StyleUniqueId` in the
corpus IDML files is a GUID of lowercase hexadecimal digits in groups of
8, 4, 4, 4 and 12 (7,953) or `$ID/` (4,299), so the converter accepts
only that form, searching from the end of the name. A search from the
name's own flag byte took a style name of 36 characters for the GUID.

IDML has `StyleUniqueId` on every paragraph and character style, the
root styles included, when the document was saved by 11.2 or later
(header or last session of the save history): the stored GUID, or
`$ID/` for a style without one. Earlier documents have none. Over the
trustworthy pairs the converter reproduces 6,313 of 6,316 paragraph and
3,734 of 3,735 character style values. The exceptions are pairs whose
IDML was exported by another minor version than the INDD records: four
11.4 documents whose IDML has DOM 11.0 and no `StyleUniqueId` (34
values written though the IDML has none), and one 11.0 document whose
IDML (DOM 11.4) has `$ID/` (3 values missing); one paragraph style has
another GUID in IDML.

**Kind field.** The kind and the `Imported` u16 after it can be read
as one u32 in most files, because the second u16 is 0. The InDesign 7.5
template fixture `scml-template/scml.indt` (no IDML) has 1 there in 600
of its styles: 151 have `00 00 01 00` before the flag and 449 have
`01 00 01 00`. The 151 are all listed in the tree of the root character
style group (chunk 0x28C2 = 0xCA0D, below) and the 449 in the tree of the
root paragraph style group, so the kind is the first u16 alone. Read as a
u32, the 151 character styles were taken for paragraph styles, which gave
`ParagraphStyle/…` references and a `NextStyle` that the schema does not
allow on a character style. In every other distinct little-endian corpus
file the u32 is 0 or 1 (2,125 styles), and the kind agrees with the root
group in all of them. In the two big-endian files the u16 kind is also
the first two bytes (`00 01` for their three paragraph styles, `00 00`
for their two character styles).

IDML writes a `BasedOn` of the root `[No paragraph style]` or
`[No character style]` as a string (`$ID/[No paragraph style]`), and any
other base as an object reference.

**Attribute list (chunk 0x23F).** The style's text attributes
(`attributes.md`, text attribute lists). IDML writes on a style exactly
the text attributes this list stores. It adds no value from the base
style, and it writes a stored value also when it equals the base's.
Evidence: the 11,983 paragraph and character styles of the trustworthy
pairs that match an INDD style by kind and name. In every one, each
stored ID with a known mapping is an attribute of the IDML style, and
every IDML text attribute comes from a stored ID, except `Label` (6
styles) and `HyphenQuality` (7 styles).

Most lists are short. The root paragraph style stores the full list
(287 to 362 IDs, more in later versions). Over all pairs, no other paragraph
style stores more than 14 % of its root's list (the most is 41 of 289),
and `NormalParagraphStyle` stores at most 21. The only other styles with
a full list are 7 character styles, the same style object in 7 files of
one document family (DOM 10.0 and 10.2): each stores 120 IDs, and IDML
writes 126 or 128 attributes on them, almost all with the root values.
Nothing else in these styles differs from the short-list styles: they
have a base (the root character style), `Imported` is false, and their
chunks are the same.

**Keyboard shortcuts.** Ten bytes before the name's flag byte is a u32
key; the two bytes 6 and 5 before the flag are modifier bytes *m0* and
*m1*. Evidence: the 6,957 paragraph and character styles of all pairs
whose IDML style has the same name.

| Key | `KeyboardShortcut` | `ExtendedKeyboardShortcut` |
|---|---|---|
| 0 (5,272 styles) | `0 0` | `0 0 0` |
| 0xC000*xx*, *xx* a digit `0`–`9` | *m0* + 256 × *m1* and the code below | `0 0 0` |
| 0x8000*xx* | `0 0` | *m0* + 256 × *m1*, *xx*, then 1 |

The 103 styles with a key other than 0 are in 38 files. With key
0x8000*xx*, *xx* is a character code: over all pairs, the 21 styles of
11 trustworthy pairs with such a key and DOM 15 or later (codes 49 to 52,
66 and 73; modifiers 1, 17, 256 and 257) have this
`ExtendedKeyboardShortcut` in IDML, for example `256 49 1` for *m0* 0,
*m1* 1 and code 0x31.

The key code follows the platform of the last session of the save
history (the second u16 of its record), not the style: with platform 0
digits 0 to 7 give 82 to 89, 8 gives 91 and 9 gives 92 (every digit
occurs); with platform 3 the code is 96 + the digit. This is a rule of
the exporting edition (`preferences.md`): over the trustworthy pairs of
the corpus of 2026-10 it reproduces `KeyboardShortcut` for 3,154 of
3,154 character styles and 6,927 of 6,928 paragraph styles (a rule by
*m1* failed for 26 character and 33 paragraph styles with *m1* 0 and
platform 3). The converter writes the attribute on every style other
than the root styles. IDML has `ExtendedKeyboardShortcut` from DOM 15 on;
the converter writes it from version 15.

**Preview colour.** Fourteen bytes before the name's flag byte is a
u32: 0, or the UID of an interface colour (class 0x1F11). IDML writes it
as the style's `PreviewColor` property: `Nothing` for 0, otherwise the
colour named as for layers. Over all pairs of the corpus after 2026-10
(styles other than the root styles, matched by name): 8,457 of 8,457
paragraph styles and 3,638 of 3,638 character styles. Two character
styles of one document (two pairs) have a colour: UID 0x20, RGB 0.5 0.5
0.5, `Gray`, and UID 0x35, RGB 0.48 0.73 0.85, `GridBlue`. The root
styles have no `PreviewColor` in IDML. The converter leaves the property
to the observed values when the u32 is neither 0 nor an interface colour.

**Export settings (chunk 0x28F0).** A u32 count of export tag maps,
the maps, then u16 values to the end of the chunk:

| Field | Contents |
|---|---|
| u32 | number of tag maps *n* |
| *n* maps | export type, tag, class (three strings), u32 attribute count *k*, *k* pairs of strings |
| u16 | `SplitDocument` (1 `true`, 0 `false`) |
| u16 | `EmitCss` |
| u16 | `IncludeClass` (DOM 13 on) |
| u16 … | more values in later versions, not identified |

A string is a u32 length in UTF-16 units followed by text segments (as
the font version, `fonts.md`); an empty string is a length of 0. Export
types are `EPUB` and `PDF`; tags such as `p` or `H3` and classes may be
empty. No map in the corpus has attributes, so the layout of the pairs
is assumed and the converter writes no map that has them. The chunks
of every paragraph, character and object style of all pairs parse.

IDML has `SplitDocument` and `EmitCss` when the document was saved by
10.1 or later, and `IncludeClass` from 13.0 (header or last session of
the save history), on every paragraph and character style, the root
styles included. Without the chunk the values are `false`, `true`,
`true`. The trustworthy pairs include four whose INDD header says 10.0
and whose IDML (DOM 10.2) has the attributes, and six of 10.0 whose
IDML (DOM 10.0) has none. Over the trustworthy pairs the converter
reproduces `SplitDocument` and `EmitCss` of 7,287 of 7,287 paragraph
styles and 4,067 of 4,067 character styles, and `IncludeClass` of 5,760
and 3,500, with no extra value.

**Tag maps.** IDML writes one `StyleExportTagMap` per map after the
style's `Properties`, in stored order:
`Self="u<style UID in hex>StyleExportTagMapn<type>"`, `ExportType`, `ExportTag`, `ExportClass`, `ExportAttributes=""`,
and the style's `SplitDocument` and `EmitCss`. 306 of 306 elements in
the trustworthy pairs, every attribute equal.

**Empty nested, line and GREP styles.** Paragraph styles have
`EmptyNestedStyles`, `EmptyLineStyles` and `EmptyGrepStyles` (DOM 8.1,
and 10 on). Each is `false` when the style's list of nested styles
(attribute 0x1B75), line styles (0x1BBB) or GREP styles (0x1BBA) has
items, where the list is that of the first style in the based-on chain
that has the attribute; an empty list is stored as a count of 0. This
gives the IDML value of 5,401 of 5,403 paragraph styles for each of the
three (all pairs; the other 2 are not written).

## Style groups and object styles

**Root groups** (class 0xCA8C for paragraph and character styles, 0x1B972
object styles, 0x20241 cell styles, 0x1044F table styles) are named in
IDML by UID (`RootParagraphStyleGroup Self="u7e"`). Chunk 0x28C2 says
which kind: 0xCA0C paragraph, 0xCA0D character, 0x1B924 object, 0xB669
cell, 0xB668 table (all 75 pairs). Children: chunk 0x28DC (text styles)
or 0x1B95A (object styles), each u32, u32, UID list. A built-in root style
such as `[No character style]` need not be listed.

**Groups** (class 0xCA8B): chunk 0x28D3 is parent, root, UID list of
children; chunk 0x28D2 a flag byte and the name. IDML names every group
`$ID/<name>` (20 of 21 groups), and a style inside groups is referenced as
`ParagraphStyle/<group>:<group>:<name>` with `:` escaped as `%3a`. All 412
paragraph styles and 21 groups in the pairs match.
A group inside another group is named by its full path from the
top-level group, joined by `:`, as in its `Self`:
`Self="ParagraphStyleGroup/$ID/Page > Running Text%3aNotes"` has
`Name="$ID/Page > Running Text:Notes"`. Over all pairs: 72 of 72 nested
paragraph style groups, 25 of 25 character style groups and 2 of 2
object style groups (three levels at most); top-level groups have `$ID/`
and their name (622 of 622).
A `:` inside a group's name is written `\:`, as in a style's own name
(below): such a group has `Name="$ID/<a>\: <b>"` and
`Self="ParagraphStyleGroup/$ID/<a>\%3a <b>"`, and its styles have
`Name="<a>\: <b>:<style>"`. No pair of the corpus has such a group; this
rests on one privately held sample (1 top-level paragraph style group
with 6 paragraph styles, all matching).
Cell style groups work the same way: the root cell style group's
children (chunk 0x2024E) include groups of class 0xCA8B, and IDML writes
each as `<CellStyleGroup Self="CellStyleGroup/$ID/<group>"
Name="$ID/<group>">` holding its styles, in the order of the children (2
groups with 6 cell styles in 1 trustworthy DOM 18 pair). No table style
group occurs in the pairs; the converter writes them the same way.

**Style names.** Over the styles of the 489 trustworthy pairs, a style's
IDML `Name` is its group path and its own name joined by `:` (1,170
paragraph, 759 character, 8 object and 6 cell styles in groups, such as
`Name="Listes:Liste non numérotée"`), and its `Self` is the tag and that
name with `%` written `%25`, `:` written `%3a` and CR written `%0d`. Two
more rules:

- A `:` inside a style's own name is written `\:` in `Name`, so `\%3a`
  in `Self`: `Name="ss01\: People"`, `Self="CharacterStyle/ss01\%3a
  People"` (12 paragraph, 6 character and 3 object styles).
- A built-in style inside a group has `$ID/` before the whole path:
  `ParagraphStyle/$ID/INDEX%3aIndex Section Head` with
  `Name="$ID/INDEX:Index Section Head"` (one paragraph and one character
  style).

The same holds for object styles and cell styles in groups
(`ObjectStyle/Worksheet%3aworksheet`, `CellStyle/Standard
Table%3aHeader Row`). Elsewhere `%3a` stays plain: languages are
`Language/$ID/English%3a USA`.

**Object styles** (class 0x1B901): chunk 0x1B907 is u32 based-on style,
u8 1 for a built-in name, then the name. A page item's chunk 0x1B916 is
its applied object style (`AppliedObjectStyle`, 671/671 rectangles). A
style based on the root `[None]` has its `BasedOn` written as a string.

## Object style settings

Object styles (class 0x1B901) hold their settings in these chunks.
Evidence: the 337 object styles of the same-version pairs whose name
matches an IDML `ObjectStyle` (259 of them other than `[None]`); all
counts are matches of 337 unless stated.

| Chunk | Contents | IDML |
|---|---|---|
| 0x1B92B | Attribute list, u16 count, page item records (`attributes.md`) | `FillColor`, `StrokeColor`, `StrokeWeight`, `StrokeType`, `CornerOption`, `CornerRadius`, `GradientFillAngle` (0x551E), `GradientStrokeAngle` (0x5524) |
| 0x1B956 | Attribute list, frame fitting (see frame fitting) | `FrameFittingOption` |
| 0x1B924 | Text frame settings (below) | `TextFramePreference`, `TextFrameFootnoteOptionsObject` |
| 0x285B | u16 story orientation at 0, f64 at 2, u16 at 12, u16 frame type at 14 | `StoryPreference` (`OpticalMarginSize` from the f64, `OpticalMarginAlignment` from the u16 at 12: 0 `false`, 1 `true`) |
| 0x50F28 | u16 story direction | `StoryPreference/StoryDirection` |
| 0x3776, 0x3777 | Text wrap, as chunks 0x3703 and 0x373D of page items | `TextWrapPreference` |
| 0x1B92E | u32 count, IDs of the categories the style turns on | `Enable…` attributes (below) |
| 0x1B946 | u32 paragraph style, 0 = none | `AppliedParagraphStyle` (`n` for none, and for a style without the chunk: 5,556 of 5,556 values of the trustworthy pairs of the corpus after 2026-10, 173 of them without the chunk) |
| 0x1E244 | 78 bytes: six f64 at 30 | `FlexLayoutAttributeOption` `FlexPaddingTop`, `FlexPaddingRight`, `FlexPaddingBottom`, `FlexPaddingLeft`, `FlexGapRow`, `FlexGapColumn`; without the chunk 12, 12, 12, 12, 20, 20 (below) |
| 0xCD32 | u16 0 for none, or u16 1 and the UID of a named grid | `AppliedNamedGrid` (named grids, above) |
| 0x1B94D | u16, 1 = true | `ApplyNextParagraphStyle` (below) |
| 0x2800 | Anchored object settings, as in the preferences | `AnchoredObjectSetting` (below) |
| 0x1B907 | u32 based-on style, flag byte and name, u32 shortcut key | `BasedOn`, `Name`, `KeyboardShortcut` (below) |
| 0x1E234 | 52 bytes: u16 flags at 8, 10, 12 and 14, f64 at 16, 24, 32 and 40 | `TransformAttributeOption`: `TransformAttrX`, `TransformAttrY`, `TransformAttrHeight` and `TransformAttrWidth`, each written when its flag is 1 (below) |

**Flex layout spacing.** In chunk 0x1E244 the six f64 at 30 are the
four paddings and the two gaps. A style without the chunk (`[None]`
included) has 12 for each padding and 20 for each gap. With this the
converter reproduces all 226 values of each of the six attributes in the
trustworthy pairs (none was written before). The four paddings are
equal in every sample, as are the two gaps, so the order inside each
group follows the IDML attribute order and is not proven.

**Settings found in the corpus after 2026-10.** Object styles other
than `[None]`, matched by `Self` with their IDML style over the 1,251
trustworthy pairs (4,305 styles; with `[None]`, 5,556):

| Field | IDML | Matches |
|---|---|---|
| 0x1B94D u16: 0 `false`, 1 `true`; no chunk `false` | `ApplyNextParagraphStyle` | 3,963 with 0, 3 with 1, 339 without the chunk; `[None]` has no attribute |
| 0x1B907, u32 after the name: 0 | `KeyboardShortcut="0 0"` | every style with 0 (other keys are followed by two modifier bytes as for paragraph styles, but their codes differ from those of paragraph styles; the converter leaves them out) |
| 0x3776 (44 bytes) u16 at 40: 1 `false`, 0 `true`; no chunk `false` | `TextWrapPreference` `ApplyToMasterPageOnly` | 3,914, 84 and 1,558 of 5,556 |
| 0x1B924 u16 at 140, 1 = true (142 bytes or more) | `TextFramePreference` `UseNoLineBreaksForAutoSizing` | 5,518 of 5,518 (24 true) |
| 0x2800 f64 at 16 | `AnchoredObjectSetting` `AnchorXoffset` | 5,551 of 5,551 (28 not 0); 5 styles have no chunk |
| 0x2800 u16 at 42: 1 `TextFrame`, 4 `AnchorLocation` | `AnchoredObjectSetting` `HorizontalReferencePoint` | 5,547 and 4 |

The converter reads the anchor fields wherever it reads chunk 0x2800:
object styles, anchored frames and the preferences.

**Attribute list.** The IDs of page items carry over. `CornerOption` 0
is `None` and 0x5A16 `InverseRoundedCorner` (1 style). The two gradient
angles are told apart by one style with −90 and 0. Each corner radius
and corner option has its own ID, as in page items (`attributes.md`,
corners), and the converter writes each corner from its ID.

**Overprint.** The attribute list holds the overprint flags of the
fill (0x6E6A), the stroke (0x6E67) and the gap (0x6E8B), u16 1 `true`,
0 `false`. IDML writes `OverprintFill` only when the fill colour (0x6E68)
puts ink on the page: it is not the swatch `None` or `Paper`, its tint
(0x6E69) is not 0, and it is not a white process colour (CMYK 0 0 0 0,
or RGB 255 255 255). The same holds for `OverprintStroke` with 0x6E64
and 0x6E66, and `OverprintGap` with 0x6E89 and 0x6E8A. The root `[None]`
has none of the three. Documents saved by 21.4 or later (12 trustworthy
pairs of 21.4 and 21.5) have all three on every object style, the root
included, whatever the colours. Over the trustworthy pairs the converter
reproduces 235 of 235 `OverprintFill`, 1,512 of 1,512 `OverprintStroke`
and 49 of 49 `OverprintGap` values with no extra value. The white RGB
case rests on 2 styles of two pairs of one 13.1 document (89 other RGB
fills and strokes are written). Every object style flag in the corpus
is 0. Page items use the same IDs (page item settings, overprint).

**Export settings (chunk 0x28F0).** Object styles have the chunk of
paragraph and character styles (styles, export settings). IDML writes
`EmitCss` (the second u16) for documents saved by 10.1 and
`IncludeClass` (the third) for documents saved by 13.0, the root
`[None]` included; there is no `SplitDocument`. Without the chunk both
are `true`. Trustworthy pairs: `EmitCss` 5,031 of 5,031,
`IncludeClass` 4,019 of 4,019 (every value is `true`). One 12.1
document has an object style tag map (`ObjectStyleExportTagMap`); it is
not written.

**ARIA label.** From DOM 21.2 every `ObjectStyle`, `[None]` included,
has `EpubAriaLabel=""` and `EpubAriaLabelSourceType="NoneARIALabel"`,
after `EpubAriaRole`: 178 of 178 styles in the 44 corpus IDML files of
DOM 21.2 to 21.5. The 60 styles of the 15 files of DOM 21.1 have
neither, nor does any style of an earlier DOM. No value varies, so no
INDD field is identified; the converter writes both on every object
style from DOM 21.2. They are attributes of the style. Its
`ObjectExportOption` has `EpubAriaLabel="$ID/"` and
`EpubAriaLabelSourceType="AutomaticARIALabel"` from DOM 21.1, as page
items do (`idml-values.md`, export options of page items).

**Optical margin (chunk 0x285B).** The f64 at 2 is `OpticalMarginSize`
and the u16 at 12 `OpticalMarginAlignment` (1 = true), as in the story
chunk 0x2EE (stories). Over all pairs of the corpus of 2026-10, both
match in 3,515 of 3,515 object styles (2,661 of 2,661 in the trustworthy
pairs). The IDML sizes are 12 (3,172 styles), 9.2126 (336), 8 (4) and
9.9213 (3); 4 styles have `OpticalMarginAlignment="true"`. The u16 at 10
is not identified.

**Text frame settings (chunk 0x1B924).** 222 bytes; 162, 142 or 106 in
files from older versions (942, 48, 66 and 61 object styles in the
little-endian corpus). The converter reads it only with these sizes.
The InDesign 4.0 file has 104 bytes, with another layout; the converter
leaves it out with a warning (`big-endian.md`). In the corpus after
2026-10, 1,129 object styles in 263 files of InDesign 4.0 to 6.0 have 104
bytes; none of these files has an IDML, so the layout cannot be checked.

| Offset | Contents | Attribute |
|---|---|---|
| 0 | f64 | `TextColumnFixedWidth` |
| 8 | f64 | `TextColumnGutter` |
| 32 | u16, 1 = true | `VerticalBalanceColumns` |
| 34, 42, 50, 58 | four f64, inset spacing | `InsetSpacing` (below) |
| 66 | u32 | `TextColumnCount` |
| 70 | u16, 1 = true | `UseFixedColumnWidth` |
| 74 | u16, codes as for frames | `VerticalJustification` |
| 76 | u16, codes as for frames, and 4 `XHeight` | `FirstBaselineOffset` |
| 116 | u16, codes as for frames | `AutoSizingType` |
| 118 | u16, codes as for frames | `AutoSizingReferencePoint` |
| 120, 122 | u16 1 = true, f64 | `UseMinimumHeightForAutoSizing`, `MinimumHeightForAutoSizing` |
| 130, 132 | u16 1 = true, f64 | `UseMinimumWidthForAutoSizing`, `MinimumWidthForAutoSizing` |
| 144 | u16, 1 = true | `FootnotesSpanAcrossColumns`, `SpanFootnotesAcross` |
| 146 | f64 | `FootnotesMinimumSpacing`, `MinimumSpacingOption` |
| 154 | f64 | `FootnotesSpaceBetween`, `SpaceBetweenFootnotes` |
| 190 | f64 | `ColumnRuleStrokeWidth` |
| 198 | u32 swatch, 0 = `n` | `ColumnRuleStrokeColor` |
| 210 | f64 | `ColumnRuleStrokeTint` |

The fields from offset 32 to 132 were found by testing every offset
against the IDML style, over the 2,155 object styles of the trustworthy
pairs that match an IDML style by name (1,000 with 222 bytes, 607 with
142 and 520 with 162). Each is the only offset that matches all styles
of the 222- and 142-byte layouts:

| Offset | Values seen (222 / 142 bytes) |
|---|---|
| 32 | 1 true in each layout, all match |
| 70 | 142: 1 true; 222: all false |
| 74 | 222: 4 `CenterAlign`; 142: 3 `CenterAlign`, 2 `BottomAlign` |
| 76 | 222: 952 `AscentOffset`, 42 `EmboxHeight`, 6 `XHeight`; 142: 584, 1 and 22; 162: 516 `AscentOffset`, 4 `EmboxHeight` |
| 116 | 3 distinct values in each layout |
| 118 | 142: 5 distinct values |
| 120, 122 | 142: 6 true, 6 distinct heights |
| 130, 132 | 142: 2 true, 3 distinct widths |

In the 162-byte layout the justification and auto-sizing fields have one
value in every style, and in the 106-byte layout (28 styles) every field
but offset 76 does; the converter assumes the offsets of the larger
layouts for 162 bytes and reads only offset 76 from 106 bytes.
`IgnoreWrap`, `MinimumFirstBaselineOffset`, `VerticalThreshold`,
`UseFlexibleColumnWidth`, `TextColumnMaxWidth` and
`UseNoLineBreaksForAutoSizing` have one value in every style (false or
0), so no field can be shown for them; the converter writes the observed
value (`idml-values.md`). Code 4 of `FirstBaselineOffset` is shown only
in styles; the converter reads it in frames too.

Footnote and column rule values: 305 of 305 styles whose chunk has them.
The column rule colour maps one to one over 41 (file, UID) pairs. Only
one style (3 copies) has unequal insets, and it shows only the top inset
(offset 50), so the converter writes `InsetSpacing` only when all four
are equal (334 of 337).

**Story settings.** Frame type: 0 `Unknown`, 1 `TextFrameType`, 2
`FrameGridType`. Orientation: 0 `Unknown`, 1 `Horizontal`. Direction: 1
`LeftToRightDirection`, 0 or no chunk `UnknownDirection`. 337 of 337
each. The f64 at 2 is 12 in every style, like `OpticalMarginSize`, and is
not used.

**Text wrap.** 234 styles have chunk 0x3776 with mode 0, offsets 0 and
the u32 at 40 = 1, and chunk 0x3777 type 5: IDML `None` with
`ContourType="SameAsClipping"`. 103 styles have neither chunk and IDML
`None`. Chunk 0x3777 is the contour with the layout of chunk 0x373D
(text wrap, contour). In the trustworthy pairs of the corpus of 2026-10,
3,965 object styles have it: 3,964 with type 5 and one with type 0, whose
IDML style is the only one with `ContourType="BoundingBox"`. Every other
IDML object style, with or without the chunk, has `SameAsClipping`
(5,555 of 5,556 styles), so a style without the chunk takes type 5.
`ContourPathName` is `$ID/` in every `ContourOption` of the object
styles of the corpus pairs (7,423 of 7,423, DOM 7 to 21).

**Categories (chunk 0x1B92E).** An ID in the list means the attribute is
`true`. Over the 1,684 object styles other than `[None]` of the
trustworthy pairs whose name matches an IDML style (fewer for attributes
that IDML has only from a later version), each attribute below equals
the presence of its ID in every style:

| ID | Attribute | From DOM |
|---|---|---|
| 0x1B933 | `EnableFill` | 7 |
| 0x1B934 | `EnableStroke` | 7 |
| 0x1B935, 0x1B936 | `EnableStrokeAndCornerOptions` | 7 |
| 0x1B93E | `EnableTextFrameGeneralOptions` | 7 |
| 0xADC8 | `EnableTextFrameBaselineOptions` | 7 |
| 0xADC9 | `EnableTextFrameAutoSizingOptions` | 8 |
| 0x1B940 | `EnableStoryOptions` | 7 |
| 0x1B960 | `EnableFrameFittingOptions` | 7 |
| 0x1B93F | `EnableParagraphStyle` | 7 |
| 0xADCB | `EnableTextFrameColumnRuleOptions` | 15.1 |
| 0xCA2F | `EnableAnchoredObjectOptions` | 7 |
| 0x1B942, 0x37C8, 0x37C9 | `EnableTextWrapAndOthers` | 7 |
| 0xADCA | `EnableTextFrameFootnoteOptions` | 12 |
| 0x6EA1 to 0x6EA7 | `EnableTransformAttributes` | 13 |
| 0x1B97A, 0x1B97C to 0x1B97E | `EnableExportTagging`, `EnableObjectExportAltTextOptions`, `EnableObjectExportEpubOptions`, `EnableObjectExportTaggedPdfOptions` | 9 |
| 0x1B937 | `EnableTransparency` of `ObjectStyleObjectEffectsCategorySettings` | 7 |
| 0x1B948, 0x1B950, 0x1B958 | `EnableTransparency` of the fill, stroke and content effects categories | 7 |

Where a row has several IDs, they are all present or all absent in every
style, so the corpus does not tell which one the attribute follows; the
converter writes the attribute only when they agree.

The transparency categories do not change which transparency values
IDML writes on a style (`transparency.md`, object styles).

Versions are those of the INDD header, major and minor. Over the 489
trustworthy pairs: `EnableTextFrameColumnRuleOptions` is on no style in
the 42 documents of 15.0 (four builds), and on every style in the 7 of
15.1 and on all later ones. The footnote values of the text frame
settings (`FootnotesSpanAcrossColumns`, `FootnotesMinimumSpacing`,
`FootnotesSpaceBetween` of `TextFramePreference`) are on no style of
DOM 12 (127 styles) or 13.0 (12 styles), and on every style from 13.1.

**The root style `[None]`** has none of the `Enable…` attributes and none
of the four `ObjectStyle…EffectsCategorySettings` elements in 489 of 489
trustworthy references, though its INDD object has the category list.
The converter still reads the list: the text frames that use `[None]`
depend on it (text frame preferences). The four export
attributes are equal in every style (84 true, 1,358 false). With the 88
pairs of the earlier corpus, fill and stroke, and the general and
baseline frame options, were not told apart; the larger corpus has
styles that differ in them (10 without fill, 13 without stroke, 506
without general options and 531 without baseline options).

**Anchored object settings** are in chunk 0x2800, as for anchors (see
stories). A style without the chunk (6 styles of versions 10.1 and 10.2)
has the values of preferences without it (`preferences.md`, anchored
objects).
The other values of object styles come from IDML observation
(`idml-values.md`).

**Transform attributes (chunk 0x1E234).** Over the 1,251 trustworthy
pairs after 2026-10, 2,426 object styles have the chunk, 52 bytes each.
The u16 at 8, 10, 12 and 14 are 1 exactly where IDML writes
`TransformAttrX`, `TransformAttrY`, `TransformAttrHeight` and
`TransformAttrWidth`, and the f64 at 16, 24, 32 and 40 are their
values: 3 of 3 X, 1 of 1 Y, 11 of 11 height and 30 of 30 width values
(36 styles; 2,390 styles have all four flags 0 and no such attribute,
though many store values other than 0). Bytes 0–7 and 48–51 are 0 in
every chunk. IDML has the element from DOM 13 (`idml-values.md`); the
converter writes the values the style sets into it.

## Master spread names and sections

Master spread chunk 0x1402: a flag byte and the name prefix (`A`), a flag
byte and the base name (`Master`). IDML `Name` is `<prefix>-<base>`, and
the master's pages are named by the prefix (99/99 match).

**Sections (class 0x4C01).** The document's chunk 0x4C01 lists them.
Section chunk 0x4C02:

| Field | Contents |
|---|---|
| u8, string | `SectionPrefix` |
| u8, string | `Marker` |
| u32 | First page of the section; 0 for the section that starts at the document's first page |
| u32 | Page number start (`PageNumberStart`) |
| u32 | Page number style: 0x4C15 = `Arabic`, 0x4C17 = `LowerRoman`, 0x4C12 = `Kanji`, 0x4C13 = `DoubleLeadingZeros`, 0x4C16 = `UpperRoman`, 0x4C19 = `LowerLetters`, 0x4C1A = `SingleLeadingZeros` (below) |
| u32 | Continue numbering (1 = true, `ContinueNumbering`) |
| u16 | 0 in every chunk |
| u16 | Include section prefix (1 = true, `IncludeSectionPrefix`) |
| u8 flag, string | `AlternateLayout`: the string, with `$ID/` before it if the flag is 1 |

In the 546 sections of the trustworthy pairs, 539 chunks have all these
fields; the prefix, marker and alternate layout equal the IDML values in
539 of 539 (417 named layouts such as `Letter V`, 82 empty, 40 `$ID/`).
The 7 shorter chunks are in DOM 7 and 8 files and end after the second
u32 above.

**Include section prefix.** Over the 1,345 sections of the 1,252
trustworthy pairs after 2026-10, 1,336 chunks have the u16 pair after
continue numbering. The first u16 is 0 in all of them. The second is 1
in all 11 sections with IDML `IncludeSectionPrefix="true"` (10 distinct
files) and 0 in all 1,325 with `false`. The other 9 chunks, one each in
9 DOM 7 files, end before the field; IDML gives them `false`.

**Section name.** IDML `Name` equals `SectionPrefix` in all 1,345
sections: 15 sections in 14 distinct files have a prefix, and 1,330
have an empty prefix and an empty name. The prefix does not depend on
`IncludeSectionPrefix` (9 of the 15 include it, 6 do not).

**Alternate layouts.** A section with a layout name (not empty and not
`$ID/`) starts an alternate layout, and so does the first section.
`AlternateLayoutLength` of a section is the number of pages from its
first page to the start of the next layout, or to the end of the
document (538 of 539). The `AppliedAlternateLayout` of a document page
is the section that starts its layout; master pages have `n`. Both are
in IDML from DOM 8 on. Over the trustworthy pairs the converted
`AppliedAlternateLayout` equals the IDML value for 9,347 of 9,347 pages.

**Page descriptor.** A document page has a `Descriptor` property, a list
of the section prefix, the page number style, `ContinueNumbering`,
`IncludeSectionPrefix`, the page number and the marker; from DOM 20 on
the page number is in the list twice. Over the trustworthy pairs: 7,800
of 7,924 pages; 64 others are in sections with a style the converter
does not know (IDML gives it as an empty string), and the converter
leaves their descriptor out.

In the descriptor, a return (U+000D) in the marker is a paragraph
separator (U+2029); the section's `Marker` attribute keeps the return.
Over the 15,855 pages of the trustworthy pairs after 2026-10, 568 pages
(6 distinct files) are in sections whose marker ends with a return, and
their descriptors end with U+2029 in its place (568 of 568). No marker
has a return elsewhere. 1,080 pages have a marker without a return,
which the descriptor repeats as it is, and 14,207 have an empty marker.

Evidence:

- In every corpus pair the first section's IDML `PageStart` is the
  document's first page, and its first-page field is 0. All but one pair
  have one section; one has four. All 79 sections that the converter
  writes match on `PageStart`, `Length`, `ContinueNumbering` and, where
  IDML writes it, `PageNumberStart`. IDML writes `PageNumberStart` only for sections with
  `ContinueNumbering="false"` (14 of 14); a continuing section can store a
  start number that IDML leaves out.
- 84 distinct INDD files without an IDML have more than one section (254
  sections). The first section listed has first page 0 in all 84. Each of
  the other 170 names a page of the document (170 of 170). The list is not
  always in page order, so the converter sorts sections by their first
  page. A section ends where the next one starts; that gives IDML
  `Length`.
- The style is 0x4C15 in all 423 sections of the 250 distinct
  little-endian files, and those in the pairs are `Arabic`. The
  InDesign 3.0 file stores 0x4C06 (`big-endian.md`).
- Style 0x4C17 is lower-case Roman. No pair has it. The evidence is a
  sample and its print PDF. The PDF's page labels (`/PageLabels` in the
  document catalog) give the pages of the sample's 0x4C17 sections style
  `/r`, lower-case Roman, and the pages of its 0x4C15 section style
  `/D`, decimal.
- Style 0x4C12 has no pair either. The evidence is a sample typeset
  vertically and its print PDF. The folios on the pages of its 0x4C12
  section are Chinese numerals written digit by digit (〇 一 二 … 九,
  so 10 is 一〇 and 100 is 一〇〇), counting from the section's start
  number. The PDF's page labels belong to a larger document and do not
  show the style. `Kanji` is the only value of the schema's
  `PageNumberStyle` enumeration that uses such numerals.

Pairs of the corpus after 2026-10 show four more codes. Over every
valid distinct file, each section whose code the converter did not know
was matched with the IDML `Section` of the same UID:

| Code | IDML `PageNumberStyle` | Sections with an IDML section | Sections without |
|---|---|---|---|
| 0x4C1A | `SingleLeadingZeros` | 51 | 30 |
| 0x4C13 | `DoubleLeadingZeros` | 21 | 3 |
| 0x4C19 | `LowerLetters` | 14 | 2 |
| 0x4C16 | `UpperRoman` | 3 | 10 |

No section has another value for these codes. In the `Descriptor` of a
page, whose second item is the section's style, IDML writes a page of a
`SingleLeadingZeros` section with a string there, the section prefix
again: 1,249 descriptors in the IDML files of the corpus (1,223 with an
empty prefix), none with the enumeration value. 0x4C18 (one section) and
0x4C06 (InDesign 2.0 and 3.0 files) occur only in files without IDML.
The converter writes `PageNumberStyle` for the seven codes and leaves it
out (with a warning) for other codes.

The converter names document pages by their number in their section,
counting on from the previous section when numbering continues. The
section at the document's first page starts at its page number start
even when it continues numbering: the document continues the numbering
of a book. Over the 1,252 trustworthy pairs after 2026-10, the IDML
page number of the first page equals that field in 1,252 of 1,252,
including 18 documents (18 distinct files) whose first section
continues numbering from a start above 1. The 49 later sections that
continue numbering count on from the previous section (49 of 49), though
41 store another start. In a section with `IncludeSectionPrefix`, the
page name is the section prefix followed by the number: `2-1`, `2-2`
for prefix `2-`, and `1장_01` for a prefix `1장_` and style 0x4C1A.
After these rules, page names and descriptors equal the IDML values on
15,855 of 15,855 pages of those pairs. In a
section with style 0x4C17 it writes the number in lower-case Roman (i,
ii, iii), as in the PDF labels above, and in a section with style 0x4C12
in Chinese digits, as the folios show. No pair shows IDML page names in
such sections. It writes upper-case Roman numerals for 0x4C16, two and
three digits with leading zeros for 0x4C1A and 0x4C13, and the letters a
to z for 0x4C19 (later numbers as Arabic numbers); the page names of the
pairs above measure these.

## Document preferences (class 0x2202)

One object of class 0x2202 holds document-wide preferences. Chunk 0x533
(`DocumentPreference`): f64 page width at 0, f64 page height at 8, byte 58
2 for facing pages and 1 otherwise, four f64 bleeds from offset 70 (inside,
top, outside, bottom; `preferences.md`), u32 intent at 142 (0 print, 1
web, 2 mobile). All values match the 75 pairs. Files from InDesign 7.0
and 7.5 have a 126-byte chunk with the fields at other offsets
(`preferences.md`, document setup). Files from InDesign 3.0 and 4.0 have
a 118-byte chunk with another layout; the converter leaves it out with a
warning (`big-endian.md`).

**Page binding (u16 at 64).** 0 is `PageBinding="LeftToRight"`, 1
`RightToLeft`. Evidence:

- The u16 is 0 in all 247 distinct little-endian public files that have
  the 146-byte chunk. All 88 distinct corpus IDML files have
  `PageBinding="LeftToRight"`, and in all 73 two-page spreads and master
  spreads of those files the first page lies left of the second.
- A sample typeset vertically, whose print PDF shows columns ordered
  right to left, stores 1. In each of its two-page spreads and master
  spreads the first page lies right of the second: the book is bound on
  the right. The schema's other value, `Default`, has no sample.

The converter writes `PageBinding` from this field and leaves it out,
with a warning, for other codes. Pages are written in the order the
spread lists them, for both bindings.

## Text frame preferences

From the frame's multi-column frame object (class 0x263):

| Chunk | Offset | `TextFramePreference` attribute |
|---|---|---|
| 0x2D1 | u32 0 | `TextColumnCount` (1,052/1,052) |
| 0x2D1 | f64 4 | `TextColumnGutter` (93/93) |
| 0x2D1 | f64 14 | `TextColumnFixedWidth` (1,045/1,045) |
| 0x2CE | u16 0 | `FirstBaselineOffset`: 0 LeadingOffset, 1 AscentOffset, 2 CapHeight, 3 EmboxHeight, 4 XHeight (below) |
| 0x2CE | u16 2 | `VerticalJustification`: 0 Top, 1 Center, 2 Bottom, 3 Justify (350/350) |
| 0x2CE | u16 20 | `VerticalBalanceColumns` (18/18) |
| 0x2CE | u16 22 | `AutoSizingType`: 0 Off, 1 HeightOnly, 2 WidthOnly, 3 HeightAndWidth (61/61) |
| 0x2CE | u16 24 | `AutoSizingReferencePoint`: 0–8, top-left to bottom-right by rows (58/58) |
| 0x2CE | u16 26, f64 28 | `UseMinimumHeightForAutoSizing`, `MinimumHeightForAutoSizing` |
| 0x2CE | u16 36, f64 38 | `UseMinimumWidthForAutoSizing`, `MinimumWidthForAutoSizing` |
| 0x2CE | u16 46 | `UseNoLineBreaksForAutoSizing` |
| 0x2D1 | u8 12 | `UseFixedColumnWidth` |
| 0x2D1 | f64 32 | `TextColumnMaxWidth` (in 40-byte chunks) |
| 0x3730 | u16 | `IgnoreWrap` |
| 0x22646 | u32 0 | `ColumnRuleOverride`: 1 true, 0 false (below) |
| 0x22646 | f64 28, u32 36 | `ColumnRuleStrokeWidth`, `ColumnRuleStrokeColor` (a swatch; 0 = `n`) |
| 0x22608 | u16 2, f64 4, f64 12 | `FootnotesSpanAcrossColumns` (1 = true), `FootnotesMinimumSpacing`, `FootnotesSpaceBetween` |
| 0x2834 | 24 bytes | `BaselineFrameGridOption` (below) |

**Span of footnotes.** The u16 at 2 of chunk 0x22608 is
`FootnotesSpanAcrossColumns`. Before 2026-10, 2,655 frames with 0 have
`false` in IDML; the 22 frames with 1 (8 documents) all have an object
style with span `true` and the footnote category on, and IDML leaves the
attribute out, as for any value equal to the style's. Writing it from
the chunk removes the 82 extra `FootnotesSpanAcrossColumns` values of
the trustworthy pairs after 2026-10 and adds no wrong value.

**Frames without chunk 0x2CE, and the 22-byte chunk.** Frames of
InDesign 14.0 and 15.0 can lack chunk 0x2CE (517 frames in 24 documents
before 2026-10). Their IDML has `FirstBaselineOffset="LeadingOffset"`
on all 517, although their styles have `AscentOffset`; where IDML writes
the other values they are `TopAlign`, `false`, `Off`, `CenterPoint`,
minimums off and 0, and no line breaks off. These are codes 0, reference
point 4 and no minimums. InDesign 7.0 frames can have a 22-byte chunk
0x2CE (125 frames, 4 documents): offsets 0, 2 and 20 are as in the
longer chunk (11 of 11, 11 of 11 and 13 of 13 values); it has no
auto-sizing fields, and DOM 7 IDML has no auto-sizing attributes. The
converter writes these frames' `TextFramePreference` with those values:
over the trustworthy pairs after 2026-10, 670 more elements, with no
wrong or extra value (`FirstBaselineOffset` 2,731 of 2,731,
`AutoSizingType` 16,202 of 16,202).

**First baseline of InDesign 2.0.** Frames of InDesign 2.0 have a
26-byte chunk 0x2CE. In three privately held 2.0 documents (above, the
story list of InDesign 2.0) the u16 at 0 is 0 in 95 of 95 frames, but
the first lines are not set as code 0 (`LeadingOffset`) sets them. In
their print PDFs, the first baseline of the first frame of a story lies
below the frame's top inset by the height of the font's `d`, which is
what `AscentOffset` sets: over the 36 such frames whose font is embedded
with a `d`, the offset divided by the point size equals the `d`'s top
divided by the em in 27 (within 0.004; Times New Roman regular, italic
and bold italic, Garamond bold and bold italic, Palatino) and is within
0.015 in 4 more (Arial: 0.728, `d` 0.716). Five frames are further off
and are not explained (very large type, a frame whose first line is not
the first text). None is one leading (about 1.2 times the size) below
the top. The converter writes `FirstBaselineOffset="AscentOffset"` for
every 2.0 frame. With `LeadingOffset`, each frame's first line sat about
6 pt too low, and short frames such as running heads could not hold
their text.

The frame itself (class 0x6201) holds the inset spacing in chunk 0x3723
(44 bytes): f64, u32, then four f64 (left, top, right, bottom). IDML
writes `InsetSpacing` as a list of top, left, bottom and right, or as
the f64 at offset 0 (below). A frame without the chunk has 0 for all
five.

Evidence over the text frames of the trustworthy pairs that the
converter writes (`TextFramePreference` values reproduced): the
auto-sizing minimums and their flags 4,069 to 4,071 of 4,069 to 4,071
each (5 frames with 108 pt minimums, 4 without line breaks),
`UseFixedColumnWidth` 747 of 747 (9 true), `TextColumnMaxWidth` 23,043
of 23,043 (one frame with 210.24; the 30-byte chunks of DOM 7 files have
no such field and their IDML no such attribute), `IgnoreWrap` 643 of the
817 frames that have it (74 true; frames without the chunk have `false`,
which the converter writes: 1,871 of 1,871 values in the corpus after
2026-10), column rule width and colour
7,277 and 7,323 of 7,277 and 7,323, footnote spacings 4,351 of 4,351
each. Without chunk 0x22646 the IDML has width 1 and `Color/Black`
(2,816 frames); without chunk 0x22608, 12 and 6 (1,430 frames). The
column rule and footnote settings are written from the versions in
which IDML has them (`idml-values.md`).

`InsetSpacing`: IDML writes it as a list where the frame is a plain
rectangle and as one number (`<InsetSpacing type="unit">15</InsetSpacing>`)
where it is not. A plain rectangle is a frame whose path (as written in
`PathGeometry`) is one path of exactly four straight points (both
direction points on the anchor), each sharing its x or y coordinate with
the next (compared to 1e-4 pt), so that they are the corners of a
rectangle along the axes of the frame's inner coordinates, and whose
corner options in effect (its own `TopLeftCornerOption` to
`BottomRightCornerOption`, else its object style's along `BasedOn`) are
all `None`. Chunk 0x3723 holds no shape flag: its u32 at offset 8 is a
UID (the frame's UID plus a few in most frames). Evidence over the text
frames of the trustworthy pairs (except `own/`), in spreads, master
spreads and stories (anchored frames):

| Frame | List | One number | None |
|---|---|---|---|
| Plain rectangle | 40,603 | 0 | 6,551 (DOM 7 to 10) |
| Four-point rectangle with a corner option in effect | 0 | 584 | 0 |
| Curved path | 0 | 771 | 0 |
| Straight path of 2 or 3 points | 0 | 29 | 0 |
| Straight path of 4 points, not along the axes | 0 | 131 | 0 |
| Straight path of 5 to 7 points | 4 | 77 | 0 |
| Straight path of 9 to 32 points | 0 | 124 | 0 |

The tolerance matters: 4 plain rectangles have corners whose coordinates
differ in the last digit (-2395.597908385931 and -2395.597908385932),
and IDML writes them as lists. A path with more than four points is not
a plain rectangle even when its extra points add no corner: 32 of the 77
frames with 5 to 7 straight points reduce to a rectangle once repeated
points and points on a side are removed (a closed path that repeats its
first point at the end, open paths that repeat the first point, a
7-point path with 3 extra points on one side, a 6-point path with an
extra point on the bottom side), and all 77 are one number (29 with 5
points, 34 with 6, 14 with 7). The 4 lists among them are exceptions,
all with 5 points: 3 copies of one frame in one document, and one frame
whose closing point differs from its first by 3.5e-15.

The one number is the f64 at offset 0 of chunk 0x3723, whatever the
four insets are, equal or not. In one DOM 15 document, 101 anchored
star-shaped frames (10 points) have 0 at offset 0 and four insets of 9,
and two curved frames (44 and 28 points) have 0 at offset 0 and insets
of 36, 0, 25.2 and 0; IDML writes `0` for all 103. In every other
one-number frame of the trustworthy pairs the f64 at offset 0 equals
the four insets.

From DOM 11 every frame has `InsetSpacing`. Before DOM 11 a frame
that is not a plain rectangle always has it: all 200 such frames of the
trustworthy pairs of DOM 7 to 10 (one number: 0, 4, 4.94, 5 or 15). For
a plain rectangle it follows the rule of the general category below
(857 frames with it, 6,551 without): IDML writes it where
the frame's insets differ from its object style's (chunk 0x1B924 at 34,
compared as a set of four values, since the order of the style's
fields is not shown) or the style turns the category off. Over the
trustworthy pairs of the corpus after 2026-10 the converter reproduces
42,337 of 42,341 `InsetSpacing` values; the 4 that differ are the list exceptions above.

`MinimumFirstBaselineOffset` is 0 on every frame and object style of the
corpus IDML files, so its field cannot be found. IDML writes it on a
frame exactly where the baseline category rule below writes the
baseline attributes; the converter writes `0` under that rule: 1,753 of
1,753 values, none extra.

**Values written on frames.** IDML writes an attribute of a frame's
`TextFramePreference` when its category is off in the frame's object
style (the category ID is not in the style's list, chunk 0x1B92E, also
for `[None]`), or when the frame's value differs from the style's
value. The style's values are its own, or, if it has no text frame
settings, those of the style it is based on, then `[None]`'s.

| Category | ID | Attributes |
|---|---|---|
| General | 0x1B93E | `TextColumnCount`, `TextColumnGutter`, `TextColumnFixedWidth`, `UseFixedColumnWidth`, `UseFlexibleColumnWidth`, `TextColumnMaxWidth`, `VerticalJustification`, `VerticalThreshold`, `IgnoreWrap`, `VerticalBalanceColumns`, `InsetSpacing` |
| Baseline | 0xADC8 | `FirstBaselineOffset`, `MinimumFirstBaselineOffset` |
| Auto-sizing | 0xADC9 | the seven auto-sizing attributes |
| Footnotes | 0xADCA | the four `Footnotes…` attributes |
| Column rules | 0xADCB | the ten `ColumnRule…` attributes |

Evidence over the 489 trustworthy pairs. Frames with style `[None]`
write a whole category in a document if and only if `[None]`'s list
lacks the ID (counting documents with such frames):

| Category | ID missing | ID present |
|---|---|---|
| Auto-sizing (DOM 8 on) | written in 28 of 28 documents | written in 0 of 224 (one more has frames whose values differ) |
| Footnotes (13.1 on) | 21 of 21 | 0 of 153 |
| Column rules (DOM 15 on) | 35 of 35 | 0 of 64 |
| General, baseline | (ID present in all 254) | 0 of 254 |

Exceptions: `TextColumnMaxWidth` is on every frame from DOM 8 (22,775 of
22,775), `TextColumnCount` on every frame from DOM 10 (18,679 of 18,680;
before, it follows the rule: absent in 4,006 frames where it equals the
style), and `InsetSpacing` on every frame from DOM 11. The footnote
values are on no frame before 13.1: none of the 3 documents of 13.0 has
them, though one has frames whose style has the category off.

Applied to the converter's frame values and the reference styles'
values, the rule leaves out 379,953 values that the IDML does not have,
keeps the 200,434 it has, and leaves out one value that the IDML has (a
`TextColumnFixedWidth` equal to its style's).

`ColumnRuleOverride`: the u32 at offset 0 of chunk 0x22646 (60 bytes).
In the conversions of the 1,251 trustworthy pairs after 2026-10, 11,319
frames have the chunk: 2 frames (a 15.0 and an 18.4 document) have 1,
and IDML has `ColumnRuleOverride="true"` on both; the other 11,317 have
0, and IDML has `false` wherever it writes the attribute. The converter
writes `true` for 1 and `false` otherwise, also for frames without the
chunk: 21,167 of 21,167 values, no extra value. The same 2 frames are
the only ones whose chunk 0x2265A (32 bytes) is not all zero, so the
corpus cannot tell the two chunks apart; offset 0 of 0x22646 is taken
because the chunk holds the other column rule values of the frame.

**Baseline frame grid of text frames.** Chunk 0x2834 of the
multi-column frame (24 bytes) holds the frame's baseline grid:

| Offset | Type | `BaselineFrameGridOption` attribute |
|---|---|---|
| 0 | u16, 1 = true | `UseCustomBaselineFrameGrid` |
| 2 | f64 | `StartingOffsetForBaselineFrameGrid` |
| 10 | u16: 0 `TopOfPage`, 1 `TopOfMargin`, 2 `TopOfFrame`, 3 `TopOfInset` | `BaselineFrameGridRelativeOption` |
| 12 | f64 | `BaselineFrameGridIncrement` |
| 20 | u32 | UID of an interface colour (class 0x1F11), 0 for the document's colour: `BaselineFrameGridColor` |

Without the chunk the values are `false`, 0, `TopOfInset`, 12 and the
document's colour (187 of 187 frames before 2026-10). The text frame
settings of an object style (chunk 0x1B924) hold the same fields at
offset 82 (u16 82, f64 84, u16 92, f64 94, u32 102).

IDML writes the element by the category rule above with category
0xADC8: when the category is off in the applied style, or the frame has
no style object, it writes all four attributes and the colour; otherwise
each attribute whose value differs from the style's, and the colour when
the frame's colour UID differs from the style's. A frame with nothing to
write has no element. The colour is the document's baseline frame grid
colour (`preferences.md`) for UID 0, otherwise the interface colour named
as for layers. The element has the colour as its only `Properties`
child and follows `TextFramePreference`.

Evidence before 2026-10: all 23,940 text frames of the 606 trustworthy
pairs (category off: 749 of 749 with every value; no style object: 4 of
4; category on with values that differ: 269 of 269 with exactly those
values; only the colour UID differs: 32 of 32; nothing differs: no
element in 22,886 of 22,886). One frame with offset 118 places the
starting offset at 2. Object styles: 1,412 of 1,412 match their
`ObjectStyle/BaselineFrameGridOption`.

Over the 1,251 trustworthy pairs after 2026-10, the converter's elements
by this rule are 1,961 of the 1,961 reference elements on the frames it
writes (the other 372 are on frames it does not write), with no extra
element. All attributes are reproduced (1,692 to 1,915 each); 1,685 of
1,727 colours are, and the 42 others are on frames with UID 0 in
documents whose document colour the converter gets wrong
(`preferences.md`). The object styles' four values are reproduced in
5,556 of 5,556 styles; code 1 `TopOfMargin` occurs in 6 styles of 6
documents, with `UseCustomBaselineFrameGrid="true"`.

**Text orientation (chunk 0x2DE).** The multi-column frame holds a
matrix (six f64) in chunk 0x2DE. Its first four values are 1 0 0 1 for
horizontal text and 0 1 −1 0 (a quarter turn) for vertical text. IDML
has no frame attribute for this; it writes the orientation on the story
(`StoryPreference`, `StoryOrientation`). Evidence:

- All 18,207 multi-column frames in the 239 distinct little-endian
  public files that have text frames have 1 0 0 1. All 2,094 stories in
  the corpus IDML files have `StoryOrientation="Horizontal"`.
- A sample typeset vertically has 0 1 −1 0 in the frames of some
  stories and 1 0 0 1 in the others. In its print PDF, the text of every
  frame with the quarter turn is set in vertical columns ordered right to
  left (glyphs of a column share an x position and follow each other
  downwards), and the text of every frame with 1 0 0 1 is set in
  horizontal lines, such as the running heads. The frames of one story
  always agree.
- In the rotated frames, `TextColumnFixedWidth` (chunk 0x2D1) equals
  the frame's height, not its width: the columns run down the page.

The last two values of the matrix are not identified. The converter
writes `Vertical` for a story whose frames all have the quarter turn,
and otherwise `Horizontal`, the value every IDML has. A story whose
frames disagree or have another matrix gets a warning. Two frames in
the InDesign 3.0 file have no chunk 0x2DE; they do not count.

**First baseline offset.** IDML writes `FirstBaselineOffset` on a frame's
`TextFramePreference` only when it differs from the frame's object style.
Over the frames of the corpus pairs, compared with the value written on
the frame or, if absent, on its object style:

| Code | IDML value | Frames |
|---|---|---|
| 1 | `AscentOffset` | 1,121 of 1,121 (13 on the frame, 1,108 from the object style) |
| 2 | `CapHeight` | 10 of 10 (copies of one document) |

No pair has code 0. The evidence for it is a sample and its print PDF. In
the PDF, the first baseline in each frame with code 0 lies below the
frame's top edge by exactly the leading of that first line, for first
lines with several different leadings. Of the values in the IDML schema
(`AscentOffset`, `CapHeight`, `LeadingOffset`, `EmboxHeight`, `XHeight`,
`FixedHeight`), only `LeadingOffset` depends on the leading.

Code 3 is `EmboxHeight`. No public file has it: the 18,207 multi-column
frames of the 239 distinct little-endian public files have code 1 or 2
only. The evidence is a sample and its print PDF, and a sample typeset
vertically with the same print PDF:

- The frames measured have code 3, `TopAlign` and one line. No chunk
  of these frames or of their multi-column frame objects holds the
  measured distance as a number, so it is not a stored minimum offset.
- A first line set in a CJK font lies below the frame's top edge by
  0.880 of its point size. The font's ideographic em box (its `BASE`
  table) has its bottom at −0.120 em, so the em box top is at 0.880 em.
  The font's cap height (0.742 em), its x-height (0.503 em), the leading
  and a fixed offset of 0 give other distances. Its ascent is also
  0.880 em.
- A first line set in a Latin font lies below the frame's top edge by
  0.825 of its point size. The font's ascent is 0.710 em, its cap height
  0.650 em and its x-height 0.400 em; the leading and a fixed offset of
  0 do not match either. So code 3 is not `AscentOffset`, which is code
  1 in any case.
- Of the values in the IDML schema, only `EmboxHeight` remains. The
  Latin font has no em box data; the measured 0.825 em equals an em box
  centred on half the cap height (0.325 + 0.5).

The converter leaves out codes other than 0 to 3.

**Text frame defaults of the document.** The preferences object (class
0x2202) has chunks 0x22646 and 0x22608 with the frame layouts, for the
document's `TextFramePreference`: f64 at 28 `ColumnRuleStrokeWidth` and
u32 at 36 `ColumnRuleStrokeColor` (a swatch UID; 195 of 195 documents
before 2026-10), and u16 at 2, f64 at 4 and f64 at 12 the footnote
span and spacings. Without chunk 0x22646 the values are 1 and
`Color/Black`. Every chunk 0x22608 in the corpus holds 0, 0, 12 and 6,
and every IDML has `false`, `false`, 12 and 6, so its offsets rest on the
frame layout. IDML writes the column rule values from DOM 15 and the
four `Footnotes…` values from 13.1 (save history, above). Over the
trustworthy pairs after 2026-10: column rule width and colour 576 of
576 each; footnote values 860 of 876 each (the other 16 were exported by
InDesign 13.1 from a document the INDD shows as saved by 13.0).

## Kinsoku and mojikumi tables

Kinsoku tables are objects of classes 0x4209 (hard), 0x420A (soft),
0x42B4 (Korean), 0x42B5 (Simplified Chinese), 0x42B6 (Traditional
Chinese) and 0x4204 (custom); mojikumi tables are classes 0x4206 and
0x4203 (custom, below). IDML writes one `KinsokuTable` or
`MojikumiTable` per object in `designmap.xml`, kinsoku tables first,
each in UID order (mojikumi tables of both classes together). Chunk 0x100B is
a flag byte (1 = built-in key, `$ID/`) and the name; `Self` is
`KinsokuTable/<Name>` or `MojikumiTable/<Name>`. Implemented in
`src/model/cjk.rs`.

Evidence: 8 pairs have such objects, and their IDML lists exactly them,
by name and in that order (6 with one table of each kind, 2 with 6
kinsoku and 16 mojikumi tables). The other 70 pairs have neither objects
nor elements.

**Custom kinsoku table (chunk 0x4214).** Five u16 counts, then that many
UTF-16 units for each of five lists: `CantBeginLineChars`,
`CantEndLineChars`, a list not identified, `HangingPunctuationChars` and
`CantBeSeparatedChars`. The evidence is one table in two copies of one
document: the four non-empty lists have different lengths (66, 22, 4
and 3), and each matches the IDML attribute of that length. The layout
parses in all 85 objects of class 0x4204 in the 250 distinct
little-endian files, and the third count is 0 in all of them.

Built-in tables have only `Self` and `Name` in IDML. Mojikumi chunk
0x421E (a u16 from 1 to 16) is not used.

**Custom mojikumi tables (class 0x4203).** Chunk 0x100B holds a flag
byte (0 in all of them) and the name, as for the other tables. Chunk
0x420A holds the settings:

| Field | Contents |
|---|---|
| u32 | 26 in all 835 objects of the 129 little-endian files that have the class |
| u32 | 0 in all |
| u32 *n* | entry count |
| *n* × 34 bytes | entries, below |
| u32, u32 | not identified |
| u16 | `BasedOnMojikumiSet`: 0 `Nothing`; *n* > 0 the built-in table `kMojikumiDefaultName<n>` (attributes.md): 1 `LineEndAllOneHalfEmEnum`, 10 `LineEndAllOneEmEnum`, 16 `SimpChineseDefault` |

Each entry is one IDML `OverrideMojikumiAkiType`:

| Offset | Type | IDML |
|---|---|---|
| 0 | u16 | `TargetMojikumiClass` |
| 2 | u16 | `SideMojikumiClass` |
| 4, 12, 20 | f64 | `Minimum`, `Desired`, `Maximum` |
| 28 | u16 | `CompressionPriority` |
| 30 | u16 | `AkiDoesNotFloat` (0 `false`, 1 `true`) |
| 32 | u16 | `SideIsAfterTarget` (0 `false`, 1 `true`) |

IDML writes the table as `<MojikumiTable Self="MojikumiTable/<name>"
Name="<name>" BasedOnMojikumiSet="…">` with `Properties`,
`OverrideMojikumiAkiList` and one `OverrideMojikumiAkiType` per entry, in
file order, with the attributes in the order `TargetMojikumiClass`,
`SideMojikumiClass`, `SideIsAfterTarget`, `Minimum`, `Desired`,
`Maximum`, `CompressionPriority`, `AkiDoesNotFloat`.

Evidence, in the corpus of 2026-10: 185 tables in 66 pairs. The entry
count matches in 185 of 185 tables, all eight fields of the entries in
file order in 62,439 of 62,439 entries, and `BasedOnMojikumiSet` in 185
of 185 (code 1: 151 tables, 0: 33, 16: 1). Seven more trustworthy pairs
(three of version 10.0, four of 10.0 and 10.1 whose IDML is from 10.2) have one table
with code 10 each, and IDML `LineEndAllOneEmEnum` in 7 of 7. `Self` is
`MojikumiTable/` and the name in all 187 custom tables of the corpus
IDML files. The layout parses in all 835 objects of the corpus; codes
3, 6, 9 and 11 also occur (232 objects, none with an IDML), and code 5
in a sample without an IDML. The built-in tables of these codes occur
in no IDML, so the converter leaves `BasedOnMojikumiSet` out for them.
The code is not the position of the value in the schema's
`MojikumiTableDefaults` list by any rule the corpus proves: the proven
codes 0, 1, 2, 10, 14, 15 and 16 are at their positions, but codes of
other enumerations are not (12 of the 50 text attribute enumerations
the converter maps differ from the schema order). A chunk whose length is not 22 + 34
*n*, or a flag other than 0 or 1, leaves the table out with a warning.
In the 66 pairs, the tables of classes 0x4206 and 0x4203 are listed
together in UID order, after all kinsoku tables.

## Colours (0x1F05)

| Chunk | Contents |
|---|---|
| 0x1F10 | u8 1 if the name is a built-in key; name; u32 flags: bit 0 removable, bit 1 visible, bit 2 editable |
| 0x1F01 | u32 space (5 RGB, 6 CMYK, 7 LAB, 14 HSB, below), u16 count, f64 components: fractions for RGB, CMYK and HSB; for LAB the L, a and b values as IDML writes them |
| 0x1F09 | u32 model: 0 Process, 1 Spot, 2 Registration (below) |
| 0x1F24 | f64 tint value (−1 for a colour, see tints below), then u32 `ColorOverride`: 0 Normal, 1 Specialpaper, 2 Specialblack, 3 Specialregistration, 4 Hiddenreserved |
| 0x1F0A | Alternate colour, as 0x1F01: space 3 none, 6 CMYK, 7 LAB |

After the flags, chunk 0x1F10 has two u32 and the `SwatchCreatorID`
(7937 for colours made in the document, other numbers for colours from a
swatch library, such as 31527). `AlternateSpace` is `NoAlternateColor`
with `AlternateColorValue=""` without chunk 0x1F0A or with space 3;
otherwise the space and the values, CMYK as percentages. Evidence over
the 12,197 colours of the trustworthy pairs whose IDML colour has the
same name: `SwatchCreatorID` 12,197 of 12,197; alternate space and
values 12,197 of 12,197 (7,527 without the chunk, 4,624 with space 3,
33 LAB and 13 CMYK alternates, all library colours).

Space 7 is `Space="LAB"`, with the three stored values as `ColorValue`
(for example `79.2156862745098 38 53`). The corpus IDML files have 92
LAB colours in 40 distinct files: 61 spot colours from swatch libraries
(41 with a LAB alternate) and 31 process colours made in the document.
Over all 803 pairs, `Space` is reproduced
for 18,935 of 18,936 colours and `ColorValue` for 18,926.

Space 14 holds hue, saturation and brightness as fractions. IDML writes
such a colour as `RGB` with `ConvertToHsb="true"`; the 9 such colours in
the pairs (all hue 0, saturation 1, brightness 1) are `255 0 0`. The
converter converts with the usual hexagon formula. Every other colour
has `ConvertToHsb="false"` (4,695 of 4,695 colours of the IDML files
from DOM 16 on, where the attribute exists).

**Colour groups.** From DOM 12 on, every swatch (colour, tint, gradient,
`None`) has `SwatchColorGroupReference`: the `ColorGroupSwatch` of the
colour group that lists it, or `n`. Over the trustworthy pairs from DOM
12 on: colours 9,135 of 9,135, gradients 450 of 450, `None` 353 of 353,
tints 24 of 24. DOM 11 files have the attribute on some swatches only
(639 of 1,584 colours), and the converter writes it from DOM 12 on.

Unnamed colours are referenced by UID (`Color/u93`). All 1,296 colours in
the pairs match on `Model`, `Space`, `ColorValue`, `ColorOverride`,
`Name` and the three flags.

**Which unnamed colours IDML writes.** A colour without a name is in the
IDML only if a value written in the package refers to it (an attribute
or element text `Color/u…`; a tint counts), if a stop of any gradient
refers to it, even of an unnamed gradient that IDML leaves out (below),
or if an entry of class 0x1F05 in one of the two tables of the
page item defaults (`preferences.md`) names it with either UID. Over the
489 trustworthy pairs, of the unnamed colours the converter wrote:

| | In the IDML | Not in the IDML |
|---|---:|---:|
| Referenced by the output | 2,309 | 0 |
| Not referenced, named in a page item defaults table | 564 | 0 |
| Neither | 10 | 8,684 |

All colours, written or not, are in the swatch list of the preferences
object (class 0x2202, chunk 0x1F02), and the colour objects of the two
groups have the same chunks and flags, so neither decides it. Named
colours are always written. Unnamed gradients follow the same rule with
class 0x5503: 457 unnamed gradients are referenced nowhere and named in
no table, and none is in the IDML; 526 of the 548 unnamed gradients
IDML writes meet the rule. Their stops go with them.

The other 22 are named by chunk 0x6E06 of the page item defaults object
(class 0x6E07). The chunk has 34 bytes in all 489 trustworthy documents
before 2026-10: u32 5, u32 0, u32 0, u32 14, then three entries of u16 0
and a u32 UID. Entry 0 names an object of class 0x5533, entry 1 a
gradient and entry 2 a colour (489 of 489 each). IDML writes the
gradient of entry 1 in every document: 480 are unnamed, and all 480 are
in the IDML; none of the 162 unnamed gradients IDML leaves out (in the
documents checked) is in the chunk. The colour of entry 2 is in the
IDML wherever the converter writes it (82 of 82). The converter treats
the swatches of chunk 0x6E06 as named by the page item defaults. With
it, every unnamed gradient of the trustworthy pairs of 2026-10 that IDML
writes is written (783 of 783 `Gradient` elements).

**Model codes.** The 250 distinct little-endian files hold 13,067
colours with chunk 0x1F09: 12,814 with code 0, 250 with code 2 (one
`Registration` colour per file) and 3 with code 1. Two of the three are
copies of one colour in two pairs, and their IDML has `Model="Spot"`;
the third is in a file without an IDML. The converter leaves `Model`
out, with a warning, for any other code.

**Code 3: mixed inks.** The corpus after 2026-10 has 47 colours with
code 3 in 27 files, 16 of them pairs (InDesign 12.1 to 17.0, typeset in
Korean). Their IDML has no `Color` element for them; it lists `MixedInk`
and `MixedInkGroup` elements in `Resources/Graphic.xml`. A colour object
with model code 3 is a mixed ink:

| Chunk | Layout | IDML |
|---|---|---|
| 0x1F09 | u32 3, u32 *n*, *n* ink UIDs (class 0x1F07) | `InkList` (`Ink/` and each ink's `Self` name), `InkNameList` (the ink names, `$ID/` for built-in ones) |
| 0x1F01 | u32 9 (colour space), u16 *n*, *n* f64 fractions | `InkPercentages`: each fraction × 100 in the shortest form that reads back (`30.000000000000004` where 0.3 × 100 gives it) |
| 0x102D | u32 *m*, *m* colour UIDs (the spot colours) | `MixedInkSpotColorList` (`Color/` and the name), `MixedInkSpotColorNameList` (the names) |
| 0x1F10 | as for colours | `Name`, `ColorEditable`, `ColorRemovable`, `Visible`, `SwatchCreatorID` |

In these lists a space of a name is written `%20`. A **mixed ink
group** is a code 3 colour that also has chunk 0x1F24 (f64 −1, u32 5);
its percentages are 0. IDML writes it as `MixedInkGroup` with `Self`,
`Model="Mixedinkmodel"`, the four lists, `Name`, the three flags,
`SwatchCreatorID` and `SwatchColorGroupReference`. The **members** of a
group are colour objects with colour space 9 in chunk 0x1F01, their own
percentages and name, chunk 0x117 the group's UID, and no chunk 0x1F09 or
0x102D: they take the ink and spot lists from the group and have
`BaseColor="MixedInkGroup/<group name>"`. Other mixed inks have
`BaseColor="n"`.

`MixedInk` writes `Self`, `Model="Mixedinkmodel"`, `Space="MixedInk"`,
`InkList`, `InkPercentages`, `BaseColor`, `InkNameList`,
`MixedInkSpotColorNameList`, `MixedInkSpotColorList`, `Name`,
`ColorEditable`, `ColorRemovable`, `Visible`, `SwatchCreatorID` and
`SwatchColorGroupReference`, in this order. `Self` is `MixedInk/` or
`MixedInkGroup/` and the name, escaped as colour names. In `Graphic.xml`
the groups follow the inks, then the mixed inks sorted by `Name` in code
point order (`… 1`, `… 10`, `… 11`, `… 2`; 16 of 16 files, UID order
fails in 3). Fill colours and colour group swatches refer to them by
`Self`.

Evidence: the 16 trustworthy pairs (DOM 12.1 to 17.0): 30 mixed inks
without a group, 48 group members and 3 groups; every attribute above
matches 81 of 81. No tint has a mixed ink as its base colour.

**Tints** are objects of the same class without chunks 0x1F10 and
0x1F01. Chunk 0x117 is the UID of the base colour, and chunk 0x1F24 holds
the tint value in percent (IDML `TintValue`) where a colour has −1,
followed by the colour override as for colours. IDML names a tint after
its base colour and value: `Name="Gold 80%"` and `Self="Tint/Gold 80%25"`
for 80 % of a colour named `Gold`. The black swatch (override
Specialblack) is written in brackets: `[Black] 40%`.

Evidence: all 2,107 colours with a name in the distinct little-endian
files have −1 at the start of chunk 0x1F24. The same files have 26 tints
in 24 files, all with this layout and a named base colour. The pairs have
3 IDML `Tint` elements (2 files, one of them on black), and all 3 match on
`TintValue`, `BaseColor`, `Name` and `ColorOverride`. Their 19 references
(`FillColor` of 15 page items, `StrokeColor` of 3, one gradient stop)
match too.

A tint's `AlternateSpace` and `AlternateColorValue` are those of its
base colour (`NoAlternateColor` and an empty list where the colour has
none): 260 of 260 IDML tints of the corpus of 2026-10 (204 in
trustworthy pairs; 9 with `LAB`).

Tints have no flags chunk, so `ColorEditable`, `ColorRemovable` and
`Visible` (true in all 3 IDML tints) are not written. Brackets are known
only for black; other reserved colours as a tint base do not occur in the
pairs.

The `None` swatch is class 0x6E0B (name in chunk 0x1F30).

**Gradients (0x5503).** Chunk 0x5503: u16 stop count *n*, *n* u32 stop
colour UIDs, *n* f64 locations (0–1), *n* f64 midpoints, u32 type (1
linear). Chunk 0x5505: flag byte, name, u32 flags as for colours. All 99
gradients and 198 stops in the pairs match.

The midpoint stored with stop *i* is the position of the midpoint between
stops *i* and *i*+1, measured from the start of the gradient (0–1), as
for opacity gradient stops (`transparency.md`). IDML writes it on stop
*i*+1 as a percentage of the distance between the two stops:
`Midpoint` = (midpoint *i* − location *i*) / (location *i*+1 − location
*i*) × 100. Evidence:

- Most pairs have two stops at locations 0 and 1, where both readings
  give the same value. Two gradients in one pair have their first stop at
  0.0037; stored midpoints 0.50184 and 0.60147 give IDML `Midpoint` 50
  and 60 with this formula (2 of 2), not 50.18 and 60.15.
- Gradients with more than two stops, or with stops not at 0 and 1,
  occur in 26 files, 392 gradients. Only the two above are in a
  reference IDML. In all 426 stop gaps of non-zero width, the stored midpoint lies
  between the two stops' locations, as a position from the start of the
  gradient must (for example stops at 0, 0.5 and 1 with stored midpoints
  0.25 and 0.75). Written to IDML unconverted, 13 of them were outside
  the 13–87 that the IDML schema allows for `Midpoint`; converted, all
  426 are inside it.
- 8 gaps, all in one file, have width 0 (two stops at the same
  location), so the formula gives no value.

When the computed value is outside 13–87 or there is none, the converter
leaves the `Midpoint` out and warns.

## Inks (0x1F07)

Each ink object with chunk 0x1F0D is one IDML `Ink` in
`Resources/Graphic.xml`, in UID order (314 of 314 inks, 78 of 78 pairs;
the four process inks are UIDs 7 to 10). `Self` is `Ink/<Name>` (973 of
973 inks in the corpus IDML files). Chunk 0x1F0D is a flag byte (1 =
built-in key, `$ID/`) and the name, then 86 bytes. Offsets from the end
of the name:

| Offset | Contents | IDML | Evidence |
|---|---|---|---|
| 14 | f64 | `NeutralDensity` | 314 of 314 (2,786 of 2,786 over the 654 pairs of the later corpus) |
| 26 | u32, one less than the IDML value | `TrapOrder` | 314 of 314 |
| 32 | f64 | `Frequency` | 314 of 314 |
| 40 | f64 | `Angle` | 314 of 314 (0, 15, 27, 45, 63 and 75) |
| 56 | u8 | `ConvertToProcess` (1 `true`) | 2,065 of 2,065 inks of the trustworthy pairs (3 `true`) |

Byte 2 is 1 for the four process inks and 0 for spot inks; its meaning
is not known. The u8 at 56 is `ConvertToProcess` (1 `true`, 0 `false`):
it is 1 for the 3 inks of the trustworthy pairs whose IDML has `true`
(one spot ink, in three copies of one document) and 0 for the other
2,062; no other byte separates them. `InkType="Normal"` and
`PrintInk="true"` are in every IDML ink and are written from that
observation (`idml-values.md`). The schema puts inks after the
colours and before the tints.

In 41 files without an IDML (InDesign 6.0, 13.x and 14.x, from two
font projects), the neutral density of all four process inks, their
only inks, is −1. The IDML schema allows 0.001 to 10 and no corpus IDML
shows what InDesign writes for such an ink, so the converter leaves
`NeutralDensity` out (with a warning) when the value is outside that
range.

## Colour groups (0x1F39)

Chunk 0x13C is a flag byte and the group name, chunk 0x1F60 a UID list
of the group's swatches. The preferences object (class 0x2202) lists the
groups in chunk 0x1F61 (u32 count, UIDs). IDML writes a `ColorGroup` in
`designmap.xml` for each listed group, in that order, with
`Self="ColorGroup/<Name>"`; the first has `IsRootColorGroup="true"`, the
others `false` (77 of 77 pairs; the DOM 7 pair has no chunk 0x1F61 and
no groups). The root group is named `[Root Color Group]` with no `$ID/`,
though its flag byte is 0.

Before DOM 11.3, `Self` is the group's UID (`u<hex>`) instead: in 58 of
the 59 trustworthy pairs of versions 10.0 to 11.2 that have groups (one
11.0 file uses names), and names in 358 of 358 pairs from 11.3 on. The
DOM version decides, not the header: header 11.4 files whose last
session was 11.0 (DOM 11.0) use the UID. With the DOM version the
converter writes (`idml-values.md`, DOM version), the extra `ColorGroup`
elements of the trustworthy pairs fall from 15 values in 5 documents to
6 in 2 (not examined), and 3 more groups are reproduced.

Each swatch of the list is a `ColorGroupSwatch` with
`Self="u<group UID>ColorGroupSwatch<index in hex>"` and `SwatchItemRef`
the swatch's reference (`Swatch/None`, `Color/…`, `Tint/…` or
`Gradient/…`). In 99 of 101 groups the list gives exactly the IDML
swatches, in order; the other two are in the pairs whose IDML was
exported from another save. The converter writes 907 of the 942 IDML
swatches, all with the IDML reference. A swatch without a reference is
left out, and the indices of the others are kept.

## Bullet characters

The preferences object (class 0x2202) lists the bullet characters
offered for lists in chunk 0x1A488: u16 1, u32 count, then for each
bullet u32 character type (0 `UnicodeOnly`, 1 `UnicodeWithFont`, 2
`GlyphWithFont`, as in `attributes.md`), u32 character value, u32 font
family UID (0 = none), a flag byte (1 = built-in key, `$ID/`) and the
font style as an in-object string, then a byte: 0, or 1 followed by four
more bytes. IDML writes one `ABullet` per entry in `designmap.xml`, with
`Self="dABullet<index>"` (the index in hexadecimal: `dABullet9`,
`dABulleta`, `dABulletb` in the 2 documents with more than ten bullets),
`CharacterType`, `CharacterValue` and the
`BulletsFont` (the family name, `$ID/` for none) and `BulletsFontStyle`
properties.

Evidence: all 78 pairs have the chunk; the count equals the number of
IDML `ABullet` elements in 78 of 78 and the records end the chunk. All
396 bullets match on type, value and font style, and 395 on the font;
the other is in the pair whose IDML names the family `Minion Pro (OTF)`
(`fonts.md`).

The byte after the font style is 0 in 7,443 of the 7,447 bullets in the
1,428 distinct little-endian files that have the chunk. In one InDesign
20.0 file (with IDML) it is 1 in four bullets, each with flag byte 2, and
four bytes follow: `00 00 90 01` after `Regular` and `00 00 BC 02` after
`Bold` (read as two u16: 0 and 400, 0 and 700). Their meaning is not
known, and IDML writes nothing for them. With this layout the records end
the chunk in all 1,428 files. In the same file, the same five bytes also
follow `Regular` in `FontStyle` attribute values (0x1B02,
`attributes.md`), whose length prefix includes them. Flag byte 2 occurs in
46 bullets; in that file's IDML its style names have no `$ID/` prefix, as
with flag 0.

## XML tags (0xBF19)

Chunk 0xBF2F is a u32 length and the tag name as text segments (in files
from InDesign 2.0 to 5.0, a flag byte and an in-object string;
`big-endian.md`); chunk
0x117 is the UID of the tag's colour, an object of class 0x1F11 whose
chunk 0x1F01 holds u32 colour space 5 (RGB), u16 3 and three f64
fractions, as for swatches. IDML writes each tag as an `XMLTag` in
`XML/Tags.xml`, with `Self="XMLTag/<Name>"` and the colour as the
`TagColor` property. In all 78 pairs the number of tag objects equals the
number of `XMLTag` elements and the names match (87 tags). IDML lists
them by name, ignoring case (the one pair with more than one tag, 10
tags).

| Red, green, blue | `TagColor` | Evidence |
|---|---|---|
| 0.31, 0.6, 1 | `LightBlue` | 78 tags (`Root`) |
| 1, 0, 0 | `Red` | 1 |
| 0.31, 1, 0.31 | `Green` | 1 |
| 0, 0, 1 | `Blue` | 1 |
| 1, 1, 0.31 | `Yellow` | 1 |
| 1, 0.31, 1 | `Magenta` | 1 |
| 0, 1, 1 | `Cyan` | 1 |
| 0.5, 0.5, 0.5 | `Gray` | 1 |
| 0, 0, 0 | `Black` | 1 |
| 0.6, 0, 0 | `BrickRed` | 1 |

Each colour other than light blue occurs once, all in one file. The
converter leaves out `TagColor` for other colours.

The XML structure (the backing story and the elements) is described in
`xml.md`.
