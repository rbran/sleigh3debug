pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    PSW,
    PC,
    SP,
    ZRlo,
    ZRhi,
    AD_resultlo,
    AD_resulthi,
    HSI_timelo,
    HSI_timehi,
    HSI_status,
    SBUF,
    INT_MASK,
    INT_PEND,
    TIMER1lo,
    TIMER1hi,
    TIMER2lo,
    TIMER2hi,
    PORT0,
    PORT1,
    PORT2,
    SP_STAT,
    INT_PEND1,
    INT_MASK1,
    WSR,
    IOS0,
    IOS1,
    IOS2,
    ZR,
    AD_result,
    HSI_time,
    HSI_SBUF,
    INTERRUPT,
    TIMER1,
    TIMER2,
    PORT01,
    PORT2_SPS,
    INT1,
    WSR_IOS0,
    IOS12,
    ZR_AD,
    HSI,
    INT_TIMER1,
    TIMER2_PORT01,
    PORT2_INT1,
    WSR_IOS012,
    SPlo,
    SPhi,
    SPR,
    SPR1A,
    R1A,
    R1B,
    R1C,
    R1D,
    R1E,
    R1F,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R2A,
    R2B,
    R2C,
    R2D,
    R2E,
    R2F,
    R30,
    R31,
    R32,
    R33,
    R34,
    R35,
    R36,
    R37,
    R38,
    R39,
    R3A,
    R3B,
    R3C,
    R3D,
    R3E,
    R3F,
    R40,
    R41,
    R42,
    R43,
    R44,
    R45,
    R46,
    R47,
    R48,
    R49,
    R4A,
    R4B,
    R4C,
    R4D,
    R4E,
    R4F,
    R50,
    R51,
    R52,
    R53,
    R54,
    R55,
    R56,
    R57,
    R58,
    R59,
    R5A,
    R5B,
    R5C,
    R5D,
    R5E,
    R5F,
    R60,
    R61,
    R62,
    R63,
    R64,
    R65,
    R66,
    R67,
    R68,
    R69,
    R6A,
    R6B,
    R6C,
    R6D,
    R6E,
    R6F,
    R70,
    R71,
    R72,
    R73,
    R74,
    R75,
    R76,
    R77,
    R78,
    R79,
    R7A,
    R7B,
    R7C,
    R7D,
    R7E,
    R7F,
    R80,
    R81,
    R82,
    R83,
    R84,
    R85,
    R86,
    R87,
    R88,
    R89,
    R8A,
    R8B,
    R8C,
    R8D,
    R8E,
    R8F,
    R90,
    R91,
    R92,
    R93,
    R94,
    R95,
    R96,
    R97,
    R98,
    R99,
    R9A,
    R9B,
    R9C,
    R9D,
    R9E,
    R9F,
    RA0,
    RA1,
    RA2,
    RA3,
    RA4,
    RA5,
    RA6,
    RA7,
    RA8,
    RA9,
    RAA,
    RAB,
    RAC,
    RAD,
    RAE,
    RAF,
    RB0,
    RB1,
    RB2,
    RB3,
    RB4,
    RB5,
    RB6,
    RB7,
    RB8,
    RB9,
    RBA,
    RBB,
    RBC,
    RBD,
    RBE,
    RBF,
    RC0,
    RC1,
    RC2,
    RC3,
    RC4,
    RC5,
    RC6,
    RC7,
    RC8,
    RC9,
    RCA,
    RCB,
    RCC,
    RCD,
    RCE,
    RCF,
    RD0,
    RD1,
    RD2,
    RD3,
    RD4,
    RD5,
    RD6,
    RD7,
    RD8,
    RD9,
    RDA,
    RDB,
    RDC,
    RDD,
    RDE,
    RDF,
    RE0,
    RE1,
    RE2,
    RE3,
    RE4,
    RE5,
    RE6,
    RE7,
    RE8,
    RE9,
    REA,
    REB,
    REC,
    RED,
    REE,
    REF,
    RF0,
    RF1,
    RF2,
    RF3,
    RF4,
    RF5,
    RF6,
    RF7,
    RF8,
    RF9,
    RFA,
    RFB,
    RFC,
    RFD,
    RFE,
    RFF,
    R100,
    R101,
    R102,
    R103,
    R104,
    R105,
    R106,
    R107,
    R108,
    R109,
    R10A,
    R10B,
    R10C,
    R10D,
    R10E,
    R10F,
    R110,
    R111,
    R112,
    R113,
    R114,
    R115,
    R116,
    R117,
    R118,
    R119,
    R11A,
    R11B,
    R11C,
    R11D,
    R11E,
    R11F,
    R120,
    R121,
    R122,
    R123,
    R124,
    R125,
    R126,
    R127,
    R128,
    R129,
    R12A,
    R12B,
    R12C,
    R12D,
    R12E,
    R12F,
    R130,
    R131,
    R132,
    R133,
    R134,
    R135,
    R136,
    R137,
    R138,
    R139,
    R13A,
    R13B,
    R13C,
    R13D,
    R13E,
    R13F,
    R140,
    R141,
    R142,
    R143,
    R144,
    R145,
    R146,
    R147,
    R148,
    R149,
    R14A,
    R14B,
    R14C,
    R14D,
    R14E,
    R14F,
    R150,
    R151,
    R152,
    R153,
    R154,
    R155,
    R156,
    R157,
    R158,
    R159,
    R15A,
    R15B,
    R15C,
    R15D,
    R15E,
    R15F,
    R160,
    R161,
    R162,
    R163,
    R164,
    R165,
    R166,
    R167,
    R168,
    R169,
    R16A,
    R16B,
    R16C,
    R16D,
    R16E,
    R16F,
    R170,
    R171,
    R172,
    R173,
    R174,
    R175,
    R176,
    R177,
    R178,
    R179,
    R17A,
    R17B,
    R17C,
    R17D,
    R17E,
    R17F,
    R180,
    R181,
    R182,
    R183,
    R184,
    R185,
    R186,
    R187,
    R188,
    R189,
    R18A,
    R18B,
    R18C,
    R18D,
    R18E,
    R18F,
    R190,
    R191,
    R192,
    R193,
    R194,
    R195,
    R196,
    R197,
    R198,
    R199,
    R19A,
    R19B,
    R19C,
    R19D,
    R19E,
    R19F,
    R1A0,
    R1A1,
    R1A2,
    R1A3,
    R1A4,
    R1A5,
    R1A6,
    R1A7,
    R1A8,
    R1A9,
    R1AA,
    R1AB,
    R1AC,
    R1AD,
    R1AE,
    R1AF,
    R1B0,
    R1B1,
    R1B2,
    R1B3,
    R1B4,
    R1B5,
    R1B6,
    R1B7,
    R1B8,
    R1B9,
    R1BA,
    R1BB,
    R1BC,
    R1BD,
    R1BE,
    R1BF,
    R1C0,
    R1C1,
    R1C2,
    R1C3,
    R1C4,
    R1C5,
    R1C6,
    R1C7,
    R1C8,
    R1C9,
    R1CA,
    R1CB,
    R1CC,
    R1CD,
    R1CE,
    R1CF,
    R1D0,
    R1D1,
    R1D2,
    R1D3,
    R1D4,
    R1D5,
    R1D6,
    R1D7,
    R1D8,
    R1D9,
    R1DA,
    R1DB,
    R1DC,
    R1DD,
    R1DE,
    R1DF,
    R1E0,
    R1E1,
    R1E2,
    R1E3,
    R1E4,
    R1E5,
    R1E6,
    R1E7,
    R1E8,
    R1E9,
    R1EA,
    R1EB,
    R1EC,
    R1ED,
    R1EE,
    R1EF,
    R1F0,
    R1F1,
    R1F2,
    R1F3,
    R1F4,
    R1F5,
    R1F6,
    R1F7,
    R1F8,
    R1F9,
    R1FA,
    R1FB,
    R1FC,
    R1FD,
    R1FE,
    R1FF,
    RW1A,
    RW1C,
    RW1E,
    RW20,
    RW22,
    RW24,
    RW26,
    RW28,
    RW2A,
    RW2C,
    RW2E,
    RW30,
    RW32,
    RW34,
    RW36,
    RW38,
    RW3A,
    RW3C,
    RW3E,
    RW40,
    RW42,
    RW44,
    RW46,
    RW48,
    RW4A,
    RW4C,
    RW4E,
    RW50,
    RW52,
    RW54,
    RW56,
    RW58,
    RW5A,
    RW5C,
    RW5E,
    RW60,
    RW62,
    RW64,
    RW66,
    RW68,
    RW6A,
    RW6C,
    RW6E,
    RW70,
    RW72,
    RW74,
    RW76,
    RW78,
    RW7A,
    RW7C,
    RW7E,
    RW80,
    RW82,
    RW84,
    RW86,
    RW88,
    RW8A,
    RW8C,
    RW8E,
    RW90,
    RW92,
    RW94,
    RW96,
    RW98,
    RW9A,
    RW9C,
    RW9E,
    RWA0,
    RWA2,
    RWA4,
    RWA6,
    RWA8,
    RWAA,
    RWAC,
    RWAE,
    RWB0,
    RWB2,
    RWB4,
    RWB6,
    RWB8,
    RWBA,
    RWBC,
    RWBE,
    RWC0,
    RWC2,
    RWC4,
    RWC6,
    RWC8,
    RWCA,
    RWCC,
    RWCE,
    RWD0,
    RWD2,
    RWD4,
    RWD6,
    RWD8,
    RWDA,
    RWDC,
    RWDE,
    RWE0,
    RWE2,
    RWE4,
    RWE6,
    RWE8,
    RWEA,
    RWEC,
    RWEE,
    RWF0,
    RWF2,
    RWF4,
    RWF6,
    RWF8,
    RWFA,
    RWFC,
    RWFE,
    RW100,
    RW102,
    RW104,
    RW106,
    RW108,
    RW10A,
    RW10C,
    RW10E,
    RW110,
    RW112,
    RW114,
    RW116,
    RW118,
    RW11A,
    RW11C,
    RW11E,
    RW120,
    RW122,
    RW124,
    RW126,
    RW128,
    RW12A,
    RW12C,
    RW12E,
    RW130,
    RW132,
    RW134,
    RW136,
    RW138,
    RW13A,
    RW13C,
    RW13E,
    RW140,
    RW142,
    RW144,
    RW146,
    RW148,
    RW14A,
    RW14C,
    RW14E,
    RW150,
    RW152,
    RW154,
    RW156,
    RW158,
    RW15A,
    RW15C,
    RW15E,
    RW160,
    RW162,
    RW164,
    RW166,
    RW168,
    RW16A,
    RW16C,
    RW16E,
    RW170,
    RW172,
    RW174,
    RW176,
    RW178,
    RW17A,
    RW17C,
    RW17E,
    RW180,
    RW182,
    RW184,
    RW186,
    RW188,
    RW18A,
    RW18C,
    RW18E,
    RW190,
    RW192,
    RW194,
    RW196,
    RW198,
    RW19A,
    RW19C,
    RW19E,
    RW1A0,
    RW1A2,
    RW1A4,
    RW1A6,
    RW1A8,
    RW1AA,
    RW1AC,
    RW1AE,
    RW1B0,
    RW1B2,
    RW1B4,
    RW1B6,
    RW1B8,
    RW1BA,
    RW1BC,
    RW1BE,
    RW1C0,
    RW1C2,
    RW1C4,
    RW1C6,
    RW1C8,
    RW1CA,
    RW1CC,
    RW1CE,
    RW1D0,
    RW1D2,
    RW1D4,
    RW1D6,
    RW1D8,
    RW1DA,
    RW1DC,
    RW1DE,
    RW1E0,
    RW1E2,
    RW1E4,
    RW1E6,
    RW1E8,
    RW1EA,
    RW1EC,
    RW1EE,
    RW1F0,
    RW1F2,
    RW1F4,
    RW1F6,
    RW1F8,
    RW1FA,
    RW1FC,
    RW1FE,
    RL1C,
    RL20,
    RL24,
    RL28,
    RL2C,
    RL30,
    RL34,
    RL38,
    RL3C,
    RL40,
    RL44,
    RL48,
    RL4C,
    RL50,
    RL54,
    RL58,
    RL5C,
    RL60,
    RL64,
    RL68,
    RL6C,
    RL70,
    RL74,
    RL78,
    RL7C,
    RL80,
    RL84,
    RL88,
    RL8C,
    RL90,
    RL94,
    RL98,
    RL9C,
    RLA0,
    RLA4,
    RLA8,
    RLAC,
    RLB0,
    RLB4,
    RLB8,
    RLBC,
    RLC0,
    RLC4,
    RLC8,
    RLCC,
    RLD0,
    RLD4,
    RLD8,
    RLDC,
    RLE0,
    RLE4,
    RLE8,
    RLEC,
    RLF0,
    RLF4,
    RLF8,
    RLFC,
    RL100,
    RL104,
    RL108,
    RL10C,
    RL110,
    RL114,
    RL118,
    RL11C,
    RL120,
    RL124,
    RL128,
    RL12C,
    RL130,
    RL134,
    RL138,
    RL13C,
    RL140,
    RL144,
    RL148,
    RL14C,
    RL150,
    RL154,
    RL158,
    RL15C,
    RL160,
    RL164,
    RL168,
    RL16C,
    RL170,
    RL174,
    RL178,
    RL17C,
    RL180,
    RL184,
    RL188,
    RL18C,
    RL190,
    RL194,
    RL198,
    RL19C,
    RL1A0,
    RL1A4,
    RL1A8,
    RL1AC,
    RL1B0,
    RL1B4,
    RL1B8,
    RL1BC,
    RL1C0,
    RL1C4,
    RL1C8,
    RL1CC,
    RL1D0,
    RL1D4,
    RL1D8,
    RL1DC,
    RL1E0,
    RL1E4,
    RL1E8,
    RL1EC,
    RL1F0,
    RL1F4,
    RL1F8,
    RL1FC,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::PSW => "PSW",
            Self::PC => "PC",
            Self::SP => "SP",
            Self::ZRlo => "ZRlo",
            Self::ZRhi => "ZRhi",
            Self::AD_resultlo => "AD_resultlo",
            Self::AD_resulthi => "AD_resulthi",
            Self::HSI_timelo => "HSI_timelo",
            Self::HSI_timehi => "HSI_timehi",
            Self::HSI_status => "HSI_status",
            Self::SBUF => "SBUF",
            Self::INT_MASK => "INT_MASK",
            Self::INT_PEND => "INT_PEND",
            Self::TIMER1lo => "TIMER1lo",
            Self::TIMER1hi => "TIMER1hi",
            Self::TIMER2lo => "TIMER2lo",
            Self::TIMER2hi => "TIMER2hi",
            Self::PORT0 => "PORT0",
            Self::PORT1 => "PORT1",
            Self::PORT2 => "PORT2",
            Self::SP_STAT => "SP_STAT",
            Self::INT_PEND1 => "INT_PEND1",
            Self::INT_MASK1 => "INT_MASK1",
            Self::WSR => "WSR",
            Self::IOS0 => "IOS0",
            Self::IOS1 => "IOS1",
            Self::IOS2 => "IOS2",
            Self::ZR => "ZR",
            Self::AD_result => "AD_result",
            Self::HSI_time => "HSI_time",
            Self::HSI_SBUF => "HSI_SBUF",
            Self::INTERRUPT => "INTERRUPT",
            Self::TIMER1 => "TIMER1",
            Self::TIMER2 => "TIMER2",
            Self::PORT01 => "PORT01",
            Self::PORT2_SPS => "PORT2_SPS",
            Self::INT1 => "INT1",
            Self::WSR_IOS0 => "WSR_IOS0",
            Self::IOS12 => "IOS12",
            Self::ZR_AD => "ZR_AD",
            Self::HSI => "HSI",
            Self::INT_TIMER1 => "INT_TIMER1",
            Self::TIMER2_PORT01 => "TIMER2_PORT01",
            Self::PORT2_INT1 => "PORT2_INT1",
            Self::WSR_IOS012 => "WSR_IOS012",
            Self::SPlo => "SPlo",
            Self::SPhi => "SPhi",
            Self::SPR => "SPR",
            Self::SPR1A => "SPR1A",
            Self::R1A => "R1A",
            Self::R1B => "R1B",
            Self::R1C => "R1C",
            Self::R1D => "R1D",
            Self::R1E => "R1E",
            Self::R1F => "R1F",
            Self::R20 => "R20",
            Self::R21 => "R21",
            Self::R22 => "R22",
            Self::R23 => "R23",
            Self::R24 => "R24",
            Self::R25 => "R25",
            Self::R26 => "R26",
            Self::R27 => "R27",
            Self::R28 => "R28",
            Self::R29 => "R29",
            Self::R2A => "R2A",
            Self::R2B => "R2B",
            Self::R2C => "R2C",
            Self::R2D => "R2D",
            Self::R2E => "R2E",
            Self::R2F => "R2F",
            Self::R30 => "R30",
            Self::R31 => "R31",
            Self::R32 => "R32",
            Self::R33 => "R33",
            Self::R34 => "R34",
            Self::R35 => "R35",
            Self::R36 => "R36",
            Self::R37 => "R37",
            Self::R38 => "R38",
            Self::R39 => "R39",
            Self::R3A => "R3A",
            Self::R3B => "R3B",
            Self::R3C => "R3C",
            Self::R3D => "R3D",
            Self::R3E => "R3E",
            Self::R3F => "R3F",
            Self::R40 => "R40",
            Self::R41 => "R41",
            Self::R42 => "R42",
            Self::R43 => "R43",
            Self::R44 => "R44",
            Self::R45 => "R45",
            Self::R46 => "R46",
            Self::R47 => "R47",
            Self::R48 => "R48",
            Self::R49 => "R49",
            Self::R4A => "R4A",
            Self::R4B => "R4B",
            Self::R4C => "R4C",
            Self::R4D => "R4D",
            Self::R4E => "R4E",
            Self::R4F => "R4F",
            Self::R50 => "R50",
            Self::R51 => "R51",
            Self::R52 => "R52",
            Self::R53 => "R53",
            Self::R54 => "R54",
            Self::R55 => "R55",
            Self::R56 => "R56",
            Self::R57 => "R57",
            Self::R58 => "R58",
            Self::R59 => "R59",
            Self::R5A => "R5A",
            Self::R5B => "R5B",
            Self::R5C => "R5C",
            Self::R5D => "R5D",
            Self::R5E => "R5E",
            Self::R5F => "R5F",
            Self::R60 => "R60",
            Self::R61 => "R61",
            Self::R62 => "R62",
            Self::R63 => "R63",
            Self::R64 => "R64",
            Self::R65 => "R65",
            Self::R66 => "R66",
            Self::R67 => "R67",
            Self::R68 => "R68",
            Self::R69 => "R69",
            Self::R6A => "R6A",
            Self::R6B => "R6B",
            Self::R6C => "R6C",
            Self::R6D => "R6D",
            Self::R6E => "R6E",
            Self::R6F => "R6F",
            Self::R70 => "R70",
            Self::R71 => "R71",
            Self::R72 => "R72",
            Self::R73 => "R73",
            Self::R74 => "R74",
            Self::R75 => "R75",
            Self::R76 => "R76",
            Self::R77 => "R77",
            Self::R78 => "R78",
            Self::R79 => "R79",
            Self::R7A => "R7A",
            Self::R7B => "R7B",
            Self::R7C => "R7C",
            Self::R7D => "R7D",
            Self::R7E => "R7E",
            Self::R7F => "R7F",
            Self::R80 => "R80",
            Self::R81 => "R81",
            Self::R82 => "R82",
            Self::R83 => "R83",
            Self::R84 => "R84",
            Self::R85 => "R85",
            Self::R86 => "R86",
            Self::R87 => "R87",
            Self::R88 => "R88",
            Self::R89 => "R89",
            Self::R8A => "R8A",
            Self::R8B => "R8B",
            Self::R8C => "R8C",
            Self::R8D => "R8D",
            Self::R8E => "R8E",
            Self::R8F => "R8F",
            Self::R90 => "R90",
            Self::R91 => "R91",
            Self::R92 => "R92",
            Self::R93 => "R93",
            Self::R94 => "R94",
            Self::R95 => "R95",
            Self::R96 => "R96",
            Self::R97 => "R97",
            Self::R98 => "R98",
            Self::R99 => "R99",
            Self::R9A => "R9A",
            Self::R9B => "R9B",
            Self::R9C => "R9C",
            Self::R9D => "R9D",
            Self::R9E => "R9E",
            Self::R9F => "R9F",
            Self::RA0 => "RA0",
            Self::RA1 => "RA1",
            Self::RA2 => "RA2",
            Self::RA3 => "RA3",
            Self::RA4 => "RA4",
            Self::RA5 => "RA5",
            Self::RA6 => "RA6",
            Self::RA7 => "RA7",
            Self::RA8 => "RA8",
            Self::RA9 => "RA9",
            Self::RAA => "RAA",
            Self::RAB => "RAB",
            Self::RAC => "RAC",
            Self::RAD => "RAD",
            Self::RAE => "RAE",
            Self::RAF => "RAF",
            Self::RB0 => "RB0",
            Self::RB1 => "RB1",
            Self::RB2 => "RB2",
            Self::RB3 => "RB3",
            Self::RB4 => "RB4",
            Self::RB5 => "RB5",
            Self::RB6 => "RB6",
            Self::RB7 => "RB7",
            Self::RB8 => "RB8",
            Self::RB9 => "RB9",
            Self::RBA => "RBA",
            Self::RBB => "RBB",
            Self::RBC => "RBC",
            Self::RBD => "RBD",
            Self::RBE => "RBE",
            Self::RBF => "RBF",
            Self::RC0 => "RC0",
            Self::RC1 => "RC1",
            Self::RC2 => "RC2",
            Self::RC3 => "RC3",
            Self::RC4 => "RC4",
            Self::RC5 => "RC5",
            Self::RC6 => "RC6",
            Self::RC7 => "RC7",
            Self::RC8 => "RC8",
            Self::RC9 => "RC9",
            Self::RCA => "RCA",
            Self::RCB => "RCB",
            Self::RCC => "RCC",
            Self::RCD => "RCD",
            Self::RCE => "RCE",
            Self::RCF => "RCF",
            Self::RD0 => "RD0",
            Self::RD1 => "RD1",
            Self::RD2 => "RD2",
            Self::RD3 => "RD3",
            Self::RD4 => "RD4",
            Self::RD5 => "RD5",
            Self::RD6 => "RD6",
            Self::RD7 => "RD7",
            Self::RD8 => "RD8",
            Self::RD9 => "RD9",
            Self::RDA => "RDA",
            Self::RDB => "RDB",
            Self::RDC => "RDC",
            Self::RDD => "RDD",
            Self::RDE => "RDE",
            Self::RDF => "RDF",
            Self::RE0 => "RE0",
            Self::RE1 => "RE1",
            Self::RE2 => "RE2",
            Self::RE3 => "RE3",
            Self::RE4 => "RE4",
            Self::RE5 => "RE5",
            Self::RE6 => "RE6",
            Self::RE7 => "RE7",
            Self::RE8 => "RE8",
            Self::RE9 => "RE9",
            Self::REA => "REA",
            Self::REB => "REB",
            Self::REC => "REC",
            Self::RED => "RED",
            Self::REE => "REE",
            Self::REF => "REF",
            Self::RF0 => "RF0",
            Self::RF1 => "RF1",
            Self::RF2 => "RF2",
            Self::RF3 => "RF3",
            Self::RF4 => "RF4",
            Self::RF5 => "RF5",
            Self::RF6 => "RF6",
            Self::RF7 => "RF7",
            Self::RF8 => "RF8",
            Self::RF9 => "RF9",
            Self::RFA => "RFA",
            Self::RFB => "RFB",
            Self::RFC => "RFC",
            Self::RFD => "RFD",
            Self::RFE => "RFE",
            Self::RFF => "RFF",
            Self::R100 => "R100",
            Self::R101 => "R101",
            Self::R102 => "R102",
            Self::R103 => "R103",
            Self::R104 => "R104",
            Self::R105 => "R105",
            Self::R106 => "R106",
            Self::R107 => "R107",
            Self::R108 => "R108",
            Self::R109 => "R109",
            Self::R10A => "R10A",
            Self::R10B => "R10B",
            Self::R10C => "R10C",
            Self::R10D => "R10D",
            Self::R10E => "R10E",
            Self::R10F => "R10F",
            Self::R110 => "R110",
            Self::R111 => "R111",
            Self::R112 => "R112",
            Self::R113 => "R113",
            Self::R114 => "R114",
            Self::R115 => "R115",
            Self::R116 => "R116",
            Self::R117 => "R117",
            Self::R118 => "R118",
            Self::R119 => "R119",
            Self::R11A => "R11A",
            Self::R11B => "R11B",
            Self::R11C => "R11C",
            Self::R11D => "R11D",
            Self::R11E => "R11E",
            Self::R11F => "R11F",
            Self::R120 => "R120",
            Self::R121 => "R121",
            Self::R122 => "R122",
            Self::R123 => "R123",
            Self::R124 => "R124",
            Self::R125 => "R125",
            Self::R126 => "R126",
            Self::R127 => "R127",
            Self::R128 => "R128",
            Self::R129 => "R129",
            Self::R12A => "R12A",
            Self::R12B => "R12B",
            Self::R12C => "R12C",
            Self::R12D => "R12D",
            Self::R12E => "R12E",
            Self::R12F => "R12F",
            Self::R130 => "R130",
            Self::R131 => "R131",
            Self::R132 => "R132",
            Self::R133 => "R133",
            Self::R134 => "R134",
            Self::R135 => "R135",
            Self::R136 => "R136",
            Self::R137 => "R137",
            Self::R138 => "R138",
            Self::R139 => "R139",
            Self::R13A => "R13A",
            Self::R13B => "R13B",
            Self::R13C => "R13C",
            Self::R13D => "R13D",
            Self::R13E => "R13E",
            Self::R13F => "R13F",
            Self::R140 => "R140",
            Self::R141 => "R141",
            Self::R142 => "R142",
            Self::R143 => "R143",
            Self::R144 => "R144",
            Self::R145 => "R145",
            Self::R146 => "R146",
            Self::R147 => "R147",
            Self::R148 => "R148",
            Self::R149 => "R149",
            Self::R14A => "R14A",
            Self::R14B => "R14B",
            Self::R14C => "R14C",
            Self::R14D => "R14D",
            Self::R14E => "R14E",
            Self::R14F => "R14F",
            Self::R150 => "R150",
            Self::R151 => "R151",
            Self::R152 => "R152",
            Self::R153 => "R153",
            Self::R154 => "R154",
            Self::R155 => "R155",
            Self::R156 => "R156",
            Self::R157 => "R157",
            Self::R158 => "R158",
            Self::R159 => "R159",
            Self::R15A => "R15A",
            Self::R15B => "R15B",
            Self::R15C => "R15C",
            Self::R15D => "R15D",
            Self::R15E => "R15E",
            Self::R15F => "R15F",
            Self::R160 => "R160",
            Self::R161 => "R161",
            Self::R162 => "R162",
            Self::R163 => "R163",
            Self::R164 => "R164",
            Self::R165 => "R165",
            Self::R166 => "R166",
            Self::R167 => "R167",
            Self::R168 => "R168",
            Self::R169 => "R169",
            Self::R16A => "R16A",
            Self::R16B => "R16B",
            Self::R16C => "R16C",
            Self::R16D => "R16D",
            Self::R16E => "R16E",
            Self::R16F => "R16F",
            Self::R170 => "R170",
            Self::R171 => "R171",
            Self::R172 => "R172",
            Self::R173 => "R173",
            Self::R174 => "R174",
            Self::R175 => "R175",
            Self::R176 => "R176",
            Self::R177 => "R177",
            Self::R178 => "R178",
            Self::R179 => "R179",
            Self::R17A => "R17A",
            Self::R17B => "R17B",
            Self::R17C => "R17C",
            Self::R17D => "R17D",
            Self::R17E => "R17E",
            Self::R17F => "R17F",
            Self::R180 => "R180",
            Self::R181 => "R181",
            Self::R182 => "R182",
            Self::R183 => "R183",
            Self::R184 => "R184",
            Self::R185 => "R185",
            Self::R186 => "R186",
            Self::R187 => "R187",
            Self::R188 => "R188",
            Self::R189 => "R189",
            Self::R18A => "R18A",
            Self::R18B => "R18B",
            Self::R18C => "R18C",
            Self::R18D => "R18D",
            Self::R18E => "R18E",
            Self::R18F => "R18F",
            Self::R190 => "R190",
            Self::R191 => "R191",
            Self::R192 => "R192",
            Self::R193 => "R193",
            Self::R194 => "R194",
            Self::R195 => "R195",
            Self::R196 => "R196",
            Self::R197 => "R197",
            Self::R198 => "R198",
            Self::R199 => "R199",
            Self::R19A => "R19A",
            Self::R19B => "R19B",
            Self::R19C => "R19C",
            Self::R19D => "R19D",
            Self::R19E => "R19E",
            Self::R19F => "R19F",
            Self::R1A0 => "R1A0",
            Self::R1A1 => "R1A1",
            Self::R1A2 => "R1A2",
            Self::R1A3 => "R1A3",
            Self::R1A4 => "R1A4",
            Self::R1A5 => "R1A5",
            Self::R1A6 => "R1A6",
            Self::R1A7 => "R1A7",
            Self::R1A8 => "R1A8",
            Self::R1A9 => "R1A9",
            Self::R1AA => "R1AA",
            Self::R1AB => "R1AB",
            Self::R1AC => "R1AC",
            Self::R1AD => "R1AD",
            Self::R1AE => "R1AE",
            Self::R1AF => "R1AF",
            Self::R1B0 => "R1B0",
            Self::R1B1 => "R1B1",
            Self::R1B2 => "R1B2",
            Self::R1B3 => "R1B3",
            Self::R1B4 => "R1B4",
            Self::R1B5 => "R1B5",
            Self::R1B6 => "R1B6",
            Self::R1B7 => "R1B7",
            Self::R1B8 => "R1B8",
            Self::R1B9 => "R1B9",
            Self::R1BA => "R1BA",
            Self::R1BB => "R1BB",
            Self::R1BC => "R1BC",
            Self::R1BD => "R1BD",
            Self::R1BE => "R1BE",
            Self::R1BF => "R1BF",
            Self::R1C0 => "R1C0",
            Self::R1C1 => "R1C1",
            Self::R1C2 => "R1C2",
            Self::R1C3 => "R1C3",
            Self::R1C4 => "R1C4",
            Self::R1C5 => "R1C5",
            Self::R1C6 => "R1C6",
            Self::R1C7 => "R1C7",
            Self::R1C8 => "R1C8",
            Self::R1C9 => "R1C9",
            Self::R1CA => "R1CA",
            Self::R1CB => "R1CB",
            Self::R1CC => "R1CC",
            Self::R1CD => "R1CD",
            Self::R1CE => "R1CE",
            Self::R1CF => "R1CF",
            Self::R1D0 => "R1D0",
            Self::R1D1 => "R1D1",
            Self::R1D2 => "R1D2",
            Self::R1D3 => "R1D3",
            Self::R1D4 => "R1D4",
            Self::R1D5 => "R1D5",
            Self::R1D6 => "R1D6",
            Self::R1D7 => "R1D7",
            Self::R1D8 => "R1D8",
            Self::R1D9 => "R1D9",
            Self::R1DA => "R1DA",
            Self::R1DB => "R1DB",
            Self::R1DC => "R1DC",
            Self::R1DD => "R1DD",
            Self::R1DE => "R1DE",
            Self::R1DF => "R1DF",
            Self::R1E0 => "R1E0",
            Self::R1E1 => "R1E1",
            Self::R1E2 => "R1E2",
            Self::R1E3 => "R1E3",
            Self::R1E4 => "R1E4",
            Self::R1E5 => "R1E5",
            Self::R1E6 => "R1E6",
            Self::R1E7 => "R1E7",
            Self::R1E8 => "R1E8",
            Self::R1E9 => "R1E9",
            Self::R1EA => "R1EA",
            Self::R1EB => "R1EB",
            Self::R1EC => "R1EC",
            Self::R1ED => "R1ED",
            Self::R1EE => "R1EE",
            Self::R1EF => "R1EF",
            Self::R1F0 => "R1F0",
            Self::R1F1 => "R1F1",
            Self::R1F2 => "R1F2",
            Self::R1F3 => "R1F3",
            Self::R1F4 => "R1F4",
            Self::R1F5 => "R1F5",
            Self::R1F6 => "R1F6",
            Self::R1F7 => "R1F7",
            Self::R1F8 => "R1F8",
            Self::R1F9 => "R1F9",
            Self::R1FA => "R1FA",
            Self::R1FB => "R1FB",
            Self::R1FC => "R1FC",
            Self::R1FD => "R1FD",
            Self::R1FE => "R1FE",
            Self::R1FF => "R1FF",
            Self::RW1A => "RW1A",
            Self::RW1C => "RW1C",
            Self::RW1E => "RW1E",
            Self::RW20 => "RW20",
            Self::RW22 => "RW22",
            Self::RW24 => "RW24",
            Self::RW26 => "RW26",
            Self::RW28 => "RW28",
            Self::RW2A => "RW2A",
            Self::RW2C => "RW2C",
            Self::RW2E => "RW2E",
            Self::RW30 => "RW30",
            Self::RW32 => "RW32",
            Self::RW34 => "RW34",
            Self::RW36 => "RW36",
            Self::RW38 => "RW38",
            Self::RW3A => "RW3A",
            Self::RW3C => "RW3C",
            Self::RW3E => "RW3E",
            Self::RW40 => "RW40",
            Self::RW42 => "RW42",
            Self::RW44 => "RW44",
            Self::RW46 => "RW46",
            Self::RW48 => "RW48",
            Self::RW4A => "RW4A",
            Self::RW4C => "RW4C",
            Self::RW4E => "RW4E",
            Self::RW50 => "RW50",
            Self::RW52 => "RW52",
            Self::RW54 => "RW54",
            Self::RW56 => "RW56",
            Self::RW58 => "RW58",
            Self::RW5A => "RW5A",
            Self::RW5C => "RW5C",
            Self::RW5E => "RW5E",
            Self::RW60 => "RW60",
            Self::RW62 => "RW62",
            Self::RW64 => "RW64",
            Self::RW66 => "RW66",
            Self::RW68 => "RW68",
            Self::RW6A => "RW6A",
            Self::RW6C => "RW6C",
            Self::RW6E => "RW6E",
            Self::RW70 => "RW70",
            Self::RW72 => "RW72",
            Self::RW74 => "RW74",
            Self::RW76 => "RW76",
            Self::RW78 => "RW78",
            Self::RW7A => "RW7A",
            Self::RW7C => "RW7C",
            Self::RW7E => "RW7E",
            Self::RW80 => "RW80",
            Self::RW82 => "RW82",
            Self::RW84 => "RW84",
            Self::RW86 => "RW86",
            Self::RW88 => "RW88",
            Self::RW8A => "RW8A",
            Self::RW8C => "RW8C",
            Self::RW8E => "RW8E",
            Self::RW90 => "RW90",
            Self::RW92 => "RW92",
            Self::RW94 => "RW94",
            Self::RW96 => "RW96",
            Self::RW98 => "RW98",
            Self::RW9A => "RW9A",
            Self::RW9C => "RW9C",
            Self::RW9E => "RW9E",
            Self::RWA0 => "RWA0",
            Self::RWA2 => "RWA2",
            Self::RWA4 => "RWA4",
            Self::RWA6 => "RWA6",
            Self::RWA8 => "RWA8",
            Self::RWAA => "RWAA",
            Self::RWAC => "RWAC",
            Self::RWAE => "RWAE",
            Self::RWB0 => "RWB0",
            Self::RWB2 => "RWB2",
            Self::RWB4 => "RWB4",
            Self::RWB6 => "RWB6",
            Self::RWB8 => "RWB8",
            Self::RWBA => "RWBA",
            Self::RWBC => "RWBC",
            Self::RWBE => "RWBE",
            Self::RWC0 => "RWC0",
            Self::RWC2 => "RWC2",
            Self::RWC4 => "RWC4",
            Self::RWC6 => "RWC6",
            Self::RWC8 => "RWC8",
            Self::RWCA => "RWCA",
            Self::RWCC => "RWCC",
            Self::RWCE => "RWCE",
            Self::RWD0 => "RWD0",
            Self::RWD2 => "RWD2",
            Self::RWD4 => "RWD4",
            Self::RWD6 => "RWD6",
            Self::RWD8 => "RWD8",
            Self::RWDA => "RWDA",
            Self::RWDC => "RWDC",
            Self::RWDE => "RWDE",
            Self::RWE0 => "RWE0",
            Self::RWE2 => "RWE2",
            Self::RWE4 => "RWE4",
            Self::RWE6 => "RWE6",
            Self::RWE8 => "RWE8",
            Self::RWEA => "RWEA",
            Self::RWEC => "RWEC",
            Self::RWEE => "RWEE",
            Self::RWF0 => "RWF0",
            Self::RWF2 => "RWF2",
            Self::RWF4 => "RWF4",
            Self::RWF6 => "RWF6",
            Self::RWF8 => "RWF8",
            Self::RWFA => "RWFA",
            Self::RWFC => "RWFC",
            Self::RWFE => "RWFE",
            Self::RW100 => "RW100",
            Self::RW102 => "RW102",
            Self::RW104 => "RW104",
            Self::RW106 => "RW106",
            Self::RW108 => "RW108",
            Self::RW10A => "RW10A",
            Self::RW10C => "RW10C",
            Self::RW10E => "RW10E",
            Self::RW110 => "RW110",
            Self::RW112 => "RW112",
            Self::RW114 => "RW114",
            Self::RW116 => "RW116",
            Self::RW118 => "RW118",
            Self::RW11A => "RW11A",
            Self::RW11C => "RW11C",
            Self::RW11E => "RW11E",
            Self::RW120 => "RW120",
            Self::RW122 => "RW122",
            Self::RW124 => "RW124",
            Self::RW126 => "RW126",
            Self::RW128 => "RW128",
            Self::RW12A => "RW12A",
            Self::RW12C => "RW12C",
            Self::RW12E => "RW12E",
            Self::RW130 => "RW130",
            Self::RW132 => "RW132",
            Self::RW134 => "RW134",
            Self::RW136 => "RW136",
            Self::RW138 => "RW138",
            Self::RW13A => "RW13A",
            Self::RW13C => "RW13C",
            Self::RW13E => "RW13E",
            Self::RW140 => "RW140",
            Self::RW142 => "RW142",
            Self::RW144 => "RW144",
            Self::RW146 => "RW146",
            Self::RW148 => "RW148",
            Self::RW14A => "RW14A",
            Self::RW14C => "RW14C",
            Self::RW14E => "RW14E",
            Self::RW150 => "RW150",
            Self::RW152 => "RW152",
            Self::RW154 => "RW154",
            Self::RW156 => "RW156",
            Self::RW158 => "RW158",
            Self::RW15A => "RW15A",
            Self::RW15C => "RW15C",
            Self::RW15E => "RW15E",
            Self::RW160 => "RW160",
            Self::RW162 => "RW162",
            Self::RW164 => "RW164",
            Self::RW166 => "RW166",
            Self::RW168 => "RW168",
            Self::RW16A => "RW16A",
            Self::RW16C => "RW16C",
            Self::RW16E => "RW16E",
            Self::RW170 => "RW170",
            Self::RW172 => "RW172",
            Self::RW174 => "RW174",
            Self::RW176 => "RW176",
            Self::RW178 => "RW178",
            Self::RW17A => "RW17A",
            Self::RW17C => "RW17C",
            Self::RW17E => "RW17E",
            Self::RW180 => "RW180",
            Self::RW182 => "RW182",
            Self::RW184 => "RW184",
            Self::RW186 => "RW186",
            Self::RW188 => "RW188",
            Self::RW18A => "RW18A",
            Self::RW18C => "RW18C",
            Self::RW18E => "RW18E",
            Self::RW190 => "RW190",
            Self::RW192 => "RW192",
            Self::RW194 => "RW194",
            Self::RW196 => "RW196",
            Self::RW198 => "RW198",
            Self::RW19A => "RW19A",
            Self::RW19C => "RW19C",
            Self::RW19E => "RW19E",
            Self::RW1A0 => "RW1A0",
            Self::RW1A2 => "RW1A2",
            Self::RW1A4 => "RW1A4",
            Self::RW1A6 => "RW1A6",
            Self::RW1A8 => "RW1A8",
            Self::RW1AA => "RW1AA",
            Self::RW1AC => "RW1AC",
            Self::RW1AE => "RW1AE",
            Self::RW1B0 => "RW1B0",
            Self::RW1B2 => "RW1B2",
            Self::RW1B4 => "RW1B4",
            Self::RW1B6 => "RW1B6",
            Self::RW1B8 => "RW1B8",
            Self::RW1BA => "RW1BA",
            Self::RW1BC => "RW1BC",
            Self::RW1BE => "RW1BE",
            Self::RW1C0 => "RW1C0",
            Self::RW1C2 => "RW1C2",
            Self::RW1C4 => "RW1C4",
            Self::RW1C6 => "RW1C6",
            Self::RW1C8 => "RW1C8",
            Self::RW1CA => "RW1CA",
            Self::RW1CC => "RW1CC",
            Self::RW1CE => "RW1CE",
            Self::RW1D0 => "RW1D0",
            Self::RW1D2 => "RW1D2",
            Self::RW1D4 => "RW1D4",
            Self::RW1D6 => "RW1D6",
            Self::RW1D8 => "RW1D8",
            Self::RW1DA => "RW1DA",
            Self::RW1DC => "RW1DC",
            Self::RW1DE => "RW1DE",
            Self::RW1E0 => "RW1E0",
            Self::RW1E2 => "RW1E2",
            Self::RW1E4 => "RW1E4",
            Self::RW1E6 => "RW1E6",
            Self::RW1E8 => "RW1E8",
            Self::RW1EA => "RW1EA",
            Self::RW1EC => "RW1EC",
            Self::RW1EE => "RW1EE",
            Self::RW1F0 => "RW1F0",
            Self::RW1F2 => "RW1F2",
            Self::RW1F4 => "RW1F4",
            Self::RW1F6 => "RW1F6",
            Self::RW1F8 => "RW1F8",
            Self::RW1FA => "RW1FA",
            Self::RW1FC => "RW1FC",
            Self::RW1FE => "RW1FE",
            Self::RL1C => "RL1C",
            Self::RL20 => "RL20",
            Self::RL24 => "RL24",
            Self::RL28 => "RL28",
            Self::RL2C => "RL2C",
            Self::RL30 => "RL30",
            Self::RL34 => "RL34",
            Self::RL38 => "RL38",
            Self::RL3C => "RL3C",
            Self::RL40 => "RL40",
            Self::RL44 => "RL44",
            Self::RL48 => "RL48",
            Self::RL4C => "RL4C",
            Self::RL50 => "RL50",
            Self::RL54 => "RL54",
            Self::RL58 => "RL58",
            Self::RL5C => "RL5C",
            Self::RL60 => "RL60",
            Self::RL64 => "RL64",
            Self::RL68 => "RL68",
            Self::RL6C => "RL6C",
            Self::RL70 => "RL70",
            Self::RL74 => "RL74",
            Self::RL78 => "RL78",
            Self::RL7C => "RL7C",
            Self::RL80 => "RL80",
            Self::RL84 => "RL84",
            Self::RL88 => "RL88",
            Self::RL8C => "RL8C",
            Self::RL90 => "RL90",
            Self::RL94 => "RL94",
            Self::RL98 => "RL98",
            Self::RL9C => "RL9C",
            Self::RLA0 => "RLA0",
            Self::RLA4 => "RLA4",
            Self::RLA8 => "RLA8",
            Self::RLAC => "RLAC",
            Self::RLB0 => "RLB0",
            Self::RLB4 => "RLB4",
            Self::RLB8 => "RLB8",
            Self::RLBC => "RLBC",
            Self::RLC0 => "RLC0",
            Self::RLC4 => "RLC4",
            Self::RLC8 => "RLC8",
            Self::RLCC => "RLCC",
            Self::RLD0 => "RLD0",
            Self::RLD4 => "RLD4",
            Self::RLD8 => "RLD8",
            Self::RLDC => "RLDC",
            Self::RLE0 => "RLE0",
            Self::RLE4 => "RLE4",
            Self::RLE8 => "RLE8",
            Self::RLEC => "RLEC",
            Self::RLF0 => "RLF0",
            Self::RLF4 => "RLF4",
            Self::RLF8 => "RLF8",
            Self::RLFC => "RLFC",
            Self::RL100 => "RL100",
            Self::RL104 => "RL104",
            Self::RL108 => "RL108",
            Self::RL10C => "RL10C",
            Self::RL110 => "RL110",
            Self::RL114 => "RL114",
            Self::RL118 => "RL118",
            Self::RL11C => "RL11C",
            Self::RL120 => "RL120",
            Self::RL124 => "RL124",
            Self::RL128 => "RL128",
            Self::RL12C => "RL12C",
            Self::RL130 => "RL130",
            Self::RL134 => "RL134",
            Self::RL138 => "RL138",
            Self::RL13C => "RL13C",
            Self::RL140 => "RL140",
            Self::RL144 => "RL144",
            Self::RL148 => "RL148",
            Self::RL14C => "RL14C",
            Self::RL150 => "RL150",
            Self::RL154 => "RL154",
            Self::RL158 => "RL158",
            Self::RL15C => "RL15C",
            Self::RL160 => "RL160",
            Self::RL164 => "RL164",
            Self::RL168 => "RL168",
            Self::RL16C => "RL16C",
            Self::RL170 => "RL170",
            Self::RL174 => "RL174",
            Self::RL178 => "RL178",
            Self::RL17C => "RL17C",
            Self::RL180 => "RL180",
            Self::RL184 => "RL184",
            Self::RL188 => "RL188",
            Self::RL18C => "RL18C",
            Self::RL190 => "RL190",
            Self::RL194 => "RL194",
            Self::RL198 => "RL198",
            Self::RL19C => "RL19C",
            Self::RL1A0 => "RL1A0",
            Self::RL1A4 => "RL1A4",
            Self::RL1A8 => "RL1A8",
            Self::RL1AC => "RL1AC",
            Self::RL1B0 => "RL1B0",
            Self::RL1B4 => "RL1B4",
            Self::RL1B8 => "RL1B8",
            Self::RL1BC => "RL1BC",
            Self::RL1C0 => "RL1C0",
            Self::RL1C4 => "RL1C4",
            Self::RL1C8 => "RL1C8",
            Self::RL1CC => "RL1CC",
            Self::RL1D0 => "RL1D0",
            Self::RL1D4 => "RL1D4",
            Self::RL1D8 => "RL1D8",
            Self::RL1DC => "RL1DC",
            Self::RL1E0 => "RL1E0",
            Self::RL1E4 => "RL1E4",
            Self::RL1E8 => "RL1E8",
            Self::RL1EC => "RL1EC",
            Self::RL1F0 => "RL1F0",
            Self::RL1F4 => "RL1F4",
            Self::RL1F8 => "RL1F8",
            Self::RL1FC => "RL1FC",
        }
    }
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
fn meaning_0_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZRlo,
        1 => Register::ZRhi,
        2 => Register::AD_resultlo,
        3 => Register::AD_resulthi,
        4 => Register::HSI_timelo,
        5 => Register::HSI_timehi,
        6 => Register::HSI_status,
        7 => Register::SBUF,
        8 => Register::INT_MASK,
        9 => Register::INT_PEND,
        10 => Register::TIMER1lo,
        11 => Register::TIMER1hi,
        12 => Register::TIMER2lo,
        13 => Register::TIMER2hi,
        14 => Register::PORT0,
        15 => Register::PORT1,
        16 => Register::PORT2,
        17 => Register::SP_STAT,
        18 => Register::INT_PEND1,
        19 => Register::INT_MASK1,
        20 => Register::WSR,
        21 => Register::IOS0,
        22 => Register::IOS1,
        23 => Register::IOS2,
        24 => Register::SPlo,
        25 => Register::SPhi,
        26 => Register::R1A,
        27 => Register::R1B,
        28 => Register::R1C,
        29 => Register::R1D,
        30 => Register::R1E,
        31 => Register::R1F,
        32 => Register::R20,
        33 => Register::R21,
        34 => Register::R22,
        35 => Register::R23,
        36 => Register::R24,
        37 => Register::R25,
        38 => Register::R26,
        39 => Register::R27,
        40 => Register::R28,
        41 => Register::R29,
        42 => Register::R2A,
        43 => Register::R2B,
        44 => Register::R2C,
        45 => Register::R2D,
        46 => Register::R2E,
        47 => Register::R2F,
        48 => Register::R30,
        49 => Register::R31,
        50 => Register::R32,
        51 => Register::R33,
        52 => Register::R34,
        53 => Register::R35,
        54 => Register::R36,
        55 => Register::R37,
        56 => Register::R38,
        57 => Register::R39,
        58 => Register::R3A,
        59 => Register::R3B,
        60 => Register::R3C,
        61 => Register::R3D,
        62 => Register::R3E,
        63 => Register::R3F,
        64 => Register::R40,
        65 => Register::R41,
        66 => Register::R42,
        67 => Register::R43,
        68 => Register::R44,
        69 => Register::R45,
        70 => Register::R46,
        71 => Register::R47,
        72 => Register::R48,
        73 => Register::R49,
        74 => Register::R4A,
        75 => Register::R4B,
        76 => Register::R4C,
        77 => Register::R4D,
        78 => Register::R4E,
        79 => Register::R4F,
        80 => Register::R50,
        81 => Register::R51,
        82 => Register::R52,
        83 => Register::R53,
        84 => Register::R54,
        85 => Register::R55,
        86 => Register::R56,
        87 => Register::R57,
        88 => Register::R58,
        89 => Register::R59,
        90 => Register::R5A,
        91 => Register::R5B,
        92 => Register::R5C,
        93 => Register::R5D,
        94 => Register::R5E,
        95 => Register::R5F,
        96 => Register::R60,
        97 => Register::R61,
        98 => Register::R62,
        99 => Register::R63,
        100 => Register::R64,
        101 => Register::R65,
        102 => Register::R66,
        103 => Register::R67,
        104 => Register::R68,
        105 => Register::R69,
        106 => Register::R6A,
        107 => Register::R6B,
        108 => Register::R6C,
        109 => Register::R6D,
        110 => Register::R6E,
        111 => Register::R6F,
        112 => Register::R70,
        113 => Register::R71,
        114 => Register::R72,
        115 => Register::R73,
        116 => Register::R74,
        117 => Register::R75,
        118 => Register::R76,
        119 => Register::R77,
        120 => Register::R78,
        121 => Register::R79,
        122 => Register::R7A,
        123 => Register::R7B,
        124 => Register::R7C,
        125 => Register::R7D,
        126 => Register::R7E,
        127 => Register::R7F,
        128 => Register::R80,
        129 => Register::R81,
        130 => Register::R82,
        131 => Register::R83,
        132 => Register::R84,
        133 => Register::R85,
        134 => Register::R86,
        135 => Register::R87,
        136 => Register::R88,
        137 => Register::R89,
        138 => Register::R8A,
        139 => Register::R8B,
        140 => Register::R8C,
        141 => Register::R8D,
        142 => Register::R8E,
        143 => Register::R8F,
        144 => Register::R90,
        145 => Register::R91,
        146 => Register::R92,
        147 => Register::R93,
        148 => Register::R94,
        149 => Register::R95,
        150 => Register::R96,
        151 => Register::R97,
        152 => Register::R98,
        153 => Register::R99,
        154 => Register::R9A,
        155 => Register::R9B,
        156 => Register::R9C,
        157 => Register::R9D,
        158 => Register::R9E,
        159 => Register::R9F,
        160 => Register::RA0,
        161 => Register::RA1,
        162 => Register::RA2,
        163 => Register::RA3,
        164 => Register::RA4,
        165 => Register::RA5,
        166 => Register::RA6,
        167 => Register::RA7,
        168 => Register::RA8,
        169 => Register::RA9,
        170 => Register::RAA,
        171 => Register::RAB,
        172 => Register::RAC,
        173 => Register::RAD,
        174 => Register::RAE,
        175 => Register::RAF,
        176 => Register::RB0,
        177 => Register::RB1,
        178 => Register::RB2,
        179 => Register::RB3,
        180 => Register::RB4,
        181 => Register::RB5,
        182 => Register::RB6,
        183 => Register::RB7,
        184 => Register::RB8,
        185 => Register::RB9,
        186 => Register::RBA,
        187 => Register::RBB,
        188 => Register::RBC,
        189 => Register::RBD,
        190 => Register::RBE,
        191 => Register::RBF,
        192 => Register::RC0,
        193 => Register::RC1,
        194 => Register::RC2,
        195 => Register::RC3,
        196 => Register::RC4,
        197 => Register::RC5,
        198 => Register::RC6,
        199 => Register::RC7,
        200 => Register::RC8,
        201 => Register::RC9,
        202 => Register::RCA,
        203 => Register::RCB,
        204 => Register::RCC,
        205 => Register::RCD,
        206 => Register::RCE,
        207 => Register::RCF,
        208 => Register::RD0,
        209 => Register::RD1,
        210 => Register::RD2,
        211 => Register::RD3,
        212 => Register::RD4,
        213 => Register::RD5,
        214 => Register::RD6,
        215 => Register::RD7,
        216 => Register::RD8,
        217 => Register::RD9,
        218 => Register::RDA,
        219 => Register::RDB,
        220 => Register::RDC,
        221 => Register::RDD,
        222 => Register::RDE,
        223 => Register::RDF,
        224 => Register::RE0,
        225 => Register::RE1,
        226 => Register::RE2,
        227 => Register::RE3,
        228 => Register::RE4,
        229 => Register::RE5,
        230 => Register::RE6,
        231 => Register::RE7,
        232 => Register::RE8,
        233 => Register::RE9,
        234 => Register::REA,
        235 => Register::REB,
        236 => Register::REC,
        237 => Register::RED,
        238 => Register::REE,
        239 => Register::REF,
        240 => Register::RF0,
        241 => Register::RF1,
        242 => Register::RF2,
        243 => Register::RF3,
        244 => Register::RF4,
        245 => Register::RF5,
        246 => Register::RF6,
        247 => Register::RF7,
        248 => Register::RF8,
        249 => Register::RF9,
        250 => Register::RFA,
        251 => Register::RFB,
        252 => Register::RFC,
        253 => Register::RFD,
        254 => Register::RFE,
        255 => Register::RFF,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_0_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_1_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZR,
        2 => Register::AD_result,
        4 => Register::HSI_time,
        6 => Register::HSI_SBUF,
        8 => Register::INTERRUPT,
        10 => Register::TIMER1,
        12 => Register::TIMER2,
        14 => Register::PORT01,
        16 => Register::PORT2_SPS,
        18 => Register::INT1,
        20 => Register::WSR_IOS0,
        22 => Register::IOS12,
        24 => Register::SP,
        26 => Register::RW1A,
        28 => Register::RW1C,
        30 => Register::RW1E,
        32 => Register::RW20,
        34 => Register::RW22,
        36 => Register::RW24,
        38 => Register::RW26,
        40 => Register::RW28,
        42 => Register::RW2A,
        44 => Register::RW2C,
        46 => Register::RW2E,
        48 => Register::RW30,
        50 => Register::RW32,
        52 => Register::RW34,
        54 => Register::RW36,
        56 => Register::RW38,
        58 => Register::RW3A,
        60 => Register::RW3C,
        62 => Register::RW3E,
        64 => Register::RW40,
        66 => Register::RW42,
        68 => Register::RW44,
        70 => Register::RW46,
        72 => Register::RW48,
        74 => Register::RW4A,
        76 => Register::RW4C,
        78 => Register::RW4E,
        80 => Register::RW50,
        82 => Register::RW52,
        84 => Register::RW54,
        86 => Register::RW56,
        88 => Register::RW58,
        90 => Register::RW5A,
        92 => Register::RW5C,
        94 => Register::RW5E,
        96 => Register::RW60,
        98 => Register::RW62,
        100 => Register::RW64,
        102 => Register::RW66,
        104 => Register::RW68,
        106 => Register::RW6A,
        108 => Register::RW6C,
        110 => Register::RW6E,
        112 => Register::RW70,
        114 => Register::RW72,
        116 => Register::RW74,
        118 => Register::RW76,
        120 => Register::RW78,
        122 => Register::RW7A,
        124 => Register::RW7C,
        126 => Register::RW7E,
        128 => Register::RW80,
        130 => Register::RW82,
        132 => Register::RW84,
        134 => Register::RW86,
        136 => Register::RW88,
        138 => Register::RW8A,
        140 => Register::RW8C,
        142 => Register::RW8E,
        144 => Register::RW90,
        146 => Register::RW92,
        148 => Register::RW94,
        150 => Register::RW96,
        152 => Register::RW98,
        154 => Register::RW9A,
        156 => Register::RW9C,
        158 => Register::RW9E,
        160 => Register::RWA0,
        162 => Register::RWA2,
        164 => Register::RWA4,
        166 => Register::RWA6,
        168 => Register::RWA8,
        170 => Register::RWAA,
        172 => Register::RWAC,
        174 => Register::RWAE,
        176 => Register::RWB0,
        178 => Register::RWB2,
        180 => Register::RWB4,
        182 => Register::RWB6,
        184 => Register::RWB8,
        186 => Register::RWBA,
        188 => Register::RWBC,
        190 => Register::RWBE,
        192 => Register::RWC0,
        194 => Register::RWC2,
        196 => Register::RWC4,
        198 => Register::RWC6,
        200 => Register::RWC8,
        202 => Register::RWCA,
        204 => Register::RWCC,
        206 => Register::RWCE,
        208 => Register::RWD0,
        210 => Register::RWD2,
        212 => Register::RWD4,
        214 => Register::RWD6,
        216 => Register::RWD8,
        218 => Register::RWDA,
        220 => Register::RWDC,
        222 => Register::RWDE,
        224 => Register::RWE0,
        226 => Register::RWE2,
        228 => Register::RWE4,
        230 => Register::RWE6,
        232 => Register::RWE8,
        234 => Register::RWEA,
        236 => Register::RWEC,
        238 => Register::RWEE,
        240 => Register::RWF0,
        242 => Register::RWF2,
        244 => Register::RWF4,
        246 => Register::RWF6,
        248 => Register::RWF8,
        250 => Register::RWFA,
        252 => Register::RWFC,
        254 => Register::RWFE,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_1_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_2_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::ZR,
        1 => Register::AD_result,
        2 => Register::HSI_time,
        3 => Register::HSI_SBUF,
        4 => Register::INTERRUPT,
        5 => Register::TIMER1,
        6 => Register::TIMER2,
        7 => Register::PORT01,
        8 => Register::PORT2_SPS,
        9 => Register::INT1,
        10 => Register::WSR_IOS0,
        11 => Register::IOS12,
        12 => Register::SP,
        13 => Register::RW1A,
        14 => Register::RW1C,
        15 => Register::RW1E,
        16 => Register::RW20,
        17 => Register::RW22,
        18 => Register::RW24,
        19 => Register::RW26,
        20 => Register::RW28,
        21 => Register::RW2A,
        22 => Register::RW2C,
        23 => Register::RW2E,
        24 => Register::RW30,
        25 => Register::RW32,
        26 => Register::RW34,
        27 => Register::RW36,
        28 => Register::RW38,
        29 => Register::RW3A,
        30 => Register::RW3C,
        31 => Register::RW3E,
        32 => Register::RW40,
        33 => Register::RW42,
        34 => Register::RW44,
        35 => Register::RW46,
        36 => Register::RW48,
        37 => Register::RW4A,
        38 => Register::RW4C,
        39 => Register::RW4E,
        40 => Register::RW50,
        41 => Register::RW52,
        42 => Register::RW54,
        43 => Register::RW56,
        44 => Register::RW58,
        45 => Register::RW5A,
        46 => Register::RW5C,
        47 => Register::RW5E,
        48 => Register::RW60,
        49 => Register::RW62,
        50 => Register::RW64,
        51 => Register::RW66,
        52 => Register::RW68,
        53 => Register::RW6A,
        54 => Register::RW6C,
        55 => Register::RW6E,
        56 => Register::RW70,
        57 => Register::RW72,
        58 => Register::RW74,
        59 => Register::RW76,
        60 => Register::RW78,
        61 => Register::RW7A,
        62 => Register::RW7C,
        63 => Register::RW7E,
        64 => Register::RW80,
        65 => Register::RW82,
        66 => Register::RW84,
        67 => Register::RW86,
        68 => Register::RW88,
        69 => Register::RW8A,
        70 => Register::RW8C,
        71 => Register::RW8E,
        72 => Register::RW90,
        73 => Register::RW92,
        74 => Register::RW94,
        75 => Register::RW96,
        76 => Register::RW98,
        77 => Register::RW9A,
        78 => Register::RW9C,
        79 => Register::RW9E,
        80 => Register::RWA0,
        81 => Register::RWA2,
        82 => Register::RWA4,
        83 => Register::RWA6,
        84 => Register::RWA8,
        85 => Register::RWAA,
        86 => Register::RWAC,
        87 => Register::RWAE,
        88 => Register::RWB0,
        89 => Register::RWB2,
        90 => Register::RWB4,
        91 => Register::RWB6,
        92 => Register::RWB8,
        93 => Register::RWBA,
        94 => Register::RWBC,
        95 => Register::RWBE,
        96 => Register::RWC0,
        97 => Register::RWC2,
        98 => Register::RWC4,
        99 => Register::RWC6,
        100 => Register::RWC8,
        101 => Register::RWCA,
        102 => Register::RWCC,
        103 => Register::RWCE,
        104 => Register::RWD0,
        105 => Register::RWD2,
        106 => Register::RWD4,
        107 => Register::RWD6,
        108 => Register::RWD8,
        109 => Register::RWDA,
        110 => Register::RWDC,
        111 => Register::RWDE,
        112 => Register::RWE0,
        113 => Register::RWE2,
        114 => Register::RWE4,
        115 => Register::RWE6,
        116 => Register::RWE8,
        117 => Register::RWEA,
        118 => Register::RWEC,
        119 => Register::RWEE,
        120 => Register::RWF0,
        121 => Register::RWF2,
        122 => Register::RWF4,
        123 => Register::RWF6,
        124 => Register::RWF8,
        125 => Register::RWFA,
        126 => Register::RWFC,
        127 => Register::RWFE,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_2_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_2_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_3_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZR_AD,
        4 => Register::HSI,
        8 => Register::INT_TIMER1,
        12 => Register::TIMER2_PORT01,
        16 => Register::PORT2_INT1,
        20 => Register::WSR_IOS012,
        24 => Register::SPR1A,
        28 => Register::RL1C,
        32 => Register::RL20,
        36 => Register::RL24,
        40 => Register::RL28,
        44 => Register::RL2C,
        48 => Register::RL30,
        52 => Register::RL34,
        56 => Register::RL38,
        60 => Register::RL3C,
        64 => Register::RL40,
        68 => Register::RL44,
        72 => Register::RL48,
        76 => Register::RL4C,
        80 => Register::RL50,
        84 => Register::RL54,
        88 => Register::RL58,
        92 => Register::RL5C,
        96 => Register::RL60,
        100 => Register::RL64,
        104 => Register::RL68,
        108 => Register::RL6C,
        112 => Register::RL70,
        116 => Register::RL74,
        120 => Register::RL78,
        124 => Register::RL7C,
        128 => Register::RL80,
        132 => Register::RL84,
        136 => Register::RL88,
        140 => Register::RL8C,
        144 => Register::RL90,
        148 => Register::RL94,
        152 => Register::RL98,
        156 => Register::RL9C,
        160 => Register::RLA0,
        164 => Register::RLA4,
        168 => Register::RLA8,
        172 => Register::RLAC,
        176 => Register::RLB0,
        180 => Register::RLB4,
        184 => Register::RLB8,
        188 => Register::RLBC,
        192 => Register::RLC0,
        196 => Register::RLC4,
        200 => Register::RLC8,
        204 => Register::RLCC,
        208 => Register::RLD0,
        212 => Register::RLD4,
        216 => Register::RLD8,
        220 => Register::RLDC,
        224 => Register::RLE0,
        228 => Register::RLE4,
        232 => Register::RLE8,
        236 => Register::RLEC,
        240 => Register::RLF0,
        244 => Register::RLF4,
        248 => Register::RLF8,
        252 => Register::RLFC,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_3_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_3_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Number(bool, bool, u64),
}
impl core::fmt::Display for DisplayElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Literal(lit) => lit.fmt(f),
            Self::Register(reg) => reg.fmt(f),
            Self::Number(true, false, value) => {
                write!(f, "0x{:x}", value)
            }
            Self::Number(true, true, value) => {
                write!(f, "-0x{:x}", value)
            }
            Self::Number(false, false, value) => value.fmt(f),
            Self::Number(false, true, value) => {
                write!(f, "-{:x}", value)
            }
        }
    }
}
#[doc = "Create token_fields: cond"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: imm16 disp16"]
fn token_10(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: addbit8"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: op6"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 2) & 63) as u8)
}
#[doc = "Create token_fields: op8 imm8 simm8 baop breg8 dbreg waop wreg8 dwreg lreg dlreg"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: op5"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 3) & 31) as u8)
}
#[doc = "Create token_fields: jmp11_lo"]
fn token_13(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 255) as u8)
}
#[doc = "Create token_fields: op16"]
fn token_11(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 31) as u8)
}
#[doc = "Create token_fields: aa"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 3) as u8)
}
#[doc = "Create token_fields: op4 highb"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: jmp11_hi"]
fn token_12(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: imm7 iwreg7"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 1) & 127) as u8)
}
#[doc = "Create token_fields: bitno"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 7) as u8)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub ());
#[derive(Clone, Copy, Default)]
pub struct GlobalSet(());
impl GlobalSet {
    pub fn new(_: ContextMemory) -> Self {
        Self(())
    }
    pub fn set(&mut self, _: Option<AddrType>, _: impl FnOnce(&mut ContextMemory)) {
        unreachable!()
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:645:1, end:645:2))"]
#[derive(Clone, Debug)]
struct DIV_instructionVar0 {
    lreg: u8,
    oper16: Tableoper16,
}
impl DIV_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DIV"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:657:1, end:657:2))"]
#[derive(Clone, Debug)]
struct DIVB_instructionVar1 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl DIVB_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DIVB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:815:1, end:815:2))"]
#[derive(Clone, Debug)]
struct MUL_instructionVar2 {
    lreg: u8,
    oper16: Tableoper16,
}
impl MUL_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MUL"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:823:1, end:823:2))"]
#[derive(Clone, Debug)]
struct MUL_instructionVar3 {
    lreg: u8,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl MUL_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MUL"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:831:1, end:831:2))"]
#[derive(Clone, Debug)]
struct MULB_instructionVar4 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl MULB_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:839:1, end:839:2))"]
#[derive(Clone, Debug)]
struct MULB_instructionVar5 {
    oper8: Tableoper8,
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl MULB_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1004:1, end:1004:2))"]
#[derive(Clone, Debug)]
struct SHL_instructionVar6 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl SHL_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1015:1, end:1015:2))"]
#[derive(Clone, Debug)]
struct SHL_instructionVar7 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl SHL_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1026:1, end:1026:2))"]
#[derive(Clone, Debug)]
struct SHLB_instructionVar8 {
    dbreg: u8,
    immed8: Tableimmed8,
}
impl SHLB_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHLB"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1037:1, end:1037:2))"]
#[derive(Clone, Debug)]
struct SHLB_instructionVar9 {
    dbreg: u8,
    breg: Tablebreg,
}
impl SHLB_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHLB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1048:1, end:1048:2))"]
#[derive(Clone, Debug)]
struct SHLL_instructionVar10 {
    lreg: u8,
    immed8: Tableimmed8,
}
impl SHLL_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHLL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1059:1, end:1059:2))"]
#[derive(Clone, Debug)]
struct SHLL_instructionVar11 {
    lreg: u8,
    breg: Tablebreg,
}
impl SHLL_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHLL"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1070:1, end:1070:2))"]
#[derive(Clone, Debug)]
struct SHR_instructionVar12 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl SHR_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1083:1, end:1083:2))"]
#[derive(Clone, Debug)]
struct SHR_instructionVar13 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl SHR_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1095:1, end:1095:2))"]
#[derive(Clone, Debug)]
struct SHRA_instructionVar14 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl SHRA_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1107:1, end:1107:2))"]
#[derive(Clone, Debug)]
struct SHRA_instructionVar15 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl SHRA_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1119:1, end:1119:2))"]
#[derive(Clone, Debug)]
struct SHRAB_instructionVar16 {
    dbreg: u8,
    immed8: Tableimmed8,
}
impl SHRAB_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRAB"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1131:1, end:1131:2))"]
#[derive(Clone, Debug)]
struct SHRAB_instructionVar17 {
    dbreg: u8,
    breg: Tablebreg,
}
impl SHRAB_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRAB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1143:1, end:1143:2))"]
#[derive(Clone, Debug)]
struct SHRAL_instructionVar18 {
    lreg: u8,
    immed8: Tableimmed8,
}
impl SHRAL_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRAL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1155:1, end:1155:2))"]
#[derive(Clone, Debug)]
struct SHRAL_instructionVar19 {
    lreg: u8,
    breg: Tablebreg,
}
impl SHRAL_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRAL"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1167:1, end:1167:2))"]
#[derive(Clone, Debug)]
struct SHRB_instructionVar20 {
    dbreg: u8,
    immed8: Tableimmed8,
}
impl SHRB_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRB"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1179:1, end:1179:2))"]
#[derive(Clone, Debug)]
struct SHRB_instructionVar21 {
    dbreg: u8,
    breg: Tablebreg,
}
impl SHRB_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1191:1, end:1191:2))"]
#[derive(Clone, Debug)]
struct SHRL_instructionVar22 {
    lreg: u8,
    immed8: Tableimmed8,
}
impl SHRL_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c59 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c59(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1203:1, end:1203:2))"]
#[derive(Clone, Debug)]
struct SHRL_instructionVar23 {
    lreg: u8,
    breg: Tablebreg,
}
impl SHRL_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHRL"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c57 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if token_5(tokens) == 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c57(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1308:1, end:1308:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar24 {
    iwreg: Tableiwreg,
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl XCH_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCH"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed8,
                wreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1318:1, end:1318:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar25 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
    wreg: Tablewreg,
}
impl XCH_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCH"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.immed16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let immed16 = if let Some((len, table)) =
            Tableimmed16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed16,
                wreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1337:1, end:1337:2))"]
#[derive(Clone, Debug)]
struct XCHB_instructionVar26 {
    iwreg: Tableiwreg,
    immed8: Tableimmed8,
    breg: Tablebreg,
}
impl XCHB_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCHB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed8,
                breg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1347:1, end:1347:2))"]
#[derive(Clone, Debug)]
struct XCHB_instructionVar27 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
    breg: Tablebreg,
}
impl XCHB_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCHB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.immed16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let immed16 = if let Some((len, table)) =
            Tableimmed16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed16,
                breg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:568:1, end:568:2))"]
#[derive(Clone, Debug)]
struct BMOV_instructionVar28 {
    lreg: u8,
    wreg: Tablewreg,
}
impl BMOV_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BMOV"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:572:1, end:572:2))"]
#[derive(Clone, Debug)]
struct BMOVI_instructionVar29 {
    lreg: u8,
    wreg: Tablewreg,
}
impl BMOVI_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BMOVI"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:576:1, end:576:2))"]
#[derive(Clone, Debug)]
struct BR_instructionVar30 {
    wreg: Tablewreg,
}
impl BR_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BR"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("]"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:580:1, end:580:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar31 {
    wreg: Tablewreg,
}
impl CLR_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:588:1, end:588:2))"]
#[derive(Clone, Debug)]
struct CLRB_instructionVar32 {
    breg: Tablebreg,
}
impl CLRB_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:596:1, end:596:2))"]
#[derive(Clone, Debug)]
struct CLRC_instructionVar33 {}
impl CLRC_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:600:1, end:600:2))"]
#[derive(Clone, Debug)]
struct CLRVT_instructionVar34 {}
impl CLRVT_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRVT"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:617:1, end:617:2))"]
#[derive(Clone, Debug)]
struct CMPL_instructionVar35 {
    dlreg: u8,
    lreg: u8,
}
impl CMPL_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMPL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.dlreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dlreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { dlreg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:629:1, end:629:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar36 {
    wreg: Tablewreg,
}
impl DEC_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:635:1, end:635:2))"]
#[derive(Clone, Debug)]
struct DECB_instructionVar37 {
    breg: Tablebreg,
}
impl DECB_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:641:1, end:641:2))"]
#[derive(Clone, Debug)]
struct DI_instructionVar38 {}
impl DI_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:689:1, end:689:2))"]
#[derive(Clone, Debug)]
struct DJNZ_instructionVar39 {
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl DJNZ_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DJNZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let jmpdest = if let Some((len, table)) =
            Tablejmpdest::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, jmpdest }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:695:1, end:695:2))"]
#[derive(Clone, Debug)]
struct DJNZW_instructionVar40 {
    wreg: Tablewreg,
    jmpdest: Tablejmpdest,
}
impl DJNZW_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DJNZW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let jmpdest = if let Some((len, table)) =
            Tablejmpdest::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, jmpdest }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:703:1, end:703:2))"]
#[derive(Clone, Debug)]
struct DPTS_instructionVar41 {}
impl DPTS_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DPTS"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:709:1, end:709:2))"]
#[derive(Clone, Debug)]
struct EI_instructionVar42 {}
impl EI_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:714:1, end:714:2))"]
#[derive(Clone, Debug)]
struct EPTS_instructionVar43 {}
impl EPTS_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EPTS"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:720:1, end:720:2))"]
#[derive(Clone, Debug)]
struct EXT_instructionVar44 {
    lreg: u8,
}
impl EXT_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EXT"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_3_display(self.lreg)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:726:1, end:726:2))"]
#[derive(Clone, Debug)]
struct EXTB_instructionVar45 {
    wreg: Tablewreg,
}
impl EXTB_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EXTB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:732:1, end:732:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar46 {
    wreg: Tablewreg,
}
impl INC_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:738:1, end:738:2))"]
#[derive(Clone, Debug)]
struct INCB_instructionVar47 {
    breg: Tablebreg,
}
impl INCB_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:745:1, end:745:2))"]
#[derive(Clone, Debug)]
struct IDLPD_instructionVar48 {
    immed8: Tableimmed8,
}
impl IDLPD_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IDLPD"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:788:1, end:788:2))"]
#[derive(Clone, Debug)]
struct LCALL_instructionVar49 {
    jmpdest16: Tablejmpdest16,
}
impl LCALL_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LCALL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:810:1, end:810:2))"]
#[derive(Clone, Debug)]
struct LJMP_instructionVar50 {
    jmpdest16: Tablejmpdest16,
}
impl LJMP_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LJMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:878:1, end:878:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar51 {
    wreg: Tablewreg,
}
impl NEG_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEG"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:885:1, end:885:2))"]
#[derive(Clone, Debug)]
struct NEGB_instructionVar52 {
    breg: Tablebreg,
}
impl NEGB_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEGB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:892:1, end:892:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar53 {}
impl NOP_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NOP"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:894:1, end:894:2))"]
#[derive(Clone, Debug)]
struct NORML_instructionVar54 {
    lreg: u8,
    breg: Tablebreg,
}
impl NORML_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NORML"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:905:1, end:905:2))"]
#[derive(Clone, Debug)]
struct NOT_instructionVar55 {
    wreg: Tablewreg,
}
impl NOT_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NOT"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:910:1, end:910:2))"]
#[derive(Clone, Debug)]
struct NOTB_instructionVar56 {
    breg: Tablebreg,
}
impl NOTB_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NOTB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:938:1, end:938:2))"]
#[derive(Clone, Debug)]
struct POPA_instructionVar57 {}
impl POPA_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("POPA"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:952:1, end:952:2))"]
#[derive(Clone, Debug)]
struct POPF_instructionVar58 {}
impl POPF_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("POPF"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:966:1, end:966:2))"]
#[derive(Clone, Debug)]
struct PUSHA_instructionVar59 {}
impl PUSHA_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PUSHA"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:976:1, end:976:2))"]
#[derive(Clone, Debug)]
struct PUSHF_instructionVar60 {}
impl PUSHF_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PUSHF"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:982:1, end:982:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar61 {}
impl RET_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RET"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:988:1, end:988:2))"]
#[derive(Clone, Debug)]
struct RST_instructionVar62 {}
impl RST_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RST"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1000:1, end:1000:2))"]
#[derive(Clone, Debug)]
struct SETC_instructionVar63 {}
impl SETC_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SETC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1219:1, end:1219:2))"]
#[derive(Clone, Debug)]
struct SKIP_instructionVar64 {}
impl SKIP_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SKIP"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1273:1, end:1273:2))"]
#[derive(Clone, Debug)]
struct TIJMP_instructionVar65 {
    dwreg: u8,
    wreg: Tablewreg,
    immed8: Tableimmed8,
}
impl TIJMP_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TIJMP"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dwreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("]"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let dwreg = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                wreg,
                immed8,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1285:1, end:1285:2))"]
#[derive(Clone, Debug)]
struct TRAP_instructionVar66 {}
impl TRAP_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TRAP"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1302:1, end:1302:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar67 {
    waop16: Tablewaop16,
    wreg: Tablewreg,
}
impl XCH_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCH"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.waop16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let waop16 = if let Some((len, table)) =
            Tablewaop16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1331:1, end:1331:2))"]
#[derive(Clone, Debug)]
struct XCHB_instructionVar68 {
    baop8: Tablebaop8,
    breg: Tablebreg,
}
impl XCHB_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCHB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.baop8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let baop8 = if let Some((len, table)) =
            Tablebaop8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:502:1, end:502:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar69 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl ADD_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:509:1, end:509:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar70 {
    dwreg: u8,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl ADD_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dwreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dwreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:516:1, end:516:2))"]
#[derive(Clone, Debug)]
struct ADDB_instructionVar71 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ADDB_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:522:1, end:522:2))"]
#[derive(Clone, Debug)]
struct ADDB_instructionVar72 {
    dbreg: u8,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ADDB_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:528:1, end:528:2))"]
#[derive(Clone, Debug)]
struct ADDC_instructionVar73 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl ADDC_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:537:1, end:537:2))"]
#[derive(Clone, Debug)]
struct ADDCB_instructionVar74 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ADDCB_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDCB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:547:1, end:547:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar75 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl AND_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:553:1, end:553:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar76 {
    dwreg: u8,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl AND_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dwreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dwreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:558:1, end:558:2))"]
#[derive(Clone, Debug)]
struct ANDB_instructionVar77 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ANDB_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:563:1, end:563:2))"]
#[derive(Clone, Debug)]
struct ANDB_instructionVar78 {
    dbreg: u8,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ANDB_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:604:1, end:604:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar79 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl CMP_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:610:1, end:610:2))"]
#[derive(Clone, Debug)]
struct CMPB_instructionVar80 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl CMPB_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMPB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:669:1, end:669:2))"]
#[derive(Clone, Debug)]
struct DIVU_instructionVar81 {
    lreg: u8,
    oper16: Tableoper16,
}
impl DIVU_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DIVU"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:679:1, end:679:2))"]
#[derive(Clone, Debug)]
struct DIVUB_instructionVar82 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl DIVUB_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DIVUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:794:1, end:794:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar83 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl LD_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:798:1, end:798:2))"]
#[derive(Clone, Debug)]
struct LDB_instructionVar84 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl LDB_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:802:1, end:802:2))"]
#[derive(Clone, Debug)]
struct LDBSE_instructionVar85 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl LDBSE_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDBSE"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:806:1, end:806:2))"]
#[derive(Clone, Debug)]
struct LDBZE_instructionVar86 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl LDBZE_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDBZE"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:847:1, end:847:2))"]
#[derive(Clone, Debug)]
struct MULU_instructionVar87 {
    lreg: u8,
    oper16: Tableoper16,
}
impl MULU_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULU"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:855:1, end:855:2))"]
#[derive(Clone, Debug)]
struct MULU_instructionVar88 {
    lreg: u8,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl MULU_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULU"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg, lreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:863:1, end:863:2))"]
#[derive(Clone, Debug)]
struct MULUB_instructionVar89 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl MULUB_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:871:1, end:871:2))"]
#[derive(Clone, Debug)]
struct MULUB_instructionVar90 {
    oper8: Tableoper8,
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl MULUB_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MULUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:915:1, end:915:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar91 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl OR_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:923:1, end:923:2))"]
#[derive(Clone, Debug)]
struct ORB_instructionVar92 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl ORB_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ORB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:931:1, end:931:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar93 {
    oper16: Tableoper16,
}
impl POP_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("POP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:960:1, end:960:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar94 {
    oper16: Tableoper16,
}
impl PUSH_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PUSH"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1224:1, end:1224:2))"]
#[derive(Clone, Debug)]
struct ST_instructionVar95 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl ST_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ST"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1228:1, end:1228:2))"]
#[derive(Clone, Debug)]
struct STB_instructionVar96 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl STB_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1233:1, end:1233:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar97 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl SUB_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1240:1, end:1240:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar98 {
    dwreg: u8,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl SUB_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dwreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dwreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1246:1, end:1246:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar99 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl SUBB_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1251:1, end:1251:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar100 {
    dbreg: u8,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl SUBB_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.dbreg),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let dbreg = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1256:1, end:1256:2))"]
#[derive(Clone, Debug)]
struct SUBC_instructionVar101 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl SUBC_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1264:1, end:1264:2))"]
#[derive(Clone, Debug)]
struct SUBCB_instructionVar102 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl SUBCB_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBCB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1293:1, end:1293:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar103 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl XOR_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper16 = if let Some((len, table)) =
            Tableoper16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1359:1, end:1359:2))"]
#[derive(Clone, Debug)]
struct XORB_instructionVar104 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl XORB_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:757:1, end:757:2))"]
#[derive(Clone, Debug)]
struct JBC_instructionVar105 {
    bitno: u8,
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl JBC_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JBC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bitno as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let bitno = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let jmpdest = if let Some((len, table)) =
            Tablejmpdest::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                breg,
                jmpdest,
                bitno,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:762:1, end:762:2))"]
#[derive(Clone, Debug)]
struct JBS_instructionVar106 {
    bitno: u8,
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl JBS_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JBS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bitno as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let bitno = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let jmpdest = if let Some((len, table)) =
            Tablejmpdest::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                breg,
                jmpdest,
                bitno,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:994:1, end:994:2))"]
#[derive(Clone, Debug)]
struct SCALL_instructionVar107 {
    jmpdest11: Tablejmpdest11,
}
impl SCALL_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SCALL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest11
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let jmpdest11 = if let Some((len, table)) =
            Tablejmpdest11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:1215:1, end:1215:2))"]
#[derive(Clone, Debug)]
struct SJMP_instructionVar108 {
    jmpdest11: Tablejmpdest11,
}
impl SJMP_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SJMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest11
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let jmpdest11 = if let Some((len, table)) =
            Tablejmpdest11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:784:1, end:784:2))"]
#[derive(Clone, Debug)]
struct J_instructionVar109 {
    cc: Tablecc,
    jmpdest: Tablejmpdest,
}
impl J_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("J"));
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let cc = if let Some((len, table)) =
            Tablecc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let jmpdest = if let Some((len, table)) =
            Tablejmpdest::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc, jmpdest }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(DIV_instructionVar0),
    Var1(DIVB_instructionVar1),
    Var2(MUL_instructionVar2),
    Var3(MUL_instructionVar3),
    Var4(MULB_instructionVar4),
    Var5(MULB_instructionVar5),
    Var6(SHL_instructionVar6),
    Var7(SHL_instructionVar7),
    Var8(SHLB_instructionVar8),
    Var9(SHLB_instructionVar9),
    Var10(SHLL_instructionVar10),
    Var11(SHLL_instructionVar11),
    Var12(SHR_instructionVar12),
    Var13(SHR_instructionVar13),
    Var14(SHRA_instructionVar14),
    Var15(SHRA_instructionVar15),
    Var16(SHRAB_instructionVar16),
    Var17(SHRAB_instructionVar17),
    Var18(SHRAL_instructionVar18),
    Var19(SHRAL_instructionVar19),
    Var20(SHRB_instructionVar20),
    Var21(SHRB_instructionVar21),
    Var22(SHRL_instructionVar22),
    Var23(SHRL_instructionVar23),
    Var24(XCH_instructionVar24),
    Var25(XCH_instructionVar25),
    Var26(XCHB_instructionVar26),
    Var27(XCHB_instructionVar27),
    Var28(BMOV_instructionVar28),
    Var29(BMOVI_instructionVar29),
    Var30(BR_instructionVar30),
    Var31(CLR_instructionVar31),
    Var32(CLRB_instructionVar32),
    Var33(CLRC_instructionVar33),
    Var34(CLRVT_instructionVar34),
    Var35(CMPL_instructionVar35),
    Var36(DEC_instructionVar36),
    Var37(DECB_instructionVar37),
    Var38(DI_instructionVar38),
    Var39(DJNZ_instructionVar39),
    Var40(DJNZW_instructionVar40),
    Var41(DPTS_instructionVar41),
    Var42(EI_instructionVar42),
    Var43(EPTS_instructionVar43),
    Var44(EXT_instructionVar44),
    Var45(EXTB_instructionVar45),
    Var46(INC_instructionVar46),
    Var47(INCB_instructionVar47),
    Var48(IDLPD_instructionVar48),
    Var49(LCALL_instructionVar49),
    Var50(LJMP_instructionVar50),
    Var51(NEG_instructionVar51),
    Var52(NEGB_instructionVar52),
    Var53(NOP_instructionVar53),
    Var54(NORML_instructionVar54),
    Var55(NOT_instructionVar55),
    Var56(NOTB_instructionVar56),
    Var57(POPA_instructionVar57),
    Var58(POPF_instructionVar58),
    Var59(PUSHA_instructionVar59),
    Var60(PUSHF_instructionVar60),
    Var61(RET_instructionVar61),
    Var62(RST_instructionVar62),
    Var63(SETC_instructionVar63),
    Var64(SKIP_instructionVar64),
    Var65(TIJMP_instructionVar65),
    Var66(TRAP_instructionVar66),
    Var67(XCH_instructionVar67),
    Var68(XCHB_instructionVar68),
    Var69(ADD_instructionVar69),
    Var70(ADD_instructionVar70),
    Var71(ADDB_instructionVar71),
    Var72(ADDB_instructionVar72),
    Var73(ADDC_instructionVar73),
    Var74(ADDCB_instructionVar74),
    Var75(AND_instructionVar75),
    Var76(AND_instructionVar76),
    Var77(ANDB_instructionVar77),
    Var78(ANDB_instructionVar78),
    Var79(CMP_instructionVar79),
    Var80(CMPB_instructionVar80),
    Var81(DIVU_instructionVar81),
    Var82(DIVUB_instructionVar82),
    Var83(LD_instructionVar83),
    Var84(LDB_instructionVar84),
    Var85(LDBSE_instructionVar85),
    Var86(LDBZE_instructionVar86),
    Var87(MULU_instructionVar87),
    Var88(MULU_instructionVar88),
    Var89(MULUB_instructionVar89),
    Var90(MULUB_instructionVar90),
    Var91(OR_instructionVar91),
    Var92(ORB_instructionVar92),
    Var93(POP_instructionVar93),
    Var94(PUSH_instructionVar94),
    Var95(ST_instructionVar95),
    Var96(STB_instructionVar96),
    Var97(SUB_instructionVar97),
    Var98(SUB_instructionVar98),
    Var99(SUBB_instructionVar99),
    Var100(SUBB_instructionVar100),
    Var101(SUBC_instructionVar101),
    Var102(SUBCB_instructionVar102),
    Var103(XOR_instructionVar103),
    Var104(XORB_instructionVar104),
    Var105(JBC_instructionVar105),
    Var106(JBS_instructionVar106),
    Var107(SCALL_instructionVar107),
    Var108(SJMP_instructionVar108),
    Var109(J_instructionVar109),
}
impl Tableinstruction {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var3(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var4(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var5(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var6(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var7(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var8(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var9(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var10(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var11(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var12(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var13(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var14(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var15(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var16(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var17(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var18(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var19(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var20(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var21(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var22(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var23(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var24(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var25(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var26(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var27(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var28(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var29(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var30(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var31(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var32(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var33(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var34(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var35(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var36(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var37(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var38(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var39(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var40(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var41(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var42(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var43(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var44(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var45(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var46(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var47(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var48(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var49(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var50(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var51(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var52(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var53(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var54(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var55(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var56(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var57(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var58(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var59(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var60(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var61(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var62(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var63(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var64(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var65(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var66(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var67(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var68(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var69(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var70(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var71(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var72(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var73(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var74(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var75(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var76(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var77(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var78(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var79(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var80(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var81(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var82(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var83(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var84(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var85(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var86(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var87(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var88(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var89(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var90(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var91(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var92(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var93(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var94(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var95(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var96(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var97(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var98(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var99(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var100(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var101(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var102(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var103(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var104(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var105(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var106(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var107(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var108(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var109(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 140
        {
            if let Some((inst_len, parsed)) =
                DIV_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 156
        {
            if let Some((inst_len, parsed)) =
                DIVB_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 108
        {
            if let Some((inst_len, parsed)) =
                MUL_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 76
        {
            if let Some((inst_len, parsed)) =
                MUL_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 124
        {
            if let Some((inst_len, parsed)) =
                MULB_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && (tokens_param[0] & 255) == 254
            && (tokens_param[1] & 252) == 92
        {
            if let Some((inst_len, parsed)) =
                MULB_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 9 && (tokens_param[1] & 240) == 0 {
            if let Some((inst_len, parsed)) =
                SHL_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 9 {
            if let Some((inst_len, parsed)) =
                SHL_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 25 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHLB_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 25 {
            if let Some((inst_len, parsed)) =
                SHLB_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 13 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHLL_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 13 {
            if let Some((inst_len, parsed)) =
                SHLL_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 8 && (tokens_param[1] & 240) == 0 {
            if let Some((inst_len, parsed)) =
                SHR_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 8 {
            if let Some((inst_len, parsed)) =
                SHR_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 10 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHRA_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 10 {
            if let Some((inst_len, parsed)) =
                SHRA_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 26 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHRAB_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 26 {
            if let Some((inst_len, parsed)) =
                SHRAB_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 14 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHRAL_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 14 {
            if let Some((inst_len, parsed)) =
                SHRAL_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 24 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHRB_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 24 {
            if let Some((inst_len, parsed)) =
                SHRB_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 12 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                SHRL_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 12 {
            if let Some((inst_len, parsed)) =
                SHRL_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 11 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                XCH_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 5 && (tokens_param[0] & 255) == 11 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                XCH_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 27 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                XCHB_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 5 && (tokens_param[0] & 255) == 27 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                XCHB_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 193 {
            if let Some((inst_len, parsed)) =
                BMOV_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                BMOVI_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 227 {
            if let Some((inst_len, parsed)) =
                BR_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                CLR_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 17 {
            if let Some((inst_len, parsed)) =
                CLRB_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 248 {
            if let Some((inst_len, parsed)) =
                CLRC_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 252 {
            if let Some((inst_len, parsed)) =
                CLRVT_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 197 {
            if let Some((inst_len, parsed)) =
                CMPL_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 5 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 21 {
            if let Some((inst_len, parsed)) =
                DECB_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 250 {
            if let Some((inst_len, parsed)) =
                DI_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 224 {
            if let Some((inst_len, parsed)) =
                DJNZ_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 225 {
            if let Some((inst_len, parsed)) =
                DJNZW_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 236 {
            if let Some((inst_len, parsed)) =
                DPTS_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 251 {
            if let Some((inst_len, parsed)) =
                EI_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 237 {
            if let Some((inst_len, parsed)) =
                EPTS_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 6 {
            if let Some((inst_len, parsed)) =
                EXT_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 22 {
            if let Some((inst_len, parsed)) =
                EXTB_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 23 {
            if let Some((inst_len, parsed)) =
                INCB_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 246 {
            if let Some((inst_len, parsed)) =
                IDLPD_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 239 {
            if let Some((inst_len, parsed)) =
                LCALL_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 231 {
            if let Some((inst_len, parsed)) =
                LJMP_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 3 {
            if let Some((inst_len, parsed)) =
                NEG_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 19 {
            if let Some((inst_len, parsed)) =
                NEGB_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 253 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 15 {
            if let Some((inst_len, parsed)) =
                NORML_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                NOT_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 18 {
            if let Some((inst_len, parsed)) =
                NOTB_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 245 {
            if let Some((inst_len, parsed)) =
                POPA_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 243 {
            if let Some((inst_len, parsed)) =
                POPF_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 244 {
            if let Some((inst_len, parsed)) =
                PUSHA_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 242 {
            if let Some((inst_len, parsed)) =
                PUSHF_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 240 {
            if let Some((inst_len, parsed)) =
                RET_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 255 {
            if let Some((inst_len, parsed)) =
                RST_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 249 {
            if let Some((inst_len, parsed)) =
                SETC_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                SKIP_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 226 {
            if let Some((inst_len, parsed)) =
                TIJMP_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 247 {
            if let Some((inst_len, parsed)) =
                TRAP_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 4 {
            if let Some((inst_len, parsed)) =
                XCH_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 20 {
            if let Some((inst_len, parsed)) =
                XCHB_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 100 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 68 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 116 {
            if let Some((inst_len, parsed)) =
                ADDB_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 84 {
            if let Some((inst_len, parsed)) =
                ADDB_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 164 {
            if let Some((inst_len, parsed)) =
                ADDC_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 180 {
            if let Some((inst_len, parsed)) =
                ADDCB_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 96 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 64 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 112 {
            if let Some((inst_len, parsed)) =
                ANDB_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 80 {
            if let Some((inst_len, parsed)) =
                ANDB_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 136 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 152 {
            if let Some((inst_len, parsed)) =
                CMPB_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 140 {
            if let Some((inst_len, parsed)) =
                DIVU_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 156 {
            if let Some((inst_len, parsed)) =
                DIVUB_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 160 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 176 {
            if let Some((inst_len, parsed)) =
                LDB_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 188 {
            if let Some((inst_len, parsed)) =
                LDBSE_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 172 {
            if let Some((inst_len, parsed)) =
                LDBZE_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 108 {
            if let Some((inst_len, parsed)) =
                MULU_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 76 {
            if let Some((inst_len, parsed)) =
                MULU_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 124 {
            if let Some((inst_len, parsed)) =
                MULUB_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 92 {
            if let Some((inst_len, parsed)) =
                MULUB_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 128 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 144 {
            if let Some((inst_len, parsed)) =
                ORB_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 252) == 204 {
            if let Some((inst_len, parsed)) =
                POP_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 252) == 200 {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 192 {
            if let Some((inst_len, parsed)) =
                ST_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 196 {
            if let Some((inst_len, parsed)) =
                STB_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 104 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 72 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 120 {
            if let Some((inst_len, parsed)) =
                SUBB_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 252) == 88 {
            if let Some((inst_len, parsed)) =
                SUBB_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 168 {
            if let Some((inst_len, parsed)) =
                SUBC_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 184 {
            if let Some((inst_len, parsed)) =
                SUBCB_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 132 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 252) == 148 {
            if let Some((inst_len, parsed)) =
                XORB_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 248) == 48 {
            if let Some((inst_len, parsed)) =
                JBC_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 248) == 56 {
            if let Some((inst_len, parsed)) =
                JBS_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 248) == 40 {
            if let Some((inst_len, parsed)) =
                SCALL_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 248) == 32 {
            if let Some((inst_len, parsed)) =
                SJMP_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 208 {
            if let Some((inst_len, parsed)) =
                J_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:317:1, end:317:7))"]
#[derive(Clone, Debug)]
struct immed8Var0 {
    imm8: u8,
}
impl immed8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Number(
            true,
            (if self.imm8 & 128 != 0 { -1 & !127 } else { 0 } | self.imm8 as i8).is_negative(),
            (if self.imm8 & 128 != 0 { -1 & !127 } else { 0 } | self.imm8 as i8).abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimmed8 {
    Var0(immed8Var0),
}
impl Tableimmed8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                immed8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:319:1, end:319:8))"]
#[derive(Clone, Debug)]
struct simmed8Var0 {
    simm8: u8,
}
impl simmed8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Number(
            true,
            (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).is_negative(),
            (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablesimmed8 {
    Var0(simmed8Var0),
}
impl Tablesimmed8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                simmed8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:321:1, end:321:8))"]
#[derive(Clone, Debug)]
struct immed16Var0 {
    imm16: u16,
}
impl immed16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.imm16 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let imm16 = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimmed16 {
    Var0(immed16Var0),
}
impl Tableimmed16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                immed16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:323:1, end:323:6))"]
#[derive(Clone, Debug)]
struct baop8Var0 {
    baop: u8,
}
impl baop8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.baop)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let baop = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:324:1, end:324:6))"]
#[derive(Clone, Debug)]
struct baop8Var1 {
    baop: u8,
}
impl baop8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.baop)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let baop = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:325:1, end:325:6))"]
#[derive(Clone, Debug)]
struct baop8Var2 {
    baop: u8,
}
impl baop8Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.baop)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let baop = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[derive(Clone, Debug)]
enum Tablebaop8 {
    Var0(baop8Var0),
    Var1(baop8Var1),
    Var2(baop8Var2),
}
impl Tablebaop8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                baop8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                baop8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                baop8Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:327:1, end:327:7))"]
#[derive(Clone, Debug)]
struct waop16Var0 {
    waop: u8,
}
impl waop16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.waop)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let waop = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:328:1, end:328:7))"]
#[derive(Clone, Debug)]
struct waop16Var1 {
    waop: u8,
}
impl waop16Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.waop)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let waop = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop }))
    }
}
#[derive(Clone, Debug)]
enum Tablewaop16 {
    Var0(waop16Var0),
    Var1(waop16Var1),
}
impl Tablewaop16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                waop16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                waop16Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:330:1, end:330:6))"]
#[derive(Clone, Debug)]
struct iwregVar0 {
    iwreg7: u8,
}
impl iwregVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_2_display(self.iwreg7)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let iwreg7 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg7 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:331:1, end:331:6))"]
#[derive(Clone, Debug)]
struct iwregVar1 {
    iwreg7: u8,
}
impl iwregVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_2_display(self.iwreg7)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let iwreg7 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg7 }))
    }
}
#[derive(Clone, Debug)]
enum Tableiwreg {
    Var0(iwregVar0),
    Var1(iwregVar1),
}
impl Tableiwreg {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 254) == 0 {
            if let Some((inst_len, parsed)) =
                iwregVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                iwregVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:333:1, end:333:5))"]
#[derive(Clone, Debug)]
struct bregVar0 {
    breg8: u8,
}
impl bregVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.breg8)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let breg8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:334:1, end:334:5))"]
#[derive(Clone, Debug)]
struct bregVar1 {
    breg8: u8,
}
impl bregVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.breg8)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let breg8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:335:1, end:335:5))"]
#[derive(Clone, Debug)]
struct bregVar2 {
    breg8: u8,
}
impl bregVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self.breg8)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let breg8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebreg {
    Var0(bregVar0),
    Var1(bregVar1),
    Var2(bregVar2),
}
impl Tablebreg {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                bregVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                bregVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                bregVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:337:1, end:337:5))"]
#[derive(Clone, Debug)]
struct wregVar0 {
    wreg8: u8,
}
impl wregVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.wreg8)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let wreg8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:338:1, end:338:5))"]
#[derive(Clone, Debug)]
struct wregVar1 {
    wreg8: u8,
}
impl wregVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.wreg8)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let wreg8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablewreg {
    Var0(wregVar0),
    Var1(wregVar1),
}
impl Tablewreg {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                wregVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                wregVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:344:1, end:344:6))"]
#[derive(Clone, Debug)]
struct oper8Var0 {
    iwreg: Tableiwreg,
}
impl oper8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:348:1, end:348:6))"]
#[derive(Clone, Debug)]
struct oper8Var1 {
    iwreg: Tableiwreg,
}
impl oper8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]+")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:353:1, end:353:6))"]
#[derive(Clone, Debug)]
struct oper8Var2 {
    iwreg: Tableiwreg,
    simmed8: Tablesimmed8,
}
impl oper8Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.simmed8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let simmed8 = if let Some((len, table)) =
            Tablesimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, simmed8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:358:1, end:358:6))"]
#[derive(Clone, Debug)]
struct oper8Var3 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
}
impl oper8Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.immed16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", LOOKUP[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let immed16 = if let Some((len, table)) =
            Tableimmed16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, immed16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:342:1, end:342:6))"]
#[derive(Clone, Debug)]
struct oper8Var4 {
    baop8: Tablebaop8,
}
impl oper8Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.baop8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let baop8 = if let Some((len, table)) =
            Tablebaop8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:343:1, end:343:6))"]
#[derive(Clone, Debug)]
struct oper8Var5 {
    immed8: Tableimmed8,
}
impl oper8Var5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let immed8 = if let Some((len, table)) =
            Tableimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoper8 {
    Var0(oper8Var0),
    Var1(oper8Var1),
    Var2(oper8Var2),
    Var3(oper8Var3),
    Var4(oper8Var4),
    Var5(oper8Var5),
}
impl Tableoper8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var3(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var4(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var5(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                oper8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                oper8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                oper8Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                oper8Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 {
            if let Some((inst_len, parsed)) =
                oper8Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 {
            if let Some((inst_len, parsed)) =
                oper8Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:367:1, end:367:7))"]
#[derive(Clone, Debug)]
struct oper16Var0 {
    iwreg: Tableiwreg,
}
impl oper16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:371:1, end:371:7))"]
#[derive(Clone, Debug)]
struct oper16Var1 {
    iwreg: Tableiwreg,
}
impl oper16Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]+")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:376:1, end:376:7))"]
#[derive(Clone, Debug)]
struct oper16Var2 {
    iwreg: Tableiwreg,
    simmed8: Tablesimmed8,
}
impl oper16Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.simmed8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let simmed8 = if let Some((len, table)) =
            Tablesimmed8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, simmed8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:381:1, end:381:7))"]
#[derive(Clone, Debug)]
struct oper16Var3 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
}
impl oper16Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.immed16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let immed16 = if let Some((len, table)) =
            Tableimmed16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, immed16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:365:1, end:365:7))"]
#[derive(Clone, Debug)]
struct oper16Var4 {
    waop16: Tablewaop16,
}
impl oper16Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.waop16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let waop16 = if let Some((len, table)) =
            Tablewaop16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:366:1, end:366:7))"]
#[derive(Clone, Debug)]
struct oper16Var5 {
    immed16: Tableimmed16,
}
impl oper16Var5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let immed16 = if let Some((len, table)) =
            Tableimmed16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed16 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoper16 {
    Var0(oper16Var0),
    Var1(oper16Var1),
    Var2(oper16Var2),
    Var3(oper16Var3),
    Var4(oper16Var4),
    Var5(oper16Var5),
}
impl Tableoper16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var3(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var4(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var5(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                oper16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                oper16Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                oper16Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                oper16Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 {
            if let Some((inst_len, parsed)) =
                oper16Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 3) == 1 {
            if let Some((inst_len, parsed)) =
                oper16Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:388:1, end:388:8))"]
#[derive(Clone, Debug)]
struct jmpdestVar0 {
    simm8: u8,
}
impl jmpdestVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_next).unwrap().wrapping_add(
            i128::try_from((if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8))
                .unwrap(),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest {
    Var0(jmpdestVar0),
}
impl Tablejmpdest {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                jmpdestVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:391:1, end:391:10))"]
#[derive(Clone, Debug)]
struct jmpdest11Var0 {
    jmp11_hi: u8,
    jmp11_lo: u8,
}
impl jmpdest11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_next).unwrap().wrapping_add(
            (u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(
                        (if self.jmp11_hi & 4 != 0 { -1 & !3 } else { 0 } | self.jmp11_hi as i8),
                    )
                    .unwrap()
                    .checked_shl(shl)
                })
                .unwrap_or(0)
                | i128::try_from(self.jmp11_lo).unwrap()),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2;
        let jmp11_hi = token_12(tokens_current);
        let jmp11_lo = token_13(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmp11_hi, jmp11_lo }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest11 {
    Var0(jmpdest11Var0),
}
impl Tablejmpdest11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                jmpdest11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:392:1, end:392:10))"]
#[derive(Clone, Debug)]
struct jmpdest16Var0 {
    disp16: u16,
}
impl jmpdest16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_next).unwrap().wrapping_add(
            i128::try_from(
                (if self.disp16 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.disp16 as i16),
            )
            .unwrap(),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2;
        let disp16 = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp16 }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest16 {
    Var0(jmpdest16Var0),
}
impl Tablejmpdest16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                jmpdest16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:767:1, end:767:3))"]
#[derive(Clone, Debug)]
struct ccVar0 {}
impl ccVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NST")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:768:1, end:768:3))"]
#[derive(Clone, Debug)]
struct ccVar1 {}
impl ccVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NH")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:769:1, end:769:3))"]
#[derive(Clone, Debug)]
struct ccVar2 {}
impl ccVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("GT")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:770:1, end:770:3))"]
#[derive(Clone, Debug)]
struct ccVar3 {}
impl ccVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NC")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:771:1, end:771:3))"]
#[derive(Clone, Debug)]
struct ccVar4 {}
impl ccVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NVT")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:772:1, end:772:3))"]
#[derive(Clone, Debug)]
struct ccVar5 {}
impl ccVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NV")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:773:1, end:773:3))"]
#[derive(Clone, Debug)]
struct ccVar6 {}
impl ccVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("GE")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:774:1, end:774:3))"]
#[derive(Clone, Debug)]
struct ccVar7 {}
impl ccVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NE")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:775:1, end:775:3))"]
#[derive(Clone, Debug)]
struct ccVar8 {}
impl ccVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ST")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:776:1, end:776:3))"]
#[derive(Clone, Debug)]
struct ccVar9 {}
impl ccVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("H")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:777:1, end:777:3))"]
#[derive(Clone, Debug)]
struct ccVar10 {}
impl ccVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("LE")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:778:1, end:778:3))"]
#[derive(Clone, Debug)]
struct ccVar11 {}
impl ccVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("C")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:779:1, end:779:3))"]
#[derive(Clone, Debug)]
struct ccVar12 {}
impl ccVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("VT")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:780:1, end:780:3))"]
#[derive(Clone, Debug)]
struct ccVar13 {}
impl ccVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("V")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:781:1, end:781:3))"]
#[derive(Clone, Debug)]
struct ccVar14 {}
impl ccVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("LT")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc, start:782:1, end:782:3))"]
#[derive(Clone, Debug)]
struct ccVar15 {}
impl ccVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("E")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecc {
    Var0(ccVar0),
    Var1(ccVar1),
    Var2(ccVar2),
    Var3(ccVar3),
    Var4(ccVar4),
    Var5(ccVar5),
    Var6(ccVar6),
    Var7(ccVar7),
    Var8(ccVar8),
    Var9(ccVar9),
    Var10(ccVar10),
    Var11(ccVar11),
    Var12(ccVar12),
    Var13(ccVar13),
    Var14(ccVar14),
    Var15(ccVar15),
}
impl Tablecc {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var3(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var4(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var5(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var6(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var7(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var8(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var9(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var10(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var11(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var12(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var13(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var14(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var15(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                ccVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                ccVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                ccVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                ccVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                ccVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                ccVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                ccVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                ccVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                ccVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                ccVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                ccVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                ccVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                ccVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                ccVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                ccVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                ccVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        None
    }
}
pub fn parse_instruction(
    tokens: &[u8],
    context: &mut ContextMemory,
    inst_start: AddrType,
    global_set: &mut GlobalSet,
) -> Option<(u16, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
