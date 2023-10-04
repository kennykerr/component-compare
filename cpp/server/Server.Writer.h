#pragma once
#include "Server.Writer.g.h"

namespace winrt::Server::implementation
{
    struct Writer : WriterT<Writer>
    {
        Writer() = default;

        hstring P0() { std::shared_lock lock(m_lock); return m_array[0]; }
        void P0(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[0] = value;
        }

        hstring P1() { std::shared_lock lock(m_lock); return m_array[1]; }
        void P1(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[1] = value;
        }

        hstring P2() { std::shared_lock lock(m_lock); return m_array[2]; }
        void P2(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[2] = value;
        }

        hstring P3() { std::shared_lock lock(m_lock); return m_array[3]; }
        void P3(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[3] = value;
        }

        hstring P4() { std::shared_lock lock(m_lock); return m_array[4]; }
        void P4(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[4] = value;
        }

        hstring P5() { std::shared_lock lock(m_lock); return m_array[5]; }
        void P5(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[5] = value;
        }

        hstring P6() { std::shared_lock lock(m_lock); return m_array[6]; }
        void P6(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[6] = value;
        }

        hstring P7() { std::shared_lock lock(m_lock); return m_array[7]; }
        void P7(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[7] = value;
        }

        hstring P8() { std::shared_lock lock(m_lock); return m_array[8]; }
        void P8(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[8] = value;
        }

        hstring P9() { std::shared_lock lock(m_lock); return m_array[9]; }
        void P9(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[9] = value;
        }

        hstring P10() { std::shared_lock lock(m_lock); return m_array[10]; }
        void P10(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[10] = value;
        }

        hstring P11() { std::shared_lock lock(m_lock); return m_array[11]; }
        void P11(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[11] = value;
        }

        hstring P12() { std::shared_lock lock(m_lock); return m_array[12]; }
        void P12(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[12] = value;
        }

        hstring P13() { std::shared_lock lock(m_lock); return m_array[13]; }
        void P13(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[13] = value;
        }

        hstring P14() { std::shared_lock lock(m_lock); return m_array[14]; }
        void P14(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[14] = value;
        }

        hstring P15() { std::shared_lock lock(m_lock); return m_array[15]; }
        void P15(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[15] = value;
        }

        hstring P16() { std::shared_lock lock(m_lock); return m_array[16]; }
        void P16(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[16] = value;
        }

        hstring P17() { std::shared_lock lock(m_lock); return m_array[17]; }
        void P17(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[17] = value;
        }

        hstring P18() { std::shared_lock lock(m_lock); return m_array[18]; }
        void P18(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[18] = value;
        }

        hstring P19() { std::shared_lock lock(m_lock); return m_array[19]; }
        void P19(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[19] = value;
        }

        hstring P20() { std::shared_lock lock(m_lock); return m_array[20]; }
        void P20(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[20] = value;
        }

        hstring P21() { std::shared_lock lock(m_lock); return m_array[21]; }
        void P21(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[21] = value;
        }

        hstring P22() { std::shared_lock lock(m_lock); return m_array[22]; }
        void P22(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[22] = value;
        }

        hstring P23() { std::shared_lock lock(m_lock); return m_array[23]; }
        void P23(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[23] = value;
        }

        hstring P24() { std::shared_lock lock(m_lock); return m_array[24]; }
        void P24(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[24] = value;
        }

        hstring P25() { std::shared_lock lock(m_lock); return m_array[25]; }
        void P25(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[25] = value;
        }

        hstring P26() { std::shared_lock lock(m_lock); return m_array[26]; }
        void P26(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[26] = value;
        }

        hstring P27() { std::shared_lock lock(m_lock); return m_array[27]; }
        void P27(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[27] = value;
        }

        hstring P28() { std::shared_lock lock(m_lock); return m_array[28]; }
        void P28(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[28] = value;
        }

        hstring P29() { std::shared_lock lock(m_lock); return m_array[29]; }
        void P29(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[29] = value;
        }

        hstring P30() { std::shared_lock lock(m_lock); return m_array[30]; }
        void P30(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[30] = value;
        }

        hstring P31() { std::shared_lock lock(m_lock); return m_array[31]; }
        void P31(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[31] = value;
        }

        hstring P32() { std::shared_lock lock(m_lock); return m_array[32]; }
        void P32(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[32] = value;
        }

        hstring P33() { std::shared_lock lock(m_lock); return m_array[33]; }
        void P33(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[33] = value;
        }

        hstring P34() { std::shared_lock lock(m_lock); return m_array[34]; }
        void P34(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[34] = value;
        }

        hstring P35() { std::shared_lock lock(m_lock); return m_array[35]; }
        void P35(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[35] = value;
        }

        hstring P36() { std::shared_lock lock(m_lock); return m_array[36]; }
        void P36(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[36] = value;
        }

        hstring P37() { std::shared_lock lock(m_lock); return m_array[37]; }
        void P37(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[37] = value;
        }

        hstring P38() { std::shared_lock lock(m_lock); return m_array[38]; }
        void P38(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[38] = value;
        }

        hstring P39() { std::shared_lock lock(m_lock); return m_array[39]; }
        void P39(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[39] = value;
        }

        hstring P40() { std::shared_lock lock(m_lock); return m_array[40]; }
        void P40(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[40] = value;
        }

        hstring P41() { std::shared_lock lock(m_lock); return m_array[41]; }
        void P41(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[41] = value;
        }

        hstring P42() { std::shared_lock lock(m_lock); return m_array[42]; }
        void P42(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[42] = value;
        }

        hstring P43() { std::shared_lock lock(m_lock); return m_array[43]; }
        void P43(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[43] = value;
        }

        hstring P44() { std::shared_lock lock(m_lock); return m_array[44]; }
        void P44(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[44] = value;
        }

        hstring P45() { std::shared_lock lock(m_lock); return m_array[45]; }
        void P45(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[45] = value;
        }

        hstring P46() { std::shared_lock lock(m_lock); return m_array[46]; }
        void P46(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[46] = value;
        }

        hstring P47() { std::shared_lock lock(m_lock); return m_array[47]; }
        void P47(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[47] = value;
        }

        hstring P48() { std::shared_lock lock(m_lock); return m_array[48]; }
        void P48(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[48] = value;
        }

        hstring P49() { std::shared_lock lock(m_lock); return m_array[49]; }
        void P49(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[49] = value;
        }

        hstring P50() { std::shared_lock lock(m_lock); return m_array[50]; }
        void P50(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[50] = value;
        }

        hstring P51() { std::shared_lock lock(m_lock); return m_array[51]; }
        void P51(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[51] = value;
        }

        hstring P52() { std::shared_lock lock(m_lock); return m_array[52]; }
        void P52(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[52] = value;
        }

        hstring P53() { std::shared_lock lock(m_lock); return m_array[53]; }
        void P53(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[53] = value;
        }

        hstring P54() { std::shared_lock lock(m_lock); return m_array[54]; }
        void P54(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[54] = value;
        }

        hstring P55() { std::shared_lock lock(m_lock); return m_array[55]; }
        void P55(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[55] = value;
        }

        hstring P56() { std::shared_lock lock(m_lock); return m_array[56]; }
        void P56(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[56] = value;
        }

        hstring P57() { std::shared_lock lock(m_lock); return m_array[57]; }
        void P57(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[57] = value;
        }

        hstring P58() { std::shared_lock lock(m_lock); return m_array[58]; }
        void P58(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[58] = value;
        }

        hstring P59() { std::shared_lock lock(m_lock); return m_array[59]; }
        void P59(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[59] = value;
        }

        hstring P60() { std::shared_lock lock(m_lock); return m_array[60]; }
        void P60(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[60] = value;
        }

        hstring P61() { std::shared_lock lock(m_lock); return m_array[61]; }
        void P61(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[61] = value;
        }

        hstring P62() { std::shared_lock lock(m_lock); return m_array[62]; }
        void P62(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[62] = value;
        }

        hstring P63() { std::shared_lock lock(m_lock); return m_array[63]; }
        void P63(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[63] = value;
        }

        hstring P64() { std::shared_lock lock(m_lock); return m_array[64]; }
        void P64(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[64] = value;
        }

        hstring P65() { std::shared_lock lock(m_lock); return m_array[65]; }
        void P65(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[65] = value;
        }

        hstring P66() { std::shared_lock lock(m_lock); return m_array[66]; }
        void P66(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[66] = value;
        }

        hstring P67() { std::shared_lock lock(m_lock); return m_array[67]; }
        void P67(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[67] = value;
        }

        hstring P68() { std::shared_lock lock(m_lock); return m_array[68]; }
        void P68(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[68] = value;
        }

        hstring P69() { std::shared_lock lock(m_lock); return m_array[69]; }
        void P69(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[69] = value;
        }

        hstring P70() { std::shared_lock lock(m_lock); return m_array[70]; }
        void P70(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[70] = value;
        }

        hstring P71() { std::shared_lock lock(m_lock); return m_array[71]; }
        void P71(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[71] = value;
        }

        hstring P72() { std::shared_lock lock(m_lock); return m_array[72]; }
        void P72(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[72] = value;
        }

        hstring P73() { std::shared_lock lock(m_lock); return m_array[73]; }
        void P73(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[73] = value;
        }

        hstring P74() { std::shared_lock lock(m_lock); return m_array[74]; }
        void P74(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[74] = value;
        }

        hstring P75() { std::shared_lock lock(m_lock); return m_array[75]; }
        void P75(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[75] = value;
        }

        hstring P76() { std::shared_lock lock(m_lock); return m_array[76]; }
        void P76(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[76] = value;
        }

        hstring P77() { std::shared_lock lock(m_lock); return m_array[77]; }
        void P77(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[77] = value;
        }

        hstring P78() { std::shared_lock lock(m_lock); return m_array[78]; }
        void P78(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[78] = value;
        }

        hstring P79() { std::shared_lock lock(m_lock); return m_array[79]; }
        void P79(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[79] = value;
        }

        hstring P80() { std::shared_lock lock(m_lock); return m_array[80]; }
        void P80(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[80] = value;
        }

        hstring P81() { std::shared_lock lock(m_lock); return m_array[81]; }
        void P81(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[81] = value;
        }

        hstring P82() { std::shared_lock lock(m_lock); return m_array[82]; }
        void P82(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[82] = value;
        }

        hstring P83() { std::shared_lock lock(m_lock); return m_array[83]; }
        void P83(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[83] = value;
        }

        hstring P84() { std::shared_lock lock(m_lock); return m_array[84]; }
        void P84(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[84] = value;
        }

        hstring P85() { std::shared_lock lock(m_lock); return m_array[85]; }
        void P85(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[85] = value;
        }

        hstring P86() { std::shared_lock lock(m_lock); return m_array[86]; }
        void P86(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[86] = value;
        }

        hstring P87() { std::shared_lock lock(m_lock); return m_array[87]; }
        void P87(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[87] = value;
        }

        hstring P88() { std::shared_lock lock(m_lock); return m_array[88]; }
        void P88(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[88] = value;
        }

        hstring P89() { std::shared_lock lock(m_lock); return m_array[89]; }
        void P89(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[89] = value;
        }

        hstring P90() { std::shared_lock lock(m_lock); return m_array[90]; }
        void P90(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[90] = value;
        }

        hstring P91() { std::shared_lock lock(m_lock); return m_array[91]; }
        void P91(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[91] = value;
        }

        hstring P92() { std::shared_lock lock(m_lock); return m_array[92]; }
        void P92(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[92] = value;
        }

        hstring P93() { std::shared_lock lock(m_lock); return m_array[93]; }
        void P93(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[93] = value;
        }

        hstring P94() { std::shared_lock lock(m_lock); return m_array[94]; }
        void P94(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[94] = value;
        }

        hstring P95() { std::shared_lock lock(m_lock); return m_array[95]; }
        void P95(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[95] = value;
        }

        hstring P96() { std::shared_lock lock(m_lock); return m_array[96]; }
        void P96(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[96] = value;
        }

        hstring P97() { std::shared_lock lock(m_lock); return m_array[97]; }
        void P97(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[97] = value;
        }

        hstring P98() { std::shared_lock lock(m_lock); return m_array[98]; }
        void P98(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[98] = value;
        }

        hstring P99() { std::shared_lock lock(m_lock); return m_array[99]; }
        void P99(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[99] = value;
        }

        hstring P100() { std::shared_lock lock(m_lock); return m_array[100]; }
        void P100(hstring const& value)
        {
            std::unique_lock lock(m_lock);
            m_array[100] = value;
        }

        std::shared_mutex m_lock;
        std::array<hstring, 101> m_array;
    };
}
namespace winrt::Server::factory_implementation
{
    struct Writer : WriterT<Writer, implementation::Writer>
    {
    };
}
