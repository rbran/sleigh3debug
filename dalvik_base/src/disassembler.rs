pub type AddrType = u32;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    sp,
    fp,
    resultreg,
    resultregw,
    iv0,
    iv1,
    iv2,
    iv3,
    iv4,
    iv5,
    iv6,
    iv7,
    iv8,
    iv9,
    iv10,
    iv11,
    iv12,
    iv13,
    iv14,
    iv15,
    ivw1,
    ivw3,
    ivw5,
    ivw7,
    ivw9,
    ivw11,
    ivw13,
    ivw0,
    ivw2,
    ivw4,
    ivw6,
    ivw8,
    ivw10,
    ivw12,
    ivw14,
    v0,
    v1,
    v2,
    v3,
    v4,
    v5,
    v6,
    v7,
    v8,
    v9,
    v10,
    v11,
    v12,
    v13,
    v14,
    v15,
    v16,
    v17,
    v18,
    v19,
    v20,
    v21,
    v22,
    v23,
    v24,
    v25,
    v26,
    v27,
    v28,
    v29,
    v30,
    v31,
    v32,
    v33,
    v34,
    v35,
    v36,
    v37,
    v38,
    v39,
    v40,
    v41,
    v42,
    v43,
    v44,
    v45,
    v46,
    v47,
    v48,
    v49,
    v50,
    v51,
    v52,
    v53,
    v54,
    v55,
    v56,
    v57,
    v58,
    v59,
    v60,
    v61,
    v62,
    v63,
    v64,
    v65,
    v66,
    v67,
    v68,
    v69,
    v70,
    v71,
    v72,
    v73,
    v74,
    v75,
    v76,
    v77,
    v78,
    v79,
    v80,
    v81,
    v82,
    v83,
    v84,
    v85,
    v86,
    v87,
    v88,
    v89,
    v90,
    v91,
    v92,
    v93,
    v94,
    v95,
    v96,
    v97,
    v98,
    v99,
    v100,
    v101,
    v102,
    v103,
    v104,
    v105,
    v106,
    v107,
    v108,
    v109,
    v110,
    v111,
    v112,
    v113,
    v114,
    v115,
    v116,
    v117,
    v118,
    v119,
    v120,
    v121,
    v122,
    v123,
    v124,
    v125,
    v126,
    v127,
    v128,
    v129,
    v130,
    v131,
    v132,
    v133,
    v134,
    v135,
    v136,
    v137,
    v138,
    v139,
    v140,
    v141,
    v142,
    v143,
    v144,
    v145,
    v146,
    v147,
    v148,
    v149,
    v150,
    v151,
    v152,
    v153,
    v154,
    v155,
    v156,
    v157,
    v158,
    v159,
    v160,
    v161,
    v162,
    v163,
    v164,
    v165,
    v166,
    v167,
    v168,
    v169,
    v170,
    v171,
    v172,
    v173,
    v174,
    v175,
    v176,
    v177,
    v178,
    v179,
    v180,
    v181,
    v182,
    v183,
    v184,
    v185,
    v186,
    v187,
    v188,
    v189,
    v190,
    v191,
    v192,
    v193,
    v194,
    v195,
    v196,
    v197,
    v198,
    v199,
    v200,
    v201,
    v202,
    v203,
    v204,
    v205,
    v206,
    v207,
    v208,
    v209,
    v210,
    v211,
    v212,
    v213,
    v214,
    v215,
    v216,
    v217,
    v218,
    v219,
    v220,
    v221,
    v222,
    v223,
    v224,
    v225,
    v226,
    v227,
    v228,
    v229,
    v230,
    v231,
    v232,
    v233,
    v234,
    v235,
    v236,
    v237,
    v238,
    v239,
    v240,
    v241,
    v242,
    v243,
    v244,
    v245,
    v246,
    v247,
    v248,
    v249,
    v250,
    v251,
    v252,
    v253,
    v254,
    v255,
    vw1,
    vw3,
    vw5,
    vw7,
    vw9,
    vw11,
    vw13,
    vw15,
    vw17,
    vw19,
    vw21,
    vw23,
    vw25,
    vw27,
    vw29,
    vw31,
    vw33,
    vw35,
    vw37,
    vw39,
    vw41,
    vw43,
    vw45,
    vw47,
    vw49,
    vw51,
    vw53,
    vw55,
    vw57,
    vw59,
    vw61,
    vw63,
    vw65,
    vw67,
    vw69,
    vw71,
    vw73,
    vw75,
    vw77,
    vw79,
    vw81,
    vw83,
    vw85,
    vw87,
    vw89,
    vw91,
    vw93,
    vw95,
    vw97,
    vw99,
    vw101,
    vw103,
    vw105,
    vw107,
    vw109,
    vw111,
    vw113,
    vw115,
    vw117,
    vw119,
    vw121,
    vw123,
    vw125,
    vw127,
    vw129,
    vw131,
    vw133,
    vw135,
    vw137,
    vw139,
    vw141,
    vw143,
    vw145,
    vw147,
    vw149,
    vw151,
    vw153,
    vw155,
    vw157,
    vw159,
    vw161,
    vw163,
    vw165,
    vw167,
    vw169,
    vw171,
    vw173,
    vw175,
    vw177,
    vw179,
    vw181,
    vw183,
    vw185,
    vw187,
    vw189,
    vw191,
    vw193,
    vw195,
    vw197,
    vw199,
    vw201,
    vw203,
    vw205,
    vw207,
    vw209,
    vw211,
    vw213,
    vw215,
    vw217,
    vw219,
    vw221,
    vw223,
    vw225,
    vw227,
    vw229,
    vw231,
    vw233,
    vw235,
    vw237,
    vw239,
    vw241,
    vw243,
    vw245,
    vw247,
    vw249,
    vw251,
    vw253,
    vw0,
    vw2,
    vw4,
    vw6,
    vw8,
    vw10,
    vw12,
    vw14,
    vw16,
    vw18,
    vw20,
    vw22,
    vw24,
    vw26,
    vw28,
    vw30,
    vw32,
    vw34,
    vw36,
    vw38,
    vw40,
    vw42,
    vw44,
    vw46,
    vw48,
    vw50,
    vw52,
    vw54,
    vw56,
    vw58,
    vw60,
    vw62,
    vw64,
    vw66,
    vw68,
    vw70,
    vw72,
    vw74,
    vw76,
    vw78,
    vw80,
    vw82,
    vw84,
    vw86,
    vw88,
    vw90,
    vw92,
    vw94,
    vw96,
    vw98,
    vw100,
    vw102,
    vw104,
    vw106,
    vw108,
    vw110,
    vw112,
    vw114,
    vw116,
    vw118,
    vw120,
    vw122,
    vw124,
    vw126,
    vw128,
    vw130,
    vw132,
    vw134,
    vw136,
    vw138,
    vw140,
    vw142,
    vw144,
    vw146,
    vw148,
    vw150,
    vw152,
    vw154,
    vw156,
    vw158,
    vw160,
    vw162,
    vw164,
    vw166,
    vw168,
    vw170,
    vw172,
    vw174,
    vw176,
    vw178,
    vw180,
    vw182,
    vw184,
    vw186,
    vw188,
    vw190,
    vw192,
    vw194,
    vw196,
    vw198,
    vw200,
    vw202,
    vw204,
    vw206,
    vw208,
    vw210,
    vw212,
    vw214,
    vw216,
    vw218,
    vw220,
    vw222,
    vw224,
    vw226,
    vw228,
    vw230,
    vw232,
    vw234,
    vw236,
    vw238,
    vw240,
    vw242,
    vw244,
    vw246,
    vw248,
    vw250,
    vw252,
    vw254,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::sp => "sp",
            Self::fp => "fp",
            Self::resultreg => "resultreg",
            Self::resultregw => "resultregw",
            Self::iv0 => "iv0",
            Self::iv1 => "iv1",
            Self::iv2 => "iv2",
            Self::iv3 => "iv3",
            Self::iv4 => "iv4",
            Self::iv5 => "iv5",
            Self::iv6 => "iv6",
            Self::iv7 => "iv7",
            Self::iv8 => "iv8",
            Self::iv9 => "iv9",
            Self::iv10 => "iv10",
            Self::iv11 => "iv11",
            Self::iv12 => "iv12",
            Self::iv13 => "iv13",
            Self::iv14 => "iv14",
            Self::iv15 => "iv15",
            Self::ivw1 => "ivw1",
            Self::ivw3 => "ivw3",
            Self::ivw5 => "ivw5",
            Self::ivw7 => "ivw7",
            Self::ivw9 => "ivw9",
            Self::ivw11 => "ivw11",
            Self::ivw13 => "ivw13",
            Self::ivw0 => "ivw0",
            Self::ivw2 => "ivw2",
            Self::ivw4 => "ivw4",
            Self::ivw6 => "ivw6",
            Self::ivw8 => "ivw8",
            Self::ivw10 => "ivw10",
            Self::ivw12 => "ivw12",
            Self::ivw14 => "ivw14",
            Self::v0 => "v0",
            Self::v1 => "v1",
            Self::v2 => "v2",
            Self::v3 => "v3",
            Self::v4 => "v4",
            Self::v5 => "v5",
            Self::v6 => "v6",
            Self::v7 => "v7",
            Self::v8 => "v8",
            Self::v9 => "v9",
            Self::v10 => "v10",
            Self::v11 => "v11",
            Self::v12 => "v12",
            Self::v13 => "v13",
            Self::v14 => "v14",
            Self::v15 => "v15",
            Self::v16 => "v16",
            Self::v17 => "v17",
            Self::v18 => "v18",
            Self::v19 => "v19",
            Self::v20 => "v20",
            Self::v21 => "v21",
            Self::v22 => "v22",
            Self::v23 => "v23",
            Self::v24 => "v24",
            Self::v25 => "v25",
            Self::v26 => "v26",
            Self::v27 => "v27",
            Self::v28 => "v28",
            Self::v29 => "v29",
            Self::v30 => "v30",
            Self::v31 => "v31",
            Self::v32 => "v32",
            Self::v33 => "v33",
            Self::v34 => "v34",
            Self::v35 => "v35",
            Self::v36 => "v36",
            Self::v37 => "v37",
            Self::v38 => "v38",
            Self::v39 => "v39",
            Self::v40 => "v40",
            Self::v41 => "v41",
            Self::v42 => "v42",
            Self::v43 => "v43",
            Self::v44 => "v44",
            Self::v45 => "v45",
            Self::v46 => "v46",
            Self::v47 => "v47",
            Self::v48 => "v48",
            Self::v49 => "v49",
            Self::v50 => "v50",
            Self::v51 => "v51",
            Self::v52 => "v52",
            Self::v53 => "v53",
            Self::v54 => "v54",
            Self::v55 => "v55",
            Self::v56 => "v56",
            Self::v57 => "v57",
            Self::v58 => "v58",
            Self::v59 => "v59",
            Self::v60 => "v60",
            Self::v61 => "v61",
            Self::v62 => "v62",
            Self::v63 => "v63",
            Self::v64 => "v64",
            Self::v65 => "v65",
            Self::v66 => "v66",
            Self::v67 => "v67",
            Self::v68 => "v68",
            Self::v69 => "v69",
            Self::v70 => "v70",
            Self::v71 => "v71",
            Self::v72 => "v72",
            Self::v73 => "v73",
            Self::v74 => "v74",
            Self::v75 => "v75",
            Self::v76 => "v76",
            Self::v77 => "v77",
            Self::v78 => "v78",
            Self::v79 => "v79",
            Self::v80 => "v80",
            Self::v81 => "v81",
            Self::v82 => "v82",
            Self::v83 => "v83",
            Self::v84 => "v84",
            Self::v85 => "v85",
            Self::v86 => "v86",
            Self::v87 => "v87",
            Self::v88 => "v88",
            Self::v89 => "v89",
            Self::v90 => "v90",
            Self::v91 => "v91",
            Self::v92 => "v92",
            Self::v93 => "v93",
            Self::v94 => "v94",
            Self::v95 => "v95",
            Self::v96 => "v96",
            Self::v97 => "v97",
            Self::v98 => "v98",
            Self::v99 => "v99",
            Self::v100 => "v100",
            Self::v101 => "v101",
            Self::v102 => "v102",
            Self::v103 => "v103",
            Self::v104 => "v104",
            Self::v105 => "v105",
            Self::v106 => "v106",
            Self::v107 => "v107",
            Self::v108 => "v108",
            Self::v109 => "v109",
            Self::v110 => "v110",
            Self::v111 => "v111",
            Self::v112 => "v112",
            Self::v113 => "v113",
            Self::v114 => "v114",
            Self::v115 => "v115",
            Self::v116 => "v116",
            Self::v117 => "v117",
            Self::v118 => "v118",
            Self::v119 => "v119",
            Self::v120 => "v120",
            Self::v121 => "v121",
            Self::v122 => "v122",
            Self::v123 => "v123",
            Self::v124 => "v124",
            Self::v125 => "v125",
            Self::v126 => "v126",
            Self::v127 => "v127",
            Self::v128 => "v128",
            Self::v129 => "v129",
            Self::v130 => "v130",
            Self::v131 => "v131",
            Self::v132 => "v132",
            Self::v133 => "v133",
            Self::v134 => "v134",
            Self::v135 => "v135",
            Self::v136 => "v136",
            Self::v137 => "v137",
            Self::v138 => "v138",
            Self::v139 => "v139",
            Self::v140 => "v140",
            Self::v141 => "v141",
            Self::v142 => "v142",
            Self::v143 => "v143",
            Self::v144 => "v144",
            Self::v145 => "v145",
            Self::v146 => "v146",
            Self::v147 => "v147",
            Self::v148 => "v148",
            Self::v149 => "v149",
            Self::v150 => "v150",
            Self::v151 => "v151",
            Self::v152 => "v152",
            Self::v153 => "v153",
            Self::v154 => "v154",
            Self::v155 => "v155",
            Self::v156 => "v156",
            Self::v157 => "v157",
            Self::v158 => "v158",
            Self::v159 => "v159",
            Self::v160 => "v160",
            Self::v161 => "v161",
            Self::v162 => "v162",
            Self::v163 => "v163",
            Self::v164 => "v164",
            Self::v165 => "v165",
            Self::v166 => "v166",
            Self::v167 => "v167",
            Self::v168 => "v168",
            Self::v169 => "v169",
            Self::v170 => "v170",
            Self::v171 => "v171",
            Self::v172 => "v172",
            Self::v173 => "v173",
            Self::v174 => "v174",
            Self::v175 => "v175",
            Self::v176 => "v176",
            Self::v177 => "v177",
            Self::v178 => "v178",
            Self::v179 => "v179",
            Self::v180 => "v180",
            Self::v181 => "v181",
            Self::v182 => "v182",
            Self::v183 => "v183",
            Self::v184 => "v184",
            Self::v185 => "v185",
            Self::v186 => "v186",
            Self::v187 => "v187",
            Self::v188 => "v188",
            Self::v189 => "v189",
            Self::v190 => "v190",
            Self::v191 => "v191",
            Self::v192 => "v192",
            Self::v193 => "v193",
            Self::v194 => "v194",
            Self::v195 => "v195",
            Self::v196 => "v196",
            Self::v197 => "v197",
            Self::v198 => "v198",
            Self::v199 => "v199",
            Self::v200 => "v200",
            Self::v201 => "v201",
            Self::v202 => "v202",
            Self::v203 => "v203",
            Self::v204 => "v204",
            Self::v205 => "v205",
            Self::v206 => "v206",
            Self::v207 => "v207",
            Self::v208 => "v208",
            Self::v209 => "v209",
            Self::v210 => "v210",
            Self::v211 => "v211",
            Self::v212 => "v212",
            Self::v213 => "v213",
            Self::v214 => "v214",
            Self::v215 => "v215",
            Self::v216 => "v216",
            Self::v217 => "v217",
            Self::v218 => "v218",
            Self::v219 => "v219",
            Self::v220 => "v220",
            Self::v221 => "v221",
            Self::v222 => "v222",
            Self::v223 => "v223",
            Self::v224 => "v224",
            Self::v225 => "v225",
            Self::v226 => "v226",
            Self::v227 => "v227",
            Self::v228 => "v228",
            Self::v229 => "v229",
            Self::v230 => "v230",
            Self::v231 => "v231",
            Self::v232 => "v232",
            Self::v233 => "v233",
            Self::v234 => "v234",
            Self::v235 => "v235",
            Self::v236 => "v236",
            Self::v237 => "v237",
            Self::v238 => "v238",
            Self::v239 => "v239",
            Self::v240 => "v240",
            Self::v241 => "v241",
            Self::v242 => "v242",
            Self::v243 => "v243",
            Self::v244 => "v244",
            Self::v245 => "v245",
            Self::v246 => "v246",
            Self::v247 => "v247",
            Self::v248 => "v248",
            Self::v249 => "v249",
            Self::v250 => "v250",
            Self::v251 => "v251",
            Self::v252 => "v252",
            Self::v253 => "v253",
            Self::v254 => "v254",
            Self::v255 => "v255",
            Self::vw1 => "vw1",
            Self::vw3 => "vw3",
            Self::vw5 => "vw5",
            Self::vw7 => "vw7",
            Self::vw9 => "vw9",
            Self::vw11 => "vw11",
            Self::vw13 => "vw13",
            Self::vw15 => "vw15",
            Self::vw17 => "vw17",
            Self::vw19 => "vw19",
            Self::vw21 => "vw21",
            Self::vw23 => "vw23",
            Self::vw25 => "vw25",
            Self::vw27 => "vw27",
            Self::vw29 => "vw29",
            Self::vw31 => "vw31",
            Self::vw33 => "vw33",
            Self::vw35 => "vw35",
            Self::vw37 => "vw37",
            Self::vw39 => "vw39",
            Self::vw41 => "vw41",
            Self::vw43 => "vw43",
            Self::vw45 => "vw45",
            Self::vw47 => "vw47",
            Self::vw49 => "vw49",
            Self::vw51 => "vw51",
            Self::vw53 => "vw53",
            Self::vw55 => "vw55",
            Self::vw57 => "vw57",
            Self::vw59 => "vw59",
            Self::vw61 => "vw61",
            Self::vw63 => "vw63",
            Self::vw65 => "vw65",
            Self::vw67 => "vw67",
            Self::vw69 => "vw69",
            Self::vw71 => "vw71",
            Self::vw73 => "vw73",
            Self::vw75 => "vw75",
            Self::vw77 => "vw77",
            Self::vw79 => "vw79",
            Self::vw81 => "vw81",
            Self::vw83 => "vw83",
            Self::vw85 => "vw85",
            Self::vw87 => "vw87",
            Self::vw89 => "vw89",
            Self::vw91 => "vw91",
            Self::vw93 => "vw93",
            Self::vw95 => "vw95",
            Self::vw97 => "vw97",
            Self::vw99 => "vw99",
            Self::vw101 => "vw101",
            Self::vw103 => "vw103",
            Self::vw105 => "vw105",
            Self::vw107 => "vw107",
            Self::vw109 => "vw109",
            Self::vw111 => "vw111",
            Self::vw113 => "vw113",
            Self::vw115 => "vw115",
            Self::vw117 => "vw117",
            Self::vw119 => "vw119",
            Self::vw121 => "vw121",
            Self::vw123 => "vw123",
            Self::vw125 => "vw125",
            Self::vw127 => "vw127",
            Self::vw129 => "vw129",
            Self::vw131 => "vw131",
            Self::vw133 => "vw133",
            Self::vw135 => "vw135",
            Self::vw137 => "vw137",
            Self::vw139 => "vw139",
            Self::vw141 => "vw141",
            Self::vw143 => "vw143",
            Self::vw145 => "vw145",
            Self::vw147 => "vw147",
            Self::vw149 => "vw149",
            Self::vw151 => "vw151",
            Self::vw153 => "vw153",
            Self::vw155 => "vw155",
            Self::vw157 => "vw157",
            Self::vw159 => "vw159",
            Self::vw161 => "vw161",
            Self::vw163 => "vw163",
            Self::vw165 => "vw165",
            Self::vw167 => "vw167",
            Self::vw169 => "vw169",
            Self::vw171 => "vw171",
            Self::vw173 => "vw173",
            Self::vw175 => "vw175",
            Self::vw177 => "vw177",
            Self::vw179 => "vw179",
            Self::vw181 => "vw181",
            Self::vw183 => "vw183",
            Self::vw185 => "vw185",
            Self::vw187 => "vw187",
            Self::vw189 => "vw189",
            Self::vw191 => "vw191",
            Self::vw193 => "vw193",
            Self::vw195 => "vw195",
            Self::vw197 => "vw197",
            Self::vw199 => "vw199",
            Self::vw201 => "vw201",
            Self::vw203 => "vw203",
            Self::vw205 => "vw205",
            Self::vw207 => "vw207",
            Self::vw209 => "vw209",
            Self::vw211 => "vw211",
            Self::vw213 => "vw213",
            Self::vw215 => "vw215",
            Self::vw217 => "vw217",
            Self::vw219 => "vw219",
            Self::vw221 => "vw221",
            Self::vw223 => "vw223",
            Self::vw225 => "vw225",
            Self::vw227 => "vw227",
            Self::vw229 => "vw229",
            Self::vw231 => "vw231",
            Self::vw233 => "vw233",
            Self::vw235 => "vw235",
            Self::vw237 => "vw237",
            Self::vw239 => "vw239",
            Self::vw241 => "vw241",
            Self::vw243 => "vw243",
            Self::vw245 => "vw245",
            Self::vw247 => "vw247",
            Self::vw249 => "vw249",
            Self::vw251 => "vw251",
            Self::vw253 => "vw253",
            Self::vw0 => "vw0",
            Self::vw2 => "vw2",
            Self::vw4 => "vw4",
            Self::vw6 => "vw6",
            Self::vw8 => "vw8",
            Self::vw10 => "vw10",
            Self::vw12 => "vw12",
            Self::vw14 => "vw14",
            Self::vw16 => "vw16",
            Self::vw18 => "vw18",
            Self::vw20 => "vw20",
            Self::vw22 => "vw22",
            Self::vw24 => "vw24",
            Self::vw26 => "vw26",
            Self::vw28 => "vw28",
            Self::vw30 => "vw30",
            Self::vw32 => "vw32",
            Self::vw34 => "vw34",
            Self::vw36 => "vw36",
            Self::vw38 => "vw38",
            Self::vw40 => "vw40",
            Self::vw42 => "vw42",
            Self::vw44 => "vw44",
            Self::vw46 => "vw46",
            Self::vw48 => "vw48",
            Self::vw50 => "vw50",
            Self::vw52 => "vw52",
            Self::vw54 => "vw54",
            Self::vw56 => "vw56",
            Self::vw58 => "vw58",
            Self::vw60 => "vw60",
            Self::vw62 => "vw62",
            Self::vw64 => "vw64",
            Self::vw66 => "vw66",
            Self::vw68 => "vw68",
            Self::vw70 => "vw70",
            Self::vw72 => "vw72",
            Self::vw74 => "vw74",
            Self::vw76 => "vw76",
            Self::vw78 => "vw78",
            Self::vw80 => "vw80",
            Self::vw82 => "vw82",
            Self::vw84 => "vw84",
            Self::vw86 => "vw86",
            Self::vw88 => "vw88",
            Self::vw90 => "vw90",
            Self::vw92 => "vw92",
            Self::vw94 => "vw94",
            Self::vw96 => "vw96",
            Self::vw98 => "vw98",
            Self::vw100 => "vw100",
            Self::vw102 => "vw102",
            Self::vw104 => "vw104",
            Self::vw106 => "vw106",
            Self::vw108 => "vw108",
            Self::vw110 => "vw110",
            Self::vw112 => "vw112",
            Self::vw114 => "vw114",
            Self::vw116 => "vw116",
            Self::vw118 => "vw118",
            Self::vw120 => "vw120",
            Self::vw122 => "vw122",
            Self::vw124 => "vw124",
            Self::vw126 => "vw126",
            Self::vw128 => "vw128",
            Self::vw130 => "vw130",
            Self::vw132 => "vw132",
            Self::vw134 => "vw134",
            Self::vw136 => "vw136",
            Self::vw138 => "vw138",
            Self::vw140 => "vw140",
            Self::vw142 => "vw142",
            Self::vw144 => "vw144",
            Self::vw146 => "vw146",
            Self::vw148 => "vw148",
            Self::vw150 => "vw150",
            Self::vw152 => "vw152",
            Self::vw154 => "vw154",
            Self::vw156 => "vw156",
            Self::vw158 => "vw158",
            Self::vw160 => "vw160",
            Self::vw162 => "vw162",
            Self::vw164 => "vw164",
            Self::vw166 => "vw166",
            Self::vw168 => "vw168",
            Self::vw170 => "vw170",
            Self::vw172 => "vw172",
            Self::vw174 => "vw174",
            Self::vw176 => "vw176",
            Self::vw178 => "vw178",
            Self::vw180 => "vw180",
            Self::vw182 => "vw182",
            Self::vw184 => "vw184",
            Self::vw186 => "vw186",
            Self::vw188 => "vw188",
            Self::vw190 => "vw190",
            Self::vw192 => "vw192",
            Self::vw194 => "vw194",
            Self::vw196 => "vw196",
            Self::vw198 => "vw198",
            Self::vw200 => "vw200",
            Self::vw202 => "vw202",
            Self::vw204 => "vw204",
            Self::vw206 => "vw206",
            Self::vw208 => "vw208",
            Self::vw210 => "vw210",
            Self::vw212 => "vw212",
            Self::vw214 => "vw214",
            Self::vw216 => "vw216",
            Self::vw218 => "vw218",
            Self::vw220 => "vw220",
            Self::vw222 => "vw222",
            Self::vw224 => "vw224",
            Self::vw226 => "vw226",
            Self::vw228 => "vw228",
            Self::vw230 => "vw230",
            Self::vw232 => "vw232",
            Self::vw234 => "vw234",
            Self::vw236 => "vw236",
            Self::vw238 => "vw238",
            Self::vw240 => "vw240",
            Self::vw242 => "vw242",
            Self::vw244 => "vw244",
            Self::vw246 => "vw246",
            Self::vw248 => "vw248",
            Self::vw250 => "vw250",
            Self::vw252 => "vw252",
            Self::vw254 => "vw254",
        }
    }
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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
#[doc = "Create token_fields: inst1"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: A_BITS_0_3 B_BITS_0_3"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: PARAM_C ELEMENT_C"]
fn token_11(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 24) & 15) as u8)
}
#[doc = "Create token_fields: A_BITS_4_7 B_BITS_4_7 B_BITS_4_7_S"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: PARAM_D ELEMENT_D"]
fn token_10(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 28) & 15) as u8)
}
#[doc = "Create token_fields: A_BITS_0_31 A_BITS_0_31_S B_BITS_0_31 B_BITS_0_31_S C_BITS_0_31 C_BITS_0_31_S constant32 constant32s"]
fn token_6(tokens: &[u8]) -> u32 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 4294967295) as u32)
}
#[doc = "Create token_fields: inst1_padding A_BITS_0_15 A_BITS_0_15_S B_BITS_0_15 B_BITS_0_15_S C_BITS_0_15 C_BITS_0_15_S constant16 constant16s"]
fn token_3(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: inst0 A_BITS_0_7 A_BITS_0_7_S B_BITS_0_7 B_BITS_0_7_S C_BITS_0_7 C_BITS_0_7_S"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: PARAM_G ELEMENT_G"]
fn token_8(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 0) & 15) as u8)
}
#[doc = "Create token_fields: N_PARAMS N_ELEMENTS"]
fn token_7(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 4) & 15) as u8)
}
#[doc = "Create token_fields: METHOD_INDEX VTABLE_OFFSET INLINE TYPE_INDEX"]
fn token_9(tokens: &[u8]) -> u16 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 8) & 65535) as u16)
}
#[doc = "Create token_fields: PARAM_F ELEMENT_F"]
fn token_12(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 36) & 15) as u8)
}
#[doc = "Create token_fields: PARAM_E ELEMENT_E"]
fn token_13(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..5].copy_from_slice(&tokens[0..5]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 32) & 15) as u8)
}
#[doc = "Create token_fields: constant64"]
fn token_14(tokens: &[u8]) -> u64 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 0) & 18446744073709551615) as u64)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:696:1, end:696:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar0 {
    TYPE_INDEX: u16,
    regElemF: TableregElemF,
    regElemE: TableregElemE,
    regElemD: TableregElemD,
    regElemG: TableregElemG,
    regElemC: TableregElemC,
}
impl filled_new_array_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemF,
                regElemE,
                regElemD,
                regElemG,
                regElemC,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:701:1, end:701:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar1 {
    TYPE_INDEX: u16,
    regElemE: TableregElemE,
    regElemF: TableregElemF,
    regElemG: TableregElemG,
    regElemC: TableregElemC,
    regElemD: TableregElemD,
}
impl filled_new_array_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regElemC
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemE,
                regElemF,
                regElemG,
                regElemC,
                regElemD,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:706:1, end:706:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar2 {
    TYPE_INDEX: u16,
    regElemD: TableregElemD,
    regElemF: TableregElemF,
    regElemC: TableregElemC,
    regElemG: TableregElemG,
    regElemE: TableregElemE,
}
impl filled_new_array_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regElemC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemD
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemD,
                regElemF,
                regElemC,
                regElemG,
                regElemE,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:711:1, end:711:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar3 {
    TYPE_INDEX: u16,
    regElemC: TableregElemC,
    regElemE: TableregElemE,
    regElemD: TableregElemD,
    regElemG: TableregElemG,
    regElemF: TableregElemF,
}
impl filled_new_array_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regElemC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemE
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemC,
                regElemE,
                regElemD,
                regElemG,
                regElemF,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:716:1, end:716:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar4 {
    TYPE_INDEX: u16,
    regElemC: TableregElemC,
    regElemE: TableregElemE,
    regElemG: TableregElemG,
    regElemF: TableregElemF,
    regElemD: TableregElemD,
}
impl filled_new_array_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regElemC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemF
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemC,
                regElemE,
                regElemG,
                regElemF,
                regElemD,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:721:1, end:721:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_instructionVar5 {
    TYPE_INDEX: u16,
    regElemE: TableregElemE,
    regElemD: TableregElemD,
    regElemG: TableregElemG,
    regElemF: TableregElemF,
    regElemC: TableregElemC,
}
impl filled_new_array_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.TYPE_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regElemC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regElemG
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
        let mut block_1_len = 5;
        let regElemC = if let Some((len, table)) =
            TableregElemC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemD = if let Some((len, table)) =
            TableregElemD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemE = if let Some((len, table)) =
            TableregElemE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemF = if let Some((len, table)) =
            TableregElemF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regElemG = if let Some((len, table)) =
            TableregElemG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let TYPE_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regElemE,
                regElemD,
                regElemG,
                regElemF,
                regElemC,
                TYPE_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1244:1, end:1244:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar6 {
    METHOD_INDEX: u16,
}
impl invoke_virtual_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
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
        let mut block_1_len = 5;
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { METHOD_INDEX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1249:1, end:1249:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar7 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
}
impl invoke_virtual_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1255:1, end:1255:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar8 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
}
impl invoke_virtual_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1262:1, end:1262:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar9 {
    METHOD_INDEX: u16,
    regParamE: TableregParamE,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
}
impl invoke_virtual_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamE,
                regParamC,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1270:1, end:1270:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar10 {
    METHOD_INDEX: u16,
    regParamE: TableregParamE,
    regParamF: TableregParamF,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
}
impl invoke_virtual_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamE,
                regParamF,
                regParamC,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1279:1, end:1279:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_instructionVar11 {
    METHOD_INDEX: u16,
    regParamF: TableregParamF,
    regParamC: TableregParamC,
    regParamG: TableregParamG,
    regParamE: TableregParamE,
    regParamD: TableregParamD,
}
impl invoke_virtual_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamG
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamG = if let Some((len, table)) =
            TableregParamG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamF,
                regParamC,
                regParamG,
                regParamE,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1294:1, end:1294:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar12 {
    METHOD_INDEX: u16,
}
impl invoke_super_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
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
        let mut block_1_len = 5;
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { METHOD_INDEX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1299:1, end:1299:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar13 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
}
impl invoke_super_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1305:1, end:1305:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar14 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
}
impl invoke_super_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1312:1, end:1312:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar15 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
}
impl invoke_super_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamD,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1320:1, end:1320:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar16 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamF: TableregParamF,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
}
impl invoke_super_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamF,
                regParamD,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1329:1, end:1329:2))"]
#[derive(Clone, Debug)]
struct invoke_super_instructionVar17 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamF: TableregParamF,
    regParamG: TableregParamG,
    regParamC: TableregParamC,
    regParamE: TableregParamE,
}
impl invoke_super_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamG
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamG = if let Some((len, table)) =
            TableregParamG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamF,
                regParamG,
                regParamC,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1345:1, end:1345:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar18 {
    METHOD_INDEX: u16,
}
impl invoke_direct_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
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
        let mut block_1_len = 5;
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { METHOD_INDEX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1350:1, end:1350:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar19 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
}
impl invoke_direct_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1356:1, end:1356:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar20 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamC: TableregParamC,
}
impl invoke_direct_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1363:1, end:1363:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar21 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
    regParamC: TableregParamC,
}
impl invoke_direct_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamE,
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1371:1, end:1371:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar22 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
    regParamF: TableregParamF,
    regParamC: TableregParamC,
}
impl invoke_direct_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamE,
                regParamF,
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1380:1, end:1380:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_instructionVar23 {
    METHOD_INDEX: u16,
    regParamG: TableregParamG,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
    regParamF: TableregParamF,
    regParamE: TableregParamE,
}
impl invoke_direct_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamG
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamG = if let Some((len, table)) =
            TableregParamG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamG,
                regParamC,
                regParamD,
                regParamF,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1394:1, end:1394:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar24 {
    METHOD_INDEX: u16,
}
impl invoke_static_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
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
        let mut block_1_len = 5;
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { METHOD_INDEX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1399:1, end:1399:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar25 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
}
impl invoke_static_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1405:1, end:1405:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar26 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamC: TableregParamC,
}
impl invoke_static_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1412:1, end:1412:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar27 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
}
impl invoke_static_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamD,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1420:1, end:1420:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar28 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamF: TableregParamF,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
}
impl invoke_static_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamF,
                regParamD,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1429:1, end:1429:2))"]
#[derive(Clone, Debug)]
struct invoke_static_instructionVar29 {
    METHOD_INDEX: u16,
    regParamE: TableregParamE,
    regParamG: TableregParamG,
    regParamD: TableregParamD,
    regParamC: TableregParamC,
    regParamF: TableregParamF,
}
impl invoke_static_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamG
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamG = if let Some((len, table)) =
            TableregParamG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamE,
                regParamG,
                regParamD,
                regParamC,
                regParamF,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1445:1, end:1445:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar30 {
    METHOD_INDEX: u16,
}
impl invoke_interface_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
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
        let mut block_1_len = 5;
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { METHOD_INDEX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1450:1, end:1450:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar31 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
}
impl invoke_interface_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1456:1, end:1456:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar32 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamD: TableregParamD,
}
impl invoke_interface_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamD,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1463:1, end:1463:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar33 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamE: TableregParamE,
    regParamC: TableregParamC,
}
impl invoke_interface_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamE,
                regParamC,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1471:1, end:1471:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar34 {
    METHOD_INDEX: u16,
    regParamD: TableregParamD,
    regParamF: TableregParamF,
    regParamC: TableregParamC,
    regParamE: TableregParamE,
}
impl invoke_interface_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamD,
                regParamF,
                regParamC,
                regParamE,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1480:1, end:1480:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_instructionVar35 {
    METHOD_INDEX: u16,
    regParamC: TableregParamC,
    regParamE: TableregParamE,
    regParamG: TableregParamG,
    regParamD: TableregParamD,
    regParamF: TableregParamF,
}
impl invoke_interface_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.METHOD_INDEX as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.regParamC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamD
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamE
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamF
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.regParamG
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
        let mut block_1_len = 5;
        let regParamC = if let Some((len, table)) =
            TableregParamC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamD = if let Some((len, table)) =
            TableregParamD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamE = if let Some((len, table)) =
            TableregParamE::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamF = if let Some((len, table)) =
            TableregParamF::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let regParamG = if let Some((len, table)) =
            TableregParamG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let METHOD_INDEX = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regParamC,
                regParamE,
                regParamG,
                regParamD,
                regParamF,
                METHOD_INDEX,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:293:1, end:293:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar36 {}
impl nop_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("nop"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:306:1, end:306:2))"]
#[derive(Clone, Debug)]
struct move_instructionVar37 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl move_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct move_from_16_instructionVar38 {
    registerA8: TableregisterA8,
    registerB16: TableregisterB16,
}
impl move_from_16_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_from_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16 = if let Some((len, table)) =
            TableregisterB16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:326:1, end:326:2))"]
#[derive(Clone, Debug)]
struct move_16_instructionVar39 {
    registerA16: TableregisterA16,
    registerB16: TableregisterB16,
}
impl move_16_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16
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
        let inst1_padding = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let registerA16 = if let Some((len, table)) =
            TableregisterA16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16 = if let Some((len, table)) =
            TableregisterB16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA16,
                registerB16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:336:1, end:336:2))"]
#[derive(Clone, Debug)]
struct move_wide_instructionVar40 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl move_wide_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:346:1, end:346:2))"]
#[derive(Clone, Debug)]
struct move_wide_from_16_instructionVar41 {
    registerA8w: TableregisterA8w,
    registerB16w: TableregisterB16w,
}
impl move_wide_from_16_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_wide_from_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16w = if let Some((len, table)) =
            TableregisterB16w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB16w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:356:1, end:356:2))"]
#[derive(Clone, Debug)]
struct move_wide_16_instructionVar42 {
    registerA16w: TableregisterA16w,
    registerB16w: TableregisterB16w,
}
impl move_wide_16_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_wide_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA16w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16w
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
        let registerA16w = if let Some((len, table)) =
            TableregisterA16w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16w = if let Some((len, table)) =
            TableregisterB16w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA16w,
                registerB16w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct move_object_instructionVar43 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl move_object_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:378:1, end:378:2))"]
#[derive(Clone, Debug)]
struct move_object_from_16_instructionVar44 {
    registerA8: TableregisterA8,
    registerB16: TableregisterB16,
}
impl move_object_from_16_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_object_from_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16 = if let Some((len, table)) =
            TableregisterB16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:388:1, end:388:2))"]
#[derive(Clone, Debug)]
struct move_object_16_instructionVar45 {
    registerA16: TableregisterA16,
    registerB16: TableregisterB16,
}
impl move_object_16_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_object_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB16
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
        let inst1_padding = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let registerA16 = if let Some((len, table)) =
            TableregisterA16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let registerB16 = if let Some((len, table)) =
            TableregisterB16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA16,
                registerB16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:400:1, end:400:2))"]
#[derive(Clone, Debug)]
struct move_result_instructionVar46 {
    registerA8: TableregisterA8,
}
impl move_result_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_result"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:412:1, end:412:2))"]
#[derive(Clone, Debug)]
struct move_result_wide_instructionVar47 {
    registerA8w: TableregisterA8w,
}
impl move_result_wide_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_result_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8w }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:424:1, end:424:2))"]
#[derive(Clone, Debug)]
struct move_result_object_instructionVar48 {
    registerA8: TableregisterA8,
}
impl move_result_object_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_result_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:437:1, end:437:2))"]
#[derive(Clone, Debug)]
struct move_exception_instructionVar49 {
    registerA8: TableregisterA8,
}
impl move_exception_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("move_exception"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:448:1, end:448:2))"]
#[derive(Clone, Debug)]
struct return_void_instructionVar50 {}
impl return_void_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("return_void"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let inst1_padding = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:457:1, end:457:2))"]
#[derive(Clone, Debug)]
struct return_instructionVar51 {
    registerA8: TableregisterA8,
}
impl return_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("return"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:467:1, end:467:2))"]
#[derive(Clone, Debug)]
struct return_wide_instructionVar52 {
    registerA8w: TableregisterA8w,
}
impl return_wide_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("return_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8w }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct return_object_instructionVar53 {
    registerA8: TableregisterA8,
}
impl return_object_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("return_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:491:1, end:491:2))"]
#[derive(Clone, Debug)]
struct const_4_instructionVar54 {
    B_BITS_4_7_S: u8,
    registerA4: TableregisterA4,
}
impl const_4_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_4"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_4_7_S & 8 != 0 {
                    -1 & !7
                } else {
                    0
                } | self.B_BITS_4_7_S as i8)
                    .is_negative(),
                (if self.B_BITS_4_7_S & 8 != 0 {
                    -1 & !7
                } else {
                    0
                } | self.B_BITS_4_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let B_BITS_4_7_S = token_5(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                B_BITS_4_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:501:1, end:501:2))"]
#[derive(Clone, Debug)]
struct const_16_instructionVar55 {
    B_BITS_0_15_S: u16,
    registerA8: TableregisterA8,
}
impl const_16_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.B_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.B_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.B_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:511:1, end:511:2))"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    constant32: u32,
    registerA8: TableregisterA8,
}
impl instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("const"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.constant32 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let constant32 = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                constant32,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:521:1, end:521:2))"]
#[derive(Clone, Debug)]
struct const_high_16_instructionVar57 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl const_high_16_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_high_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:531:1, end:531:2))"]
#[derive(Clone, Debug)]
struct const_wide_16_instructionVar58 {
    constant16s: u16,
    registerA8w: TableregisterA8w,
}
impl const_wide_16_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_wide_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.constant16s & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.constant16s as i16)
                    .is_negative(),
                (if self.constant16s & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.constant16s as i16)
                    .abs() as u64,
            ),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let constant16s = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                constant16s,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct const_wide_32_instructionVar59 {
    constant32s: u32,
    registerA8w: TableregisterA8w,
}
impl const_wide_32_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_wide_32"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.constant32s & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.constant32s as i32)
                    .is_negative(),
                (if self.constant32s & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.constant32s as i32)
                    .abs() as u64,
            ),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let constant32s = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                constant32s,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:551:1, end:551:2))"]
#[derive(Clone, Debug)]
struct const_wide_instructionVar60 {
    constant64: u64,
    registerA8w: TableregisterA8w,
}
impl const_wide_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.constant64 as u64),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 8;
        let constant64 = token_14(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                constant64,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:561:1, end:561:2))"]
#[derive(Clone, Debug)]
struct const_wide_high_16_instructionVar61 {
    B_BITS_0_15_S: u16,
    registerA8w: TableregisterA8w,
}
impl const_wide_high_16_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_wide_high_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.B_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.B_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.B_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                B_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:571:1, end:571:2))"]
#[derive(Clone, Debug)]
struct const_string_instructionVar62 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl const_string_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_string"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:581:1, end:581:2))"]
#[derive(Clone, Debug)]
struct const_string_jumbo_instructionVar63 {
    B_BITS_0_31: u32,
    registerA8: TableregisterA8,
}
impl const_string_jumbo_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_string_jumbo"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_31 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let B_BITS_0_31 = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_31,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:593:1, end:593:2))"]
#[derive(Clone, Debug)]
struct const_class_instructionVar64 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl const_class_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("const_class"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:605:1, end:605:2))"]
#[derive(Clone, Debug)]
struct monitor_enter_instructionVar65 {
    registerA8: TableregisterA8,
}
impl monitor_enter_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("monitor_enter"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:614:1, end:614:2))"]
#[derive(Clone, Debug)]
struct monitor_exit_instructionVar66 {
    registerA8: TableregisterA8,
}
impl monitor_exit_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("monitor_exit"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:628:1, end:628:2))"]
#[derive(Clone, Debug)]
struct check_cast_instructionVar67 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl check_cast_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("check_cast"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:643:1, end:643:2))"]
#[derive(Clone, Debug)]
struct instance_of_instructionVar68 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl instance_of_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("instance_of"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:657:1, end:657:2))"]
#[derive(Clone, Debug)]
struct array_length_instructionVar69 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl array_length_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("array_length"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:668:1, end:668:2))"]
#[derive(Clone, Debug)]
struct new_instance_instructionVar70 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl new_instance_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("new_instance"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:679:1, end:679:2))"]
#[derive(Clone, Debug)]
struct new_array_instructionVar71 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl new_array_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("new_array"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:736:1, end:736:2))"]
#[derive(Clone, Debug)]
struct filled_new_array_range_instructionVar72 {
    A_BITS_0_7: u8,
    B_BITS_0_15: u16,
    registerC16: TableregisterC16,
}
impl filled_new_array_range_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("filled_new_array_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                A_BITS_0_7,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:751:1, end:751:2))"]
#[derive(Clone, Debug)]
struct fill_array_data_instructionVar73 {
    B_BITS_0_31_S: u32,
    registerA8: TableregisterA8,
}
impl fill_array_data_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fill_array_data"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .is_negative(),
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let B_BITS_0_31_S = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_31_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:764:1, end:764:2))"]
#[derive(Clone, Debug)]
struct throw_instructionVar74 {
    registerA8: TableregisterA8,
}
impl throw_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("throw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:777:1, end:777:2))"]
#[derive(Clone, Debug)]
struct goto_instructionVar75 {
    goto8: Tablegoto8,
}
impl goto_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("goto"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.goto8
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
        let goto8 = if let Some((len, table)) =
            Tablegoto8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { goto8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:786:1, end:786:2))"]
#[derive(Clone, Debug)]
struct goto_16_instructionVar76 {
    goto16: Tablegoto16,
}
impl goto_16_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("goto_16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.goto16
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
        let inst1_padding = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let goto16 = if let Some((len, table)) =
            Tablegoto16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { goto16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:795:1, end:795:2))"]
#[derive(Clone, Debug)]
struct goto_32_instructionVar77 {
    goto32: Tablegoto32,
}
impl goto_32_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("goto_32"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.goto32
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
        let inst1_padding = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let goto32 = if let Some((len, table)) =
            Tablegoto32::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { goto32 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:814:1, end:814:2))"]
#[derive(Clone, Debug)]
struct packed_switch_instructionVar78 {
    B_BITS_0_31_S: u32,
    registerA8: TableregisterA8,
}
impl packed_switch_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("packed_switch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .is_negative(),
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let B_BITS_0_31_S = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_31_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:842:1, end:842:2))"]
#[derive(Clone, Debug)]
struct sparse_switch_instructionVar79 {
    B_BITS_0_31_S: u32,
    registerA8: TableregisterA8,
}
impl sparse_switch_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sparse_switch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .is_negative(),
                (if self.B_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.B_BITS_0_31_S as i32)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 4;
        let B_BITS_0_31_S = token_6(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_31_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:873:1, end:873:2))"]
#[derive(Clone, Debug)]
struct cmpl_float_instructionVar80 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl cmpl_float_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpl_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:878:1, end:878:2))"]
#[derive(Clone, Debug)]
struct cmpg_float_instructionVar81 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl cmpg_float_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpg_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:883:1, end:883:2))"]
#[derive(Clone, Debug)]
struct cmpl_double_instructionVar82 {
    registerA8: TableregisterA8,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl cmpl_double_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpl_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:888:1, end:888:2))"]
#[derive(Clone, Debug)]
struct cmpg_double_instructionVar83 {
    registerA8: TableregisterA8,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl cmpg_double_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpg_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:893:1, end:893:2))"]
#[derive(Clone, Debug)]
struct cmp_long_instructionVar84 {
    registerA8: TableregisterA8,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl cmp_long_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:907:1, end:907:2))"]
#[derive(Clone, Debug)]
struct if_eq_instructionVar85 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
    rel16: Tablerel16,
}
impl if_eq_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_eq"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerB4,
                registerA4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:912:1, end:912:2))"]
#[derive(Clone, Debug)]
struct if_ne_instructionVar86 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
    rel16: Tablerel16,
}
impl if_ne_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_ne"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerB4,
                registerA4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:917:1, end:917:2))"]
#[derive(Clone, Debug)]
struct if_lt_instructionVar87 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
    rel16: Tablerel16,
}
impl if_lt_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_lt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA4,
                registerB4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:922:1, end:922:2))"]
#[derive(Clone, Debug)]
struct if_ge_instructionVar88 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
    rel16: Tablerel16,
}
impl if_ge_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_ge"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA4,
                registerB4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:927:1, end:927:2))"]
#[derive(Clone, Debug)]
struct if_gt_instructionVar89 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
    rel16: Tablerel16,
}
impl if_gt_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_gt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerB4,
                registerA4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:932:1, end:932:2))"]
#[derive(Clone, Debug)]
struct if_le_instructionVar90 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
    rel16: Tablerel16,
}
impl if_le_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_le"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA4,
                registerB4,
                rel16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:945:1, end:945:2))"]
#[derive(Clone, Debug)]
struct if_eqz_instructionVar91 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_eqz_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_eqz"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:950:1, end:950:2))"]
#[derive(Clone, Debug)]
struct if_nez_instructionVar92 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_nez_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_nez"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:955:1, end:955:2))"]
#[derive(Clone, Debug)]
struct if_ltz_instructionVar93 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_ltz_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_ltz"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:960:1, end:960:2))"]
#[derive(Clone, Debug)]
struct if_gez_instructionVar94 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_gez_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_gez"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:965:1, end:965:2))"]
#[derive(Clone, Debug)]
struct if_gtz_instructionVar95 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_gtz_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_gtz"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:970:1, end:970:2))"]
#[derive(Clone, Debug)]
struct if_lez_instructionVar96 {
    registerA8: TableregisterA8,
    rel16: Tablerel16,
}
impl if_lez_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_lez"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.rel16
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let rel16 = if let Some((len, table)) =
            Tablerel16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { registerA8, rel16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:985:1, end:985:2))"]
#[derive(Clone, Debug)]
struct aget_instructionVar97 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:990:1, end:990:2))"]
#[derive(Clone, Debug)]
struct aget_wide_instructionVar98 {
    registerA8w: TableregisterA8w,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_wide_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:995:1, end:995:2))"]
#[derive(Clone, Debug)]
struct aget_object_instructionVar99 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_object_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1000:1, end:1000:2))"]
#[derive(Clone, Debug)]
struct aget_boolean_instructionVar100 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_boolean_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1005:1, end:1005:2))"]
#[derive(Clone, Debug)]
struct aget_byte_instructionVar101 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_byte_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1010:1, end:1010:2))"]
#[derive(Clone, Debug)]
struct aget_char_instructionVar102 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_char_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1015:1, end:1015:2))"]
#[derive(Clone, Debug)]
struct aget_short_instructionVar103 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aget_short_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aget_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1020:1, end:1020:2))"]
#[derive(Clone, Debug)]
struct aput_instructionVar104 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1025:1, end:1025:2))"]
#[derive(Clone, Debug)]
struct aput_wide_instructionVar105 {
    registerA8w: TableregisterA8w,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_wide_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1030:1, end:1030:2))"]
#[derive(Clone, Debug)]
struct aput_object_instructionVar106 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_object_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1035:1, end:1035:2))"]
#[derive(Clone, Debug)]
struct aput_boolean_instructionVar107 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_boolean_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1040:1, end:1040:2))"]
#[derive(Clone, Debug)]
struct aput_byte_instructionVar108 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_byte_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1045:1, end:1045:2))"]
#[derive(Clone, Debug)]
struct aput_char_instructionVar109 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_char_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1050:1, end:1050:2))"]
#[derive(Clone, Debug)]
struct aput_short_instructionVar110 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl aput_short_instructionVar110 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aput_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1065:1, end:1065:2))"]
#[derive(Clone, Debug)]
struct iget_instructionVar111 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iget_instructionVar111 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1071:1, end:1071:2))"]
#[derive(Clone, Debug)]
struct iget_wide_instructionVar112 {
    C_BITS_0_15: u16,
    registerA4w: TableregisterA4w,
    registerB4: TableregisterB4,
}
impl iget_wide_instructionVar112 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1077:1, end:1077:2))"]
#[derive(Clone, Debug)]
struct iget_object_instructionVar113 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iget_object_instructionVar113 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1083:1, end:1083:2))"]
#[derive(Clone, Debug)]
struct iget_boolean_instructionVar114 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iget_boolean_instructionVar114 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1089:1, end:1089:2))"]
#[derive(Clone, Debug)]
struct iget_byte_instructionVar115 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iget_byte_instructionVar115 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1095:1, end:1095:2))"]
#[derive(Clone, Debug)]
struct iget_char_instructionVar116 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iget_char_instructionVar116 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1101:1, end:1101:2))"]
#[derive(Clone, Debug)]
struct iget_short_instructionVar117 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iget_short_instructionVar117 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iget_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1107:1, end:1107:2))"]
#[derive(Clone, Debug)]
struct iput_instructionVar118 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iput_instructionVar118 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1113:1, end:1113:2))"]
#[derive(Clone, Debug)]
struct iput_wide_instructionVar119 {
    C_BITS_0_15: u16,
    registerA4w: TableregisterA4w,
    registerB4: TableregisterB4,
}
impl iput_wide_instructionVar119 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1119:1, end:1119:2))"]
#[derive(Clone, Debug)]
struct iput_object_instructionVar120 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iput_object_instructionVar120 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1125:1, end:1125:2))"]
#[derive(Clone, Debug)]
struct iput_boolean_instructionVar121 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iput_boolean_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1131:1, end:1131:2))"]
#[derive(Clone, Debug)]
struct iput_byte_instructionVar122 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iput_byte_instructionVar122 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1137:1, end:1137:2))"]
#[derive(Clone, Debug)]
struct iput_char_instructionVar123 {
    C_BITS_0_15: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl iput_char_instructionVar123 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1143:1, end:1143:2))"]
#[derive(Clone, Debug)]
struct iput_short_instructionVar124 {
    C_BITS_0_15: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl iput_short_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iput_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",[")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(":"),
            DisplayElement::Number(true, false, self.C_BITS_0_15 as u64),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1158:1, end:1158:2))"]
#[derive(Clone, Debug)]
struct sget_instructionVar125 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_instructionVar125 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1163:1, end:1163:2))"]
#[derive(Clone, Debug)]
struct sget_wide_instructionVar126 {
    B_BITS_0_15: u16,
    registerA8w: TableregisterA8w,
}
impl sget_wide_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1168:1, end:1168:2))"]
#[derive(Clone, Debug)]
struct sget_object_instructionVar127 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_object_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1173:1, end:1173:2))"]
#[derive(Clone, Debug)]
struct sget_boolean_instructionVar128 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_boolean_instructionVar128 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1178:1, end:1178:2))"]
#[derive(Clone, Debug)]
struct sget_byte_instructionVar129 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_byte_instructionVar129 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1183:1, end:1183:2))"]
#[derive(Clone, Debug)]
struct sget_char_instructionVar130 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_char_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1188:1, end:1188:2))"]
#[derive(Clone, Debug)]
struct sget_short_instructionVar131 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sget_short_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sget_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1194:1, end:1194:2))"]
#[derive(Clone, Debug)]
struct sput_instructionVar132 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1199:1, end:1199:2))"]
#[derive(Clone, Debug)]
struct sput_wide_instructionVar133 {
    B_BITS_0_15: u16,
    registerA8w: TableregisterA8w,
}
impl sput_wide_instructionVar133 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_wide"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8w,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1204:1, end:1204:2))"]
#[derive(Clone, Debug)]
struct sput_object_instructionVar134 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_object_instructionVar134 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_object"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1209:1, end:1209:2))"]
#[derive(Clone, Debug)]
struct sput_boolean_instructionVar135 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_boolean_instructionVar135 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_boolean"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1214:1, end:1214:2))"]
#[derive(Clone, Debug)]
struct sput_byte_instructionVar136 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_byte_instructionVar136 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1219:1, end:1219:2))"]
#[derive(Clone, Debug)]
struct sput_char_instructionVar137 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_char_instructionVar137 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1224:1, end:1224:2))"]
#[derive(Clone, Debug)]
struct sput_short_instructionVar138 {
    B_BITS_0_15: u16,
    registerA8: TableregisterA8,
}
impl sput_short_instructionVar138 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sput_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                B_BITS_0_15,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1502:1, end:1502:2))"]
#[derive(Clone, Debug)]
struct invoke_virtual_range_instructionVar139 {
    B_BITS_0_15: u16,
    A_BITS_0_7: u8,
    registerC16: TableregisterC16,
}
impl invoke_virtual_range_instructionVar139 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_virtual_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                B_BITS_0_15,
                A_BITS_0_7,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1508:1, end:1508:2))"]
#[derive(Clone, Debug)]
struct invoke_super_range_instructionVar140 {
    B_BITS_0_15: u16,
    A_BITS_0_7: u8,
    registerC16: TableregisterC16,
}
impl invoke_super_range_instructionVar140 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_super_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                B_BITS_0_15,
                A_BITS_0_7,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1514:1, end:1514:2))"]
#[derive(Clone, Debug)]
struct invoke_direct_range_instructionVar141 {
    B_BITS_0_15: u16,
    A_BITS_0_7: u8,
    registerC16: TableregisterC16,
}
impl invoke_direct_range_instructionVar141 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_direct_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                B_BITS_0_15,
                A_BITS_0_7,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1520:1, end:1520:2))"]
#[derive(Clone, Debug)]
struct invoke_static_range_instructionVar142 {
    B_BITS_0_15: u16,
    A_BITS_0_7: u8,
    registerC16: TableregisterC16,
}
impl invoke_static_range_instructionVar142 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_static_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                B_BITS_0_15,
                A_BITS_0_7,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1526:1, end:1526:2))"]
#[derive(Clone, Debug)]
struct invoke_interface_range_instructionVar143 {
    B_BITS_0_15: u16,
    A_BITS_0_7: u8,
    registerC16: TableregisterC16,
}
impl invoke_interface_range_instructionVar143 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("invoke_interface_range"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.B_BITS_0_15 as u64),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.A_BITS_0_7 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.registerC16
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
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 2;
        let registerC16 = if let Some((len, table)) =
            TableregisterC16::parse(tokens_current, &mut context_instance, inst_start)
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
                registerC16,
                B_BITS_0_15,
                A_BITS_0_7,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1542:1, end:1542:2))"]
#[derive(Clone, Debug)]
struct neg_int_instructionVar144 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl neg_int_instructionVar144 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1547:1, end:1547:2))"]
#[derive(Clone, Debug)]
struct not_int_instructionVar145 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl not_int_instructionVar145 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1552:1, end:1552:2))"]
#[derive(Clone, Debug)]
struct neg_long_instructionVar146 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl neg_long_instructionVar146 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1557:1, end:1557:2))"]
#[derive(Clone, Debug)]
struct not_long_instructionVar147 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl not_long_instructionVar147 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1562:1, end:1562:2))"]
#[derive(Clone, Debug)]
struct neg_float_instructionVar148 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl neg_float_instructionVar148 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1567:1, end:1567:2))"]
#[derive(Clone, Debug)]
struct neg_double_instructionVar149 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl neg_double_instructionVar149 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1572:1, end:1572:2))"]
#[derive(Clone, Debug)]
struct int_to_long_instructionVar150 {
    registerA4w: TableregisterA4w,
    registerB4: TableregisterB4,
}
impl int_to_long_instructionVar150 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1577:1, end:1577:2))"]
#[derive(Clone, Debug)]
struct int_to_float_instructionVar151 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl int_to_float_instructionVar151 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1582:1, end:1582:2))"]
#[derive(Clone, Debug)]
struct int_to_double_instructionVar152 {
    registerB4: TableregisterB4,
    registerA4w: TableregisterA4w,
}
impl int_to_double_instructionVar152 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1587:1, end:1587:2))"]
#[derive(Clone, Debug)]
struct long_to_int_instructionVar153 {
    registerA4: TableregisterA4,
    registerB4w: TableregisterB4w,
}
impl long_to_int_instructionVar153 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("long_to_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1591:1, end:1591:2))"]
#[derive(Clone, Debug)]
struct long_to_float_instructionVar154 {
    registerB4w: TableregisterB4w,
    registerA4: TableregisterA4,
}
impl long_to_float_instructionVar154 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("long_to_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1596:1, end:1596:2))"]
#[derive(Clone, Debug)]
struct long_to_double_instructionVar155 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl long_to_double_instructionVar155 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("long_to_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1601:1, end:1601:2))"]
#[derive(Clone, Debug)]
struct float_to_int_instructionVar156 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl float_to_int_instructionVar156 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("float_to_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1606:1, end:1606:2))"]
#[derive(Clone, Debug)]
struct float_to_long_instructionVar157 {
    registerB4: TableregisterB4,
    registerA4w: TableregisterA4w,
}
impl float_to_long_instructionVar157 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("float_to_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1611:1, end:1611:2))"]
#[derive(Clone, Debug)]
struct float_to_double_instructionVar158 {
    registerB4: TableregisterB4,
    registerA4w: TableregisterA4w,
}
impl float_to_double_instructionVar158 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("float_to_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1616:1, end:1616:2))"]
#[derive(Clone, Debug)]
struct double_to_int_instructionVar159 {
    registerA4: TableregisterA4,
    registerB4w: TableregisterB4w,
}
impl double_to_int_instructionVar159 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("double_to_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1621:1, end:1621:2))"]
#[derive(Clone, Debug)]
struct double_to_long_instructionVar160 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl double_to_long_instructionVar160 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("double_to_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1626:1, end:1626:2))"]
#[derive(Clone, Debug)]
struct double_to_float_instructionVar161 {
    registerB4w: TableregisterB4w,
    registerA4: TableregisterA4,
}
impl double_to_float_instructionVar161 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("double_to_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1631:1, end:1631:2))"]
#[derive(Clone, Debug)]
struct int_to_byte_instructionVar162 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl int_to_byte_instructionVar162 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_byte"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1636:1, end:1636:2))"]
#[derive(Clone, Debug)]
struct int_to_char_instructionVar163 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl int_to_char_instructionVar163 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_char"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1641:1, end:1641:2))"]
#[derive(Clone, Debug)]
struct int_to_short_instructionVar164 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl int_to_short_instructionVar164 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("int_to_short"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1656:1, end:1656:2))"]
#[derive(Clone, Debug)]
struct add_int_instructionVar165 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl add_int_instructionVar165 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1660:1, end:1660:2))"]
#[derive(Clone, Debug)]
struct sub_int_instructionVar166 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl sub_int_instructionVar166 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1664:1, end:1664:2))"]
#[derive(Clone, Debug)]
struct mul_int_instructionVar167 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl mul_int_instructionVar167 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1668:1, end:1668:2))"]
#[derive(Clone, Debug)]
struct div_int_instructionVar168 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl div_int_instructionVar168 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1672:1, end:1672:2))"]
#[derive(Clone, Debug)]
struct rem_int_instructionVar169 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl rem_int_instructionVar169 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1676:1, end:1676:2))"]
#[derive(Clone, Debug)]
struct and_int_instructionVar170 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl and_int_instructionVar170 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1680:1, end:1680:2))"]
#[derive(Clone, Debug)]
struct or_int_instructionVar171 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl or_int_instructionVar171 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1684:1, end:1684:2))"]
#[derive(Clone, Debug)]
struct xor_int_instructionVar172 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl xor_int_instructionVar172 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1688:1, end:1688:2))"]
#[derive(Clone, Debug)]
struct shl_int_instructionVar173 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl shl_int_instructionVar173 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1692:1, end:1692:2))"]
#[derive(Clone, Debug)]
struct shr_int_instructionVar174 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl shr_int_instructionVar174 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1696:1, end:1696:2))"]
#[derive(Clone, Debug)]
struct ushr_int_instructionVar175 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl ushr_int_instructionVar175 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ushr_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1700:1, end:1700:2))"]
#[derive(Clone, Debug)]
struct add_long_instructionVar176 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl add_long_instructionVar176 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1704:1, end:1704:2))"]
#[derive(Clone, Debug)]
struct sub_long_instructionVar177 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl sub_long_instructionVar177 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1708:1, end:1708:2))"]
#[derive(Clone, Debug)]
struct mul_long_instructionVar178 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl mul_long_instructionVar178 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1712:1, end:1712:2))"]
#[derive(Clone, Debug)]
struct div_long_instructionVar179 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl div_long_instructionVar179 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1716:1, end:1716:2))"]
#[derive(Clone, Debug)]
struct rem_long_instructionVar180 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl rem_long_instructionVar180 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1720:1, end:1720:2))"]
#[derive(Clone, Debug)]
struct and_long_instructionVar181 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl and_long_instructionVar181 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1724:1, end:1724:2))"]
#[derive(Clone, Debug)]
struct or_long_instructionVar182 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl or_long_instructionVar182 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1728:1, end:1728:2))"]
#[derive(Clone, Debug)]
struct xor_long_instructionVar183 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl xor_long_instructionVar183 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1732:1, end:1732:2))"]
#[derive(Clone, Debug)]
struct shl_long_instructionVar184 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8: TableregisterC8,
}
impl shl_long_instructionVar184 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1736:1, end:1736:2))"]
#[derive(Clone, Debug)]
struct shr_long_instructionVar185 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8: TableregisterC8,
}
impl shr_long_instructionVar185 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1740:1, end:1740:2))"]
#[derive(Clone, Debug)]
struct ushr_long_instructionVar186 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8: TableregisterC8,
}
impl ushr_long_instructionVar186 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ushr_long"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1744:1, end:1744:2))"]
#[derive(Clone, Debug)]
struct add_float_instructionVar187 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl add_float_instructionVar187 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1748:1, end:1748:2))"]
#[derive(Clone, Debug)]
struct sub_float_instructionVar188 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl sub_float_instructionVar188 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1752:1, end:1752:2))"]
#[derive(Clone, Debug)]
struct mul_float_instructionVar189 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl mul_float_instructionVar189 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1756:1, end:1756:2))"]
#[derive(Clone, Debug)]
struct div_float_instructionVar190 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl div_float_instructionVar190 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1760:1, end:1760:2))"]
#[derive(Clone, Debug)]
struct rem_float_instructionVar191 {
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
    registerC8: TableregisterC8,
}
impl rem_float_instructionVar191 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_float"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8 = if let Some((len, table)) =
            TableregisterC8::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8,
                registerB8,
                registerC8,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1764:1, end:1764:2))"]
#[derive(Clone, Debug)]
struct add_double_instructionVar192 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl add_double_instructionVar192 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1768:1, end:1768:2))"]
#[derive(Clone, Debug)]
struct sub_double_instructionVar193 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl sub_double_instructionVar193 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1772:1, end:1772:2))"]
#[derive(Clone, Debug)]
struct mul_double_instructionVar194 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl mul_double_instructionVar194 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1776:1, end:1776:2))"]
#[derive(Clone, Debug)]
struct div_double_instructionVar195 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl div_double_instructionVar195 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1780:1, end:1780:2))"]
#[derive(Clone, Debug)]
struct rem_double_instructionVar196 {
    registerA8w: TableregisterA8w,
    registerB8w: TableregisterB8w,
    registerC8w: TableregisterC8w,
}
impl rem_double_instructionVar196 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_double"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerC8w
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
        let registerA8w = if let Some((len, table)) =
            TableregisterA8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8w = if let Some((len, table)) =
            TableregisterB8w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let registerC8w = if let Some((len, table)) =
            TableregisterC8w::parse(tokens_current, &mut context_instance, inst_start)
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
                registerA8w,
                registerB8w,
                registerC8w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1794:1, end:1794:2))"]
#[derive(Clone, Debug)]
struct add_int_2addr_instructionVar197 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl add_int_2addr_instructionVar197 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1798:1, end:1798:2))"]
#[derive(Clone, Debug)]
struct sub_int_2addr_instructionVar198 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl sub_int_2addr_instructionVar198 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1802:1, end:1802:2))"]
#[derive(Clone, Debug)]
struct mul_int_2addr_instructionVar199 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl mul_int_2addr_instructionVar199 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1806:1, end:1806:2))"]
#[derive(Clone, Debug)]
struct div_int_2addr_instructionVar200 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl div_int_2addr_instructionVar200 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1810:1, end:1810:2))"]
#[derive(Clone, Debug)]
struct rem_int_2addr_instructionVar201 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl rem_int_2addr_instructionVar201 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1814:1, end:1814:2))"]
#[derive(Clone, Debug)]
struct and_int_2addr_instructionVar202 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl and_int_2addr_instructionVar202 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1818:1, end:1818:2))"]
#[derive(Clone, Debug)]
struct or_int_2addr_instructionVar203 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl or_int_2addr_instructionVar203 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1822:1, end:1822:2))"]
#[derive(Clone, Debug)]
struct xor_int_2addr_instructionVar204 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl xor_int_2addr_instructionVar204 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1826:1, end:1826:2))"]
#[derive(Clone, Debug)]
struct shl_int_2addr_instructionVar205 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl shl_int_2addr_instructionVar205 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1830:1, end:1830:2))"]
#[derive(Clone, Debug)]
struct shr_int_2addr_instructionVar206 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl shr_int_2addr_instructionVar206 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1834:1, end:1834:2))"]
#[derive(Clone, Debug)]
struct ushr_int_2addr_instructionVar207 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl ushr_int_2addr_instructionVar207 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ushr_int_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1838:1, end:1838:2))"]
#[derive(Clone, Debug)]
struct add_long_2addr_instructionVar208 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl add_long_2addr_instructionVar208 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1842:1, end:1842:2))"]
#[derive(Clone, Debug)]
struct sub_long_2addr_instructionVar209 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl sub_long_2addr_instructionVar209 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1846:1, end:1846:2))"]
#[derive(Clone, Debug)]
struct mul_long_2addr_instructionVar210 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl mul_long_2addr_instructionVar210 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1850:1, end:1850:2))"]
#[derive(Clone, Debug)]
struct div_long_2addr_instructionVar211 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl div_long_2addr_instructionVar211 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1854:1, end:1854:2))"]
#[derive(Clone, Debug)]
struct rem_long_2addr_instructionVar212 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl rem_long_2addr_instructionVar212 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1858:1, end:1858:2))"]
#[derive(Clone, Debug)]
struct and_long_2addr_instructionVar213 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl and_long_2addr_instructionVar213 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1862:1, end:1862:2))"]
#[derive(Clone, Debug)]
struct or_long_2addr_instructionVar214 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl or_long_2addr_instructionVar214 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1866:1, end:1866:2))"]
#[derive(Clone, Debug)]
struct xor_long_2addr_instructionVar215 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl xor_long_2addr_instructionVar215 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1870:1, end:1870:2))"]
#[derive(Clone, Debug)]
struct shl_long_2addr_instructionVar216 {
    registerB4: TableregisterB4,
    registerA4w: TableregisterA4w,
}
impl shl_long_2addr_instructionVar216 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1874:1, end:1874:2))"]
#[derive(Clone, Debug)]
struct shr_long_2addr_instructionVar217 {
    registerA4w: TableregisterA4w,
    registerB4: TableregisterB4,
}
impl shr_long_2addr_instructionVar217 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1878:1, end:1878:2))"]
#[derive(Clone, Debug)]
struct ushr_long_2addr_instructionVar218 {
    registerB4: TableregisterB4,
    registerA4w: TableregisterA4w,
}
impl ushr_long_2addr_instructionVar218 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ushr_long_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1882:1, end:1882:2))"]
#[derive(Clone, Debug)]
struct add_float_2addr_instructionVar219 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl add_float_2addr_instructionVar219 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_float_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1886:1, end:1886:2))"]
#[derive(Clone, Debug)]
struct sub_float_2addr_instructionVar220 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl sub_float_2addr_instructionVar220 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_float_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1890:1, end:1890:2))"]
#[derive(Clone, Debug)]
struct mul_float_2addr_instructionVar221 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl mul_float_2addr_instructionVar221 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_float_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1894:1, end:1894:2))"]
#[derive(Clone, Debug)]
struct div_float_2addr_instructionVar222 {
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl div_float_2addr_instructionVar222 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_float_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1898:1, end:1898:2))"]
#[derive(Clone, Debug)]
struct rem_float_2addr_instructionVar223 {
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl rem_float_2addr_instructionVar223 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_float_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1902:1, end:1902:2))"]
#[derive(Clone, Debug)]
struct add_double_2addr_instructionVar224 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl add_double_2addr_instructionVar224 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_double_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1906:1, end:1906:2))"]
#[derive(Clone, Debug)]
struct sub_double_2addr_instructionVar225 {
    registerB4w: TableregisterB4w,
    registerA4w: TableregisterA4w,
}
impl sub_double_2addr_instructionVar225 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub_double_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4w,
                registerA4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1910:1, end:1910:2))"]
#[derive(Clone, Debug)]
struct mul_double_2addr_instructionVar226 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl mul_double_2addr_instructionVar226 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_double_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1914:1, end:1914:2))"]
#[derive(Clone, Debug)]
struct div_double_2addr_instructionVar227 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl div_double_2addr_instructionVar227 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_double_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1918:1, end:1918:2))"]
#[derive(Clone, Debug)]
struct rem_double_2addr_instructionVar228 {
    registerA4w: TableregisterA4w,
    registerB4w: TableregisterB4w,
}
impl rem_double_2addr_instructionVar228 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_double_2addr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4w
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4w
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
        let registerA4w = if let Some((len, table)) =
            TableregisterA4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4w = if let Some((len, table)) =
            TableregisterB4w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4w,
                registerB4w,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1934:1, end:1934:2))"]
#[derive(Clone, Debug)]
struct add_int_lit16_instructionVar229 {
    C_BITS_0_15_S: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl add_int_lit16_instructionVar229 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct rsub_int_instructionVar230 {
    C_BITS_0_15_S: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl rsub_int_instructionVar230 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsub_int"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1942:1, end:1942:2))"]
#[derive(Clone, Debug)]
struct mul_int_lit16_instructionVar231 {
    C_BITS_0_15_S: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl mul_int_lit16_instructionVar231 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1946:1, end:1946:2))"]
#[derive(Clone, Debug)]
struct div_int_lit16_instructionVar232 {
    C_BITS_0_15_S: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl div_int_lit16_instructionVar232 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1950:1, end:1950:2))"]
#[derive(Clone, Debug)]
struct rem_int_lit16_instructionVar233 {
    C_BITS_0_15_S: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl rem_int_lit16_instructionVar233 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1954:1, end:1954:2))"]
#[derive(Clone, Debug)]
struct and_int_lit16_instructionVar234 {
    C_BITS_0_15_S: u16,
    registerA4: TableregisterA4,
    registerB4: TableregisterB4,
}
impl and_int_lit16_instructionVar234 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA4,
                registerB4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1958:1, end:1958:2))"]
#[derive(Clone, Debug)]
struct or_int_lit16_instructionVar235 {
    C_BITS_0_15_S: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl or_int_lit16_instructionVar235 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1962:1, end:1962:2))"]
#[derive(Clone, Debug)]
struct xor_int_lit16_instructionVar236 {
    C_BITS_0_15_S: u16,
    registerB4: TableregisterB4,
    registerA4: TableregisterA4,
}
impl xor_int_lit16_instructionVar236 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_int_lit16"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .is_negative(),
                (if self.C_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.C_BITS_0_15_S as i16)
                    .abs() as u64,
            ),
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
        let registerA4 = if let Some((len, table)) =
            TableregisterA4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let registerB4 = if let Some((len, table)) =
            TableregisterB4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let C_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerB4,
                registerA4,
                C_BITS_0_15_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1978:1, end:1978:2))"]
#[derive(Clone, Debug)]
struct add_int_lit8_instructionVar237 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl add_int_lit8_instructionVar237 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1982:1, end:1982:2))"]
#[derive(Clone, Debug)]
struct rsub_int_lit8_instructionVar238 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl rsub_int_lit8_instructionVar238 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsub_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1986:1, end:1986:2))"]
#[derive(Clone, Debug)]
struct mul_int_lit8_instructionVar239 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl mul_int_lit8_instructionVar239 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1990:1, end:1990:2))"]
#[derive(Clone, Debug)]
struct div_int_lit8_instructionVar240 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl div_int_lit8_instructionVar240 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1994:1, end:1994:2))"]
#[derive(Clone, Debug)]
struct rem_int_lit8_instructionVar241 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl rem_int_lit8_instructionVar241 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rem_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:1998:1, end:1998:2))"]
#[derive(Clone, Debug)]
struct and_int_lit8_instructionVar242 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl and_int_lit8_instructionVar242 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:2002:1, end:2002:2))"]
#[derive(Clone, Debug)]
struct or_int_lit8_instructionVar243 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl or_int_lit8_instructionVar243 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:2006:1, end:2006:2))"]
#[derive(Clone, Debug)]
struct xor_int_lit8_instructionVar244 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl xor_int_lit8_instructionVar244 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:2010:1, end:2010:2))"]
#[derive(Clone, Debug)]
struct shl_int_lit8_instructionVar245 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl shl_int_lit8_instructionVar245 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:2014:1, end:2014:2))"]
#[derive(Clone, Debug)]
struct shr_int_lit8_instructionVar246 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl shr_int_lit8_instructionVar246 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:2018:1, end:2018:2))"]
#[derive(Clone, Debug)]
struct ushr_int_lit8_instructionVar247 {
    C_BITS_0_7_S: u8,
    registerA8: TableregisterA8,
    registerB8: TableregisterB8,
}
impl ushr_int_lit8_instructionVar247 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ushr_int_lit8"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.registerA8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.registerB8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(
                true,
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .is_negative(),
                (if self.C_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.C_BITS_0_7_S as i8)
                    .abs() as u64,
            ),
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
        let registerA8 = if let Some((len, table)) =
            TableregisterA8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let registerB8 = if let Some((len, table)) =
            TableregisterB8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let C_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                registerA8,
                registerB8,
                C_BITS_0_7_S,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(filled_new_array_instructionVar0),
    Var1(filled_new_array_instructionVar1),
    Var2(filled_new_array_instructionVar2),
    Var3(filled_new_array_instructionVar3),
    Var4(filled_new_array_instructionVar4),
    Var5(filled_new_array_instructionVar5),
    Var6(invoke_virtual_instructionVar6),
    Var7(invoke_virtual_instructionVar7),
    Var8(invoke_virtual_instructionVar8),
    Var9(invoke_virtual_instructionVar9),
    Var10(invoke_virtual_instructionVar10),
    Var11(invoke_virtual_instructionVar11),
    Var12(invoke_super_instructionVar12),
    Var13(invoke_super_instructionVar13),
    Var14(invoke_super_instructionVar14),
    Var15(invoke_super_instructionVar15),
    Var16(invoke_super_instructionVar16),
    Var17(invoke_super_instructionVar17),
    Var18(invoke_direct_instructionVar18),
    Var19(invoke_direct_instructionVar19),
    Var20(invoke_direct_instructionVar20),
    Var21(invoke_direct_instructionVar21),
    Var22(invoke_direct_instructionVar22),
    Var23(invoke_direct_instructionVar23),
    Var24(invoke_static_instructionVar24),
    Var25(invoke_static_instructionVar25),
    Var26(invoke_static_instructionVar26),
    Var27(invoke_static_instructionVar27),
    Var28(invoke_static_instructionVar28),
    Var29(invoke_static_instructionVar29),
    Var30(invoke_interface_instructionVar30),
    Var31(invoke_interface_instructionVar31),
    Var32(invoke_interface_instructionVar32),
    Var33(invoke_interface_instructionVar33),
    Var34(invoke_interface_instructionVar34),
    Var35(invoke_interface_instructionVar35),
    Var36(nop_instructionVar36),
    Var37(move_instructionVar37),
    Var38(move_from_16_instructionVar38),
    Var39(move_16_instructionVar39),
    Var40(move_wide_instructionVar40),
    Var41(move_wide_from_16_instructionVar41),
    Var42(move_wide_16_instructionVar42),
    Var43(move_object_instructionVar43),
    Var44(move_object_from_16_instructionVar44),
    Var45(move_object_16_instructionVar45),
    Var46(move_result_instructionVar46),
    Var47(move_result_wide_instructionVar47),
    Var48(move_result_object_instructionVar48),
    Var49(move_exception_instructionVar49),
    Var50(return_void_instructionVar50),
    Var51(return_instructionVar51),
    Var52(return_wide_instructionVar52),
    Var53(return_object_instructionVar53),
    Var54(const_4_instructionVar54),
    Var55(const_16_instructionVar55),
    Var56(instructionVar56),
    Var57(const_high_16_instructionVar57),
    Var58(const_wide_16_instructionVar58),
    Var59(const_wide_32_instructionVar59),
    Var60(const_wide_instructionVar60),
    Var61(const_wide_high_16_instructionVar61),
    Var62(const_string_instructionVar62),
    Var63(const_string_jumbo_instructionVar63),
    Var64(const_class_instructionVar64),
    Var65(monitor_enter_instructionVar65),
    Var66(monitor_exit_instructionVar66),
    Var67(check_cast_instructionVar67),
    Var68(instance_of_instructionVar68),
    Var69(array_length_instructionVar69),
    Var70(new_instance_instructionVar70),
    Var71(new_array_instructionVar71),
    Var72(filled_new_array_range_instructionVar72),
    Var73(fill_array_data_instructionVar73),
    Var74(throw_instructionVar74),
    Var75(goto_instructionVar75),
    Var76(goto_16_instructionVar76),
    Var77(goto_32_instructionVar77),
    Var78(packed_switch_instructionVar78),
    Var79(sparse_switch_instructionVar79),
    Var80(cmpl_float_instructionVar80),
    Var81(cmpg_float_instructionVar81),
    Var82(cmpl_double_instructionVar82),
    Var83(cmpg_double_instructionVar83),
    Var84(cmp_long_instructionVar84),
    Var85(if_eq_instructionVar85),
    Var86(if_ne_instructionVar86),
    Var87(if_lt_instructionVar87),
    Var88(if_ge_instructionVar88),
    Var89(if_gt_instructionVar89),
    Var90(if_le_instructionVar90),
    Var91(if_eqz_instructionVar91),
    Var92(if_nez_instructionVar92),
    Var93(if_ltz_instructionVar93),
    Var94(if_gez_instructionVar94),
    Var95(if_gtz_instructionVar95),
    Var96(if_lez_instructionVar96),
    Var97(aget_instructionVar97),
    Var98(aget_wide_instructionVar98),
    Var99(aget_object_instructionVar99),
    Var100(aget_boolean_instructionVar100),
    Var101(aget_byte_instructionVar101),
    Var102(aget_char_instructionVar102),
    Var103(aget_short_instructionVar103),
    Var104(aput_instructionVar104),
    Var105(aput_wide_instructionVar105),
    Var106(aput_object_instructionVar106),
    Var107(aput_boolean_instructionVar107),
    Var108(aput_byte_instructionVar108),
    Var109(aput_char_instructionVar109),
    Var110(aput_short_instructionVar110),
    Var111(iget_instructionVar111),
    Var112(iget_wide_instructionVar112),
    Var113(iget_object_instructionVar113),
    Var114(iget_boolean_instructionVar114),
    Var115(iget_byte_instructionVar115),
    Var116(iget_char_instructionVar116),
    Var117(iget_short_instructionVar117),
    Var118(iput_instructionVar118),
    Var119(iput_wide_instructionVar119),
    Var120(iput_object_instructionVar120),
    Var121(iput_boolean_instructionVar121),
    Var122(iput_byte_instructionVar122),
    Var123(iput_char_instructionVar123),
    Var124(iput_short_instructionVar124),
    Var125(sget_instructionVar125),
    Var126(sget_wide_instructionVar126),
    Var127(sget_object_instructionVar127),
    Var128(sget_boolean_instructionVar128),
    Var129(sget_byte_instructionVar129),
    Var130(sget_char_instructionVar130),
    Var131(sget_short_instructionVar131),
    Var132(sput_instructionVar132),
    Var133(sput_wide_instructionVar133),
    Var134(sput_object_instructionVar134),
    Var135(sput_boolean_instructionVar135),
    Var136(sput_byte_instructionVar136),
    Var137(sput_char_instructionVar137),
    Var138(sput_short_instructionVar138),
    Var139(invoke_virtual_range_instructionVar139),
    Var140(invoke_super_range_instructionVar140),
    Var141(invoke_direct_range_instructionVar141),
    Var142(invoke_static_range_instructionVar142),
    Var143(invoke_interface_range_instructionVar143),
    Var144(neg_int_instructionVar144),
    Var145(not_int_instructionVar145),
    Var146(neg_long_instructionVar146),
    Var147(not_long_instructionVar147),
    Var148(neg_float_instructionVar148),
    Var149(neg_double_instructionVar149),
    Var150(int_to_long_instructionVar150),
    Var151(int_to_float_instructionVar151),
    Var152(int_to_double_instructionVar152),
    Var153(long_to_int_instructionVar153),
    Var154(long_to_float_instructionVar154),
    Var155(long_to_double_instructionVar155),
    Var156(float_to_int_instructionVar156),
    Var157(float_to_long_instructionVar157),
    Var158(float_to_double_instructionVar158),
    Var159(double_to_int_instructionVar159),
    Var160(double_to_long_instructionVar160),
    Var161(double_to_float_instructionVar161),
    Var162(int_to_byte_instructionVar162),
    Var163(int_to_char_instructionVar163),
    Var164(int_to_short_instructionVar164),
    Var165(add_int_instructionVar165),
    Var166(sub_int_instructionVar166),
    Var167(mul_int_instructionVar167),
    Var168(div_int_instructionVar168),
    Var169(rem_int_instructionVar169),
    Var170(and_int_instructionVar170),
    Var171(or_int_instructionVar171),
    Var172(xor_int_instructionVar172),
    Var173(shl_int_instructionVar173),
    Var174(shr_int_instructionVar174),
    Var175(ushr_int_instructionVar175),
    Var176(add_long_instructionVar176),
    Var177(sub_long_instructionVar177),
    Var178(mul_long_instructionVar178),
    Var179(div_long_instructionVar179),
    Var180(rem_long_instructionVar180),
    Var181(and_long_instructionVar181),
    Var182(or_long_instructionVar182),
    Var183(xor_long_instructionVar183),
    Var184(shl_long_instructionVar184),
    Var185(shr_long_instructionVar185),
    Var186(ushr_long_instructionVar186),
    Var187(add_float_instructionVar187),
    Var188(sub_float_instructionVar188),
    Var189(mul_float_instructionVar189),
    Var190(div_float_instructionVar190),
    Var191(rem_float_instructionVar191),
    Var192(add_double_instructionVar192),
    Var193(sub_double_instructionVar193),
    Var194(mul_double_instructionVar194),
    Var195(div_double_instructionVar195),
    Var196(rem_double_instructionVar196),
    Var197(add_int_2addr_instructionVar197),
    Var198(sub_int_2addr_instructionVar198),
    Var199(mul_int_2addr_instructionVar199),
    Var200(div_int_2addr_instructionVar200),
    Var201(rem_int_2addr_instructionVar201),
    Var202(and_int_2addr_instructionVar202),
    Var203(or_int_2addr_instructionVar203),
    Var204(xor_int_2addr_instructionVar204),
    Var205(shl_int_2addr_instructionVar205),
    Var206(shr_int_2addr_instructionVar206),
    Var207(ushr_int_2addr_instructionVar207),
    Var208(add_long_2addr_instructionVar208),
    Var209(sub_long_2addr_instructionVar209),
    Var210(mul_long_2addr_instructionVar210),
    Var211(div_long_2addr_instructionVar211),
    Var212(rem_long_2addr_instructionVar212),
    Var213(and_long_2addr_instructionVar213),
    Var214(or_long_2addr_instructionVar214),
    Var215(xor_long_2addr_instructionVar215),
    Var216(shl_long_2addr_instructionVar216),
    Var217(shr_long_2addr_instructionVar217),
    Var218(ushr_long_2addr_instructionVar218),
    Var219(add_float_2addr_instructionVar219),
    Var220(sub_float_2addr_instructionVar220),
    Var221(mul_float_2addr_instructionVar221),
    Var222(div_float_2addr_instructionVar222),
    Var223(rem_float_2addr_instructionVar223),
    Var224(add_double_2addr_instructionVar224),
    Var225(sub_double_2addr_instructionVar225),
    Var226(mul_double_2addr_instructionVar226),
    Var227(div_double_2addr_instructionVar227),
    Var228(rem_double_2addr_instructionVar228),
    Var229(add_int_lit16_instructionVar229),
    Var230(rsub_int_instructionVar230),
    Var231(mul_int_lit16_instructionVar231),
    Var232(div_int_lit16_instructionVar232),
    Var233(rem_int_lit16_instructionVar233),
    Var234(and_int_lit16_instructionVar234),
    Var235(or_int_lit16_instructionVar235),
    Var236(xor_int_lit16_instructionVar236),
    Var237(add_int_lit8_instructionVar237),
    Var238(rsub_int_lit8_instructionVar238),
    Var239(mul_int_lit8_instructionVar239),
    Var240(div_int_lit8_instructionVar240),
    Var241(rem_int_lit8_instructionVar241),
    Var242(and_int_lit8_instructionVar242),
    Var243(or_int_lit8_instructionVar243),
    Var244(xor_int_lit8_instructionVar244),
    Var245(shl_int_lit8_instructionVar245),
    Var246(shr_int_lit8_instructionVar246),
    Var247(ushr_int_lit8_instructionVar247),
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
            Self::Var110(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var111(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var112(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var113(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var114(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var115(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var116(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var117(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var118(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var119(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var120(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var121(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var122(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var123(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var124(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var125(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var126(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var127(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var128(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var129(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var130(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var131(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var132(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var133(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var134(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var135(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var136(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var137(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var138(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var139(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var140(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var141(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var142(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var143(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var144(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var145(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var146(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var147(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var148(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var149(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var150(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var151(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var152(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var153(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var154(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var155(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var156(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var157(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var158(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var159(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var160(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var161(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var162(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var163(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var164(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var165(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var166(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var167(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var168(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var169(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var170(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var171(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var172(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var173(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var174(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var175(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var176(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var177(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var178(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var179(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var180(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var181(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var182(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var183(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var184(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var185(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var186(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var187(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var188(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var189(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var190(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var191(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var192(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var193(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var194(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var195(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var196(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var197(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var198(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var199(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var200(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var201(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var202(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var203(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var204(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var205(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var206(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var207(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var208(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var209(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var210(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var211(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var212(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var213(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var214(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var215(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var216(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var217(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var218(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var219(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var220(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var221(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var222(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var223(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var224(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var225(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var226(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var227(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var228(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var229(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var230(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var231(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var232(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var233(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var234(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var235(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var236(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var237(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var238(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var239(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var240(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var241(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var242(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var243(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var244(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var245(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var246(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var247(x) => {
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
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar0::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar1::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar2::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar3::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar4::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 36 && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) = filled_new_array_instructionVar5::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 110 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar6::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 110
            && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar7::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 110
            && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar8::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 110
            && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar9::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 110
            && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar10::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 110
            && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) = invoke_virtual_instructionVar11::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 111 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 111
            && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 111
            && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 111
            && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 111
            && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 111
            && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) =
                invoke_super_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 112 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar18::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 112
            && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar19::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 112
            && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar20::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 112
            && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar21::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 112
            && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar22::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 112
            && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) = invoke_direct_instructionVar23::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 113 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar24::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 113
            && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar25::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 113
            && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar26::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 113
            && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar27::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 113
            && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar28::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 113
            && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) = invoke_static_instructionVar29::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 114 && (tokens_param[1] & 240) == 0
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar30::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 114
            && (tokens_param[1] & 240) == 16
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar31::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 114
            && (tokens_param[1] & 240) == 32
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar32::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 114
            && (tokens_param[1] & 240) == 48
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar33::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 114
            && (tokens_param[1] & 240) == 64
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar34::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 255) == 114
            && (tokens_param[1] & 240) == 80
        {
            if let Some((inst_len, parsed)) = invoke_interface_instructionVar35::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                nop_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                move_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                move_from_16_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 3 {
            if let Some((inst_len, parsed)) =
                move_16_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 4 {
            if let Some((inst_len, parsed)) =
                move_wide_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 5 {
            if let Some((inst_len, parsed)) = move_wide_from_16_instructionVar41::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 5 && (tokens_param[0] & 255) == 6 {
            if let Some((inst_len, parsed)) =
                move_wide_16_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                move_object_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 8 {
            if let Some((inst_len, parsed)) = move_object_from_16_instructionVar44::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 9 {
            if let Some((inst_len, parsed)) = move_object_16_instructionVar45::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 10 {
            if let Some((inst_len, parsed)) =
                move_result_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 11 {
            if let Some((inst_len, parsed)) = move_result_wide_instructionVar47::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 12 {
            if let Some((inst_len, parsed)) = move_result_object_instructionVar48::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 13 {
            if let Some((inst_len, parsed)) = move_exception_instructionVar49::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 14 {
            if let Some((inst_len, parsed)) =
                return_void_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 15 {
            if let Some((inst_len, parsed)) =
                return_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 16 {
            if let Some((inst_len, parsed)) =
                return_wide_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 17 {
            if let Some((inst_len, parsed)) = return_object_instructionVar53::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 18 {
            if let Some((inst_len, parsed)) =
                const_4_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 19 {
            if let Some((inst_len, parsed)) =
                const_16_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 20 {
            if let Some((inst_len, parsed)) =
                instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 21 {
            if let Some((inst_len, parsed)) = const_high_16_instructionVar57::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 22 {
            if let Some((inst_len, parsed)) = const_wide_16_instructionVar58::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 23 {
            if let Some((inst_len, parsed)) = const_wide_32_instructionVar59::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 10 && (tokens_param[0] & 255) == 24 {
            if let Some((inst_len, parsed)) =
                const_wide_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 25 {
            if let Some((inst_len, parsed)) = const_wide_high_16_instructionVar61::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 26 {
            if let Some((inst_len, parsed)) =
                const_string_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 27 {
            if let Some((inst_len, parsed)) = const_string_jumbo_instructionVar63::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 28 {
            if let Some((inst_len, parsed)) =
                const_class_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 29 {
            if let Some((inst_len, parsed)) = monitor_enter_instructionVar65::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 30 {
            if let Some((inst_len, parsed)) =
                monitor_exit_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 31 {
            if let Some((inst_len, parsed)) =
                check_cast_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 32 {
            if let Some((inst_len, parsed)) =
                instance_of_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 33 {
            if let Some((inst_len, parsed)) =
                array_length_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 34 {
            if let Some((inst_len, parsed)) =
                new_instance_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 35 {
            if let Some((inst_len, parsed)) =
                new_array_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 37 {
            if let Some((inst_len, parsed)) = filled_new_array_range_instructionVar72::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 38 {
            if let Some((inst_len, parsed)) = fill_array_data_instructionVar73::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 39 {
            if let Some((inst_len, parsed)) =
                throw_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 40 {
            if let Some((inst_len, parsed)) =
                goto_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 41 {
            if let Some((inst_len, parsed)) =
                goto_16_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 42 {
            if let Some((inst_len, parsed)) =
                goto_32_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 43 {
            if let Some((inst_len, parsed)) = packed_switch_instructionVar78::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 44 {
            if let Some((inst_len, parsed)) = sparse_switch_instructionVar79::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 45 {
            if let Some((inst_len, parsed)) =
                cmpl_float_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 46 {
            if let Some((inst_len, parsed)) =
                cmpg_float_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 47 {
            if let Some((inst_len, parsed)) =
                cmpl_double_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 48 {
            if let Some((inst_len, parsed)) =
                cmpg_double_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 49 {
            if let Some((inst_len, parsed)) =
                cmp_long_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 50 {
            if let Some((inst_len, parsed)) =
                if_eq_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 51 {
            if let Some((inst_len, parsed)) =
                if_ne_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 52 {
            if let Some((inst_len, parsed)) =
                if_lt_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 53 {
            if let Some((inst_len, parsed)) =
                if_ge_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 54 {
            if let Some((inst_len, parsed)) =
                if_gt_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 55 {
            if let Some((inst_len, parsed)) =
                if_le_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 56 {
            if let Some((inst_len, parsed)) =
                if_eqz_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 57 {
            if let Some((inst_len, parsed)) =
                if_nez_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 58 {
            if let Some((inst_len, parsed)) =
                if_ltz_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 59 {
            if let Some((inst_len, parsed)) =
                if_gez_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 60 {
            if let Some((inst_len, parsed)) =
                if_gtz_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 61 {
            if let Some((inst_len, parsed)) =
                if_lez_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 68 {
            if let Some((inst_len, parsed)) =
                aget_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 69 {
            if let Some((inst_len, parsed)) =
                aget_wide_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 70 {
            if let Some((inst_len, parsed)) =
                aget_object_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 71 {
            if let Some((inst_len, parsed)) = aget_boolean_instructionVar100::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 72 {
            if let Some((inst_len, parsed)) =
                aget_byte_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 73 {
            if let Some((inst_len, parsed)) =
                aget_char_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 74 {
            if let Some((inst_len, parsed)) =
                aget_short_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 75 {
            if let Some((inst_len, parsed)) =
                aput_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 76 {
            if let Some((inst_len, parsed)) =
                aput_wide_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 77 {
            if let Some((inst_len, parsed)) =
                aput_object_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 78 {
            if let Some((inst_len, parsed)) = aput_boolean_instructionVar107::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 79 {
            if let Some((inst_len, parsed)) =
                aput_byte_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 80 {
            if let Some((inst_len, parsed)) =
                aput_char_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 81 {
            if let Some((inst_len, parsed)) =
                aput_short_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 82 {
            if let Some((inst_len, parsed)) =
                iget_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 83 {
            if let Some((inst_len, parsed)) =
                iget_wide_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 84 {
            if let Some((inst_len, parsed)) =
                iget_object_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 85 {
            if let Some((inst_len, parsed)) = iget_boolean_instructionVar114::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 86 {
            if let Some((inst_len, parsed)) =
                iget_byte_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 87 {
            if let Some((inst_len, parsed)) =
                iget_char_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 88 {
            if let Some((inst_len, parsed)) =
                iget_short_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 89 {
            if let Some((inst_len, parsed)) =
                iput_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 90 {
            if let Some((inst_len, parsed)) =
                iput_wide_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 91 {
            if let Some((inst_len, parsed)) =
                iput_object_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 92 {
            if let Some((inst_len, parsed)) = iput_boolean_instructionVar121::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 93 {
            if let Some((inst_len, parsed)) =
                iput_byte_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 94 {
            if let Some((inst_len, parsed)) =
                iput_char_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 95 {
            if let Some((inst_len, parsed)) =
                iput_short_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 96 {
            if let Some((inst_len, parsed)) =
                sget_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 97 {
            if let Some((inst_len, parsed)) =
                sget_wide_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 98 {
            if let Some((inst_len, parsed)) =
                sget_object_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 99 {
            if let Some((inst_len, parsed)) = sget_boolean_instructionVar128::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 100 {
            if let Some((inst_len, parsed)) =
                sget_byte_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 101 {
            if let Some((inst_len, parsed)) =
                sget_char_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 102 {
            if let Some((inst_len, parsed)) =
                sget_short_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 103 {
            if let Some((inst_len, parsed)) =
                sput_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 104 {
            if let Some((inst_len, parsed)) =
                sput_wide_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 105 {
            if let Some((inst_len, parsed)) =
                sput_object_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 106 {
            if let Some((inst_len, parsed)) = sput_boolean_instructionVar135::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 107 {
            if let Some((inst_len, parsed)) =
                sput_byte_instructionVar136::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var136(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 108 {
            if let Some((inst_len, parsed)) =
                sput_char_instructionVar137::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var137(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 109 {
            if let Some((inst_len, parsed)) =
                sput_short_instructionVar138::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var138(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 116 {
            if let Some((inst_len, parsed)) = invoke_virtual_range_instructionVar139::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var139(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 117 {
            if let Some((inst_len, parsed)) = invoke_super_range_instructionVar140::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var140(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 118 {
            if let Some((inst_len, parsed)) = invoke_direct_range_instructionVar141::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var141(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 119 {
            if let Some((inst_len, parsed)) = invoke_static_range_instructionVar142::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var142(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 120 {
            if let Some((inst_len, parsed)) = invoke_interface_range_instructionVar143::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var143(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 123 {
            if let Some((inst_len, parsed)) =
                neg_int_instructionVar144::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var144(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 124 {
            if let Some((inst_len, parsed)) =
                not_int_instructionVar145::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var145(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 125 {
            if let Some((inst_len, parsed)) =
                neg_long_instructionVar146::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var146(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 126 {
            if let Some((inst_len, parsed)) =
                not_long_instructionVar147::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var147(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 127 {
            if let Some((inst_len, parsed)) =
                neg_float_instructionVar148::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var148(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 128 {
            if let Some((inst_len, parsed)) =
                neg_double_instructionVar149::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var149(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 129 {
            if let Some((inst_len, parsed)) =
                int_to_long_instructionVar150::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var150(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 130 {
            if let Some((inst_len, parsed)) = int_to_float_instructionVar151::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var151(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 131 {
            if let Some((inst_len, parsed)) = int_to_double_instructionVar152::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var152(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 132 {
            if let Some((inst_len, parsed)) =
                long_to_int_instructionVar153::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var153(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 133 {
            if let Some((inst_len, parsed)) = long_to_float_instructionVar154::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var154(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 134 {
            if let Some((inst_len, parsed)) = long_to_double_instructionVar155::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var155(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 135 {
            if let Some((inst_len, parsed)) = float_to_int_instructionVar156::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var156(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 136 {
            if let Some((inst_len, parsed)) = float_to_long_instructionVar157::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var157(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 137 {
            if let Some((inst_len, parsed)) = float_to_double_instructionVar158::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var158(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 138 {
            if let Some((inst_len, parsed)) = double_to_int_instructionVar159::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var159(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 139 {
            if let Some((inst_len, parsed)) = double_to_long_instructionVar160::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var160(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 140 {
            if let Some((inst_len, parsed)) = double_to_float_instructionVar161::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var161(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 141 {
            if let Some((inst_len, parsed)) =
                int_to_byte_instructionVar162::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var162(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 142 {
            if let Some((inst_len, parsed)) =
                int_to_char_instructionVar163::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var163(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 143 {
            if let Some((inst_len, parsed)) = int_to_short_instructionVar164::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var164(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 144 {
            if let Some((inst_len, parsed)) =
                add_int_instructionVar165::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var165(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 145 {
            if let Some((inst_len, parsed)) =
                sub_int_instructionVar166::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var166(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 146 {
            if let Some((inst_len, parsed)) =
                mul_int_instructionVar167::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var167(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 147 {
            if let Some((inst_len, parsed)) =
                div_int_instructionVar168::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var168(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 148 {
            if let Some((inst_len, parsed)) =
                rem_int_instructionVar169::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var169(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 149 {
            if let Some((inst_len, parsed)) =
                and_int_instructionVar170::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var170(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 150 {
            if let Some((inst_len, parsed)) =
                or_int_instructionVar171::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var171(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 151 {
            if let Some((inst_len, parsed)) =
                xor_int_instructionVar172::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var172(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 152 {
            if let Some((inst_len, parsed)) =
                shl_int_instructionVar173::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var173(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 153 {
            if let Some((inst_len, parsed)) =
                shr_int_instructionVar174::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var174(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 154 {
            if let Some((inst_len, parsed)) =
                ushr_int_instructionVar175::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var175(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 155 {
            if let Some((inst_len, parsed)) =
                add_long_instructionVar176::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var176(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 156 {
            if let Some((inst_len, parsed)) =
                sub_long_instructionVar177::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var177(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 157 {
            if let Some((inst_len, parsed)) =
                mul_long_instructionVar178::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var178(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 158 {
            if let Some((inst_len, parsed)) =
                div_long_instructionVar179::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var179(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 159 {
            if let Some((inst_len, parsed)) =
                rem_long_instructionVar180::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var180(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 160 {
            if let Some((inst_len, parsed)) =
                and_long_instructionVar181::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var181(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 161 {
            if let Some((inst_len, parsed)) =
                or_long_instructionVar182::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var182(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 162 {
            if let Some((inst_len, parsed)) =
                xor_long_instructionVar183::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var183(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 163 {
            if let Some((inst_len, parsed)) =
                shl_long_instructionVar184::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var184(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 164 {
            if let Some((inst_len, parsed)) =
                shr_long_instructionVar185::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var185(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 165 {
            if let Some((inst_len, parsed)) =
                ushr_long_instructionVar186::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var186(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 166 {
            if let Some((inst_len, parsed)) =
                add_float_instructionVar187::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var187(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 167 {
            if let Some((inst_len, parsed)) =
                sub_float_instructionVar188::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var188(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 168 {
            if let Some((inst_len, parsed)) =
                mul_float_instructionVar189::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var189(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 169 {
            if let Some((inst_len, parsed)) =
                div_float_instructionVar190::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var190(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 170 {
            if let Some((inst_len, parsed)) =
                rem_float_instructionVar191::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var191(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 171 {
            if let Some((inst_len, parsed)) =
                add_double_instructionVar192::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var192(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 172 {
            if let Some((inst_len, parsed)) =
                sub_double_instructionVar193::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var193(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 173 {
            if let Some((inst_len, parsed)) =
                mul_double_instructionVar194::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var194(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 174 {
            if let Some((inst_len, parsed)) =
                div_double_instructionVar195::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var195(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 175 {
            if let Some((inst_len, parsed)) =
                rem_double_instructionVar196::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var196(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 176 {
            if let Some((inst_len, parsed)) = add_int_2addr_instructionVar197::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var197(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 177 {
            if let Some((inst_len, parsed)) = sub_int_2addr_instructionVar198::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var198(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 178 {
            if let Some((inst_len, parsed)) = mul_int_2addr_instructionVar199::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var199(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 179 {
            if let Some((inst_len, parsed)) = div_int_2addr_instructionVar200::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var200(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 180 {
            if let Some((inst_len, parsed)) = rem_int_2addr_instructionVar201::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var201(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 181 {
            if let Some((inst_len, parsed)) = and_int_2addr_instructionVar202::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var202(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 182 {
            if let Some((inst_len, parsed)) = or_int_2addr_instructionVar203::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var203(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 183 {
            if let Some((inst_len, parsed)) = xor_int_2addr_instructionVar204::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var204(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 184 {
            if let Some((inst_len, parsed)) = shl_int_2addr_instructionVar205::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var205(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 185 {
            if let Some((inst_len, parsed)) = shr_int_2addr_instructionVar206::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var206(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 186 {
            if let Some((inst_len, parsed)) = ushr_int_2addr_instructionVar207::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var207(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 187 {
            if let Some((inst_len, parsed)) = add_long_2addr_instructionVar208::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var208(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 188 {
            if let Some((inst_len, parsed)) = sub_long_2addr_instructionVar209::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var209(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 189 {
            if let Some((inst_len, parsed)) = mul_long_2addr_instructionVar210::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var210(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 190 {
            if let Some((inst_len, parsed)) = div_long_2addr_instructionVar211::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var211(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 191 {
            if let Some((inst_len, parsed)) = rem_long_2addr_instructionVar212::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var212(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 192 {
            if let Some((inst_len, parsed)) = and_long_2addr_instructionVar213::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var213(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 193 {
            if let Some((inst_len, parsed)) = or_long_2addr_instructionVar214::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var214(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 194 {
            if let Some((inst_len, parsed)) = xor_long_2addr_instructionVar215::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var215(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) = shl_long_2addr_instructionVar216::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var216(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 196 {
            if let Some((inst_len, parsed)) = shr_long_2addr_instructionVar217::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var217(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 197 {
            if let Some((inst_len, parsed)) = ushr_long_2addr_instructionVar218::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var218(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) = add_float_2addr_instructionVar219::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var219(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 199 {
            if let Some((inst_len, parsed)) = sub_float_2addr_instructionVar220::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var220(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 200 {
            if let Some((inst_len, parsed)) = mul_float_2addr_instructionVar221::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var221(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) = div_float_2addr_instructionVar222::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var222(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 202 {
            if let Some((inst_len, parsed)) = rem_float_2addr_instructionVar223::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var223(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 {
            if let Some((inst_len, parsed)) = add_double_2addr_instructionVar224::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var224(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 204 {
            if let Some((inst_len, parsed)) = sub_double_2addr_instructionVar225::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var225(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) = mul_double_2addr_instructionVar226::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var226(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) = div_double_2addr_instructionVar227::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var227(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 207 {
            if let Some((inst_len, parsed)) = rem_double_2addr_instructionVar228::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var228(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 208 {
            if let Some((inst_len, parsed)) = add_int_lit16_instructionVar229::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var229(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 209 {
            if let Some((inst_len, parsed)) =
                rsub_int_instructionVar230::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var230(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 210 {
            if let Some((inst_len, parsed)) = mul_int_lit16_instructionVar231::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var231(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 211 {
            if let Some((inst_len, parsed)) = div_int_lit16_instructionVar232::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var232(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 212 {
            if let Some((inst_len, parsed)) = rem_int_lit16_instructionVar233::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var233(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 213 {
            if let Some((inst_len, parsed)) = and_int_lit16_instructionVar234::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var234(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 214 {
            if let Some((inst_len, parsed)) = or_int_lit16_instructionVar235::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var235(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 215 {
            if let Some((inst_len, parsed)) = xor_int_lit16_instructionVar236::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var236(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 216 {
            if let Some((inst_len, parsed)) = add_int_lit8_instructionVar237::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var237(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 217 {
            if let Some((inst_len, parsed)) = rsub_int_lit8_instructionVar238::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var238(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 218 {
            if let Some((inst_len, parsed)) = mul_int_lit8_instructionVar239::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var239(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 219 {
            if let Some((inst_len, parsed)) = div_int_lit8_instructionVar240::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var240(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 220 {
            if let Some((inst_len, parsed)) = rem_int_lit8_instructionVar241::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var241(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 221 {
            if let Some((inst_len, parsed)) = and_int_lit8_instructionVar242::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var242(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 222 {
            if let Some((inst_len, parsed)) =
                or_int_lit8_instructionVar243::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var243(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 223 {
            if let Some((inst_len, parsed)) = xor_int_lit8_instructionVar244::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var244(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 224 {
            if let Some((inst_len, parsed)) = shl_int_lit8_instructionVar245::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var245(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 225 {
            if let Some((inst_len, parsed)) = shr_int_lit8_instructionVar246::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var246(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 255) == 226 {
            if let Some((inst_len, parsed)) = ushr_int_lit8_instructionVar247::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var247(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:206:1, end:206:11))"]
#[derive(Clone, Debug)]
struct registerA4Var0 {
    A_BITS_0_3: u8,
}
impl registerA4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_3)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_4(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA4 {
    Var0(registerA4Var0),
}
impl TableregisterA4 {
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
                registerA4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:207:1, end:207:11))"]
#[derive(Clone, Debug)]
struct registerA8Var0 {
    A_BITS_0_7: u8,
}
impl registerA8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA8 {
    Var0(registerA8Var0),
}
impl TableregisterA8 {
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
                registerA8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:208:1, end:208:12))"]
#[derive(Clone, Debug)]
struct registerA16Var0 {
    A_BITS_0_15: u16,
}
impl registerA16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA16 {
    Var0(registerA16Var0),
}
impl TableregisterA16 {
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
                registerA16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:210:1, end:210:12))"]
#[derive(Clone, Debug)]
struct registerA4wVar0 {
    A_BITS_0_3: u8,
}
impl registerA4wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_3)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_4(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA4w {
    Var0(registerA4wVar0),
}
impl TableregisterA4w {
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
                registerA4wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:211:1, end:211:12))"]
#[derive(Clone, Debug)]
struct registerA8wVar0 {
    A_BITS_0_7: u8,
}
impl registerA8wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA8w {
    Var0(registerA8wVar0),
}
impl TableregisterA8w {
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
                registerA8wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:212:1, end:212:13))"]
#[derive(Clone, Debug)]
struct registerA16wVar0 {
    A_BITS_0_15: u16,
}
impl registerA16wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.A_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let A_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterA16w {
    Var0(registerA16wVar0),
}
impl TableregisterA16w {
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
                registerA16wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:214:1, end:214:11))"]
#[derive(Clone, Debug)]
struct registerB4Var0 {
    B_BITS_4_7: u8,
}
impl registerB4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_4_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_5(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_4_7 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_4_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB4 {
    Var0(registerB4Var0),
}
impl TableregisterB4 {
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
                registerB4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:215:1, end:215:11))"]
#[derive(Clone, Debug)]
struct registerB8Var0 {
    B_BITS_0_7: u8,
}
impl registerB8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB8 {
    Var0(registerB8Var0),
}
impl TableregisterB8 {
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
                registerB8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:216:1, end:216:12))"]
#[derive(Clone, Debug)]
struct registerB16Var0 {
    B_BITS_0_15: u16,
}
impl registerB16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB16 {
    Var0(registerB16Var0),
}
impl TableregisterB16 {
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
                registerB16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:218:1, end:218:12))"]
#[derive(Clone, Debug)]
struct registerB4wVar0 {
    B_BITS_4_7: u8,
}
impl registerB4wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_4_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_5(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_4_7 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_4_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB4w {
    Var0(registerB4wVar0),
}
impl TableregisterB4w {
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
                registerB4wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:219:1, end:219:12))"]
#[derive(Clone, Debug)]
struct registerB8wVar0 {
    B_BITS_0_7: u8,
}
impl registerB8wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB8w {
    Var0(registerB8wVar0),
}
impl TableregisterB8w {
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
                registerB8wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:220:1, end:220:13))"]
#[derive(Clone, Debug)]
struct registerB16wVar0 {
    B_BITS_0_15: u16,
}
impl registerB16wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.B_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let B_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { B_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterB16w {
    Var0(registerB16wVar0),
}
impl TableregisterB16w {
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
                registerB16wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:222:1, end:222:11))"]
#[derive(Clone, Debug)]
struct registerC8Var0 {
    C_BITS_0_7: u8,
}
impl registerC8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC8 {
    Var0(registerC8Var0),
}
impl TableregisterC8 {
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
                registerC8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:223:1, end:223:12))"]
#[derive(Clone, Debug)]
struct registerC16Var0 {
    C_BITS_0_15: u16,
}
impl registerC16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC16 {
    Var0(registerC16Var0),
}
impl TableregisterC16 {
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
                registerC16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:224:1, end:224:12))"]
#[derive(Clone, Debug)]
struct registerC32Var0 {
    C_BITS_0_31: u32,
}
impl registerC32Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_31)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 4;
        calc_reg = i128::try_from(token_6(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_31 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_31 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC32 {
    Var0(registerC32Var0),
}
impl TableregisterC32 {
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
        if tokens_param.len() >= 4 {
            if let Some((inst_len, parsed)) =
                registerC32Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:227:1, end:227:12))"]
#[derive(Clone, Debug)]
struct registerC8wVar0 {
    C_BITS_0_7: u8,
}
impl registerC8wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_7)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 1;
        calc_reg = i128::try_from(token_1(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_7 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC8w {
    Var0(registerC8wVar0),
}
impl TableregisterC8w {
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
                registerC8wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:228:1, end:228:13))"]
#[derive(Clone, Debug)]
struct registerC16wVar0 {
    C_BITS_0_15: u16,
}
impl registerC16wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_15)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 2;
        calc_reg = i128::try_from(token_3(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_15 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC16w {
    Var0(registerC16wVar0),
}
impl TableregisterC16w {
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
                registerC16wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:229:1, end:229:13))"]
#[derive(Clone, Debug)]
struct registerC32wVar0 {
    C_BITS_0_31: u32,
}
impl registerC32wVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.C_BITS_0_31)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 4;
        calc_reg = i128::try_from(token_6(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let C_BITS_0_31 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { C_BITS_0_31 }))
    }
}
#[derive(Clone, Debug)]
enum TableregisterC32w {
    Var0(registerC32wVar0),
}
impl TableregisterC32w {
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
        if tokens_param.len() >= 4 {
            if let Some((inst_len, parsed)) =
                registerC32wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:231:1, end:231:10))"]
#[derive(Clone, Debug)]
struct regParamCVar0 {
    PARAM_C: u8,
}
impl regParamCVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.PARAM_C)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_11(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let PARAM_C = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PARAM_C }))
    }
}
#[derive(Clone, Debug)]
enum TableregParamC {
    Var0(regParamCVar0),
}
impl TableregParamC {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regParamCVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:232:1, end:232:10))"]
#[derive(Clone, Debug)]
struct regParamDVar0 {
    PARAM_D: u8,
}
impl regParamDVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.PARAM_D)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_10(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let PARAM_D = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PARAM_D }))
    }
}
#[derive(Clone, Debug)]
enum TableregParamD {
    Var0(regParamDVar0),
}
impl TableregParamD {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regParamDVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:233:1, end:233:10))"]
#[derive(Clone, Debug)]
struct regParamEVar0 {
    PARAM_E: u8,
}
impl regParamEVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.PARAM_E)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_13(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let PARAM_E = token_13(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PARAM_E }))
    }
}
#[derive(Clone, Debug)]
enum TableregParamE {
    Var0(regParamEVar0),
}
impl TableregParamE {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regParamEVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:234:1, end:234:10))"]
#[derive(Clone, Debug)]
struct regParamFVar0 {
    PARAM_F: u8,
}
impl regParamFVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.PARAM_F)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_12(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let PARAM_F = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PARAM_F }))
    }
}
#[derive(Clone, Debug)]
enum TableregParamF {
    Var0(regParamFVar0),
}
impl TableregParamF {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regParamFVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:235:1, end:235:10))"]
#[derive(Clone, Debug)]
struct regParamGVar0 {
    PARAM_G: u8,
}
impl regParamGVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.PARAM_G)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_8(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let PARAM_G = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PARAM_G }))
    }
}
#[derive(Clone, Debug)]
enum TableregParamG {
    Var0(regParamGVar0),
}
impl TableregParamG {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regParamGVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:237:1, end:237:9))"]
#[derive(Clone, Debug)]
struct regElemCVar0 {
    ELEMENT_C: u8,
}
impl regElemCVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.ELEMENT_C)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_11(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let ELEMENT_C = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ELEMENT_C }))
    }
}
#[derive(Clone, Debug)]
enum TableregElemC {
    Var0(regElemCVar0),
}
impl TableregElemC {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regElemCVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:238:1, end:238:9))"]
#[derive(Clone, Debug)]
struct regElemDVar0 {
    ELEMENT_D: u8,
}
impl regElemDVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.ELEMENT_D)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_10(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let ELEMENT_D = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ELEMENT_D }))
    }
}
#[derive(Clone, Debug)]
enum TableregElemD {
    Var0(regElemDVar0),
}
impl TableregElemD {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regElemDVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:239:1, end:239:9))"]
#[derive(Clone, Debug)]
struct regElemEVar0 {
    ELEMENT_E: u8,
}
impl regElemEVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.ELEMENT_E)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_13(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let ELEMENT_E = token_13(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ELEMENT_E }))
    }
}
#[derive(Clone, Debug)]
enum TableregElemE {
    Var0(regElemEVar0),
}
impl TableregElemE {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regElemEVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:240:1, end:240:9))"]
#[derive(Clone, Debug)]
struct regElemFVar0 {
    ELEMENT_F: u8,
}
impl regElemFVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.ELEMENT_F)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_12(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let ELEMENT_F = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ELEMENT_F }))
    }
}
#[derive(Clone, Debug)]
enum TableregElemF {
    Var0(regElemFVar0),
}
impl TableregElemF {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regElemFVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:241:1, end:241:9))"]
#[derive(Clone, Debug)]
struct regElemGVar0 {
    ELEMENT_G: u8,
}
impl regElemGVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reg: i128 = 0;
        calc_reg = i128::try_from(self.ELEMENT_G)
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reg.is_negative(),
            calc_reg.abs() as u64,
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
        let mut calc_reg: i128 = 0;
        let mut block_0_len = 5;
        calc_reg = i128::try_from(token_8(tokens_current))
            .unwrap()
            .wrapping_mul(4i128)
            .wrapping_add(4096i128);
        let ELEMENT_G = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ELEMENT_G }))
    }
}
#[derive(Clone, Debug)]
enum TableregElemG {
    Var0(regElemGVar0),
}
impl TableregElemG {
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
        if tokens_param.len() >= 5 {
            if let Some((inst_len, parsed)) =
                regElemGVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:246:1, end:246:6))"]
#[derive(Clone, Debug)]
struct rel16Var0 {
    A_BITS_0_15_S: u16,
}
impl rel16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.A_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.A_BITS_0_15_S as i16),
            )
            .unwrap()
            .wrapping_mul(2i128),
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
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(token_3(tokens_current))
                .unwrap()
                .wrapping_mul(2i128),
        );
        let A_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_15_S }))
    }
}
#[derive(Clone, Debug)]
enum Tablerel16 {
    Var0(rel16Var0),
}
impl Tablerel16 {
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
                rel16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:248:1, end:248:6))"]
#[derive(Clone, Debug)]
struct goto8Var0 {
    A_BITS_0_7_S: u8,
}
impl goto8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.A_BITS_0_7_S & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.A_BITS_0_7_S as i8),
            )
            .unwrap()
            .wrapping_mul(2i128),
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
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(token_1(tokens_current))
                .unwrap()
                .wrapping_mul(2i128),
        );
        let A_BITS_0_7_S = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_7_S }))
    }
}
#[derive(Clone, Debug)]
enum Tablegoto8 {
    Var0(goto8Var0),
}
impl Tablegoto8 {
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
                goto8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:249:1, end:249:7))"]
#[derive(Clone, Debug)]
struct goto16Var0 {
    A_BITS_0_15_S: u16,
}
impl goto16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.A_BITS_0_15_S & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.A_BITS_0_15_S as i16),
            )
            .unwrap()
            .wrapping_mul(2i128),
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
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(token_3(tokens_current))
                .unwrap()
                .wrapping_mul(2i128),
        );
        let A_BITS_0_15_S = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_15_S }))
    }
}
#[derive(Clone, Debug)]
enum Tablegoto16 {
    Var0(goto16Var0),
}
impl Tablegoto16 {
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
                goto16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Dalvik/data/languages/Dalvik_Base.sinc, start:250:1, end:250:7))"]
#[derive(Clone, Debug)]
struct goto32Var0 {
    A_BITS_0_31_S: u32,
}
impl goto32Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.A_BITS_0_31_S & 2147483648 != 0 {
                    -1 & !2147483647
                } else {
                    0
                } | self.A_BITS_0_31_S as i32),
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
        let mut block_0_len = 4;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(i128::try_from(token_6(tokens_current)).unwrap());
        let A_BITS_0_31_S = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { A_BITS_0_31_S }))
    }
}
#[derive(Clone, Debug)]
enum Tablegoto32 {
    Var0(goto32Var0),
}
impl Tablegoto32 {
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
        if tokens_param.len() >= 4 {
            if let Some((inst_len, parsed)) =
                goto32Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
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
) -> Option<(u32, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
