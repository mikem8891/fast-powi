
macro_rules! sq {
    ($e:expr) => {crate::PowU8::sq($e)};
}

macro_rules! cb {
    ($e:expr) => {crate::PowU8::cb($e)};
}

macro_rules! pow {
    ($n:ident^0) => {num_traits::One::one()};
    ($n:ident^1) => {($n).clone()};
    ($n:ident^2) => {sq!($n)};
    ($n:ident^3) => {cb!($n)};
    ($n:ident^4) => {sq!(sq!($n))};
    ($n:ident^5) => {$n * pow!($n^4)};
    ($n:ident^6) => {sq!(cb!($n))};
    ($n:ident^7) => {$n * pow!($n^6)};
    ($n:ident^8) => {sq!(pow!($n^4))};
    ($n:ident^9) => {$n * pow!($n^8)};
    ($n:ident^10) => {sq!(pow!($n^5))};
    ($n:ident^11) => {$n * pow!($n^10)};
    ($n:ident^12) => {sq!(pow!($n^6))};
    ($n:ident^13) => {$n * pow!($n^12)};
    ($n:ident^14) => {sq!(pow!($n^7))};
    ($n:ident^15) => {cb!(pow!($n^5))};
    ($n:ident^16) => {sq!(pow!($n^8))};
    ($n:ident^17) => {$n * pow!($n^16)};
    ($n:ident^18) => {sq!(pow!($n^9))};
    ($n:ident^19) => {$n * pow!($n^18)};
    ($n:ident^20) => {sq!(pow!($n^10))};
    ($n:ident^21) => {$n * pow!($n^20)};
    ($n:ident^22) => {sq!(pow!($n^11))};
    ($n:ident^23) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        n3 * pow!(n5^4)
    }};
    ($n:ident^24) => {sq!(pow!($n^12))};
    ($n:ident^25) => {$n * pow!($n^24)};
    ($n:ident^26) => {sq!(pow!($n^13))};
    ($n:ident^27) => {cb!(pow!($n^9))};
    ($n:ident^28) => {sq!(pow!($n^14))};
    ($n:ident^29) => {$n * pow!($n^28)};
    ($n:ident^30) => {sq!(pow!($n^15))};
    ($n:ident^31) => {$n * pow!($n^30)};
    ($n:ident^32) => {sq!(pow!($n^16))};
    ($n:ident^33) => {$n * pow!($n^32)};
    ($n:ident^34) => {sq!(pow!($n^17))};
    ($n:ident^35) => {$n * pow!($n^34)};
    ($n:ident^36) => {sq!(pow!($n^18))};
    ($n:ident^37) => {$n * pow!($n^36)};
    ($n:ident^38) => {sq!(pow!($n^19))};
    ($n:ident^39) => {cb!(pow!($n^13))};
    ($n:ident^40) => {sq!(pow!($n^20))};
    ($n:ident^41) => {$n * pow!($n^40)};
    ($n:ident^42) => {sq!(pow!($n^21))};
    ($n:ident^43) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        n3 * pow!(n5^8)
    }};
    ($n:ident^44) => {sq!(pow!($n^22))};
    ($n:ident^45) => {cb!(pow!($n^15))};
    ($n:ident^46) => {sq!(pow!($n^23))};
    ($n:ident^47) => {$n * pow!($n^46)};
    ($n:ident^48) => {sq!(pow!($n^24))};
    ($n:ident^49) => {$n * pow!($n^48)};
    ($n:ident^50) => {sq!(pow!($n^25))};
    ($n:ident^51) => {cb!(pow!($n^17))};
    ($n:ident^52) => {sq!(pow!($n^26))};
    ($n:ident^53) => {$n * pow!($n^52)};
    ($n:ident^54) => {sq!(pow!($n^27))};
    ($n:ident^55) => {$n * pow!($n^54)};
    ($n:ident^56) => {sq!(pow!($n^28))};
    ($n:ident^57) => {$n * pow!($n^56)};
    ($n:ident^58) => {sq!(pow!($n^29))};
    ($n:ident^59) => {{
        let n3 = cb!($n);
        let n7 = &($n * &n3 * &n3);
        let n56 = pow!(n7^8);
        n3 * n56
    }};
    ($n:ident^60) => {sq!(pow!($n^30))};
    ($n:ident^61) => {$n * pow!($n^60)};
    ($n:ident^62) => {sq!(pow!($n^31))};
    ($n:ident^63) => {cb!(pow!($n^21))};
    ($n:ident^64) => {sq!(pow!($n^32))};
    ($n:ident^65) => {$n * pow!($n^64)};
    ($n:ident^66) => {sq!(pow!($n^33))};
    ($n:ident^67) => {$n * pow!($n^66)};
    ($n:ident^68) => {sq!(pow!($n^34))};
    ($n:ident^69) => {$n * pow!($n^68)};
    ($n:ident^70) => {sq!(pow!($n^35))};
    ($n:ident^71) => {$n * pow!($n^70)};
    ($n:ident^72) => {sq!(pow!($n^36))};
    ($n:ident^73) => {$n * pow!($n^72)};
    ($n:ident^74) => {sq!(pow!($n^37))};
    ($n:ident^75) => {{
        let n25 = &pow!($n^25);
        n25 * n25 * n25
    }};
    ($n:ident^76) => {sq!(pow!($n^38))};
    ($n:ident^77) => {{
        let n2 = $n * $n ;
        let n4 = &n2 * &n2;
        let n5 = $n * &n4;
        let n9 = &(n4 * &n5);
        n5 * pow!(n9^8)
    }};
    ($n:ident^78) => {sq!(pow!($n^39))};
    ($n:ident^79) => {$n * pow!($n^78)};
    ($n:ident^80) => {sq!(pow!($n^40))};
    ($n:ident^81) => {$n * pow!($n^80)};
    ($n:ident^82) => {sq!(pow!($n^41))};
    ($n:ident^83) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        n3 * pow!(n5^16)
    }};
    ($n:ident^84) => {sq!(pow!($n^42))};
    ($n:ident^85) => {{
        let n17 = &pow!($n^17);
        pow!(n17^5)
    }};
    ($n:ident^86) => {sq!(pow!($n^43))};
    ($n:ident^87) => {$n * pow!($n^86)};
    ($n:ident^88) => {sq!(pow!($n^44))};
    ($n:ident^89) => {$n * pow!($n^88)};
    ($n:ident^90) => {sq!(pow!($n^45))};
    ($n:ident^91) => {$n * pow!($n^90)};
    ($n:ident^92) => {sq!(pow!($n^46))};
    ($n:ident^93) => {$n * pow!($n^92)};
    ($n:ident^94) => {sq!(pow!($n^47))};
    ($n:ident^95) => {{
        let n19 = &pow!($n^19);
        pow!(n19^5)
    }};
    ($n:ident^96) => {sq!(pow!($n^48))};
    ($n:ident^97) => {$n * pow!($n^96)};
    ($n:ident^98) => {sq!(pow!($n^49))};
    ($n:ident^99) => {{
        let n33 = &pow!($n^33);
        n33 * n33 * n33
    }};
    ($n:ident^100) => {sq!(pow!($n^50))};
    ($n:ident^101) => {$n * pow!($n^100)};
    ($n:ident^102) => {sq!(pow!($n^51))};
    ($n:ident^103) => {$n * pow!($n^102)};
    ($n:ident^104) => {sq!(pow!($n^52))};
    ($n:ident^105) => {$n * pow!($n^104)};
    ($n:ident^106) => {sq!(pow!($n^53))};
    ($n:ident^107) => {{
        let n3 = &($n * $n * $n) ;
        let n13 = &($n * pow!(n3^4));
        n3 * pow!(n13^8)
    }};
    ($n:ident^108) => {sq!(pow!($n^54))};
    ($n:ident^109) => {$n * pow!($n^108)};
    ($n:ident^110) => {sq!(pow!($n^55))};
    ($n:ident^111) => {{
        let n37 = &pow!($n^37);
        n37 * n37 * n37
    }};
    ($n:ident^112) => {sq!(pow!($n^56))};
    ($n:ident^113) => {$n * pow!($n^112)};
    ($n:ident^114) => {sq!(pow!($n^57))};
    ($n:ident^115) => {{
        let n23 = &pow!($n^23);
        pow!(n23^5)
    }};
    ($n:ident^116) => {sq!(pow!($n^58))};
    ($n:ident^117) => {{
        let n39 = &pow!($n^39);
        n39 * n39 * n39
    }};
    ($n:ident^118) => {sq!(pow!($n^59))};
    ($n:ident^119) => {{
        let n17 = &pow!($n^17);
        pow!(n17^7)
    }};
    ($n:ident^120) => {sq!(pow!($n^60))};
    ($n:ident^121) => {$n * pow!($n^120)};
    ($n:ident^122) => {sq!(pow!($n^61))};
    ($n:ident^123) => {{
        let n41 = &pow!($n^41);
        n41 * n41 * n41
    }};
    ($n:ident^124) => {sq!(pow!($n^62))};
    ($n:ident^125) => {{
        let n25 = &pow!($n^25);
        pow!(n25^5)
    }};
    ($n:ident^126) => {sq!(pow!($n^63))};
    ($n:ident^127) => {$n * pow!($n^126)};
    ($n:ident^128) => {sq!(pow!($n^64))};
    ($n:ident^129) => {$n * pow!($n^128)};
    ($n:ident^130) => {sq!(pow!($n^65))};
    ($n:ident^131) => {$n * pow!($n^130)};
    ($n:ident^132) => {sq!(pow!($n^66))};
    ($n:ident^133) => {$n * pow!($n^132)};
    ($n:ident^134) => {sq!(pow!($n^67))};
    ($n:ident^135) => {{
        let n45 = &pow!($n^45);
        n45 * n45 * n45
    }};
    ($n:ident^136) => {sq!(pow!($n^68))};
    ($n:ident^137) => {$n * pow!($n^136)};
    ($n:ident^138) => {sq!(pow!($n^69))};
    ($n:ident^139) => {$n * pow!($n^138)};
    ($n:ident^140) => {sq!(pow!($n^70))};
    ($n:ident^141) => {$n * pow!($n^140)};
    ($n:ident^142) => {sq!(pow!($n^71))};
    ($n:ident^143) => {{
        let n3 = $n * $n * $n ;
        let n4 = &($n * &n3);
        let n35 = &(&n3 * pow!(n4^8));
        n3 * pow!(n35^4)
    }};
    ($n:ident^144) => {sq!(pow!($n^72))};
    ($n:ident^145) => {$n * pow!($n^144)};
    ($n:ident^146) => {sq!(pow!($n^73))};
    ($n:ident^147) => {{
        let n49 = &pow!($n^49);
        n49 * n49 * n49
    }};
    ($n:ident^148) => {sq!(pow!($n^74))};
    ($n:ident^149) => {{
        let n4 = pow!($n^4);
        let n5 = $n * &n4;
        let n9 = &(n4 * &n5);
        n5 * pow!(n9^16)
    }};
    ($n:ident^150) => {sq!(pow!($n^75))};
    ($n:ident^151) => {$n * pow!($n^150)};
    ($n:ident^152) => {sq!(pow!($n^76))};
    ($n:ident^153) => {{
        let n51 = &pow!($n^51);
        n51 * n51 * n51
    }};
    ($n:ident^154) => {sq!(pow!($n^77))};
    ($n:ident^155) => {$n * pow!($n^154)};
    ($n:ident^156) => {sq!(pow!($n^78))};
    ($n:ident^157) => {$n * pow!($n^156)};
    ($n:ident^158) => {sq!(pow!($n^79))};
    ($n:ident^159) => {{
        let n53 = &pow!($n^53);
        n53 * n53 * n53
    }};
    ($n:ident^160) => {sq!(pow!($n^80))};
    ($n:ident^161) => {$n * pow!($n^160)};
    ($n:ident^162) => {sq!(pow!($n^81))};
    ($n:ident^163) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        n3 * pow!(n5^32)
    }};
    ($n:ident^164) => {sq!(pow!($n^82))};
    ($n:ident^165) => {{
        let n33 = &pow!($n^33);
        pow!(n33^5)
    }};
    ($n:ident^166) => {sq!(pow!($n^83))};
    ($n:ident^167) => {$n * pow!($n^166)};
    ($n:ident^168) => {sq!(pow!($n^84))};
    ($n:ident^169) => {$n * pow!($n^168)};
    ($n:ident^170) => {sq!(pow!($n^85))};
    ($n:ident^171) => {$n * pow!($n^170)};
    ($n:ident^172) => {sq!(pow!($n^86))};
    ($n:ident^173) => {$n * pow!($n^172)};
    ($n:ident^174) => {sq!(pow!($n^87))};
    ($n:ident^175) => {{
        let n35 = &pow!($n^35);
        pow!(n35^5)
    }};
    ($n:ident^176) => {sq!(pow!($n^88))};
    ($n:ident^177) => {$n * pow!($n^176)};
    ($n:ident^178) => {sq!(pow!($n^89))};
    ($n:ident^179) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = n2 * &n3;
        let n11 = &($n * &n5 * &n5);
        n3 * pow!(n11^16)
    }};
    ($n:ident^180) => {sq!(pow!($n^90))};
    ($n:ident^181) => {$n * pow!($n^180)};
    ($n:ident^182) => {sq!(pow!($n^91))};
    ($n:ident^183) => {{
        let n61 = &pow!($n^61);
        n61 * n61 * n61
    }};
    ($n:ident^184) => {sq!(pow!($n^92))};
    ($n:ident^185) => {$n * pow!($n^184)};
    ($n:ident^186) => {sq!(pow!($n^93))};
    ($n:ident^187) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        let n23 = &(&n3 * pow!(n5^4));
        n3 * pow!(n23^8)
    }};
    ($n:ident^188) => {sq!(pow!($n^94))};
    ($n:ident^189) => {{
        let n63 = &pow!($n^63);
        n63 * n63 * n63
    }};
    ($n:ident^190) => {sq!(pow!($n^95))};
    ($n:ident^191) => {$n * pow!($n^190)};
    ($n:ident^192) => {sq!(pow!($n^96))};
    ($n:ident^193) => {$n * pow!($n^192)};
    ($n:ident^194) => {sq!(pow!($n^97))};
    ($n:ident^195) => {{
        let n65 = &pow!($n^65);
        n65 * n65 * n65
    }};
    ($n:ident^196) => {sq!(pow!($n^98))};
    ($n:ident^197) => {$n * pow!($n^196)};
    ($n:ident^198) => {sq!(pow!($n^99))};
    ($n:ident^199) => {$n * pow!($n^198)};
    ($n:ident^200) => {sq!(pow!($n^100))};
    ($n:ident^201) => {$n * pow!($n^200)};
    ($n:ident^202) => {sq!(pow!($n^101))};
    ($n:ident^203) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = &(n2 * &n3);
        n3 * pow!(n5^40)
    }};
    ($n:ident^204) => {sq!(pow!($n^102))};
    ($n:ident^205) => {{
        let n41 = &pow!($n^41);
        pow!(n41^5)
    }};
    ($n:ident^206) => {sq!(pow!($n^103))};
    ($n:ident^207) => {{
        let n69 = &pow!($n^69);
        n69 * n69 * n69
    }};
    ($n:ident^208) => {sq!(pow!($n^104))};
    ($n:ident^209) => {$n * pow!($n^208)};
    ($n:ident^210) => {sq!(pow!($n^105))};
    ($n:ident^211) => {{
        let n3 = &($n * $n * $n);
        let n13 = &($n * pow!(n3^4));
        n3 * pow!(n13^16)
    }};
    ($n:ident^212) => {sq!(pow!($n^106))};
    ($n:ident^213) => {{
        let n2 = $n * $n ;
        let n3 = $n * &n2;
        let n5 = n2 * &n3;
        let n13 = &(n3 * &n5 * &n5);
        n5 * pow!(n13^16)
    }};
    ($n:ident^214) => {sq!(pow!($n^107))};
    ($n:ident^215) => {{
        let n43 = &pow!($n^43);
        pow!(n43^5)
    }};
    ($n:ident^216) => {sq!(pow!($n^108))};
    ($n:ident^217) => {$n * pow!($n^216)};
    ($n:ident^218) => {sq!(pow!($n^109))};
    ($n:ident^219) => {{
        let n3 = &($n * $n * $n);
        let n27 = &pow!(n3^9);
        n3 * pow!(n27^8)
    }};
    ($n:ident^220) => {sq!(pow!($n^110))};
    ($n:ident^221) => {{
        let n2 = $n * $n ;
        let n4 = &n2 * &n2;
        let n5 = $n * &n4;
        let n9 = &(n4 * &n5);
        let n27 = &pow!(n9^3);
        n5 * pow!(n27^8)
    }};
    ($n:ident^222) => {sq!(pow!($n^111))};
    ($n:ident^223) => {$n * pow!($n^222)};
    ($n:ident^224) => {sq!(pow!($n^112))};
    ($n:ident^225) => {$n * pow!($n^224)};
    ($n:ident^226) => {sq!(pow!($n^113))};
    ($n:ident^227) => {{
        let n3 = &($n * $n * $n);
        let n7 = &($n * n3 * n3);
        n3 * pow!(n7^32)
    }};
    ($n:ident^228) => {sq!(pow!($n^114))};
    ($n:ident^229) => {{
        let n2 = $n * $n ;
        let n5 = $n * &n2 * &n2;
        let n7 = &(n2 * &n5);
        n5 * pow!(n7^32)
    }};
    ($n:ident^230) => {sq!(pow!($n^115))};
    ($n:ident^231) => {{
        let n77 = pow!($n^77);
        &n77 * &n77 * &n77
    }};
    ($n:ident^232) => {sq!(pow!($n^116))};
    ($n:ident^233) => {{
        let n4 = pow!($n^4);
        let n5 = $n * &n4;
        let n9 = n4 * &n5;
        let n14 = &(n5 * &n9);
        n9 * pow!(n14^16)
    }};
    ($n:ident^234) => {sq!(pow!($n^117))};
    ($n:ident^235) => {$n * pow!($n^234)};
    ($n:ident^236) => {sq!(pow!($n^118))};
    ($n:ident^237) => {$n * pow!($n^236)};
    ($n:ident^238) => {sq!(pow!($n^119))};
    ($n:ident^239) => {$n * pow!($n^238)};
    ($n:ident^240) => {sq!(pow!($n^120))};
    ($n:ident^241) => {$n * pow!($n^240)};
    ($n:ident^242) => {sq!(pow!($n^121))};
    ($n:ident^243) => {{
        let n81 = &pow!($n^81);
        n81 * n81 * n81
    }};
    ($n:ident^244) => {sq!(pow!($n^122))};
    ($n:ident^245) => {{
        let n49 = &pow!($n^49);
        pow!(n49^5)
    }};
    ($n:ident^246) => {sq!(pow!($n^123))};
    ($n:ident^247) => {$n * pow!($n^246)};
    ($n:ident^248) => {sq!(pow!($n^124))};
    ($n:ident^249) => {{
        let n83 = &pow!($n^83);
        n83 * n83 * n83
    }};
    ($n:ident^250) => {sq!(pow!($n^125))};
    ($n:ident^251) => {$n * pow!($n^250)};
    ($n:ident^252) => {sq!(pow!($n^126))};
    ($n:ident^253) => {$n * pow!($n^252)};
    ($n:ident^254) => {sq!(pow!($n^127))};
    ($n:ident^255) => {{
        let n85 = &pow!($n^85);
        n85 * n85 * n85
    }};
}

macro_rules! pow_array_u8 {
    () => {
        [
            |_n| pow!(n^0),
            |n| pow!(n^1),
            |n| pow!(n^2),
            |n| pow!(n^3),
            |n| pow!(n^4),
            |n| pow!(n^5),
            |n| pow!(n^6),
            |n| pow!(n^7),
            |n| pow!(n^8),
            |n| pow!(n^9),
            |n| pow!(n^10),
            |n| pow!(n^11),
            |n| pow!(n^12),
            |n| pow!(n^13),
            |n| pow!(n^14),
            |n| pow!(n^15),
            |n| pow!(n^16),
            |n| pow!(n^17),
            |n| pow!(n^18),
            |n| pow!(n^19),
            |n| pow!(n^20),
            |n| pow!(n^21),
            |n| pow!(n^22),
            |n| pow!(n^23),
            |n| pow!(n^24),
            |n| pow!(n^25),
            |n| pow!(n^26),
            |n| pow!(n^27),
            |n| pow!(n^28),
            |n| pow!(n^29),
            |n| pow!(n^30),
            |n| pow!(n^31),
            |n| pow!(n^32),
            |n| pow!(n^33),
            |n| pow!(n^34),
            |n| pow!(n^35),
            |n| pow!(n^36),
            |n| pow!(n^37),
            |n| pow!(n^38),
            |n| pow!(n^39),
            |n| pow!(n^40),
            |n| pow!(n^41),
            |n| pow!(n^42),
            |n| pow!(n^43),
            |n| pow!(n^44),
            |n| pow!(n^45),
            |n| pow!(n^46),
            |n| pow!(n^47),
            |n| pow!(n^48),
            |n| pow!(n^49),
            |n| pow!(n^50),
            |n| pow!(n^51),
            |n| pow!(n^52),
            |n| pow!(n^53),
            |n| pow!(n^54),
            |n| pow!(n^55),
            |n| pow!(n^56),
            |n| pow!(n^57),
            |n| pow!(n^58),
            |n| pow!(n^59),
            |n| pow!(n^60),
            |n| pow!(n^61),
            |n| pow!(n^62),
            |n| pow!(n^63),
            |n| pow!(n^64),
            |n| pow!(n^65),
            |n| pow!(n^66),
            |n| pow!(n^67),
            |n| pow!(n^68),
            |n| pow!(n^69),
            |n| pow!(n^70),
            |n| pow!(n^71),
            |n| pow!(n^72),
            |n| pow!(n^73),
            |n| pow!(n^74),
            |n| pow!(n^75),
            |n| pow!(n^76),
            |n| pow!(n^77),
            |n| pow!(n^78),
            |n| pow!(n^79),
            |n| pow!(n^80),
            |n| pow!(n^81),
            |n| pow!(n^82),
            |n| pow!(n^83),
            |n| pow!(n^84),
            |n| pow!(n^85),
            |n| pow!(n^86),
            |n| pow!(n^87),
            |n| pow!(n^88),
            |n| pow!(n^89),
            |n| pow!(n^90),
            |n| pow!(n^91),
            |n| pow!(n^92),
            |n| pow!(n^93),
            |n| pow!(n^94),
            |n| pow!(n^95),
            |n| pow!(n^96),
            |n| pow!(n^97),
            |n| pow!(n^98),
            |n| pow!(n^99),
            |n| pow!(n^100),
            |n| pow!(n^101),
            |n| pow!(n^102),
            |n| pow!(n^103),
            |n| pow!(n^104),
            |n| pow!(n^105),
            |n| pow!(n^106),
            |n| pow!(n^107),
            |n| pow!(n^108),
            |n| pow!(n^109),
            |n| pow!(n^110),
            |n| pow!(n^111),
            |n| pow!(n^112),
            |n| pow!(n^113),
            |n| pow!(n^114),
            |n| pow!(n^115),
            |n| pow!(n^116),
            |n| pow!(n^117),
            |n| pow!(n^118),
            |n| pow!(n^119),
            |n| pow!(n^120),
            |n| pow!(n^121),
            |n| pow!(n^122),
            |n| pow!(n^123),
            |n| pow!(n^124),
            |n| pow!(n^125),
            |n| pow!(n^126),
            |n| pow!(n^127),
            |n| pow!(n^128),
            |n| pow!(n^129),
            |n| pow!(n^130),
            |n| pow!(n^131),
            |n| pow!(n^132),
            |n| pow!(n^133),
            |n| pow!(n^134),
            |n| pow!(n^135),
            |n| pow!(n^136),
            |n| pow!(n^137),
            |n| pow!(n^138),
            |n| pow!(n^139),
            |n| pow!(n^140),
            |n| pow!(n^141),
            |n| pow!(n^142),
            |n| pow!(n^143),
            |n| pow!(n^144),
            |n| pow!(n^145),
            |n| pow!(n^146),
            |n| pow!(n^147),
            |n| pow!(n^148),
            |n| pow!(n^149),
            |n| pow!(n^150),
            |n| pow!(n^151),
            |n| pow!(n^152),
            |n| pow!(n^153),
            |n| pow!(n^154),
            |n| pow!(n^155),
            |n| pow!(n^156),
            |n| pow!(n^157),
            |n| pow!(n^158),
            |n| pow!(n^159),
            |n| pow!(n^160),
            |n| pow!(n^161),
            |n| pow!(n^162),
            |n| pow!(n^163),
            |n| pow!(n^164),
            |n| pow!(n^165),
            |n| pow!(n^166),
            |n| pow!(n^167),
            |n| pow!(n^168),
            |n| pow!(n^169),
            |n| pow!(n^170),
            |n| pow!(n^171),
            |n| pow!(n^172),
            |n| pow!(n^173),
            |n| pow!(n^174),
            |n| pow!(n^175),
            |n| pow!(n^176),
            |n| pow!(n^177),
            |n| pow!(n^178),
            |n| pow!(n^179),
            |n| pow!(n^180),
            |n| pow!(n^181),
            |n| pow!(n^182),
            |n| pow!(n^183),
            |n| pow!(n^184),
            |n| pow!(n^185),
            |n| pow!(n^186),
            |n| pow!(n^187),
            |n| pow!(n^188),
            |n| pow!(n^189),
            |n| pow!(n^190),
            |n| pow!(n^191),
            |n| pow!(n^192),
            |n| pow!(n^193),
            |n| pow!(n^194),
            |n| pow!(n^195),
            |n| pow!(n^196),
            |n| pow!(n^197),
            |n| pow!(n^198),
            |n| pow!(n^199),
            |n| pow!(n^200),
            |n| pow!(n^201),
            |n| pow!(n^202),
            |n| pow!(n^203),
            |n| pow!(n^204),
            |n| pow!(n^205),
            |n| pow!(n^206),
            |n| pow!(n^207),
            |n| pow!(n^208),
            |n| pow!(n^209),
            |n| pow!(n^210),
            |n| pow!(n^211),
            |n| pow!(n^212),
            |n| pow!(n^213),
            |n| pow!(n^214),
            |n| pow!(n^215),
            |n| pow!(n^216),
            |n| pow!(n^217),
            |n| pow!(n^218),
            |n| pow!(n^219),
            |n| pow!(n^220),
            |n| pow!(n^221),
            |n| pow!(n^222),
            |n| pow!(n^223),
            |n| pow!(n^224),
            |n| pow!(n^225),
            |n| pow!(n^226),
            |n| pow!(n^227),
            |n| pow!(n^228),
            |n| pow!(n^229),
            |n| pow!(n^230),
            |n| pow!(n^231),
            |n| pow!(n^232),
            |n| pow!(n^233),
            |n| pow!(n^234),
            |n| pow!(n^235),
            |n| pow!(n^236),
            |n| pow!(n^237),
            |n| pow!(n^238),
            |n| pow!(n^239),
            |n| pow!(n^240),
            |n| pow!(n^241),
            |n| pow!(n^242),
            |n| pow!(n^243),
            |n| pow!(n^244),
            |n| pow!(n^245),
            |n| pow!(n^246),
            |n| pow!(n^247),
            |n| pow!(n^248),
            |n| pow!(n^249),
            |n| pow!(n^250),
            |n| pow!(n^251),
            |n| pow!(n^252),
            |n| pow!(n^253),
            |n| pow!(n^254),
            |n| pow!(n^255),
        ]
    };
}

macro_rules! pow_array_i8 {
    ()=> {
        [
            |_n| pow!(n^0),
            |n| pow!(n^1),
            |n| pow!(n^2),
            |n| pow!(n^3),
            |n| pow!(n^4),
            |n| pow!(n^5),
            |n| pow!(n^6),
            |n| pow!(n^7),
            |n| pow!(n^8),
            |n| pow!(n^9),
            |n| pow!(n^10),
            |n| pow!(n^11),
            |n| pow!(n^12),
            |n| pow!(n^13),
            |n| pow!(n^14),
            |n| pow!(n^15),
            |n| pow!(n^16),
            |n| pow!(n^17),
            |n| pow!(n^18),
            |n| pow!(n^19),
            |n| pow!(n^20),
            |n| pow!(n^21),
            |n| pow!(n^22),
            |n| pow!(n^23),
            |n| pow!(n^24),
            |n| pow!(n^25),
            |n| pow!(n^26),
            |n| pow!(n^27),
            |n| pow!(n^28),
            |n| pow!(n^29),
            |n| pow!(n^30),
            |n| pow!(n^31),
            |n| pow!(n^32),
            |n| pow!(n^33),
            |n| pow!(n^34),
            |n| pow!(n^35),
            |n| pow!(n^36),
            |n| pow!(n^37),
            |n| pow!(n^38),
            |n| pow!(n^39),
            |n| pow!(n^40),
            |n| pow!(n^41),
            |n| pow!(n^42),
            |n| pow!(n^43),
            |n| pow!(n^44),
            |n| pow!(n^45),
            |n| pow!(n^46),
            |n| pow!(n^47),
            |n| pow!(n^48),
            |n| pow!(n^49),
            |n| pow!(n^50),
            |n| pow!(n^51),
            |n| pow!(n^52),
            |n| pow!(n^53),
            |n| pow!(n^54),
            |n| pow!(n^55),
            |n| pow!(n^56),
            |n| pow!(n^57),
            |n| pow!(n^58),
            |n| pow!(n^59),
            |n| pow!(n^60),
            |n| pow!(n^61),
            |n| pow!(n^62),
            |n| pow!(n^63),
            |n| pow!(n^64),
            |n| pow!(n^65),
            |n| pow!(n^66),
            |n| pow!(n^67),
            |n| pow!(n^68),
            |n| pow!(n^69),
            |n| pow!(n^70),
            |n| pow!(n^71),
            |n| pow!(n^72),
            |n| pow!(n^73),
            |n| pow!(n^74),
            |n| pow!(n^75),
            |n| pow!(n^76),
            |n| pow!(n^77),
            |n| pow!(n^78),
            |n| pow!(n^79),
            |n| pow!(n^80),
            |n| pow!(n^81),
            |n| pow!(n^82),
            |n| pow!(n^83),
            |n| pow!(n^84),
            |n| pow!(n^85),
            |n| pow!(n^86),
            |n| pow!(n^87),
            |n| pow!(n^88),
            |n| pow!(n^89),
            |n| pow!(n^90),
            |n| pow!(n^91),
            |n| pow!(n^92),
            |n| pow!(n^93),
            |n| pow!(n^94),
            |n| pow!(n^95),
            |n| pow!(n^96),
            |n| pow!(n^97),
            |n| pow!(n^98),
            |n| pow!(n^99),
            |n| pow!(n^100),
            |n| pow!(n^101),
            |n| pow!(n^102),
            |n| pow!(n^103),
            |n| pow!(n^104),
            |n| pow!(n^105),
            |n| pow!(n^106),
            |n| pow!(n^107),
            |n| pow!(n^108),
            |n| pow!(n^109),
            |n| pow!(n^110),
            |n| pow!(n^111),
            |n| pow!(n^112),
            |n| pow!(n^113),
            |n| pow!(n^114),
            |n| pow!(n^115),
            |n| pow!(n^116),
            |n| pow!(n^117),
            |n| pow!(n^118),
            |n| pow!(n^119),
            |n| pow!(n^120),
            |n| pow!(n^121),
            |n| pow!(n^122),
            |n| pow!(n^123),
            |n| pow!(n^124),
            |n| pow!(n^125),
            |n| pow!(n^126),
            |n| pow!(n^127),
            |n| pow!(n^128).inv(),
            |n| pow!(n^127).inv(),
            |n| pow!(n^126).inv(),
            |n| pow!(n^125).inv(),
            |n| pow!(n^124).inv(),
            |n| pow!(n^123).inv(),
            |n| pow!(n^122).inv(),
            |n| pow!(n^121).inv(),
            |n| pow!(n^120).inv(),
            |n| pow!(n^119).inv(),
            |n| pow!(n^118).inv(),
            |n| pow!(n^117).inv(),
            |n| pow!(n^116).inv(),
            |n| pow!(n^115).inv(),
            |n| pow!(n^114).inv(),
            |n| pow!(n^113).inv(),
            |n| pow!(n^112).inv(),
            |n| pow!(n^111).inv(),
            |n| pow!(n^110).inv(),
            |n| pow!(n^109).inv(),
            |n| pow!(n^108).inv(),
            |n| pow!(n^107).inv(),
            |n| pow!(n^106).inv(),
            |n| pow!(n^105).inv(),
            |n| pow!(n^104).inv(),
            |n| pow!(n^103).inv(),
            |n| pow!(n^102).inv(),
            |n| pow!(n^101).inv(),
            |n| pow!(n^100).inv(),
            |n| pow!(n^99).inv(),
            |n| pow!(n^98).inv(),
            |n| pow!(n^97).inv(),
            |n| pow!(n^96).inv(),
            |n| pow!(n^95).inv(),
            |n| pow!(n^94).inv(),
            |n| pow!(n^93).inv(),
            |n| pow!(n^92).inv(),
            |n| pow!(n^91).inv(),
            |n| pow!(n^90).inv(),
            |n| pow!(n^89).inv(),
            |n| pow!(n^88).inv(),
            |n| pow!(n^87).inv(),
            |n| pow!(n^86).inv(),
            |n| pow!(n^85).inv(),
            |n| pow!(n^84).inv(),
            |n| pow!(n^83).inv(),
            |n| pow!(n^82).inv(),
            |n| pow!(n^81).inv(),
            |n| pow!(n^80).inv(),
            |n| pow!(n^79).inv(),
            |n| pow!(n^78).inv(),
            |n| pow!(n^77).inv(),
            |n| pow!(n^76).inv(),
            |n| pow!(n^75).inv(),
            |n| pow!(n^74).inv(),
            |n| pow!(n^73).inv(),
            |n| pow!(n^72).inv(),
            |n| pow!(n^71).inv(),
            |n| pow!(n^70).inv(),
            |n| pow!(n^69).inv(),
            |n| pow!(n^68).inv(),
            |n| pow!(n^67).inv(),
            |n| pow!(n^66).inv(),
            |n| pow!(n^65).inv(),
            |n| pow!(n^64).inv(),
            |n| pow!(n^63).inv(),
            |n| pow!(n^62).inv(),
            |n| pow!(n^61).inv(),
            |n| pow!(n^60).inv(),
            |n| pow!(n^59).inv(),
            |n| pow!(n^58).inv(),
            |n| pow!(n^57).inv(),
            |n| pow!(n^56).inv(),
            |n| pow!(n^55).inv(),
            |n| pow!(n^54).inv(),
            |n| pow!(n^53).inv(),
            |n| pow!(n^52).inv(),
            |n| pow!(n^51).inv(),
            |n| pow!(n^50).inv(),
            |n| pow!(n^49).inv(),
            |n| pow!(n^48).inv(),
            |n| pow!(n^47).inv(),
            |n| pow!(n^46).inv(),
            |n| pow!(n^45).inv(),
            |n| pow!(n^44).inv(),
            |n| pow!(n^43).inv(),
            |n| pow!(n^42).inv(),
            |n| pow!(n^41).inv(),
            |n| pow!(n^40).inv(),
            |n| pow!(n^39).inv(),
            |n| pow!(n^38).inv(),
            |n| pow!(n^37).inv(),
            |n| pow!(n^36).inv(),
            |n| pow!(n^35).inv(),
            |n| pow!(n^34).inv(),
            |n| pow!(n^33).inv(),
            |n| pow!(n^32).inv(),
            |n| pow!(n^31).inv(),
            |n| pow!(n^30).inv(),
            |n| pow!(n^29).inv(),
            |n| pow!(n^28).inv(),
            |n| pow!(n^27).inv(),
            |n| pow!(n^26).inv(),
            |n| pow!(n^25).inv(),
            |n| pow!(n^24).inv(),
            |n| pow!(n^23).inv(),
            |n| pow!(n^22).inv(),
            |n| pow!(n^21).inv(),
            |n| pow!(n^20).inv(),
            |n| pow!(n^19).inv(),
            |n| pow!(n^18).inv(),
            |n| pow!(n^17).inv(),
            |n| pow!(n^16).inv(),
            |n| pow!(n^15).inv(),
            |n| pow!(n^14).inv(),
            |n| pow!(n^13).inv(),
            |n| pow!(n^12).inv(),
            |n| pow!(n^11).inv(),
            |n| pow!(n^10).inv(),
            |n| pow!(n^9).inv(),
            |n| pow!(n^8).inv(),
            |n| pow!(n^7).inv(),
            |n| pow!(n^6).inv(),
            |n| pow!(n^5).inv(),
            |n| pow!(n^4).inv(),
            |n| pow!(n^3).inv(),
            |n| pow!(n^2).inv(),
            |n| pow!(n^1).inv(),
        ]
    }
}
