use crate::{music_xml::*, parse::*};

impl FromElement for MusicData { fn try_from<'t, 'input>(e: Node<'t, 'input>) -> Option<Self> { use MusicData::*; Some(match e.tag_name().name() {
    "note" => Note(FromElement::from(e)),
    "backup" => Backup(find(e, "duration")),
    "forward" => Forward(find(e, "duration")),
    "print" => None?,//Print,
    "attributes" => Attributes(FromElement::from(e)),
    "direction" => Direction(FromElement::try_from(e)?),
    "barline" => Barline(option(e, "bar-style")),
    _ => panic!("{e:?}")
})}}

impl FromElement for Note { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    duration: option(e, "duration"),
    voice: option(e, "voice"),
    r#type: option(e, "type"),
    accidental: option(e, "accidental"),
    time_modification: option(e, "time-modification"),
    dot: count(e, "dot"),
    ties: filter(e, "tie"),
    beams: filter(e, "beam"),
    notations: find_seq(e, "notations"),
    staff: option(e, "staff"),
    stem: option(e, "stem"),
    chord: has(e, "chord"),
    grace: has(e, "grace"),
    pitch: option(e, "pitch"),
}}}

impl FromStr for NoteType { fn from_str(s: &str) -> Self { use NoteType::*; match s {
	"1024th" => _1024th,
	"512th" => _512th,
	"256th" => _256th,
	"128th" => _128th,
	"64th" => _64th,
	"32th" => _32th,
	"16th" => _16th,
	"eighth" => Eighth,
    "quarter" => Quarter,
    "half" => Half,
    "whole" => Whole,
    "breve" => Breve,
    "long" => Long,
    "maxima" => Maxima,
    _ => panic!()
}}}

impl FromStr for Accidental { fn from_str(s: &str) -> Self { use Accidental::*; match s {
    "flat" => Flat,
    "natural" => Natural,
    "sharp" => Sharp,
    _ => panic!()
}}}

impl FromElement for TimeModification { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    actual_notes: find(e, "actual-notes"),
    normal_notes: find(e, "normal-notes"),
}}}

impl FromElement for Tie { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { use Tie::*; match e.attribute("type").unwrap() {
    "start" => Start,
    "stop" => Stop,
    _ => panic!()
}}}

impl FromElement for Beam { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    tag: {use BeamTag::*; match e.text().unwrap() {
        "begin" => Begin,
        "continue" => Continue,
        "end" => End,
        "forward hook" => ForwardHook,
        "backward hook" => BackwardHook,
        _ => panic!()
    }},
    number: option(e, "number")
}}}

impl FromElement for Notation { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { use Notation::*; match e.tag_name().name() {
    "tied" => Tied(attribute(e, "type")),
    "articulations" => Articulations(e.children().filter(|e| e.is_element()).map(|e| FromStr::from_str(e.tag_name().name())).collect()),
    "slur" => Slur(FromElement::from(e)),
    "ornaments" => Ornaments(seq(e)),
    "arpeggiate" => Arpeggiate,
    "fermata" => Fermata,
    notation => panic!("{notation}")
}}}

impl FromStr for StartStopContinue { fn from_str(s: &str) -> Self { use StartStopContinue::*; match s {
    "start" => Start,
    "stop" => Stop,
    "continue" => Continue,
    _ => panic!()
}}}

impl FromStr for Articulation { fn from_str(s: &str) -> Self { use Articulation::*; match s {
    "accent" => Accent,
    "strong-accent" => StrongAccent,
    "staccato" => Staccato,
    "tenuto" => Tenuto,
    "detached-legato" => DetachedLegato,
    "staccatissimo" =>Staccatissimo,
    "spiccato" => Spiccato,
    "scoop" => Scoop,
    "plop" => Plop,
    "doit" => Doit,
    "falloff" => Falloff,
	"breath-mark" => BreathMark,
    "caesura" => Caesura,
    "stress" => Stress,
    "unstress" => Unstress,
    "soft-accent" => SoftAccent,
    _ => panic!()
}}}

impl FromStr for Orientation { fn from_str(s: &str) -> Self { use Orientation::*; match s {
    "over" => Over,
    "under" => Under,
    _ => panic!()
}}}

impl FromElement for Slur { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    	r#type: attribute(e, "type"),
	    orientation: try_attribute(e, "orientation"),
}}}

impl FromElement for Ornament { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { use Ornament::*; match e.tag_name().name() {
    "tremolo" => Tremolo{
        r#type: e.attribute("type").map(|s| { use TremoloType::*; match s {
            "single" => Single,
            "start" => Start,
            "stop" => Stop,
            _ => panic!("{s}")
        }}).unwrap_or_default(),
        marks: FromElement::from(e)
    },
    "mordent" => Mordent,
    ornament => panic!("{ornament}")
}}}

impl FromStr for Stem { fn from_str(s: &str) -> Self { use Stem::*; match s {
    "up" => Up,
    "down" => Down,
    _ => panic!()
}}}
impl FromElement for Pitch { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    step: find(e, "step"),
    alter: option(e, "alter"),
    octave: option(e, "octave")
}}}
impl FromStr for Step { fn from_str(s: &str) -> Self { use Step::*; match s { "C" => C, "D" => D, "E" => E, "F" => F, "G" => G, "A" => A, "B" => B, _ => panic!() } } }
impl FromStr for Mode { fn from_str(s: &str) -> Self { use Mode::*; match s { "major" => Major, "minor" => Minor, _ => panic!() } } }
impl FromStr for Location { fn from_str(s: &str) -> Self { use Location::*; match s { "left" => Left, "right" => Right, "before-barline" => BeforeBarline, _ => panic!() } } }
impl FromElement for Cancel { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
	fifths: find(e, "fifths"),
	location: try_attribute(e, "location"),
}}}
impl FromElement for Key { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
	cancel: option(e, "cancel"),
	fifths: find(e, "fifths"),
	mode: option(e, "mode"),
}}}
impl FromElement for Time { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    beats: find(e, "beats"),
    beat_type: find(e, "beat-type")
}}}
impl FromStr for Sign { fn from_str(s: &str) -> Self { use Sign::*; match s { "G" => G, "F" => F, _ => panic!() }}}
impl FromElement for Clef { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
	staff: try_attribute(e, "number").unwrap_or(Staff(1)),
	sign: find(e, "sign"),
	line: option(e, "line"),
}}}

impl FromElement for Attributes { fn from<'t, 'input>(e: Node<'t, 'input>) -> Self { Self{
    divisions: option(e, "divisions"),
    key: option(e, "key"),
    time: option(e, "time"),
    staves: option(e, "staves"),
    clefs: filter(e, "clef"),
}}}
impl FromStr for UpDownStopContinue { fn from_str(s: &str) -> Self { use UpDownStopContinue::*; match s {
    "up" => Up,
    "down" => Down,
    "stop" => Stop,
    "continue" => Continue,
    _ => panic!()
}}}
impl FromStr for Wedge { fn from_str(s: &str) -> Self { use Wedge::*; match s {
    "crescendo" => Crescendo,
    "diminuendo" => Diminuendo,
    "stop" => Stop,
    "continue" => Continue,
    _ => panic!()
}}}
//impl FromStr for Dynamics { fn from_str(s: &str) -> Self { use Dynamics::*; match s { "pp" => pp, "p" => p, "mp" => mp, "mf" => mf, "f" => f, "ff" => ff,_ => panic!() }}}
impl FromElement for DirectionType { fn try_from<'t, 'input>(e: Node<'t, 'input>) -> Option<Self> {
    let e = e.first_element_child().unwrap();
    Some({use DirectionType::*; match e.tag_name().name() {
    "metronome" => Metronome{
		beat_unit: find(e, "beat-unit"),
		per_minute: find(e, "per-minute"),
	},
	"octave-shift" => OctaveShift{
        r#type: attribute(e, "type"),
		size: try_attribute(e, "size").unwrap_or(8)
    },
	"words" => Words(FromElement::try_from(e)?),
	"dynamics" => Dynamics(FromStr::from_str(e.first_element_child().unwrap().tag_name().name())),
	"wedge" => Wedge(attribute(e, "type")),
    e => panic!("{e}")
}})}}

impl FromElement for Direction { fn try_from<'t, 'input>(e: Node<'t, 'input>) -> Option<Self> { Some(Self{
        direction: Some(option(e, "direction-type")?),
	    offset: option(e, "offset"),
	    voice: option(e, "voice"),
	    staff: option(e, "staff"),
    })}
}
impl FromStr for BarStyle { fn from_str(s: &str) -> Self { use BarStyle::*; match s {
    "regular" => Regular,
    "dotted" => Dotted,
    "dashed" => Dashed,
    "heavy" => Heavy,
    "light-light" => LightLight,
    "light-heavy" => LightHeavy,
    "heavy-heavy" => HeavyHeavy,
    "tick" => Tick,
    "short" => Short,
    "none" => None,
    _ => panic!()
}}}

impl FromElement for Work { fn from<'t, 'i>(e: Node<'t, 'i>) -> Self { Self{title: find(e, "work-title") } } }
impl FromElement for Root { fn from<'t, 'i>(e: Node<'t, 'i>) -> Self { Self{part: find_filter_seq(e, "part", "measure"), work: find(e, "work") } } }

pub fn parse_(document: &Document) -> Option<Root> { FromElement::try_from(document.root_element()) }
pub fn parse(text: &str) -> Result<Root> { Ok(parse_(&Document::parse(text)?).unwrap()) }
pub fn parse_utf8(text: &[u8]) -> crate::Result<Root> { Ok(parse(std::str::from_utf8(text)?)?) }
