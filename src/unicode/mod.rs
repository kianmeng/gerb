/*
 * gerb
 *
 * Copyright 2022 - Manos Pitsidianakis
 *
 * This file is part of gerb.
 *
 * gerb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * gerb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with gerb. If not, see <http://www.gnu.org/licenses/>.
 */

pub mod names;

pub mod blocks {
    use std::cmp::Ordering;

    pub trait CharBlock {
        fn char_block(self) -> Option<usize>;
    }

    impl CharBlock for char {
        fn char_block(self) -> Option<usize> {
            CharBlock::char_block(self as u32)
        }
    }

    impl CharBlock for u32 {
        fn char_block(self) -> Option<usize> {
            match UNICODE_BLOCKS.binary_search_by(|&((lo, hi), _, _, _, _)| {
                if lo as u32 <= self && self <= hi as u32 {
                    Ordering::Equal
                } else if (hi as u32) < self {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }) {
                Ok(idx) => Some(idx),
                Err(_) => None,
            }
        }
    }

    /* https://en.wikipedia.org/wiki/Unicode_block#List_of_blocks retrieved on 2022-03-04 */
    //Block range, Block name, Code points, Assigned characters, Scripts
    pub const UNICODE_BLOCKS: &[((char, char), &str, i32, i32, &str)] = &[
    (('\u{0000}', '\u{007F}'), "Basic Latin", 128, 128, "Latin (52 characters), Common (76 characters)"),
    (('\u{0080}', '\u{00FF}'), "Latin-1 Supplement", 128, 128, "Latin (64 characters), Common (64 characters)"),
    (('\u{0100}', '\u{017F}'), "Latin Extended-A", 128, 128, "Latin"),
    (('\u{0180}', '\u{024F}'), "Latin Extended-B", 208, 208, "Latin"),
    (('\u{0250}', '\u{02AF}'), "IPA Extensions", 96, 96, "Latin"),
    (('\u{02B0}', '\u{02FF}'), "Spacing Modifier Letters", 80, 80, "Bopomofo (2 characters), Latin (14 characters), Common (64 characters)"),
    (('\u{0300}', '\u{036F}'), "Combining Diacritical Marks", 112, 112, "Inherited"),
    (('\u{0370}', '\u{03FF}'), "Greek and Coptic", 144, 135, "Coptic (14 characters), Greek (117 characters), Common (4 characters)"),
    (('\u{0400}', '\u{04FF}'), "Cyrillic", 256, 256, "Cyrillic (254 characters), Inherited (2 characters)"),
    (('\u{0500}', '\u{052F}'), "Cyrillic Supplement", 48, 48, "Cyrillic"),
    (('\u{0530}', '\u{058F}'), "Armenian", 96, 91, "Armenian"),
    (('\u{0590}', '\u{05FF}'), "Hebrew", 112, 88, "Hebrew"),
    (('\u{0600}', '\u{06FF}'), "Arabic", 256, 256, "Arabic (238 characters), Common (6 characters), Inherited (12 characters)"),
    (('\u{0700}', '\u{074F}'), "Syriac", 80, 77, "Syriac"),
    (('\u{0750}', '\u{077F}'), "Arabic Supplement", 48, 48, "Arabic"),
    (('\u{0780}', '\u{07BF}'), "Thaana", 64, 50, "Thaana"),
    (('\u{07C0}', '\u{07FF}'), "NKo", 64, 62, "Nko"),
    (('\u{0800}', '\u{083F}'), "Samaritan", 64, 61, "Samaritan"),
    (('\u{0840}', '\u{085F}'), "Mandaic", 32, 29, "Mandaic"),
    (('\u{0860}', '\u{086F}'), "Syriac Supplement", 16, 11, "Syriac"),
    (('\u{0870}', '\u{089F}'), "Arabic Extended-B", 48, 41, "Arabic"),
    (('\u{08A0}', '\u{08FF}'), "Arabic Extended-A", 96, 96, "Arabic (95 characters), Common (1 character)"),
    (('\u{0900}', '\u{097F}'), "Devanagari", 128, 128, "Devanagari (122 characters), Common (2 characters), Inherited (4 characters)"),
    (('\u{0980}', '\u{09FF}'), "Bengali", 128, 96, "Bengali"),
    (('\u{0A00}', '\u{0A7F}'), "Gurmukhi", 128, 80, "Gurmukhi"),
    (('\u{0A80}', '\u{0AFF}'), "Gujarati", 128, 91, "Gujarati"),
    (('\u{0B00}', '\u{0B7F}'), "Oriya", 128, 91, "Oriya"),
    (('\u{0B80}', '\u{0BFF}'), "Tamil", 128, 72, "Tamil"),
    (('\u{0C00}', '\u{0C7F}'), "Telugu", 128, 100, "Telugu"),
    (('\u{0C80}', '\u{0CFF}'), "Kannada", 128, 90, "Kannada"),
    (('\u{0D00}', '\u{0D7F}'), "Malayalam", 128, 118, "Malayalam"),
    (('\u{0D80}', '\u{0DFF}'), "Sinhala", 128, 91, "Sinhala"),
    (('\u{0E00}', '\u{0E7F}'), "Thai", 128, 87, "Thai (86 characters), Common (1 character)"),
    (('\u{0E80}', '\u{0EFF}'), "Lao", 128, 82, "Lao"),
    (('\u{0F00}', '\u{0FFF}'), "Tibetan", 256, 211, "Tibetan (207 characters), Common (4 characters)"),
    (('\u{1000}', '\u{109F}'), "Myanmar", 160, 160, "Myanmar"),
    (('\u{10A0}', '\u{10FF}'), "Georgian", 96, 88, "Georgian (87 characters), Common (1 character)"),
    (('\u{1100}', '\u{11FF}'), "Hangul Jamo", 256, 256, "Hangul"),
    (('\u{1200}', '\u{137F}'), "Ethiopic", 384, 358, "Ethiopic"),
    (('\u{1380}', '\u{139F}'), "Ethiopic Supplement", 32, 26, "Ethiopic"),
    (('\u{13A0}', '\u{13FF}'), "Cherokee", 96, 92, "Cherokee"),
    (('\u{1400}', '\u{167F}'), "Unified Canadian Aboriginal Syllabics", 640, 640, "Canadian Aboriginal"),
    (('\u{1680}', '\u{169F}'), "Ogham", 32, 29, "Ogham"),
    (('\u{16A0}', '\u{16FF}'), "Runic", 96, 89, "Runic (86 characters), Common (3 characters)"),
    (('\u{1700}', '\u{171F}'), "Tagalog", 32, 23, "Tagalog"),
    (('\u{1720}', '\u{173F}'), "Hanunoo", 32, 23, "Hanunoo (21 characters), Common (2 characters)"),
    (('\u{1740}', '\u{175F}'), "Buhid", 32, 20, "Buhid"),
    (('\u{1760}', '\u{177F}'), "Tagbanwa", 32, 18, "Tagbanwa"),
    (('\u{1780}', '\u{17FF}'), "Khmer", 128, 114, "Khmer"),
    (('\u{1800}', '\u{18AF}'), "Mongolian", 176, 158, "Mongolian (155 characters), Common (3 characters)"),
    (('\u{18B0}', '\u{18FF}'), "Unified Canadian Aboriginal Syllabics Extended", 80, 70, "Canadian Aboriginal"),
    (('\u{1900}', '\u{194F}'), "Limbu", 80, 68, "Limbu"),
    (('\u{1950}', '\u{197F}'), "Tai Le", 48, 35, "Tai Le"),
    (('\u{1980}', '\u{19DF}'), "New Tai Lue", 96, 83, "New Tai Lue"),
    (('\u{19E0}', '\u{19FF}'), "Khmer Symbols", 32, 32, "Khmer"),
    (('\u{1A00}', '\u{1A1F}'), "Buginese", 32, 30, "Buginese"),
    (('\u{1A20}', '\u{1AAF}'), "Tai Tham", 144, 127, "Tai Tham"),
    (('\u{1AB0}', '\u{1AFF}'), "Combining Diacritical Marks Extended", 80, 31, "Inherited"),
    (('\u{1B00}', '\u{1B7F}'), "Balinese", 128, 124, "Balinese"),
    (('\u{1B80}', '\u{1BBF}'), "Sundanese", 64, 64, "Sundanese"),
    (('\u{1BC0}', '\u{1BFF}'), "Batak", 64, 56, "Batak"),
    (('\u{1C00}', '\u{1C4F}'), "Lepcha", 80, 74, "Lepcha"),
    (('\u{1C50}', '\u{1C7F}'), "Ol Chiki", 48, 48, "Ol Chiki"),
    (('\u{1C80}', '\u{1C8F}'), "Cyrillic Extended-C", 16, 9, "Cyrillic"),
    (('\u{1C90}', '\u{1CBF}'), "Georgian Extended", 48, 46, "Georgian"),
    (('\u{1CC0}', '\u{1CCF}'), "Sundanese Supplement", 16, 8, "Sundanese"),
    (('\u{1CD0}', '\u{1CFF}'), "Vedic Extensions", 48, 43, "Common (16 characters), Inherited (27 characters)"),
    (('\u{1D00}', '\u{1D7F}'), "Phonetic Extensions", 128, 128, "Cyrillic (2 characters), Greek (15 characters), Latin (111 characters)"),
    (('\u{1D80}', '\u{1DBF}'), "Phonetic Extensions Supplement", 64, 64, "Greek (1 character), Latin (63 characters)"),
    (('\u{1DC0}', '\u{1DFF}'), "Combining Diacritical Marks Supplement", 64, 64, "Inherited"),
    (('\u{1E00}', '\u{1EFF}'), "Latin Extended Additional", 256, 256, "Latin"),
    (('\u{1F00}', '\u{1FFF}'), "Greek Extended", 256, 233, "Greek"),
    (('\u{2000}', '\u{206F}'), "General Punctuation", 112, 111, "Common (109 characters), Inherited (2 characters)"),
    (('\u{2070}', '\u{209F}'), "Superscripts and Subscripts", 48, 42, "Latin (15 characters), Common (27 characters)"),
    (('\u{20A0}', '\u{20CF}'), "Currency Symbols", 48, 33, "Common"),
    (('\u{20D0}', '\u{20FF}'), "Combining Diacritical Marks for Symbols", 48, 33, "Inherited"),
    (('\u{2100}', '\u{214F}'), "Letterlike Symbols", 80, 80, "Greek (1 character), Latin (4 characters), Common (75 characters)"),
    (('\u{2150}', '\u{218F}'), "Number Forms", 64, 60, "Latin (41 characters), Common (19 characters)"),
    (('\u{2190}', '\u{21FF}'), "Arrows", 112, 112, "Common"),
    (('\u{2200}', '\u{22FF}'), "Mathematical Operators", 256, 256, "Common"),
    (('\u{2300}', '\u{23FF}'), "Miscellaneous Technical", 256, 256, "Common"),
    (('\u{2400}', '\u{243F}'), "Control Pictures", 64, 39, "Common"),
    (('\u{2440}', '\u{245F}'), "Optical Character Recognition", 32, 11, "Common"),
    (('\u{2460}', '\u{24FF}'), "Enclosed Alphanumerics", 160, 160, "Common"),
    (('\u{2500}', '\u{257F}'), "Box Drawing", 128, 128, "Common"),
    (('\u{2580}', '\u{259F}'), "Block Elements", 32, 32, "Common"),
    (('\u{25A0}', '\u{25FF}'), "Geometric Shapes", 96, 96, "Common"),
    (('\u{2600}', '\u{26FF}'), "Miscellaneous Symbols", 256, 256, "Common"),
    (('\u{2700}', '\u{27BF}'), "Dingbats", 192, 192, "Common"),
    (('\u{27C0}', '\u{27EF}'), "Miscellaneous Mathematical Symbols-A", 48, 48, "Common"),
    (('\u{27F0}', '\u{27FF}'), "Supplemental Arrows-A", 16, 16, "Common"),
    (('\u{2800}', '\u{28FF}'), "Braille Patterns", 256, 256, "Braille"),
    (('\u{2900}', '\u{297F}'), "Supplemental Arrows-B", 128, 128, "Common"),
    (('\u{2980}', '\u{29FF}'), "Miscellaneous Mathematical Symbols-B", 128, 128, "Common"),
    (('\u{2A00}', '\u{2AFF}'), "Supplemental Mathematical Operators", 256, 256, "Common"),
    (('\u{2B00}', '\u{2BFF}'), "Miscellaneous Symbols and Arrows", 256, 253, "Common"),
    (('\u{2C00}', '\u{2C5F}'), "Glagolitic", 96, 96, "Glagolitic"),
    (('\u{2C60}', '\u{2C7F}'), "Latin Extended-C", 32, 32, "Latin"),
    (('\u{2C80}', '\u{2CFF}'), "Coptic", 128, 123, "Coptic"),
    (('\u{2D00}', '\u{2D2F}'), "Georgian Supplement", 48, 40, "Georgian"),
    (('\u{2D30}', '\u{2D7F}'), "Tifinagh", 80, 59, "Tifinagh"),
    (('\u{2D80}', '\u{2DDF}'), "Ethiopic Extended", 96, 79, "Ethiopic"),
    (('\u{2DE0}', '\u{2DFF}'), "Cyrillic Extended-A", 32, 32, "Cyrillic"),
    (('\u{2E00}', '\u{2E7F}'), "Supplemental Punctuation", 128, 94, "Common"),
    (('\u{2E80}', '\u{2EFF}'), "CJK Radicals Supplement", 128, 115, "Han"),
    (('\u{2F00}', '\u{2FDF}'), "Kangxi Radicals", 224, 214, "Han"),
    (('\u{2FF0}', '\u{2FFF}'), "Ideographic Description Characters", 16, 12, "Common"),
    (('\u{3000}', '\u{303F}'), "CJK Symbols and Punctuation", 64, 64, "Han (15 characters), Hangul (2 characters), Common (43 characters), Inherited (4 characters)"),
    (('\u{3040}', '\u{309F}'), "Hiragana", 96, 93, "Hiragana (89 characters), Common (2 characters), Inherited (2 characters)"),
    (('\u{30A0}', '\u{30FF}'), "Katakana", 96, 96, "Katakana (93 characters), Common (3 characters)"),
    (('\u{3100}', '\u{312F}'), "Bopomofo", 48, 43, "Bopomofo"),
    (('\u{3130}', '\u{318F}'), "Hangul Compatibility Jamo", 96, 94, "Hangul"),
    (('\u{3190}', '\u{319F}'), "Kanbun", 16, 16, "Common"),
    (('\u{31A0}', '\u{31BF}'), "Bopomofo Extended", 32, 32, "Bopomofo"),
    (('\u{31C0}', '\u{31EF}'), "CJK Strokes", 48, 36, "Common"),
    (('\u{31F0}', '\u{31FF}'), "Katakana Phonetic Extensions", 16, 16, "Katakana"),
    (('\u{3200}', '\u{32FF}'), "Enclosed CJK Letters and Months", 256, 255, "Hangul (62 characters), Katakana (47 characters), Common (146 characters)"),
    (('\u{3300}', '\u{33FF}'), "CJK Compatibility", 256, 256, "Katakana (88 characters), Common (168 characters)"),
    (('\u{3400}', '\u{4DBF}'), "CJK Unified Ideographs Extension A", 6592, 6592, "Han"),
    (('\u{4DC0}', '\u{4DFF}'), "Yijing Hexagram Symbols", 64, 64, "Common"),
    (('\u{4E00}', '\u{9FFF}'), "CJK Unified Ideographs", 20992, 20992, "Han"),
    (('\u{A000}', '\u{A48F}'), "Yi Syllables", 1168, 1165, "Yi"),
    (('\u{A490}', '\u{A4CF}'), "Yi Radicals", 64, 55, "Yi"),
    (('\u{A4D0}', '\u{A4FF}'), "Lisu", 48, 48, "Lisu"),
    (('\u{A500}', '\u{A63F}'), "Vai", 320, 300, "Vai"),
    (('\u{A640}', '\u{A69F}'), "Cyrillic Extended-B", 96, 96, "Cyrillic"),
    (('\u{A6A0}', '\u{A6FF}'), "Bamum", 96, 88, "Bamum"),
    (('\u{A700}', '\u{A71F}'), "Modifier Tone Letters", 32, 32, "Common"),
    (('\u{A720}', '\u{A7FF}'), "Latin Extended-D", 224, 193, "Latin (188 characters), Common (5 characters)"),
    (('\u{A800}', '\u{A82F}'), "Syloti Nagri", 48, 45, "Syloti Nagri"),
    (('\u{A830}', '\u{A83F}'), "Common Indic Number Forms", 16, 10, "Common"),
    (('\u{A840}', '\u{A87F}'), "Phags-pa", 64, 56, "Phags Pa"),
    (('\u{A880}', '\u{A8DF}'), "Saurashtra", 96, 82, "Saurashtra"),
    (('\u{A8E0}', '\u{A8FF}'), "Devanagari Extended", 32, 32, "Devanagari"),
    (('\u{A900}', '\u{A92F}'), "Kayah Li", 48, 48, "Kayah Li (47 characters), Common (1 character)"),
    (('\u{A930}', '\u{A95F}'), "Rejang", 48, 37, "Rejang"),
    (('\u{A960}', '\u{A97F}'), "Hangul Jamo Extended-A", 32, 29, "Hangul"),
    (('\u{A980}', '\u{A9DF}'), "Javanese", 96, 91, "Javanese (90 characters), Common (1 character)"),
    (('\u{A9E0}', '\u{A9FF}'), "Myanmar Extended-B", 32, 31, "Myanmar"),
    (('\u{AA00}', '\u{AA5F}'), "Cham", 96, 83, "Cham"),
    (('\u{AA60}', '\u{AA7F}'), "Myanmar Extended-A", 32, 32, "Myanmar"),
    (('\u{AA80}', '\u{AADF}'), "Tai Viet", 96, 72, "Tai Viet"),
    (('\u{AAE0}', '\u{AAFF}'), "Meetei Mayek Extensions", 32, 23, "Meetei Mayek"),
    (('\u{AB00}', '\u{AB2F}'), "Ethiopic Extended-A", 48, 32, "Ethiopic"),
    (('\u{AB30}', '\u{AB6F}'), "Latin Extended-E", 64, 60, "Latin (56 characters), Greek (1 character), Common (3 characters)"),
    (('\u{AB70}', '\u{ABBF}'), "Cherokee Supplement", 80, 80, "Cherokee"),
    (('\u{ABC0}', '\u{ABFF}'), "Meetei Mayek", 64, 56, "Meetei Mayek"),
    (('\u{AC00}', '\u{D7AF}'), "Hangul Syllables", 11184, 11172, "Hangul"),
    (('\u{D7B0}', '\u{D7FF}'), "Hangul Jamo Extended-B", 80, 72, "Hangul"),
    //(('\u{D800}', '\u{DB7F}'), "High Surrogates", 896, 0, "Unknown"),
    //(('\u{DB80}', '\u{DBFF}'), "High Private Use Surrogates", 128, 0, "Unknown"),
    //(('\u{DC00}', '\u{DFFF}'), "Low Surrogates", 1024, 0, "Unknown"),
    (('\u{E000}', '\u{F8FF}'), "Private Use Area", 6400, 6400, "Unknown"),
    (('\u{F900}', '\u{FAFF}'), "CJK Compatibility Ideographs", 512, 472, "Han"),
    (('\u{FB00}', '\u{FB4F}'), "Alphabetic Presentation Forms", 80, 58, "Armenian (5 characters), Hebrew (46 characters), Latin (7 characters)"),
    (('\u{FB50}', '\u{FDFF}'), "Arabic Presentation Forms-A", 688, 631, "Arabic (629 characters), Common (2 characters)"),
    (('\u{FE00}', '\u{FE0F}'), "Variation Selectors", 16, 16, "Inherited"),
    (('\u{FE10}', '\u{FE1F}'), "Vertical Forms", 16, 10, "Common"),
    (('\u{FE20}', '\u{FE2F}'), "Combining Half Marks", 16, 16, "Cyrillic (2 characters), Inherited (14 characters)"),
    (('\u{FE30}', '\u{FE4F}'), "CJK Compatibility Forms", 32, 32, "Common"),
    (('\u{FE50}', '\u{FE6F}'), "Small Form Variants", 32, 26, "Common"),
    (('\u{FE70}', '\u{FEFF}'), "Arabic Presentation Forms-B", 144, 141, "Arabic (140 characters), Common (1 character)"),
    (('\u{FF00}', '\u{FFEF}'), "Halfwidth and Fullwidth Forms", 240, 225, "Hangul (52 characters), Katakana (55 characters), Latin (52 characters), Common (66 characters)"),
    (('\u{FFF0}', '\u{FFFF}'), "Specials", 16, 5, "Common"),
    (('\u{10000}', '\u{1007F}'), "Linear B Syllabary", 128, 88, "Linear B"),
    (('\u{10080}', '\u{100FF}'), "Linear B Ideograms", 128, 123, "Linear B"),
    (('\u{10100}', '\u{1013F}'), "Aegean Numbers", 64, 57, "Common"),
    (('\u{10140}', '\u{1018F}'), "Ancient Greek Numbers", 80, 79, "Greek"),
    (('\u{10190}', '\u{101CF}'), "Ancient Symbols", 64, 14, "Greek (1 character), Common (13 characters)"),
    (('\u{101D0}', '\u{101FF}'), "Phaistos Disc", 48, 46, "Common (45 characters), Inherited (1 character)"),
    (('\u{10280}', '\u{1029F}'), "Lycian", 32, 29, "Lycian"),
    (('\u{102A0}', '\u{102DF}'), "Carian", 64, 49, "Carian"),
    (('\u{102E0}', '\u{102FF}'), "Coptic Epact Numbers", 32, 28, "Common (27 characters), Inherited (1 character)"),
    (('\u{10300}', '\u{1032F}'), "Old Italic", 48, 39, "Old Italic"),
    (('\u{10330}', '\u{1034F}'), "Gothic", 32, 27, "Gothic"),
    (('\u{10350}', '\u{1037F}'), "Old Permic", 48, 43, "Old Permic"),
    (('\u{10380}', '\u{1039F}'), "Ugaritic", 32, 31, "Ugaritic"),
    (('\u{103A0}', '\u{103DF}'), "Old Persian", 64, 50, "Old Persian"),
    (('\u{10400}', '\u{1044F}'), "Deseret", 80, 80, "Deseret"),
    (('\u{10450}', '\u{1047F}'), "Shavian", 48, 48, "Shavian"),
    (('\u{10480}', '\u{104AF}'), "Osmanya", 48, 40, "Osmanya"),
    (('\u{104B0}', '\u{104FF}'), "Osage", 80, 72, "Osage"),
    (('\u{10500}', '\u{1052F}'), "Elbasan", 48, 40, "Elbasan"),
    (('\u{10530}', '\u{1056F}'), "Caucasian Albanian", 64, 53, "Caucasian Albanian"),
    (('\u{10570}', '\u{105BF}'), "Vithkuqi", 80, 70, "Vithkuqi"),
    (('\u{10600}', '\u{1077F}'), "Linear A", 384, 341, "Linear A"),
    (('\u{10780}', '\u{107BF}'), "Latin Extended-F", 64, 57, "Latin"),
    (('\u{10800}', '\u{1083F}'), "Cypriot Syllabary", 64, 55, "Cypriot"),
    (('\u{10840}', '\u{1085F}'), "Imperial Aramaic", 32, 31, "Imperial Aramaic"),
    (('\u{10860}', '\u{1087F}'), "Palmyrene", 32, 32, "Palmyrene"),
    (('\u{10880}', '\u{108AF}'), "Nabataean", 48, 40, "Nabataean"),
    (('\u{108E0}', '\u{108FF}'), "Hatran", 32, 26, "Hatran"),
    (('\u{10900}', '\u{1091F}'), "Phoenician", 32, 29, "Phoenician"),
    (('\u{10920}', '\u{1093F}'), "Lydian", 32, 27, "Lydian"),
    (('\u{10980}', '\u{1099F}'), "Meroitic Hieroglyphs", 32, 32, "Meroitic Hieroglyphs"),
    (('\u{109A0}', '\u{109FF}'), "Meroitic Cursive", 96, 90, "Meroitic Cursive"),
    (('\u{10A00}', '\u{10A5F}'), "Kharoshthi", 96, 68, "Kharoshthi"),
    (('\u{10A60}', '\u{10A7F}'), "Old South Arabian", 32, 32, "Old South Arabian"),
    (('\u{10A80}', '\u{10A9F}'), "Old North Arabian", 32, 32, "Old North Arabian"),
    (('\u{10AC0}', '\u{10AFF}'), "Manichaean", 64, 51, "Manichaean"),
    (('\u{10B00}', '\u{10B3F}'), "Avestan", 64, 61, "Avestan"),
    (('\u{10B40}', '\u{10B5F}'), "Inscriptional Parthian", 32, 30, "Inscriptional Parthian"),
    (('\u{10B60}', '\u{10B7F}'), "Inscriptional Pahlavi", 32, 27, "Inscriptional Pahlavi"),
    (('\u{10B80}', '\u{10BAF}'), "Psalter Pahlavi", 48, 29, "Psalter Pahlavi"),
    (('\u{10C00}', '\u{10C4F}'), "Old Turkic", 80, 73, "Old Turkic"),
    (('\u{10C80}', '\u{10CFF}'), "Old Hungarian", 128, 108, "Old Hungarian"),
    (('\u{10D00}', '\u{10D3F}'), "Hanifi Rohingya", 64, 50, "Hanifi Rohingya"),
    (('\u{10E60}', '\u{10E7F}'), "Rumi Numeral Symbols", 32, 31, "Arabic"),
    (('\u{10E80}', '\u{10EBF}'), "Yezidi", 64, 47, "Yezidi"),
    (('\u{10F00}', '\u{10F2F}'), "Old Sogdian", 48, 40, "Old Sogdian"),
    (('\u{10F30}', '\u{10F6F}'), "Sogdian", 64, 42, "Sogdian"),
    (('\u{10F70}', '\u{10FAF}'), "Old Uyghur", 64, 26, "Old Uyghur"),
    (('\u{10FB0}', '\u{10FDF}'), "Chorasmian", 48, 28, "Chorasmian"),
    (('\u{10FE0}', '\u{10FFF}'), "Elymaic", 32, 23, "Elymaic"),
    (('\u{11000}', '\u{1107F}'), "Brahmi", 128, 115, "Brahmi"),
    (('\u{11080}', '\u{110CF}'), "Kaithi", 80, 68, "Kaithi"),
    (('\u{110D0}', '\u{110FF}'), "Sora Sompeng", 48, 35, "Sora Sompeng"),
    (('\u{11100}', '\u{1114F}'), "Chakma", 80, 71, "Chakma"),
    (('\u{11150}', '\u{1117F}'), "Mahajani", 48, 39, "Mahajani"),
    (('\u{11180}', '\u{111DF}'), "Sharada", 96, 96, "Sharada"),
    (('\u{111E0}', '\u{111FF}'), "Sinhala Archaic Numbers", 32, 20, "Sinhala"),
    (('\u{11200}', '\u{1124F}'), "Khojki", 80, 62, "Khojki"),
    (('\u{11280}', '\u{112AF}'), "Multani", 48, 38, "Multani"),
    (('\u{112B0}', '\u{112FF}'), "Khudawadi", 80, 69, "Khudawadi"),
    (('\u{11300}', '\u{1137F}'), "Grantha", 128, 86, "Grantha (85 characters), Inherited (1 character)"),
    (('\u{11400}', '\u{1147F}'), "Newa", 128, 97, "Newa"),
    (('\u{11480}', '\u{114DF}'), "Tirhuta", 96, 82, "Tirhuta"),
    (('\u{11580}', '\u{115FF}'), "Siddham", 128, 92, "Siddham"),
    (('\u{11600}', '\u{1165F}'), "Modi", 96, 79, "Modi"),
    (('\u{11660}', '\u{1167F}'), "Mongolian Supplement", 32, 13, "Mongolian"),
    (('\u{11680}', '\u{116CF}'), "Takri", 80, 68, "Takri"),
    (('\u{11700}', '\u{1174F}'), "Ahom", 80, 65, "Ahom"),
    (('\u{11800}', '\u{1184F}'), "Dogra", 80, 60, "Dogra"),
    (('\u{118A0}', '\u{118FF}'), "Warang Citi", 96, 84, "Warang Citi"),
    (('\u{11900}', '\u{1195F}'), "Dives Akuru", 96, 72, "Dives Akuru"),
    (('\u{119A0}', '\u{119FF}'), "Nandinagari", 96, 65, "Nandinagari"),
    (('\u{11A00}', '\u{11A4F}'), "Zanabazar Square", 80, 72, "Zanabazar Square"),
    (('\u{11A50}', '\u{11AAF}'), "Soyombo", 96, 83, "Soyombo"),
    (('\u{11AB0}', '\u{11ABF}'), "Unified Canadian Aboriginal Syllabics Extended-A", 16, 16, "Canadian Aboriginal"),
    (('\u{11AC0}', '\u{11AFF}'), "Pau Cin Hau", 64, 57, "Pau Cin Hau"),
    (('\u{11C00}', '\u{11C6F}'), "Bhaiksuki", 112, 97, "Bhaiksuki"),
    (('\u{11C70}', '\u{11CBF}'), "Marchen", 80, 68, "Marchen"),
    (('\u{11D00}', '\u{11D5F}'), "Masaram Gondi", 96, 75, "Masaram Gondi"),
    (('\u{11D60}', '\u{11DAF}'), "Gunjala Gondi", 80, 63, "Gunjala Gondi"),
    (('\u{11EE0}', '\u{11EFF}'), "Makasar", 32, 25, "Makasar"),
    (('\u{11FB0}', '\u{11FBF}'), "Lisu Supplement", 16, 1, "Lisu"),
    (('\u{11FC0}', '\u{11FFF}'), "Tamil Supplement", 64, 51, "Tamil"),
    (('\u{12000}', '\u{123FF}'), "Cuneiform", 1024, 922, "Cuneiform"),
    (('\u{12400}', '\u{1247F}'), "Cuneiform Numbers and Punctuation", 128, 116, "Cuneiform"),
    (('\u{12480}', '\u{1254F}'), "Early Dynastic Cuneiform", 208, 196, "Cuneiform"),
    (('\u{12F90}', '\u{12FFF}'), "Cypro-Minoan", 112, 99, "Cypro Minoan"),
    (('\u{13000}', '\u{1342F}'), "Egyptian Hieroglyphs", 1072, 1071, "Egyptian Hieroglyphs"),
    (('\u{13430}', '\u{1343F}'), "Egyptian Hieroglyph Format Controls", 16, 9, "Egyptian Hieroglyphs"),
    (('\u{14400}', '\u{1467F}'), "Anatolian Hieroglyphs", 640, 583, "Anatolian Hieroglyphs"),
    (('\u{16800}', '\u{16A3F}'), "Bamum Supplement", 576, 569, "Bamum"),
    (('\u{16A40}', '\u{16A6F}'), "Mro", 48, 43, "Mro"),
    (('\u{16A70}', '\u{16ACF}'), "Tangsa", 96, 89, "Tangsa"),
    (('\u{16AD0}', '\u{16AFF}'), "Bassa Vah", 48, 36, "Bassa Vah"),
    (('\u{16B00}', '\u{16B8F}'), "Pahawh Hmong", 144, 127, "Pahawh Hmong"),
    (('\u{16E40}', '\u{16E9F}'), "Medefaidrin", 96, 91, "Medefaidrin"),
    (('\u{16F00}', '\u{16F9F}'), "Miao", 160, 149, "Miao"),
    (('\u{16FE0}', '\u{16FFF}'), "Ideographic Symbols and Punctuation", 32, 7, "Han (4 characters), Khitan Small Script (1 character), Nushu (1 character), Tangut (1 character)"),
    (('\u{17000}', '\u{187FF}'), "Tangut", 6144, 6136, "Tangut"),
    (('\u{18800}', '\u{18AFF}'), "Tangut Components", 768, 768, "Tangut"),
    (('\u{18B00}', '\u{18CFF}'), "Khitan Small Script", 512, 470, "Khitan Small Script"),
    (('\u{18D00}', '\u{18D7F}'), "Tangut Supplement", 128, 9, "Tangut"),
    (('\u{1AFF0}', '\u{1AFFF}'), "Kana Extended-B", 16, 13, "Katakana"),
    (('\u{1B000}', '\u{1B0FF}'), "Kana Supplement", 256, 256, "Hiragana (255 characters), Katakana (1 character)"),
    (('\u{1B100}', '\u{1B12F}'), "Kana Extended-A", 48, 35, "Hiragana (32 characters), Katakana (3 characters)"),
    (('\u{1B130}', '\u{1B16F}'), "Small Kana Extension", 64, 7, "Hiragana (3 characters), Katakana (4 characters)"),
    (('\u{1B170}', '\u{1B2FF}'), "Nushu", 400, 396, "Nüshu"),
    (('\u{1BC00}', '\u{1BC9F}'), "Duployan", 160, 143, "Duployan"),
    (('\u{1BCA0}', '\u{1BCAF}'), "Shorthand Format Controls", 16, 4, "Common"),
    (('\u{1CF00}', '\u{1CFCF}'), "Znamenny Musical Notation", 208, 185, "Common (116 characters), Inherited (69 characters)"),
    (('\u{1D000}', '\u{1D0FF}'), "Byzantine Musical Symbols", 256, 246, "Common"),
    (('\u{1D100}', '\u{1D1FF}'), "Musical Symbols", 256, 233, "Common (211 characters), Inherited (22 characters)"),
    (('\u{1D200}', '\u{1D24F}'), "Ancient Greek Musical Notation", 80, 70, "Greek"),
    (('\u{1D2E0}', '\u{1D2FF}'), "Mayan Numerals", 32, 20, "Common"),
    (('\u{1D300}', '\u{1D35F}'), "Tai Xuan Jing Symbols", 96, 87, "Common"),
    (('\u{1D360}', '\u{1D37F}'), "Counting Rod Numerals", 32, 25, "Common"),
    (('\u{1D400}', '\u{1D7FF}'), "Mathematical Alphanumeric Symbols", 1024, 996, "Common"),
    (('\u{1D800}', '\u{1DAAF}'), "Sutton SignWriting", 688, 672, "SignWriting"),
    (('\u{1DF00}', '\u{1DFFF}'), "Latin Extended-G", 256, 31, "Latin"),
    (('\u{1E000}', '\u{1E02F}'), "Glagolitic Supplement", 48, 38, "Glagolitic"),
    (('\u{1E100}', '\u{1E14F}'), "Nyiakeng Puachue Hmong", 80, 71, "Nyiakeng Puachue Hmong"),
    (('\u{1E290}', '\u{1E2BF}'), "Toto", 48, 31, "Toto"),
    (('\u{1E2C0}', '\u{1E2FF}'), "Wancho", 64, 59, "Wancho"),
    (('\u{1E7E0}', '\u{1E7FF}'), "Ethiopic Extended-B", 32, 28, "Ethiopic"),
    (('\u{1E800}', '\u{1E8DF}'), "Mende Kikakui", 224, 213, "Mende Kikakui"),
    (('\u{1E900}', '\u{1E95F}'), "Adlam", 96, 88, "Adlam"),
    (('\u{1EC70}', '\u{1ECBF}'), "Indic Siyaq Numbers", 80, 68, "Common"),
    (('\u{1ED00}', '\u{1ED4F}'), "Ottoman Siyaq Numbers", 80, 61, "Common"),
    (('\u{1EE00}', '\u{1EEFF}'), "Arabic Mathematical Alphabetic Symbols", 256, 143, "Arabic"),
    (('\u{1F000}', '\u{1F02F}'), "Mahjong Tiles", 48, 44, "Common"),
    (('\u{1F030}', '\u{1F09F}'), "Domino Tiles", 112, 100, "Common"),
    (('\u{1F0A0}', '\u{1F0FF}'), "Playing Cards", 96, 82, "Common"),
    (('\u{1F100}', '\u{1F1FF}'), "Enclosed Alphanumeric Supplement", 256, 200, "Common"),
    (('\u{1F200}', '\u{1F2FF}'), "Enclosed Ideographic Supplement", 256, 64, "Hiragana (1 character), Common (63 characters)"),
    (('\u{1F300}', '\u{1F5FF}'), "Miscellaneous Symbols and Pictographs", 768, 768, "Common"),
    (('\u{1F600}', '\u{1F64F}'), "Emoticons", 80, 80, "Common"),
    (('\u{1F650}', '\u{1F67F}'), "Ornamental Dingbats", 48, 48, "Common"),
    (('\u{1F680}', '\u{1F6FF}'), "Transport and Map Symbols", 128, 117, "Common"),
    (('\u{1F700}', '\u{1F77F}'), "Alchemical Symbols", 128, 116, "Common"),
    (('\u{1F780}', '\u{1F7FF}'), "Geometric Shapes Extended", 128, 102, "Common"),
    (('\u{1F800}', '\u{1F8FF}'), "Supplemental Arrows-C", 256, 150, "Common"),
    (('\u{1F900}', '\u{1F9FF}'), "Supplemental Symbols and Pictographs", 256, 256, "Common"),
    (('\u{1FA00}', '\u{1FA6F}'), "Chess Symbols", 112, 98, "Common"),
    (('\u{1FA70}', '\u{1FAFF}'), "Symbols and Pictographs Extended-A", 144, 88, "Common"),
    (('\u{1FB00}', '\u{1FBFF}'), "Symbols for Legacy Computing", 256, 212, "Common"),
    (('\u{20000}', '\u{2A6DF}'), "CJK Unified Ideographs Extension B", 42720, 42720, "Han"),
    (('\u{2A700}', '\u{2B73F}'), "CJK Unified Ideographs Extension C", 4160, 4153, "Han"),
    (('\u{2B740}', '\u{2B81F}'), "CJK Unified Ideographs Extension D", 224, 222, "Han"),
    (('\u{2B820}', '\u{2CEAF}'), "CJK Unified Ideographs Extension E", 5776, 5762, "Han"),
    (('\u{2CEB0}', '\u{2EBEF}'), "CJK Unified Ideographs Extension F", 7488, 7473, "Han"),
    (('\u{2F800}', '\u{2FA1F}'), "CJK Compatibility Ideographs Supplement", 544, 542, "Han"),
    (('\u{30000}', '\u{3134F}'), "CJK Unified Ideographs Extension G", 4944, 4939, "Han"),
    (('\u{E0000}', '\u{E007F}'), "Tags", 128, 97, "Common"),
    (('\u{E0100}', '\u{E01EF}'), "Variation Selectors Supplement", 240, 240, "Inherited"),
    (('\u{F0000}', '\u{FFFFF}'), "Supplementary Private Use Area-A", 65536, 65534, "Unknown"),
    (('\u{100000}', '\u{10FFFF}'), "Supplementary Private Use Area-B", 65536, 65534, "Unknown"),
];
}
