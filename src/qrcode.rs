use regex::Regex;

// source: https://www.thonky.com/qr-code-tutorial/

#[derive(Debug)]
pub enum QRCodeError {
    UnsupportedVersion,
    DataSizeMissing,
}

#[derive(Debug, PartialEq)]
enum QREncoding {
    Numeric,
    AlphaNumeric,
    Byte,
    _StructuredAppendMode,
}

#[derive(Debug, PartialEq)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

#[derive(Debug)]
pub struct QRCode {
    raw_data: String,
    encoding: QREncoding,
    error_correction: ErrorCorrectionLevel,
    version: usize,
    err_metadata: ErrorCorrectionMetaData,
}

fn find_encoding(data: &str) -> QREncoding {
    let numeric_regex      = Regex::new(r"^[0-9]*$").unwrap();
    let alphanumeric_regex = Regex::new(r"^[0-9A-Z\s$%\*\+-\./:]*$").unwrap();

    if numeric_regex.is_match(data) {
        QREncoding::Numeric
    } else if alphanumeric_regex.is_match(data) {
        QREncoding::AlphaNumeric
    } else {
        QREncoding::Byte
    }
}

const CHAR_CAPACITY: [(QREncoding, ErrorCorrectionLevel, [usize; 40]); 12]
    = [
        (QREncoding::Numeric,       ErrorCorrectionLevel::L, [41,77,127,187,255,322,370,461,552,652,772,883,1022,1101,1250,1408,1548,1725,1903,2061,2232,2409,2620,2812,3057,3283,3517,3669,3909,4158,4417,4686,4965,5253,5529,5836,6153,6479,6743,7089]),
        (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::L, [25,47,77,114,154,195,224,279,335,395,468,535,619,667,758,854,938,1046,1153,1249,1352,1460,1588,1704,1853,1990,2132,2223,2369,2520,2677,2840,3009,3183,3351,3537,3729,3927,4087,4296]),
        (QREncoding::Byte,          ErrorCorrectionLevel::L, [17,32,53,78,106,134,154,192,230,271,321,367,425,458,520,586,644,718,792,858,929,1003,1091,1171,1273,1367,1465,1528,1628,1732,1840,1952,2068,2188,2303,2431,2563,2699,2809,2953]),
        (QREncoding::Numeric,       ErrorCorrectionLevel::M, [34,63,101,149,202,255,293,365,432,513,604,691,796,871,991,1082,1212,1346,1500,1600,1708,1872,2059,2188,2395,2544,2701,2857,3035,3289,3486,3693,3909,4134,4343,4588,4775,5039,5313,5596]),
        (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::M, [20,38,61,90,122,154,178,221,262,311,366,419,483,528,600,656,734,816,909,970,1035,1134,1248,1326,1451,1542,1637,1732,1839,1994,2113,2238,2369,2506,2632,2780,2894,3054,3220,3391]),
        (QREncoding::Byte,          ErrorCorrectionLevel::M, [14,26,42,62,84,106,122,152,180,213,251,287,331,362,412,450,504,560,624,666,711,779,857,911,997,1059,1125,1190,1264,1370,1452,1538,1628,1722,1809,1911,1989,2099,2213,2331]),
        (QREncoding::Numeric,       ErrorCorrectionLevel::Q, [27,48,77,111,144,178,207,259,312,364,427,489,580,621,703,775,876,948,1063,1159,1224,1358,1468,1588,1718,1804,1933,2085,2181,2358,2473,2670,2805,2949,3081,3244,3417,3599,3791,3993]),
        (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::Q, [16,29,47,67,87,108,125,157,189,221,259,296,352,376,426,470,531,574,644,702,742,823,890,963,1041,1094,1172,1263,1322,1429,1499,1618,1700,1787,1867,1966,2071,2181,2298,2420]),
        (QREncoding::Byte,          ErrorCorrectionLevel::Q, [11,20,32,46,60,74,86,108,130,151,177,203,241,258,292,322,364,394,442,482,509,565,611,661,715,751,805,868,908,982,1030,1112,1168,1228,1283,1351,1423,1499,1579,1663]),
        (QREncoding::Numeric,       ErrorCorrectionLevel::H, [17,34,58,82,106,139,154,202,235,288,331,374,427,468,530,602,674,746,813,919,969,1056,1108,1228,1286,1425,1501,1581,1677,1782,1897,2022,2157,2301,2361,2524,2625,2735,2927,3057]),
        (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::H, [10,20,35,50,64,84,93,122,143,174,200,227,259,283,321,365,408,452,493,557,587,640,672,744,779,864,910,958,1016,1080,1150,1226,1307,1394,1431,1530,1591,1658,1774,1852]),
        (QREncoding::Byte,          ErrorCorrectionLevel::H, [7,14,24,34,44,58,64,84,98,119,137,155,177,194,220,250,280,310,338,382,403,439,461,511,535,593,625,658,698,742,790,842,898,958,983,1051,1093,1139,1219,1273]),
    ];

// looks up char capacity per version depending on the encoding and error correction level
fn get_size(data_size: usize, encoding: &QREncoding, error_correction: &ErrorCorrectionLevel) -> usize {
    let current_option = (encoding, error_correction);

    for option in CHAR_CAPACITY {
        if &option.0 == current_option.0 && &option.1 == current_option.1 {
            for i in 0..20 {
                let size = option.2[i];
                if data_size <= size { return i + 1 }
            }
        }
    }

    return 0;
}

fn get_size_from_version(version: usize) -> usize {
    version * 4 + 21
}

#[derive(Debug)]
struct ErrorCorrectionMetaData {
    words_per_block: usize,
    blocks_grp1: usize,
    blocks_grp2: usize,
    words_per_block_grp1: usize,
    words_per_block_grp2: usize,
}

impl ErrorCorrectionMetaData {
    pub fn new(words_per_block: usize, blocks_grp1: usize, blocks_grp2: usize, words_per_block_grp1: usize, words_per_block_grp2: usize) -> Self {
        Self { words_per_block, blocks_grp1, blocks_grp2, words_per_block_grp1, words_per_block_grp2 }
    }

    pub fn total_code_words(&self) -> usize {
        return self.blocks_grp1 * self.words_per_block_grp1 + self.blocks_grp2 * self.words_per_block_grp2
    }
}

impl From<&[usize]> for ErrorCorrectionMetaData {
    fn from(value: &[usize]) -> Self {
        assert!(value.len() == 5, "Invalid data for ErrorCorrectionMetaData!");

        Self::new(value[0], value[1], value[3], value[2], value[4])
    }
}

// this look up table contains the number of data codewords for error correction and how to group them into blocks
// 0: codewords per block
// 1: number blocks group 1
// 2: number of data codewords in each block in group 1
// 3: number blocks group 2
// 4: number of data codewords in each block in group 2
// source: https://www.thonky.com/qr-code-tutorial/error-correction-table
const ERROR_CORRECTION_DATA: [(usize, ErrorCorrectionLevel, [usize; 5]); 160] = [
    (1, ErrorCorrectionLevel::L, [7,1,19,0,0]),
    (2, ErrorCorrectionLevel::L, [10,1,34,0,0]),
    (3, ErrorCorrectionLevel::L, [15,1,55,0,0]),
    (4, ErrorCorrectionLevel::L, [20,1,80,0,0]),
    (5, ErrorCorrectionLevel::L, [26,1,108,0,0]),
    (6, ErrorCorrectionLevel::L, [18,2,68,0,0]),
    (7, ErrorCorrectionLevel::L, [20,2,78,0,0]),
    (8, ErrorCorrectionLevel::L, [24,2,97,0,0]),
    (9, ErrorCorrectionLevel::L, [30,2,116,0,0]),
    (10, ErrorCorrectionLevel::L, [18,2,68,2,69]),
    (11, ErrorCorrectionLevel::L, [20,4,81,0,0]),
    (12, ErrorCorrectionLevel::L, [24,2,92,2,93]),
    (13, ErrorCorrectionLevel::L, [26,4,107,0,0]),
    (14, ErrorCorrectionLevel::L, [30,3,115,1,116]),
    (15, ErrorCorrectionLevel::L, [22,5,87,1,88]),
    (16, ErrorCorrectionLevel::L, [24,5,98,1,99]),
    (17, ErrorCorrectionLevel::L, [28,1,107,5,108]),
    (18, ErrorCorrectionLevel::L, [30,5,120,1,121]),
    (19, ErrorCorrectionLevel::L, [28,3,113,4,114]),
    (20, ErrorCorrectionLevel::L, [28,3,107,5,108]),
    (21, ErrorCorrectionLevel::L, [28,4,116,4,117]),
    (22, ErrorCorrectionLevel::L, [28,2,111,7,112]),
    (23, ErrorCorrectionLevel::L, [30,4,121,5,122]),
    (24, ErrorCorrectionLevel::L, [30,6,117,4,118]),
    (25, ErrorCorrectionLevel::L, [26,8,106,4,107]),
    (26, ErrorCorrectionLevel::L, [28,10,114,2,115]),
    (27, ErrorCorrectionLevel::L, [30,8,122,4,123]),
    (28, ErrorCorrectionLevel::L, [30,3,117,10,118]),
    (29, ErrorCorrectionLevel::L, [30,7,116,7,117]),
    (30, ErrorCorrectionLevel::L, [30,5,115,10,116]),
    (31, ErrorCorrectionLevel::L, [30,13,115,3,116]),
    (32, ErrorCorrectionLevel::L, [30,17,115,0,0]),
    (33, ErrorCorrectionLevel::L, [30,17,115,1,116]),
    (34, ErrorCorrectionLevel::L, [30,13,115,6,116]),
    (35, ErrorCorrectionLevel::L, [30,12,121,7,122]),
    (36, ErrorCorrectionLevel::L, [30,6,121,14,122]),
    (37, ErrorCorrectionLevel::L, [30,17,122,4,123]),
    (38, ErrorCorrectionLevel::L, [30,4,122,18,123]),
    (39, ErrorCorrectionLevel::L, [30,20,117,4,118]),
    (40, ErrorCorrectionLevel::L, [30,19,118,6,119]),
    (1, ErrorCorrectionLevel::M, [10,1,16,0,0]),
    (2, ErrorCorrectionLevel::M, [16,1,28,0,0]),
    (3, ErrorCorrectionLevel::M, [26,1,44,0,0]),
    (4, ErrorCorrectionLevel::M, [18,2,32,0,0]),
    (5, ErrorCorrectionLevel::M, [24,2,43,0,0]),
    (6, ErrorCorrectionLevel::M, [16,4,27,0,0]),
    (7, ErrorCorrectionLevel::M, [18,4,31,0,0]),
    (8, ErrorCorrectionLevel::M, [22,2,38,2,39]),
    (9, ErrorCorrectionLevel::M, [22,3,36,2,37]),
    (10, ErrorCorrectionLevel::M, [26,4,43,1,44]),
    (11, ErrorCorrectionLevel::M, [30,1,50,4,51]),
    (12, ErrorCorrectionLevel::M, [22,6,36,2,37]),
    (13, ErrorCorrectionLevel::M, [22,8,37,1,38]),
    (14, ErrorCorrectionLevel::M, [24,4,40,5,41]),
    (15, ErrorCorrectionLevel::M, [24,5,41,5,42]),
    (16, ErrorCorrectionLevel::M, [28,7,45,3,46]),
    (17, ErrorCorrectionLevel::M, [28,10,46,1,47]),
    (18, ErrorCorrectionLevel::M, [26,9,43,4,44]),
    (19, ErrorCorrectionLevel::M, [26,3,44,11,45]),
    (20, ErrorCorrectionLevel::M, [26,3,41,13,42]),
    (21, ErrorCorrectionLevel::M, [26,17,42,0,0]),
    (22, ErrorCorrectionLevel::M, [28,17,46,0,0]),
    (23, ErrorCorrectionLevel::M, [28,4,47,14,48]),
    (24, ErrorCorrectionLevel::M, [28,6,45,14,46]),
    (25, ErrorCorrectionLevel::M, [28,8,47,13,48]),
    (26, ErrorCorrectionLevel::M, [28,19,46,4,47]),
    (27, ErrorCorrectionLevel::M, [28,22,45,3,46]),
    (28, ErrorCorrectionLevel::M, [28,3,45,23,46]),
    (29, ErrorCorrectionLevel::M, [28,21,45,7,46]),
    (30, ErrorCorrectionLevel::M, [28,19,47,10,48]),
    (31, ErrorCorrectionLevel::M, [28,2,46,29,47]),
    (32, ErrorCorrectionLevel::M, [28,10,46,23,47]),
    (33, ErrorCorrectionLevel::M, [28,14,46,21,47]),
    (34, ErrorCorrectionLevel::M, [28,14,46,23,47]),
    (35, ErrorCorrectionLevel::M, [28,12,47,26,48]),
    (36, ErrorCorrectionLevel::M, [28,6,47,34,48]),
    (37, ErrorCorrectionLevel::M, [28,29,46,14,47]),
    (38, ErrorCorrectionLevel::M, [28,13,46,32,47]),
    (39, ErrorCorrectionLevel::M, [28,40,47,7,48]),
    (40, ErrorCorrectionLevel::M, [28,18,47,31,48]),
    (1, ErrorCorrectionLevel::Q, [13,1,13,0,0]),
    (2, ErrorCorrectionLevel::Q, [22,1,22,0,0]),
    (3, ErrorCorrectionLevel::Q, [18,2,17,0,0]),
    (4, ErrorCorrectionLevel::Q, [26,2,24,0,0]),
    (5, ErrorCorrectionLevel::Q, [18,2,15,2,16]),
    (6, ErrorCorrectionLevel::Q, [24,4,19,0,0]),
    (7, ErrorCorrectionLevel::Q, [18,2,14,4,15]),
    (8, ErrorCorrectionLevel::Q, [22,4,18,2,19]),
    (9, ErrorCorrectionLevel::Q, [20,4,16,4,17]),
    (10, ErrorCorrectionLevel::Q, [24,6,19,2,20]),
    (11, ErrorCorrectionLevel::Q, [28,4,22,4,23]),
    (12, ErrorCorrectionLevel::Q, [26,4,20,6,21]),
    (13, ErrorCorrectionLevel::Q, [24,8,20,4,21]),
    (14, ErrorCorrectionLevel::Q, [20,11,16,5,17]),
    (15, ErrorCorrectionLevel::Q, [30,5,24,7,25]),
    (16, ErrorCorrectionLevel::Q, [24,15,19,2,20]),
    (17, ErrorCorrectionLevel::Q, [28,1,22,15,23]),
    (18, ErrorCorrectionLevel::Q, [28,17,22,1,23]),
    (19, ErrorCorrectionLevel::Q, [26,17,21,4,22]),
    (20, ErrorCorrectionLevel::Q, [30,15,24,5,25]),
    (21, ErrorCorrectionLevel::Q, [28,17,22,6,23]),
    (22, ErrorCorrectionLevel::Q, [30,7,24,16,25]),
    (23, ErrorCorrectionLevel::Q, [30,11,24,14,25]),
    (24, ErrorCorrectionLevel::Q, [30,11,24,16,25]),
    (25, ErrorCorrectionLevel::Q, [30,7,24,22,25]),
    (26, ErrorCorrectionLevel::Q, [28,28,22,6,23]),
    (27, ErrorCorrectionLevel::Q, [30,8,23,26,24]),
    (28, ErrorCorrectionLevel::Q, [30,4,24,31,25]),
    (29, ErrorCorrectionLevel::Q, [30,1,23,37,24]),
    (30, ErrorCorrectionLevel::Q, [30,15,24,25,25]),
    (31, ErrorCorrectionLevel::Q, [30,42,24,1,25]),
    (32, ErrorCorrectionLevel::Q, [30,10,24,35,25]),
    (33, ErrorCorrectionLevel::Q, [30,29,24,19,25]),
    (34, ErrorCorrectionLevel::Q, [30,44,24,7,25]),
    (35, ErrorCorrectionLevel::Q, [30,39,24,14,25]),
    (36, ErrorCorrectionLevel::Q, [30,46,24,10,25]),
    (37, ErrorCorrectionLevel::Q, [30,49,24,10,25]),
    (38, ErrorCorrectionLevel::Q, [30,48,24,14,25]),
    (39, ErrorCorrectionLevel::Q, [30,43,24,22,25]),
    (40, ErrorCorrectionLevel::Q, [30,34,24,34,25]),
    (1, ErrorCorrectionLevel::H, [17,1,9,0,0]),
    (2, ErrorCorrectionLevel::H, [28,1,16,0,0]),
    (3, ErrorCorrectionLevel::H, [22,2,13,0,0]),
    (4, ErrorCorrectionLevel::H, [16,4,9,0,0]),
    (5, ErrorCorrectionLevel::H, [22,2,11,2,12]),
    (6, ErrorCorrectionLevel::H, [28,4,15,0,0]),
    (7, ErrorCorrectionLevel::H, [26,4,13,1,14]),
    (8, ErrorCorrectionLevel::H, [26,4,14,2,15]),
    (9, ErrorCorrectionLevel::H, [24,4,12,4,13]),
    (10, ErrorCorrectionLevel::H, [28,6,15,2,16]),
    (11, ErrorCorrectionLevel::H, [24,3,12,8,13]),
    (12, ErrorCorrectionLevel::H, [28,7,14,4,15]),
    (13, ErrorCorrectionLevel::H, [22,12,11,4,12]),
    (14, ErrorCorrectionLevel::H, [24,11,12,5,13]),
    (15, ErrorCorrectionLevel::H, [24,11,12,7,13]),
    (16, ErrorCorrectionLevel::H, [30,3,15,13,16]),
    (17, ErrorCorrectionLevel::H, [28,2,14,17,15]),
    (18, ErrorCorrectionLevel::H, [28,2,14,19,15]),
    (19, ErrorCorrectionLevel::H, [26,9,13,16,14]),
    (20, ErrorCorrectionLevel::H, [28,15,15,10,16]),
    (21, ErrorCorrectionLevel::H, [30,19,16,6,17]),
    (22, ErrorCorrectionLevel::H, [24,34,13,0,0]),
    (23, ErrorCorrectionLevel::H, [30,16,15,14,16]),
    (24, ErrorCorrectionLevel::H, [30,30,16,2,17]),
    (25, ErrorCorrectionLevel::H, [30,22,15,13,16]),
    (26, ErrorCorrectionLevel::H, [30,33,16,4,17]),
    (27, ErrorCorrectionLevel::H, [30,12,15,28,16]),
    (28, ErrorCorrectionLevel::H, [30,11,15,31,16]),
    (29, ErrorCorrectionLevel::H, [30,19,15,26,16]),
    (30, ErrorCorrectionLevel::H, [30,23,15,25,16]),
    (31, ErrorCorrectionLevel::H, [30,23,15,28,16]),
    (32, ErrorCorrectionLevel::H, [30,19,15,35,16]),
    (33, ErrorCorrectionLevel::H, [30,11,15,46,16]),
    (34, ErrorCorrectionLevel::H, [30,59,16,1,17]),
    (35, ErrorCorrectionLevel::H, [30,22,15,41,16]),
    (36, ErrorCorrectionLevel::H, [30,2,15,64,16]),
    (37, ErrorCorrectionLevel::H, [30,24,15,46,16]),
    (38, ErrorCorrectionLevel::H, [30,42,15,32,16]),
    (39, ErrorCorrectionLevel::H, [30,10,15,67,16]),
    (40, ErrorCorrectionLevel::H, [30,20,15,61,16]),
];

fn get_err_metadata(version: usize, error_correction: &ErrorCorrectionLevel) -> Option<ErrorCorrectionMetaData> {
    let current_option = (version, error_correction);
    for option in ERROR_CORRECTION_DATA.iter() {
        if option.0 == current_option.0 && &option.1 == current_option.1 {
            return Some(ErrorCorrectionMetaData::from(&option.2[..]));
        }
    }

    None
}

fn encode_numeric(bit_buffer: &mut String, current_slice: &str) {
    let parsed_int: i32 = current_slice.parse().unwrap();

    if parsed_int > 100 {
        bit_buffer.push_str(&format!("{:010b}", parsed_int));
    } else if parsed_int < 100 && parsed_int >= 10 {
        bit_buffer.push_str(&format!("{:07b}", parsed_int));
    } else {
        bit_buffer.push_str(&format!("{:04b}", parsed_int));
    };
}

fn alphanumeric_get_char_code(c: char) -> u16 {
    match c {
    '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5,
    '6' => 6, '7' => 7, '8' => 8, '9' => 9, 'A' => 10, 'B' => 11, 
    'C' => 12, 'D' => 13, 'E' => 14, 'F' => 15, 'G' => 16, 'H' => 17, 'I' => 18, 
    'J' => 19, 'K' => 20, 'L' => 21, 'M' => 22, 'N' => 23, 'O' => 24, 'P' => 25, 
    'Q' => 26, 'R' => 27, 'S' => 28, 'T' => 29, 'U' => 30, 'V' => 31, 
    'W' => 32, 'X' => 33, 'Y' => 34, 'Z' => 35, ' ' => 36, '$' => 37, '%' => 38, 
    '*' => 39, '+' => 40, '-' => 41, '.' => 42, '/' => 43, ':' => 44, _   => 99,
    }
}

fn encode_alphanumeric(current_slice: &str) -> String {
    let mut iter = current_slice.chars();

    if current_slice.len() == 2 {
        let code1 = alphanumeric_get_char_code(iter.next().unwrap());
        let code2 = alphanumeric_get_char_code(iter.next().unwrap());

        let code = code1 * 45 + code2;
        format!("{:011b}", code)
    } else {
        format!("{:06b}", alphanumeric_get_char_code(iter.next().unwrap()))
    }
}

fn get_encoding(qrcode: &QRCode) -> &'static str {
    // add magic number for encoding type
    match qrcode.encoding {
        QREncoding::Numeric      => "0001",
        QREncoding::AlphaNumeric => "0010",
        QREncoding::Byte         => "0100",
        _                        => "0000",
    }
}

fn get_data_len(qrcode: &QRCode) -> Result<String, QRCodeError> {
    // lookup length of data 0 padded to specific lenght
    Ok(match qrcode.version {
        1..=9 => match qrcode.encoding {
            QREncoding::Numeric      => format!("{:010b}", qrcode.raw_data.len()),
            QREncoding::AlphaNumeric => format!("{:09b}",  qrcode.raw_data.len()),
            QREncoding::Byte         => format!("{:08b}",  qrcode.raw_data.len()),
            _                        => "0000".to_owned(),
        },
        10..=26 => match qrcode.encoding {
            QREncoding::Numeric      => format!("{:012b}",  qrcode.raw_data.len()),
            QREncoding::AlphaNumeric => format!("{:011b}",  qrcode.raw_data.len()),
            QREncoding::Byte         => format!("{:016b}",  qrcode.raw_data.len()),
            _                        => "0000".to_owned(),
        },
        27..=40 => match qrcode.encoding {
            QREncoding::Numeric      => format!("{:014b}",  qrcode.raw_data.len()),
            QREncoding::AlphaNumeric => format!("{:013b}",  qrcode.raw_data.len()),
            QREncoding::Byte         => format!("{:016b}",  qrcode.raw_data.len()),
            _                        => "0000".to_owned(),
        },
        _ => { return Err(QRCodeError::UnsupportedVersion); }
    })
}

fn add_padding(qrcode: &QRCode, bit_buffer: &mut String) -> Result<(), QRCodeError> {
    // get the maximum number of bits
    let bit_size = qrcode.err_metadata.total_code_words();
    let missing_bits = bit_size - bit_buffer.len();

    // add terminator of 0s => at most four 0s
    let terminator_len = if missing_bits <= 4 { missing_bits } else { 4 };
    for _ in 0..terminator_len { bit_buffer.push('0'); }

    // pad to multiple of 8
    let diff_to_eight = match bit_buffer.len() % 8 {
        0 => 0,
        val => 8 - val,
    };
    for _ in 0..diff_to_eight { bit_buffer.push('0'); }

    // add padding to reach maximum data lenght
    let missing_bytes = (bit_size - bit_buffer.len()) / 8;
    let padding_bytes = vec!["11101100", "00010001"];
    for i in 0..missing_bytes {
        bit_buffer.push_str(padding_bytes[i % 2]);
    }

    return Ok(());
}

fn encode_data(qrcode: &QRCode, bit_buffer: &mut String) {
    match qrcode.encoding {
        QREncoding::Numeric => {
            let scaled_len = qrcode.raw_data.len() / 3;
            let left_over  = qrcode.raw_data.len() % 3;
        
            for i in 0..scaled_len {
                let current_slice = &qrcode.raw_data[i..i+3];
                encode_numeric(bit_buffer, current_slice);
            }
        
            if left_over > 0 {
                encode_numeric(bit_buffer, &qrcode.raw_data[qrcode.raw_data.len() - left_over..qrcode.raw_data.len()]);
            }
        },
        QREncoding::AlphaNumeric => {
            let scaled_len = qrcode.raw_data.len() / 2;
            let left_over  = qrcode.raw_data.len() % 2;

            for i in 0..scaled_len {
                let current_slice = &qrcode.raw_data[i*2..(i+1)*2];
                let encoded = encode_alphanumeric(current_slice);
                bit_buffer.push_str(&encoded);
            }

            if left_over > 0 {
                let len = qrcode.raw_data.len();
                let encoded = encode_alphanumeric(&qrcode.raw_data[len-1..len]);
                bit_buffer.push_str(&encoded);
            }
        },
        QREncoding::Byte => {
            let bytes = qrcode.raw_data.bytes();
            bytes.for_each(|byte| {
                bit_buffer.push_str(&format!("{:08b}", byte));
            });
        }
        _ => { }
    }
}

impl QRCode {
    pub fn new(data: String, error_correction: ErrorCorrectionLevel) -> Result<Self, String> {
        let encoding = find_encoding(&data);
        let version = get_size(data.len(), &encoding, &error_correction);

        if version == 0 {
            return Err(String::from("Data too long for encoding!"))
        }

        let err_metadata = get_err_metadata(version, &error_correction).unwrap();

        Ok(QRCode {
            raw_data: data,
            encoding,
            error_correction,
            version,
            err_metadata,
        })
    }

    pub fn encode(&self) -> Result<String, QRCodeError> {
        let mut bit_buffer = String::new();

        let encoding = get_encoding(self);
        bit_buffer.push_str(encoding);

        let data_len = get_data_len(self)?;
        bit_buffer.push_str(&data_len);

        encode_data(self, &mut bit_buffer);
        add_padding(self, &mut bit_buffer)?;
        
        return Ok(bit_buffer);
    }

    pub fn get_coeffs(&self) -> Result<Vec<u8>, QRCodeError> {
        let bitbuf = self.encode()?;
        assert!(bitbuf.len() % 8 == 0, "Bit buffer has invalid length that is not a multiple of 8.");
        let chunks = bitbuf.len() / 8;

        let mut coeffs = vec![0; chunks];
        for i in 0..chunks {
            let byte = &bitbuf[i*8..(i + 1)*8];
            let coeff = u8::from_str_radix(byte, 2)
                                .expect("This should be a valid bin string!");
            coeffs[i] = coeff;
        }

        Ok(coeffs)
    }
}
