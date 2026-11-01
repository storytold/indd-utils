# Open items

What the converter does not decode yet, and why. Each row is an item
that is open: no field was found, the value never varies, there is no
sample, or a rule does not explain every document. Items decoded since
are described in the page named in the "See" column; the rows here only
record what is left of them. Values with no field found that nearly
every reference has are written as defaults and listed in
`idml-values.md`, [defaults](idml-values.md#defaults).

Counts are over the 1,251 trustworthy pairs of the corpus of 2026-10
(`docs/measurement.md`), "files" over the distinct corpus files outside
the privately held samples. "Code" is the language code of the last
session of the save history (`objects.md`, save history).

Status values:

| Status | Meaning |
|---|---|
| open: no field found | the IDML value varies, but no stored byte was found that follows it |
| open: never varies | the value is the same in every sample, so its field cannot be told apart |
| open: no pair | the stored value occurs, but no pair shows its IDML value |
| open: not explained | a rule holds for most documents; the exceptions follow no stored value |
| open: two collections only | the rule rests on documents of two related collections |
| open: not examined | the values are left; nobody has looked for their rule yet |
| open: not decoded | the structure is known to exist but is not read |
| open: not stored | the value depends on the layout or the export time |
| not needed | the value depends on the exporting computer or application, not on the document |

## Preferences

| Item | Where | Files | Status | Content and reason | See |
|---|---|---:|---|---|---|
| Highlight flags without chunks 0xCA62, 0xCAD3, 0xCAD4 | `TextPreference` | 5 | open: not explained | written `false`; 4 or 5 documents have every highlight on | `preferences.md`, flags |
| `RulerOrigin` without chunk 0x808 | `ViewPreference` | 8 | open: no field found | `PageOrigin` in 8 of the 1,217 trustworthy pairs without the chunk (13 of 1,505 files); written `SpreadOrigin`, so the vertical guides of their later pages are off by the page widths (37 values in 3 pairs) | `preferences.md`, ruler origin |
| `ColumnDirection` without chunk 0x555 | `DocumentPreference`, `MarginPreference` | 2 | open: not explained | `Vertical` with a horizontal story orientation | `preferences.md`, margins and columns |
| `WatermarkFontColor` without chunk 0x16344 | `WatermarkPreference` | 57 | open: no field found | `Red`, `LightBlue`, `Yellow`, `Cyan` | `preferences.md`, watermark |
| Booklet print settings without chunk 0xAF2 | `PrintBookletPrintPreference` | 24 | open: two collections only | 88 values are the same in all 24, 10 follow the code; not written | `preferences.md`, print settings |
| Grid, gutter, column guide and baseline frame grid colours, print marks, flattener resolutions and endnote separators of two Korean collections | preferences, spreads | about 97 | not needed | code 0x0101 with the values of a Roman edition: the IDML was exported by another installation than the one that saved the file | `preferences.md`, values of the exporting edition |
| `DOMVersion` after an export by a later release | `Document` and every package part | 87 | not needed | the IDML's `product` names a later release than the last session of the save history: 68 sessions of 10.1 with 10.2, 16 of 13.0 with 13.1, 3 others. All but one of the 84 are on platform 3, but other platform-3 sessions of 10.1 and 13.0 have their own DOM version, and the IDML has the INDD's `xmpMM:InstanceID`, so nothing stored shows the export | `idml-values.md`, DOM version |

## Resources and text

| Item | Where | Files | Status | Content and reason | See |
|---|---|---:|---|---|---|
| Object style transparency without a differing sample | chunk 0x1B92C | – | open: never varies | about 45 of the 69 attributes per block, every content block attribute and the stroke, fill and content `KnockedOut`: placed by the block layout and the defaults only, never different from the root's value in a sample; not written | `transparency.md`, object styles |
| Language list | `Language` elements | 42 | open: no field found | IDML lists only the languages in use in 42 documents and all in the others; the language objects are the same in both kinds | |
| Font families IDML drops | class 0x3E03 | 101 | open: no field found | unused families left out of `Fonts.xml`, but many other unused ones kept; no record byte differs; files up to 17.x only | `fonts.md` |
| Fonts IDML adds | `Font` | 97 | not needed | IDML adds every style of a family installed on the exporting computer | `fonts.md` |
| Variable font axes | `Font` | 27 | not needed | `NumDesignAxes`, `DesignAxesName`, `DesignAxesValues` and `DesignAxesRange` come from the font file installed on the exporting computer, the names in its language; no INDD byte holds them. Not written, and left out of the measurement | `fonts.md`, not converted |
| Plugin data | `Story`, `XmlStory`, page items, guides, `Document/WordsFlowPreference` | 23 | not needed | written by a Word round-trip plugin and a Google Docs import plugin of the exporting InDesign; the IDML schema defines none of it, and the plugin list (chunk 0x191 of the class-0x156 object) does not decide it (one of 39 documents names neither plugin and has all of it). The nine attributes `Observers` … `ComponentId` and `WordImportFilterEnabled` have no stored source; `DocsFlowImportSettings` is in story chunk 0x13991E (a JSON string; IDML writes base64 of its UTF-8, 22 of 22), the `WordsFlow…Document` properties are base64 of data in story chunk 0x1632F, and the names of `WordsFlowImportContext` are in story chunk 0x146223. Not written, and left out of the measurement | `docs/measurement.md`, values left out |
| Family name ending in a space, DOM 12 to 20 | `Font/@Name` | 0 | open: no sample | DOM 11 keeps the space in `Name`, DOM 21 drops it (one privately held sample); the converter keeps it before DOM 21 | `fonts.md` |
| `$ID/` PostScript names, builds 523 to 534 and DOM 6.0 | `Font` | 0 | open: no pair | InDesign 7.0 writes `$ID/` up to build 522 (4 pairs) and not from build 535 (6 pairs); the builds between have no sample. IDML of DOM 6.0 has it on all, some or none of a document's fonts | `fonts.md`, built-in PostScript names |
| u16 0 and u16 1 after `SeparatorStyle` | TOC entry, chunk 0x11605 | – | open: never varies | one may be `SortAlphabet` (`false` in all 255 entries) | `objects.md`, table of contents styles |
| u16 after `Level` | TOC entry, chunk 0x11605 | – | open: no pair | 0 in every paired entry, 1 in some entries of files without a pair | `objects.md`, table of contents styles |
| Value 2 of 0x1B09 with no IDML attribute | text attribute | 2 | open: no field found | `NextPageNumber` stored where IDML writes no attribute | `attributes.md`, page number type |
| `BasedOnMojikumiSet` codes 3, 5, 6, 9, 11 | custom mojikumi tables, chunk 0x420A | 22 | open: no pair | the built-in tables `kMojikumiDefaultName<n>` of these codes occur in no IDML; code 5 also in a sample without an IDML; left out | `objects.md`, custom mojikumi tables |
| Several alternate glyph features | text attribute 0x42AE | 0 | open: no sample | every run stores one feature; the IDML form of more than one is not shown, so such values are left out | `attributes.md`, alternate glyphs |
| `ResultText` of other text variables | `TextVariableInstance` | 200 | open: not stored | page numbers, dates and cross-reference page numbers depend on the layout or the export time; the last page number shows `1` in every sample | `text-variables.md` |
| Saving edition rule exceptions | tints, users, endnote and index titles | 9 | not needed | the IDML was exported by another edition than the one that saved the INDD (one trustworthy document, 8 stale pairs) | `objects.md`, saving edition |
| `EndnoteSeparatorText` without chunk 0x2261E | `EndnoteOption` | 2 | open: not explained | tab in a Japanese document of code 0x0101, U+3000 in a Korean document of code 0x0100; the code gives the others | `footnotes.md`, endnote options |
| `CaptionParagraphCount`, `HeaderColumnCellStyle` | root table style, attributes 0x1047A–0x1047C | 2 | open: never varies | IDML has both on the two root styles that have these IDs, but all three are 0, so which ID is which is not shown; not written | `tables.md`, caption position |
| `ColorGroup` `Self` | `ColorGroup` | 2 | open: not examined | 6 extra values: groups whose `Self` differs from the one the DOM version gives | `objects.md`, colour groups |
| Order of stories outside the story list | `StoryList` | 2 pairs | open: not explained | IDML lists them in another order than text order | `objects.md`, stories |
| Paragraph ranges split inside one record | `ParagraphStyleRange` | 17 | open: not explained | 20 stories have 41 range boundaries inside one paragraph-style record, with the same attributes on both sides: 36 at a paragraph end, 5 before a paragraph's U+000D. Examples: every paragraph of a table-of-contents story whose paragraphs hold a hyperlink source; a final empty paragraph. No stored value was found that marks these positions | `objects.md`, stories |
| Character ranges split inside one record | `CharacterStyleRange` | 28 | open: not explained | 39 stories have character range boundaries inside one character-style record, with the same attributes on both sides | `objects.md`, stories |
| Fields of three linked story options | `Story/LinkedStoryOption`, story chunk 0x21D12 bytes 0–5 | 6 | open: never varies | `UpdateWhileSaving="false"`, `WarnOnUpdateOfEditedStory="true"`, `RemoveForcedLineBreaks="false"` and bytes `00 00 01 00 00 00` in all 30 stories; written as these values | `objects.md`, linked story options |
| Word import options without a field | `WordRTFImportPreferences`, chunk 0x12210 | 13 | open: never varies | items 3, 4, 7, 14 to 16, 18 and 19 have one value in all 39 stories and are written as that value; the order within the equal pairs (*s0*, *s1*), (*s14*, *s15*), (*s16*, *s17*) is not shown; code 1 of *s5*, *s11*, *s12* and the list length of version 13 have no sample | `objects.md`, import options of placed text |
| Excel import options without a field | `ExcelImportPreferences`, chunk 0x12213 | 9 | open: never varies | items 0, 1, 2, 4, 5 and 9 have one value in all 9 stories and are written as that value | `objects.md`, import options of placed text |
| Table and cell style mappings | chunk 0x21D30 lists 3 to 5 | 0 | open: no sample | empty in every sample, as is rule code 3; left out with a warning | `objects.md`, style mappings |
| Links of linked page items | `TextFrame`/`Group` `Link` (class 0x21D32), `LinkedPageItemOption` | 1 | open: not examined | 3 links in one trustworthy document; not written | `objects.md`, story links |

## Page items

| Item | Where | Files | Status | Content and reason | See |
|---|---|---:|---|---|---|
| Guides at the spread origin | chunk 0x3308 of guides | 5 | open: not explained | 25 page guides of one collection whose IDML `Location` puts them at 0 in spread coordinates, away from their stored positions | `objects.md`, guides |
| `GuideColor` | chunk 0x3308 of guides | 41 pairs | open: no field found | the u32 at 22 is 6 for guides that IDML colours `Cyan`, `Blue`, `Purple`, `LightBlue` or `BrickRed`; not the layer colour, not `RulerGuidesColor` | `objects.md`, guides |
| `InsetSpacing` list exceptions | chunk 0x3723 of text frames | 2 pairs | open: not explained | 4 frames of 5 straight points that IDML writes as a list, although a frame that is not a plain rectangle has one number | `objects.md`, text frame preferences |
| `KnockedOut` exceptions | attribute 0x1EB6F | 3 | open: not explained | 380 items that store nothing, style `false`, and no IDML value | `transparency.md`, drop shadow knockout |
| Group corner radii | `Group` | 3 | open: not explained | groups of versions 10 to 12 with only `CornerRadius` and `TopLeftCornerRadius`, although every child has all four | `attributes.md`, groups |
| `OverprintFill` of groups | `Group` | 82 pairs | open: not explained | 75 groups have a child without the attribute; not written on groups | `attributes.md`, groups |
| Near-zero crops | `FrameFittingOption` | 84 pairs | open: not explained | IDML leaves out some crops below 1e-10 and keeps others | |
| `Inverse` | chunk 0x3703 | – | open: never varies | `false` on every item | `objects.md`, text wrap |
| First 8 bytes of 0x5A35 | page item dash attribute | – | open: never varies | 0 in every item | `attributes.md`, dashes |
| `MinimumFirstBaselineOffset` | text frame settings | – | open: never varies | 0 on every frame and style; written by the category rule | `objects.md`, text frame preferences |
| Flex padding order | chunk 0x1E244 | – | open: never varies | the four paddings are equal in every style, and so are the two gaps | `objects.md`, object style settings |
| Layer before grouping of one master item | chunk 0x324 | 1 | open: no field found | a rectangle on a master spread whose chunk names a layer and position 1; IDML has position −1 and no `BeforeGroupingLayerReference` | `objects.md`, page item settings |
| Path names in a legacy code page | keys 0x7D and 0x80 of image chunk 0x1708 | 10 pairs | open: not decoded | 11 `PhotoshopPath` images whose path name is stored as code page 949 or Shift-JIS bytes; the file does not record the code page, and `AppliedPathName` is left out | `objects.md`, clipping path settings |
| `Index` of an `AlphaChannel` clipping | chunk 0x2C1A | 1 pair | open: no field found | the image stores -1 at 23 and IDML has 0; left out | `objects.md`, clipping path settings |
| `InvertPath` and `RestrictToFrame` bits | chunk 0x2C1A, byte 25 | 1 pair | open: never varies | bits 0x01 and 0x08 are set together in the one sample; written only when both are equal | `objects.md`, clipping path settings |
| Images that name an old link object | image chunk 0x8C1B (class 0x8C01), InDesign 2.0 to 5.0 | 79 files | open: no pair | 762 images; the converter writes no `Contents` for them and returns their preview; in 2.0 files it reads the link's path | `objects.md`, images that keep their file in their preview; links of InDesign 2.0 |
| EPS `Contents` that differ | placed graphics | – | open: not examined | 18 of 515 | `objects.md`, graphics pasted without a link |
| Anchored settings of some nested items | `AnchoredObjectSetting` | 3 | open: not examined | 388 values | `objects.md`, items inside an anchored item |
| `FlexObject` | page items | 20–28 pairs | open: not decoded | structures not written | |
| `FilePath` of relinked media | `Movie`, `Sound` | 21 elements | not needed | 21 of 29 links were resolved to another file at export; `FilePath` names that file | `objects.md`, movies and sounds |
| Controller attributes of movies | `ShowControls`, `ControllerSkin`, `ShowController` | 17 pairs | open: not explained | on every movie of DOM 8.0 to 13.1, on none of DOM 18.0; no sample between; written below DOM 18 | `objects.md`, movies and sounds |
| Unidentified playback keys | chunk 0x14004 | 170 objects | open: never varies | u16 0x140FF (1 movie), u32 0x14007, u32 0x140FE (5 always), lists 3 to 6 (always empty), key 0x14009 after list 7 (1 movie); no IDML attribute differs with them | `objects.md`, movies and sounds |
| Media constants | `Movie`, `Sound` | 19 files | open: never varies | `FloatingWindow`, `FloatingWindowPosition`, `FloatingWindowSize`, `ShowControls`, `DoNotPrintPoster`, movie `EmbedInPDF` have one value in every sample | `objects.md`, movies and sounds |
| Export attributes of 10.0 files exported by 10.2 | `ObjectExportOption` | 4 | not needed | the attribute set follows the exporting version, which the INDD does not record | `idml-values.md`, export options |
| `CustomImageSizeOption` | `ObjectExportOption` | 224 | not needed | IDML value not in the schema; left out on purpose | `idml-values.md`, export options |

## Stroke styles and tracked changes

| Item | Where | Files | Status | Content and reason | See |
|---|---|---:|---|---|---|
| Striped styles with several stripes | class 0xB016 | 98 objects | open: no pair | the paired definition has one stripe; the IDML form of several stripes and their number format are not shown | `attributes.md`, custom stroke styles |
| Corner codes 0 to 2 | dashed style chunk 0x5A4F, attribute 0x5A35 | 2 objects | open: no pair | only code 3 (`DashesAndGaps`) occurs in a pair; code 0 in 2 dashed styles | `attributes.md`, custom stroke styles |
| Name flag byte | chunks 0xB023, 0x5A48 | 78 | open: never varies in a pair | the byte before the style name, 0 or 2 | `attributes.md`, custom stroke styles |
| Dotted stroke styles | – | 0 | open: no sample | no corpus file has one | |
| Custom stroke styles outside items and object styles | text, table and cell stroke lists | – | open: no pair | only object styles and page items store code 0x5A42 | `attributes.md`, custom stroke styles |
| Order of several custom styles | `Graphic.xml` | – | open: no pair | every paired file has one; written in UID order | `attributes.md`, custom stroke styles |
| `Change` past the end of a paragraph-level source | story | 1 pair | not needed | an export defect of the reference IDML | `objects.md`, tracked changes |
