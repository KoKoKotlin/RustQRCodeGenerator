use regex::Regex;

// source: https://www.thonky.com/qr-code-tutorial/

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
    version: u8
}

fn find_encoding(data: &String) -> QREncoding {
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


fn get_size(data_size: usize, encoding: &QREncoding, error_correction: &ErrorCorrectionLevel) -> u8 { // returs the version of the qrcode
    let sizes: Vec<(QREncoding, ErrorCorrectionLevel, Vec<usize>)>
        = vec![
            (QREncoding::Numeric,       ErrorCorrectionLevel::L, vec![41,77,127,187,255,322,370,461,552,652,772,883,1022,1101,1250,1408,1548,1725,1903,2061,2232,2409,2620,2812,3057,3283,3517,3669,3909,4158,4417,4686,4965,5253,5529,5836,6153,6479,6743,7089]),
            (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::L, vec![25,47,77,114,154,195,224,279,335,395,468,535,619,667,758,854,938,1046,1153,1249,1352,1460,1588,1704,1853,1990,2132,2223,2369,2520,2677,2840,3009,3183,3351,3537,3729,3927,4087,4296]),
            (QREncoding::Byte,          ErrorCorrectionLevel::L, vec![17,32,53,78,106,134,154,192,230,271,321,367,425,458,520,586,644,718,792,858,929,1003,1091,1171,1273,1367,1465,1528,1628,1732,1840,1952,2068,2188,2303,2431,2563,2699,2809,2953]),
            (QREncoding::Numeric,       ErrorCorrectionLevel::M, vec![34,63,101,149,202,255,293,365,432,513,604,691,796,871,991,1082,1212,1346,1500,1600,1708,1872,2059,2188,2395,2544,2701,2857,3035,3289,3486,3693,3909,4134,4343,4588,4775,5039,5313,5596]),
            (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::M, vec![20,38,61,90,122,154,178,221,262,311,366,419,483,528,600,656,734,816,909,970,1035,1134,1248,1326,1451,1542,1637,1732,1839,1994,2113,2238,2369,2506,2632,2780,2894,3054,3220,3391]),
            (QREncoding::Byte,          ErrorCorrectionLevel::M, vec![14,26,42,62,84,106,122,152,180,213,251,287,331,362,412,450,504,560,624,666,711,779,857,911,997,1059,1125,1190,1264,1370,1452,1538,1628,1722,1809,1911,1989,2099,2213,2331]),
            (QREncoding::Numeric,       ErrorCorrectionLevel::Q, vec![27,48,77,111,144,178,207,259,312,364,427,489,580,621,703,775,876,948,1063,1159,1224,1358,1468,1588,1718,1804,1933,2085,2181,2358,2473,2670,2805,2949,3081,3244,3417,3599,3791,3993]),
            (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::Q, vec![16,29,47,67,87,108,125,157,189,221,259,296,352,376,426,470,531,574,644,702,742,823,890,963,1041,1094,1172,1263,1322,1429,1499,1618,1700,1787,1867,1966,2071,2181,2298,2420]),
            (QREncoding::Byte,          ErrorCorrectionLevel::Q, vec![11,20,32,46,60,74,86,108,130,151,177,203,241,258,292,322,364,394,442,482,509,565,611,661,715,751,805,868,908,982,1030,1112,1168,1228,1283,1351,1423,1499,1579,1663]),
            (QREncoding::Numeric,       ErrorCorrectionLevel::H, vec![17,34,58,82,106,139,154,202,235,288,331,374,427,468,530,602,674,746,813,919,969,1056,1108,1228,1286,1425,1501,1581,1677,1782,1897,2022,2157,2301,2361,2524,2625,2735,2927,3057]),
            (QREncoding::AlphaNumeric,  ErrorCorrectionLevel::H, vec![10,20,35,50,64,84,93,122,143,174,200,227,259,283,321,365,408,452,493,557,587,640,672,744,779,864,910,958,1016,1080,1150,1226,1307,1394,1431,1530,1591,1658,1774,1852]),
            (QREncoding::Byte,          ErrorCorrectionLevel::H, vec![7,14,24,34,44,58,64,84,98,119,137,155,177,194,220,250,280,310,338,382,403,439,461,511,535,593,625,658,698,742,790,842,898,958,983,1051,1093,1139,1219,1273]),
        ];

    let current_option = (encoding, error_correction);

    for option in sizes {
        if &option.0 == current_option.0 && &option.1 == current_option.1 {
            for i in 0..20 {
                let size = option.2[i];
                if data_size < size { return (i + 1) as u8 }
            }
        }
    }

    return 0;
}

fn get_size_from_version(version: u8) -> usize {
    (version as usize) * 4 + 21
}

fn get_data_size(version: u8, error_correction: &ErrorCorrectionLevel) -> Option<Vec<u32>> {
    let data = vec![
    (1,  ErrorCorrectionLevel::L, vec![1,19,7,1,19]),
    (2,  ErrorCorrectionLevel::L, vec![1,34,10,1,34]),
    (3,  ErrorCorrectionLevel::L, vec![1,55,15,1,55]),
    (4,  ErrorCorrectionLevel::L, vec![1,80,20,1,80]),
    (5,  ErrorCorrectionLevel::L, vec![1,108,26,1,108]),
    (6,  ErrorCorrectionLevel::L, vec![1,136,18,2,68]),
    (7,  ErrorCorrectionLevel::L, vec![1,156,20,2,78]),
    (8,  ErrorCorrectionLevel::L, vec![1,194,24,2,97]),
    (9,  ErrorCorrectionLevel::L, vec![1,232,30,2,116]),
    (10, ErrorCorrectionLevel::L, vec![2,274,18,2,68,2,69]),
    (11, ErrorCorrectionLevel::L, vec![1,324,20,4,81]),
    (12, ErrorCorrectionLevel::L, vec![2,370,24,2,92,2,93]),
    (13, ErrorCorrectionLevel::L, vec![1,428,26,4,107]),
    (14, ErrorCorrectionLevel::L, vec![2,461,30,3,115,1,116]),
    (15, ErrorCorrectionLevel::L, vec![2,523,22,5,87,1,88]),
    (16, ErrorCorrectionLevel::L, vec![2,589,24,5,98,1,99]),
    (17, ErrorCorrectionLevel::L, vec![2,647,28,1,107,5,108]),
    (18, ErrorCorrectionLevel::L, vec![2,721,30,5,120,1,121]),
    (19, ErrorCorrectionLevel::L, vec![2,795,28,3,113,4,114]),
    (20, ErrorCorrectionLevel::L, vec![2,861,28,3,107,5,108]),
    (21, ErrorCorrectionLevel::L, vec![2,932,28,4,116,4,117]),
    (22, ErrorCorrectionLevel::L, vec![2,1006,28,2,111,7,112]),
    (23, ErrorCorrectionLevel::L, vec![2,1094,30,4,121,5,122]),
    (24, ErrorCorrectionLevel::L, vec![2,1174,30,6,117,4,118]),
    (25, ErrorCorrectionLevel::L, vec![2,1276,26,8,106,4,107]),
    (26, ErrorCorrectionLevel::L, vec![2,1370,28,10,114,2,115]),
    (27, ErrorCorrectionLevel::L, vec![2,1468,30,8,122,4,123]),
    (28, ErrorCorrectionLevel::L, vec![2,1531,30,3,117,10,118]),
    (29, ErrorCorrectionLevel::L, vec![2,1631,30,7,116,7,117]),
    (30, ErrorCorrectionLevel::L, vec![2,1735,30,5,115,10,116]),
    (31, ErrorCorrectionLevel::L, vec![2,1843,30,13,115,3,116]),
    (32, ErrorCorrectionLevel::L, vec![1,1955,30,17,115]),
    (33, ErrorCorrectionLevel::L, vec![2,2071,30,17,115,1,116]),
    (34, ErrorCorrectionLevel::L, vec![2,2191,30,13,115,6,116]),
    (35, ErrorCorrectionLevel::L, vec![2,2306,30,12,121,7,122]),
    (36, ErrorCorrectionLevel::L, vec![2,2434,30,6,121,14,122]),
    (37, ErrorCorrectionLevel::L, vec![2,2566,30,17,122,4,123]),
    (38, ErrorCorrectionLevel::L, vec![2,2702,30,4,122,18,123]),
    (39, ErrorCorrectionLevel::L, vec![2,2812,30,20,117,4,118]),
    (40, ErrorCorrectionLevel::L, vec![2,2956,30,19,118,6,119]),
    (1,  ErrorCorrectionLevel::M, vec![1,16,10,1,16]),
    (2,  ErrorCorrectionLevel::M, vec![1,28,16,1,28]),
    (3,  ErrorCorrectionLevel::M, vec![1,44,26,1,44]),
    (4,  ErrorCorrectionLevel::M, vec![1,64,18,2,32]),
    (5,  ErrorCorrectionLevel::M, vec![1,86,24,2,43]),
    (6,  ErrorCorrectionLevel::M, vec![1,108,16,4,27]),
    (7,  ErrorCorrectionLevel::M, vec![1,124,18,4,31]),
    (8,  ErrorCorrectionLevel::M, vec![2,154,22,2,38,2,39]),
    (9,  ErrorCorrectionLevel::M, vec![2,182,22,3,36,2,37]),
    (10, ErrorCorrectionLevel::M, vec![2,216,26,4,43,1,44]),
    (11, ErrorCorrectionLevel::M, vec![2,254,30,1,50,4,51]),
    (12, ErrorCorrectionLevel::M, vec![2,290,22,6,36,2,37]),
    (13, ErrorCorrectionLevel::M, vec![2,334,22,8,37,1,38]),
    (14, ErrorCorrectionLevel::M, vec![2,365,24,4,40,5,41]),
    (15, ErrorCorrectionLevel::M, vec![2,415,24,5,41,5,42]),
    (16, ErrorCorrectionLevel::M, vec![2,453,28,7,45,3,46]),
    (17, ErrorCorrectionLevel::M, vec![2,507,28,10,46,1,47]),
    (18, ErrorCorrectionLevel::M, vec![2,563,26,9,43,4,44]),
    (19, ErrorCorrectionLevel::M, vec![2,627,26,3,44,11,45]),
    (20, ErrorCorrectionLevel::M, vec![2,669,26,3,41,13,42]),
    (21, ErrorCorrectionLevel::M, vec![1,714,26,17,42]),
    (22, ErrorCorrectionLevel::M, vec![1,782,28,17,46]),
    (23, ErrorCorrectionLevel::M, vec![2,860,28,4,47,14,48]),
    (24, ErrorCorrectionLevel::M, vec![2,914,28,6,45,14,46]),
    (25, ErrorCorrectionLevel::M, vec![2,1000,28,8,47,13,48]),
    (26, ErrorCorrectionLevel::M, vec![2,1062,28,19,46,4,47]),
    (27, ErrorCorrectionLevel::M, vec![2,1128,28,22,45,3,46]),
    (28, ErrorCorrectionLevel::M, vec![2,1193,28,3,45,23,46]),
    (29, ErrorCorrectionLevel::M, vec![2,1267,28,21,45,7,46]),
    (30, ErrorCorrectionLevel::M, vec![2,1373,28,19,47,10,48]),
    (31, ErrorCorrectionLevel::M, vec![2,1455,28,2,46,29,47]),
    (32, ErrorCorrectionLevel::M, vec![2,1541,28,10,46,23,47]),
    (33, ErrorCorrectionLevel::M, vec![2,1631,28,14,46,21,47]),
    (34, ErrorCorrectionLevel::M, vec![2,1725,28,14,46,23,47]),
    (35, ErrorCorrectionLevel::M, vec![2,1812,28,12,47,26,48]),
    (36, ErrorCorrectionLevel::M, vec![2,1914,28,6,47,34,48]),
    (37, ErrorCorrectionLevel::M, vec![2,1992,28,29,46,14,47]),
    (38, ErrorCorrectionLevel::M, vec![2,2102,28,13,46,32,47]),
    (39, ErrorCorrectionLevel::M, vec![2,2216,28,40,47,7,48]),
    (40, ErrorCorrectionLevel::M, vec![2,2334,28,18,47,31,48]),
    (1,  ErrorCorrectionLevel::Q, vec![1,13,13,1,13]),
    (2,  ErrorCorrectionLevel::Q, vec![1,22,22,1,22]),
    (3,  ErrorCorrectionLevel::Q, vec![1,34,18,2,17]),
    (4,  ErrorCorrectionLevel::Q, vec![1,48,26,2,24]),
    (5,  ErrorCorrectionLevel::Q, vec![2,62,18,2,15,2,16]),
    (6,  ErrorCorrectionLevel::Q, vec![1,76,24,4,19]),
    (7,  ErrorCorrectionLevel::Q, vec![2,88,18,2,14,4,15]),
    (8,  ErrorCorrectionLevel::Q, vec![2,110,22,4,18,2,19]),
    (9,  ErrorCorrectionLevel::Q, vec![2,132,20,4,16,4,17]),
    (10, ErrorCorrectionLevel::Q, vec![2,154,24,6,19,2,20]),
    (11, ErrorCorrectionLevel::Q, vec![2,180,28,4,22,4,23]),
    (12, ErrorCorrectionLevel::Q, vec![2,206,26,4,20,6,21]),
    (13, ErrorCorrectionLevel::Q, vec![2,244,24,8,20,4,21]),
    (14, ErrorCorrectionLevel::Q, vec![2,261,20,11,16,5,17]),
    (15, ErrorCorrectionLevel::Q, vec![2,295,30,5,24,7,25]),
    (16, ErrorCorrectionLevel::Q, vec![2,325,24,15,19,2,20]),
    (17, ErrorCorrectionLevel::Q, vec![2,367,28,1,22,15,23]),
    (18, ErrorCorrectionLevel::Q, vec![2,397,28,17,22,1,23]),
    (19, ErrorCorrectionLevel::Q, vec![2,445,26,17,21,4,22]),
    (20, ErrorCorrectionLevel::Q, vec![2,485,30,15,24,5,25]),
    (21, ErrorCorrectionLevel::Q, vec![2,512,28,17,22,6,23]),
    (22, ErrorCorrectionLevel::Q, vec![2,568,30,7,24,16,25]),
    (23, ErrorCorrectionLevel::Q, vec![2,614,30,11,24,14,25]),
    (24, ErrorCorrectionLevel::Q, vec![2,664,30,11,24,16,25]),
    (25, ErrorCorrectionLevel::Q, vec![2,718,30,7,24,22,25]),
    (26, ErrorCorrectionLevel::Q, vec![2,754,28,28,22,6,23]),
    (27, ErrorCorrectionLevel::Q, vec![2,808,30,8,23,26,24]),
    (28, ErrorCorrectionLevel::Q, vec![2,871,30,4,24,31,25]),
    (29, ErrorCorrectionLevel::Q, vec![2,911,30,1,23,37,24]),
    (30, ErrorCorrectionLevel::Q, vec![2,985,30,15,24,25,25]),
    (31, ErrorCorrectionLevel::Q, vec![2,1033,30,42,24,1,25]),
    (32, ErrorCorrectionLevel::Q, vec![2,1115,30,10,24,35,25]),
    (33, ErrorCorrectionLevel::Q, vec![2,1171,30,29,24,19,25]),
    (34, ErrorCorrectionLevel::Q, vec![2,1231,30,44,24,7,25]),
    (35, ErrorCorrectionLevel::Q, vec![2,1286,30,39,24,14,25]),
    (36, ErrorCorrectionLevel::Q, vec![2,1354,30,46,24,10,25]),
    (37, ErrorCorrectionLevel::Q, vec![2,1426,30,49,24,10,25]),
    (38, ErrorCorrectionLevel::Q, vec![2,1502,30,48,24,14,25]),
    (39, ErrorCorrectionLevel::Q, vec![2,1582,30,43,24,22,25]),
    (40, ErrorCorrectionLevel::Q, vec![2,1666,30,34,24,34,25]),
    (1,  ErrorCorrectionLevel::H, vec![1,9,17,1,9]),
    (2,  ErrorCorrectionLevel::H, vec![1,16,28,1,16]),
    (3,  ErrorCorrectionLevel::H, vec![1,26,22,2,13]),
    (4,  ErrorCorrectionLevel::H, vec![1,36,16,4,9]),
    (5,  ErrorCorrectionLevel::H, vec![2,46,22,2,11,2,12]),
    (6,  ErrorCorrectionLevel::H, vec![1,60,28,4,15]),
    (7,  ErrorCorrectionLevel::H, vec![2,66,26,4,13,1,14]),
    (8,  ErrorCorrectionLevel::H, vec![2,86,26,4,14,2,15]),
    (9,  ErrorCorrectionLevel::H, vec![2,100,24,4,12,4,13]),
    (10, ErrorCorrectionLevel::H, vec![2,122,28,6,15,2,16]),
    (11, ErrorCorrectionLevel::H, vec![2,140,24,3,12,8,13]),
    (12, ErrorCorrectionLevel::H, vec![2,158,28,7,14,4,15]),
    (13, ErrorCorrectionLevel::H, vec![2,180,22,12,11,4,12]),
    (14, ErrorCorrectionLevel::H, vec![2,197,24,11,12,5,13]),
    (15, ErrorCorrectionLevel::H, vec![2,223,24,11,12,7,13]),
    (16, ErrorCorrectionLevel::H, vec![2,253,30,3,15,13,16]),
    (17, ErrorCorrectionLevel::H, vec![2,283,28,2,14,17,15]),
    (18, ErrorCorrectionLevel::H, vec![2,313,28,2,14,19,15]),
    (19, ErrorCorrectionLevel::H, vec![2,341,26,9,13,16,14]),
    (20, ErrorCorrectionLevel::H, vec![2,385,28,15,15,10,16]),
    (21, ErrorCorrectionLevel::H, vec![2,406,30,19,16,6,17]),
    (22, ErrorCorrectionLevel::H, vec![1,442,24,34,13]),
    (23, ErrorCorrectionLevel::H, vec![2,464,30,16,15,14,16]),
    (24, ErrorCorrectionLevel::H, vec![2,514,30,30,16,2,17]),
    (25, ErrorCorrectionLevel::H, vec![2,538,30,22,15,13,16]),
    (26, ErrorCorrectionLevel::H, vec![2,596,30,33,16,4,17]),
    (27, ErrorCorrectionLevel::H, vec![2,628,30,12,15,28,16]),
    (28, ErrorCorrectionLevel::H, vec![2,661,30,11,15,31,16]),
    (29, ErrorCorrectionLevel::H, vec![2,701,30,19,15,26,16]),
    (30, ErrorCorrectionLevel::H, vec![2,745,30,23,15,25,16]),
    (31, ErrorCorrectionLevel::H, vec![2,793,30,23,15,28,16]),
    (32, ErrorCorrectionLevel::H, vec![2,845,30,19,15,35,16]),
    (33, ErrorCorrectionLevel::H, vec![2,901,30,11,15,46,16]),
    (34, ErrorCorrectionLevel::H, vec![2,961,30,59,16,1,17]),
    (35, ErrorCorrectionLevel::H, vec![2,986,30,22,15,41,16]),
    (36, ErrorCorrectionLevel::H, vec![2,1054,30,2,15,64,16]),
    (37, ErrorCorrectionLevel::H, vec![2,1096,30,24,15,46,16]),
    (38, ErrorCorrectionLevel::H, vec![2,1142,30,42,15,32,16]),
    (39, ErrorCorrectionLevel::H, vec![2,1222,30,10,15,67,16]),
    (40, ErrorCorrectionLevel::H, vec![2,1276,30,20,15,61,16]),
    ];

    let current_option = (version, error_correction);
    for option in data {
        if option.0 == current_option.0 && &option.1 == current_option.1 {
            return Some(option.2);
        }
    }

    None
}

fn encode_numeric(bit_buffer: & mut String, current_slice: &str) {
    let parsed_int: i32 = current_slice.parse().unwrap();

    if parsed_int > 100 {
        bit_buffer.push_str(&format!("{:010b}", parsed_int));
    } else if parsed_int < 100 && parsed_int >= 10 {
        bit_buffer.push_str(&format!("{:07b}", parsed_int));
    } else {
        bit_buffer.push_str(&format!("{:04b}", parsed_int));
    };
}

fn alphanumeric_get_char_code(c: char) -> i32 {
    match c {
    '0' => { return 0; }
    '1' => { return 1; }
    '2' => { return 2; }
    '3' => { return 3; }
    '4' => { return 4; }
    '5' => { return 5; }
    '6' => { return 6; }
    '7' => { return 7; }
    '8' => { return 8; }
    '9' => { return 9; }
    'A' => { return 10; }
    'B' => { return 11; }
    'C' => { return 12; }
    'D' => { return 13; }
    'E' => { return 14; }
    'F' => { return 15; }
    'G' => { return 16; }
    'H' => { return 17; }
    'I' => { return 18; }
    'J' => { return 19; }
    'K' => { return 20; }
    'L' => { return 21; }
    'M' => { return 22; }
    'N' => { return 23; }
    'O' => { return 24; }
    'P' => { return 25; }
    'Q' => { return 26; }
    'R' => { return 27; }
    'S' => { return 28; }
    'T' => { return 29; }
    'U' => { return 30; }
    'V' => { return 31; }
    'W' => { return 32; }
    'X' => { return 33; }
    'Y' => { return 34; }
    'Z' => { return 35; }
    ' ' => { return 36; }
    '$' => { return 37; }
    '%' => { return 38; }
    '*' => { return 39; }
    '+' => { return 40; }
    '-' => { return 41; }
    '.' => { return 42; }
    '/' => { return 43; }
    ':' => { return 44; }
    _   => { return 99; }
    }
}

fn encode_alphanumeric(bit_buffer: & mut String, current_slice: &str) {
    if current_slice.len() == 2 {
        let code1 = alphanumeric_get_char_code(current_slice.chars().nth(0).unwrap());
        let code2 = alphanumeric_get_char_code(current_slice.chars().nth(1).unwrap());

        let code = code1 * 17 + code2;
        bit_buffer.push_str(&format!("{:011b}", code));
    } else {
        bit_buffer.push_str(&format!("{:010b}", alphanumeric_get_char_code(current_slice.chars().nth(0).unwrap())));
    }
}

impl QRCode {
    pub fn new(data: String, error_correction: ErrorCorrectionLevel) -> Result<Self, String> {
        let encoding = find_encoding(&data);
        let version = get_size(data.len(), &encoding, &error_correction);

        if version == 0 {
            return Err(String::from("Data too long for encoding!"))
        }

        Ok(QRCode {
            raw_data: data,
            encoding: encoding,
            error_correction: error_correction,
            version: version
        })
    }

    pub fn encode(&self) -> Result<String, String> {
        let mut bit_buffer = String::new();

        // add magic number for encoding type
        match self.encoding {
            QREncoding::Numeric      => bit_buffer.push_str("0001"),
            QREncoding::AlphaNumeric => bit_buffer.push_str("0010"),
            QREncoding::Byte         => bit_buffer.push_str("0100"),
            _                        => bit_buffer.push_str("0000"),
        };

        // add length of data 0 padded to specific lenght
        match self.version {
            1..=9 => match self.encoding {
                QREncoding::Numeric      => bit_buffer.push_str(&format!("{:010b}", self.raw_data.len())),
                QREncoding::AlphaNumeric => bit_buffer.push_str(&format!("{:09b}", self.raw_data.len())),
                QREncoding::Byte         => bit_buffer.push_str(&format!("{:08b}", self.raw_data.len())),
                _                        => bit_buffer.push_str("0000"),
            },
            10..=26 => match self.encoding {
                QREncoding::Numeric      => bit_buffer.push_str(&format!("{:012b}", self.raw_data.len())),
                QREncoding::AlphaNumeric => bit_buffer.push_str(&format!("{:011b}", self.raw_data.len())),
                QREncoding::Byte         => bit_buffer.push_str(&format!("{:016b}", self.raw_data.len())),
                _                        => bit_buffer.push_str("0000"),
            },
            27..=40 => match self.encoding {
                QREncoding::Numeric      => bit_buffer.push_str(&format!("{:014b}", self.raw_data.len())),
                QREncoding::AlphaNumeric => bit_buffer.push_str(&format!("{:013b}", self.raw_data.len())),
                QREncoding::Byte         => bit_buffer.push_str(&format!("{:016b}", self.raw_data.len())),
                _                        => bit_buffer.push_str("0000"),
            },
            _ => { return Err(format!("Unsupportet version {}!", self.version)) }
        };

        // encode the data
        match self.encoding {
            QREncoding::Numeric => {
                let scaled_len = self.raw_data.len() / 3;
                let left_over  = self.raw_data.len() % 3;

                for i in 0..scaled_len {
                    let current_slice = &self.raw_data[i..i+3];
                    encode_numeric(& mut bit_buffer, current_slice);
                }

                if left_over > 0 {
                    encode_numeric(& mut bit_buffer, &self.raw_data[self.raw_data.len() - left_over..self.raw_data.len()]);
                }
            },
            QREncoding::AlphaNumeric => {
                let scaled_len = self.raw_data.len() / 2;
                let left_over  = self.raw_data.len() % 2;

                for i in 0..scaled_len {
                    let current_slice = &self.raw_data[i..i+2];
                    encode_alphanumeric(& mut bit_buffer, current_slice);
                }

                if left_over > 0 {
                    encode_alphanumeric(& mut bit_buffer, &self.raw_data[self.raw_data.len() - left_over..self.raw_data.len()])
                }
            },
            QREncoding::Byte => {
                let bytes = self.raw_data.bytes();
                bytes.for_each(|byte| {
                    bit_buffer.push_str(&format!("{:08b}", byte));
                });
            }
            _ => { }
        }

        // get the maximum number of bits
        let max_size = match get_data_size(self.version, &self.error_correction) {
            Some(data) => { data[1] * 8 },
            None => { return Err(format!("Couldn't find data size for given version {} and error correction !", self.version)); }
        } as usize;
        let missing_bits = max_size - bit_buffer.len();

        // add terminator of 0s => at most four 0s
        let terminator_len = if missing_bits <= 4 { missing_bits } else { 4 };
        for _ in 0..terminator_len { bit_buffer.push('0'); }

        // pad to multiple of 8
        let diff_to_eight = bit_buffer.len() % 8;
        for _ in 0..diff_to_eight { bit_buffer.push('0'); }

        // add padding to reach maximum data lenght
        let missing_bytes = (max_size - bit_buffer.len()) / 8;
        let padding_bytes = vec!["11101100", "00010001"];
        for i in 0..missing_bytes {
            bit_buffer.push_str(padding_bytes[i % 2]);
        }

        return Ok(bit_buffer);
    }
}
