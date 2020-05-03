use {
    crate::{Entry, Span},
    nom::{
        bytes::complete::{is_a, is_not, tag, take},
        character::complete::{digit1, line_ending, space0},
        combinator::{all_consuming, map, map_opt, map_res, opt, recognize, rest_len},
        multi::{many0, separated_nonempty_list},
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    },
    time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset},
};

macro_rules! parser {
    ($vis:vis $name:ident -> $output:ty = $body:expr) => {
        $vis fn $name(i: &str) -> IResult<&str, $output> {
            $body(i)
        }
    };
}

pub fn parse(i: &str) -> IResult<&str, Vec<Entry>> {
    parser!(i32_ -> i32 = map_res(digit1, |s: &str| s.parse::<i32>()));
    parser!(u8_ -> u8 = map_res(digit1, |s: &str| s.parse::<u8>()));

    parser!(date -> Date = map_res(
        tuple((
            space0,
            i32_,
            space0,
            tag("-"),
            space0,
            &u8_,
            space0,
            tag("-"),
            space0,
            &u8_,
        )),
        |(_, year, _, _, _, month, _, _, _, day)| Date::try_from_ymd(year, month, day),
    ));

    parser!(time -> Time = map_res(
        tuple((
            space0,
            &u8_,
            space0,
            tag(":"),
            space0,
            &u8_,
            space0,
            tag(":"),
            space0,
            &u8_,
        )),
        |(_, hour, _, _, _, minute, _, _, _, second)| Time::try_from_hms(hour, minute, second),
    ));

    parser!(primitive_date_time -> PrimitiveDateTime = map(tuple((date, time)), |(date, time)| date.with_time(time)));

    parser!(sign -> i8 = map(is_a("+-"), |sign| match sign {
        "+" => 1i8,
        "-" => -1i8,
        _ => unreachable!(),
    }));

    parser!(digit -> u8 = map_res(take(1usize), |s: &str| s.parse::<u8>()));

    parser!(offset -> UtcOffset = map(
        tuple((space0, sign, &digit, &digit, &digit, &digit)),
        |(_, sign, h10, h1, m10, m1)| {
            UtcOffset::minutes(
                (sign as i16) * ((h10 as i16 * 10 + h1 as i16) * 60 + m10 as i16 * 10 + m1 as i16),
            )
        },
    ));

    parser!(offset_date_time -> OffsetDateTime = map(
        tuple((primitive_date_time, offset)),
        //TODO: Is this correct?
        |(primitive_date_time, offset)| primitive_date_time.assume_offset(offset),
    ));

    parser!(span -> Span = map(
        tuple((&offset_date_time, space0, tag(".."), opt(&offset_date_time))),
        |(start, _, _, end)| match end {
            None => Span::Active { start },
            Some(end) => Span::Closed { start, end },
        },
    ));

    parser!(tags -> Vec<&str> = map(
        preceded(
            space0,
            opt(delimited(
                tag("("),
                separated_nonempty_list(tag(","), is_not(",)")),
                tag(")"),
            )),
        ),
        |tags| tags.unwrap_or_else(|| vec![]),
    ));

    parser!(comment -> &str = preceded(tag("#"), recognize(many0(is_not("#\r\n")))));
    parser!(comments -> Vec<&str> = preceded(space0, many0(comment)));

    parser!(
        entry -> Entry = map(tuple((span, tags, comments)), |(span, tags, comments)| {
            Entry {
                span,
                tags: tags.into_iter().map(|t| t.to_owned()).collect(),
                comments: comments.into_iter().map(|c| c.to_owned()).collect(),
            }
        })
    );

    parser!(line -> Option<Entry> = terminated(opt(entry), tuple((space0, many0(line_ending)))));
    parser!(file -> Vec<Option<Entry>> = all_consuming(
        many0(
            map_opt(
                tuple((rest_len, line, rest_len)),
                |(before, line, after)| if before > after {
                    Some(line)
                } else {
                    None
                }
            ),
        )
    ));

    parser!(
        file_entries -> Vec<Entry> = map(file, |entries_per_line| {
            entries_per_line.into_iter().filter_map(|e| e).collect()
        })
    );

    file_entries(i)
}
