use {
    crate::{Entry, Span},
    nom::{
        bytes::complete::{is_a, is_not, tag, take},
        character::{
            complete::{digit1, line_ending, not_line_ending, space0},
        },
        combinator::{all_consuming, map, map_parser, map_res, opt, recognize},
        multi::{many0, separated_list, separated_nonempty_list},
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    },
    time::{Date, Time, UtcOffset},
};

pub fn parse(i: &str) -> IResult<&str, Vec<Entry>> {
    let i32_ = map_res(digit1, |s: &str| s.parse::<i32>());
    let u8_ = map_res(digit1, |s: &str| s.parse::<u8>());

    let date = map_res(
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
    );

    let time = map_res(
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
    );

    let primitive_date_time = map(tuple((date, time)), |(date, time)| date.with_time(time));

    let sign = map(is_a("+-"), |sign| match sign {
        "+" => 1i8,
        "-" => -1i8,
        _ => unreachable!(),
    });

    let digit = map_res(take(1usize), |s: &str| s.parse::<u8>());

    let offset = map(
        tuple((space0, sign, &digit, &digit, &digit, &digit)),
        |(_, sign, h10, h1, m10, m1)| {
            UtcOffset::minutes(
                (sign as i16) * ((h10 as i16 * 10 + h1 as i16) * 60 + m10 as i16 * 10 + m1 as i16),
            )
        },
    );

    let offset_date_time = map(
        tuple((primitive_date_time, offset)),
        //TODO: Is this correct?
        |(primitive_date_time, offset)| primitive_date_time.assume_offset(offset),
    );

    let span = map(
        tuple((&offset_date_time, space0, tag(".."), opt(&offset_date_time))),
        |(start, _, _, end)| match end {
            None => Span::Active { start },
            Some(end) => Span::Closed { start, end },
        },
    );

    let tags = map(
        preceded(
            space0,
            opt(delimited(
                tag("("),
                separated_nonempty_list(tag(","), is_not(",)")),
                tag(")"),
            )),
        ),
        |tags| tags.unwrap_or_else(|| vec![]),
    );

    let comment = preceded(tag("#"), recognize(many0(is_not("#\r\n"))));
    let comments = preceded(space0, many0(comment));

    let entry = map(tuple((span, tags, comments)), |(span, tags, comments)| {
        Entry {
            span,
            tags: tags.into_iter().map(|t| t.to_owned()).collect(),
            comments: comments.into_iter().map(|c| c.to_owned()).collect(),
        }
    });

    let line = terminated(opt(entry), space0);
    let file = all_consuming(separated_list(
        line_ending,
        map_parser(recognize(many0(not_line_ending)), all_consuming(line)),
    ));

    let file_entries = map(file, |entries_per_line| {
        entries_per_line.into_iter().filter_map(|e| e).collect()
    });

    file_entries(i)
}
