mod bindings;
use std::sync::*;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::Reader)]
struct Reader;

impl bindings::IReader_Impl for Reader {
    fn P0(&self) -> Result<HSTRING> {
        // TODO: since Rust is always pinned this should just use an HSTRING reference
        // and not create a copy.
        Ok(h!("P0").clone())
    }
    fn P1(&self) -> Result<HSTRING> {
        Ok(h!("P1").clone())
    }
    fn P2(&self) -> Result<HSTRING> {
        Ok(h!("P2").clone())
    }
    fn P3(&self) -> Result<HSTRING> {
        Ok(h!("P3").clone())
    }
    fn P4(&self) -> Result<HSTRING> {
        Ok(h!("P4").clone())
    }
    fn P5(&self) -> Result<HSTRING> {
        Ok(h!("P5").clone())
    }
    fn P6(&self) -> Result<HSTRING> {
        Ok(h!("P6").clone())
    }
    fn P7(&self) -> Result<HSTRING> {
        Ok(h!("P7").clone())
    }
    fn P8(&self) -> Result<HSTRING> {
        Ok(h!("P8").clone())
    }
    fn P9(&self) -> Result<HSTRING> {
        Ok(h!("P9").clone())
    }
    fn P10(&self) -> Result<HSTRING> {
        Ok(h!("P10").clone())
    }
    fn P11(&self) -> Result<HSTRING> {
        Ok(h!("P11").clone())
    }
    fn P12(&self) -> Result<HSTRING> {
        Ok(h!("P12").clone())
    }
    fn P13(&self) -> Result<HSTRING> {
        Ok(h!("P13").clone())
    }
    fn P14(&self) -> Result<HSTRING> {
        Ok(h!("P14").clone())
    }
    fn P15(&self) -> Result<HSTRING> {
        Ok(h!("P15").clone())
    }
    fn P16(&self) -> Result<HSTRING> {
        Ok(h!("P16").clone())
    }
    fn P17(&self) -> Result<HSTRING> {
        Ok(h!("P17").clone())
    }
    fn P18(&self) -> Result<HSTRING> {
        Ok(h!("P18").clone())
    }
    fn P19(&self) -> Result<HSTRING> {
        Ok(h!("P19").clone())
    }
    fn P20(&self) -> Result<HSTRING> {
        Ok(h!("P20").clone())
    }
    fn P21(&self) -> Result<HSTRING> {
        Ok(h!("P21").clone())
    }
    fn P22(&self) -> Result<HSTRING> {
        Ok(h!("P22").clone())
    }
    fn P23(&self) -> Result<HSTRING> {
        Ok(h!("P23").clone())
    }
    fn P24(&self) -> Result<HSTRING> {
        Ok(h!("P24").clone())
    }
    fn P25(&self) -> Result<HSTRING> {
        Ok(h!("P25").clone())
    }
    fn P26(&self) -> Result<HSTRING> {
        Ok(h!("P26").clone())
    }
    fn P27(&self) -> Result<HSTRING> {
        Ok(h!("P27").clone())
    }
    fn P28(&self) -> Result<HSTRING> {
        Ok(h!("P28").clone())
    }
    fn P29(&self) -> Result<HSTRING> {
        Ok(h!("P29").clone())
    }
    fn P30(&self) -> Result<HSTRING> {
        Ok(h!("P30").clone())
    }
    fn P31(&self) -> Result<HSTRING> {
        Ok(h!("P31").clone())
    }
    fn P32(&self) -> Result<HSTRING> {
        Ok(h!("P32").clone())
    }
    fn P33(&self) -> Result<HSTRING> {
        Ok(h!("P33").clone())
    }
    fn P34(&self) -> Result<HSTRING> {
        Ok(h!("P34").clone())
    }
    fn P35(&self) -> Result<HSTRING> {
        Ok(h!("P35").clone())
    }
    fn P36(&self) -> Result<HSTRING> {
        Ok(h!("P36").clone())
    }
    fn P37(&self) -> Result<HSTRING> {
        Ok(h!("P37").clone())
    }
    fn P38(&self) -> Result<HSTRING> {
        Ok(h!("P38").clone())
    }
    fn P39(&self) -> Result<HSTRING> {
        Ok(h!("P39").clone())
    }
    fn P40(&self) -> Result<HSTRING> {
        Ok(h!("P40").clone())
    }
    fn P41(&self) -> Result<HSTRING> {
        Ok(h!("P41").clone())
    }
    fn P42(&self) -> Result<HSTRING> {
        Ok(h!("P42").clone())
    }
    fn P43(&self) -> Result<HSTRING> {
        Ok(h!("P43").clone())
    }
    fn P44(&self) -> Result<HSTRING> {
        Ok(h!("P44").clone())
    }
    fn P45(&self) -> Result<HSTRING> {
        Ok(h!("P45").clone())
    }
    fn P46(&self) -> Result<HSTRING> {
        Ok(h!("P46").clone())
    }
    fn P47(&self) -> Result<HSTRING> {
        Ok(h!("P47").clone())
    }
    fn P48(&self) -> Result<HSTRING> {
        Ok(h!("P48").clone())
    }
    fn P49(&self) -> Result<HSTRING> {
        Ok(h!("P49").clone())
    }
    fn P50(&self) -> Result<HSTRING> {
        Ok(h!("P50").clone())
    }
    fn P51(&self) -> Result<HSTRING> {
        Ok(h!("P51").clone())
    }
    fn P52(&self) -> Result<HSTRING> {
        Ok(h!("P52").clone())
    }
    fn P53(&self) -> Result<HSTRING> {
        Ok(h!("P53").clone())
    }
    fn P54(&self) -> Result<HSTRING> {
        Ok(h!("P54").clone())
    }
    fn P55(&self) -> Result<HSTRING> {
        Ok(h!("P55").clone())
    }
    fn P56(&self) -> Result<HSTRING> {
        Ok(h!("P56").clone())
    }
    fn P57(&self) -> Result<HSTRING> {
        Ok(h!("P57").clone())
    }
    fn P58(&self) -> Result<HSTRING> {
        Ok(h!("P58").clone())
    }
    fn P59(&self) -> Result<HSTRING> {
        Ok(h!("P59").clone())
    }
    fn P60(&self) -> Result<HSTRING> {
        Ok(h!("P60").clone())
    }
    fn P61(&self) -> Result<HSTRING> {
        Ok(h!("P61").clone())
    }
    fn P62(&self) -> Result<HSTRING> {
        Ok(h!("P62").clone())
    }
    fn P63(&self) -> Result<HSTRING> {
        Ok(h!("P63").clone())
    }
    fn P64(&self) -> Result<HSTRING> {
        Ok(h!("P64").clone())
    }
    fn P65(&self) -> Result<HSTRING> {
        Ok(h!("P65").clone())
    }
    fn P66(&self) -> Result<HSTRING> {
        Ok(h!("P66").clone())
    }
    fn P67(&self) -> Result<HSTRING> {
        Ok(h!("P67").clone())
    }
    fn P68(&self) -> Result<HSTRING> {
        Ok(h!("P68").clone())
    }
    fn P69(&self) -> Result<HSTRING> {
        Ok(h!("P69").clone())
    }
    fn P70(&self) -> Result<HSTRING> {
        Ok(h!("P70").clone())
    }
    fn P71(&self) -> Result<HSTRING> {
        Ok(h!("P71").clone())
    }
    fn P72(&self) -> Result<HSTRING> {
        Ok(h!("P72").clone())
    }
    fn P73(&self) -> Result<HSTRING> {
        Ok(h!("P73").clone())
    }
    fn P74(&self) -> Result<HSTRING> {
        Ok(h!("P74").clone())
    }
    fn P75(&self) -> Result<HSTRING> {
        Ok(h!("P75").clone())
    }
    fn P76(&self) -> Result<HSTRING> {
        Ok(h!("P76").clone())
    }
    fn P77(&self) -> Result<HSTRING> {
        Ok(h!("P77").clone())
    }
    fn P78(&self) -> Result<HSTRING> {
        Ok(h!("P78").clone())
    }
    fn P79(&self) -> Result<HSTRING> {
        Ok(h!("P79").clone())
    }
    fn P80(&self) -> Result<HSTRING> {
        Ok(h!("P80").clone())
    }
    fn P81(&self) -> Result<HSTRING> {
        Ok(h!("P81").clone())
    }
    fn P82(&self) -> Result<HSTRING> {
        Ok(h!("P82").clone())
    }
    fn P83(&self) -> Result<HSTRING> {
        Ok(h!("P83").clone())
    }
    fn P84(&self) -> Result<HSTRING> {
        Ok(h!("P84").clone())
    }
    fn P85(&self) -> Result<HSTRING> {
        Ok(h!("P85").clone())
    }
    fn P86(&self) -> Result<HSTRING> {
        Ok(h!("P86").clone())
    }
    fn P87(&self) -> Result<HSTRING> {
        Ok(h!("P87").clone())
    }
    fn P88(&self) -> Result<HSTRING> {
        Ok(h!("P88").clone())
    }
    fn P89(&self) -> Result<HSTRING> {
        Ok(h!("P89").clone())
    }
    fn P90(&self) -> Result<HSTRING> {
        Ok(h!("P90").clone())
    }
    fn P91(&self) -> Result<HSTRING> {
        Ok(h!("P91").clone())
    }
    fn P92(&self) -> Result<HSTRING> {
        Ok(h!("P92").clone())
    }
    fn P93(&self) -> Result<HSTRING> {
        Ok(h!("P93").clone())
    }
    fn P94(&self) -> Result<HSTRING> {
        Ok(h!("P94").clone())
    }
    fn P95(&self) -> Result<HSTRING> {
        Ok(h!("P95").clone())
    }
    fn P96(&self) -> Result<HSTRING> {
        Ok(h!("P96").clone())
    }
    fn P97(&self) -> Result<HSTRING> {
        Ok(h!("P97").clone())
    }
    fn P98(&self) -> Result<HSTRING> {
        Ok(h!("P98").clone())
    }
    fn P99(&self) -> Result<HSTRING> {
        Ok(h!("P99").clone())
    }
    fn P100(&self) -> Result<HSTRING> {
        Ok(h!("P100").clone())
    }
}

#[implement(IActivationFactory)]
struct ReaderFactory;

impl IActivationFactory_Impl for ReaderFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Reader.into())
    }
}

#[implement(bindings::Writer)]
struct Writer(RwLock<[HSTRING; 101]>);

impl bindings::IWriter_Impl for Writer {
    fn P0(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[0].clone())
    }
    fn SetP0(&self, p1: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[0] = p1.clone();
        Ok(())
    }

    fn P1(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[1].clone())
    }
    fn SetP1(&self, p1: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[1] = p1.clone();
        Ok(())
    }

    fn P2(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[2].clone())
    }
    fn SetP2(&self, p2: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[2] = p2.clone();
        Ok(())
    }

    fn P3(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[3].clone())
    }
    fn SetP3(&self, p3: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[3] = p3.clone();
        Ok(())
    }

    fn P4(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[4].clone())
    }
    fn SetP4(&self, p4: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[4] = p4.clone();
        Ok(())
    }

    fn P5(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[5].clone())
    }
    fn SetP5(&self, p5: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[5] = p5.clone();
        Ok(())
    }

    fn P6(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[6].clone())
    }
    fn SetP6(&self, p6: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[6] = p6.clone();
        Ok(())
    }

    fn P7(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[7].clone())
    }
    fn SetP7(&self, p7: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[7] = p7.clone();
        Ok(())
    }

    fn P8(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[8].clone())
    }
    fn SetP8(&self, p8: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[8] = p8.clone();
        Ok(())
    }

    fn P9(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[9].clone())
    }
    fn SetP9(&self, p9: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[9] = p9.clone();
        Ok(())
    }

    fn P10(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[10].clone())
    }
    fn SetP10(&self, p10: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[10] = p10.clone();
        Ok(())
    }

    fn P11(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[11].clone())
    }
    fn SetP11(&self, p11: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[11] = p11.clone();
        Ok(())
    }

    fn P12(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[12].clone())
    }
    fn SetP12(&self, p12: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[12] = p12.clone();
        Ok(())
    }

    fn P13(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[13].clone())
    }
    fn SetP13(&self, p13: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[13] = p13.clone();
        Ok(())
    }

    fn P14(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[14].clone())
    }
    fn SetP14(&self, p14: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[14] = p14.clone();
        Ok(())
    }

    fn P15(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[15].clone())
    }
    fn SetP15(&self, p15: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[15] = p15.clone();
        Ok(())
    }

    fn P16(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[16].clone())
    }
    fn SetP16(&self, p16: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[16] = p16.clone();
        Ok(())
    }

    fn P17(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[17].clone())
    }
    fn SetP17(&self, p17: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[17] = p17.clone();
        Ok(())
    }

    fn P18(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[18].clone())
    }
    fn SetP18(&self, p18: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[18] = p18.clone();
        Ok(())
    }

    fn P19(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[19].clone())
    }
    fn SetP19(&self, p19: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[19] = p19.clone();
        Ok(())
    }

    fn P20(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[20].clone())
    }
    fn SetP20(&self, p20: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[20] = p20.clone();
        Ok(())
    }

    fn P21(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[21].clone())
    }
    fn SetP21(&self, p21: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[21] = p21.clone();
        Ok(())
    }

    fn P22(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[22].clone())
    }
    fn SetP22(&self, p22: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[22] = p22.clone();
        Ok(())
    }

    fn P23(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[23].clone())
    }
    fn SetP23(&self, p23: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[23] = p23.clone();
        Ok(())
    }

    fn P24(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[24].clone())
    }
    fn SetP24(&self, p24: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[24] = p24.clone();
        Ok(())
    }

    fn P25(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[25].clone())
    }
    fn SetP25(&self, p25: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[25] = p25.clone();
        Ok(())
    }

    fn P26(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[26].clone())
    }
    fn SetP26(&self, p26: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[26] = p26.clone();
        Ok(())
    }

    fn P27(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[27].clone())
    }
    fn SetP27(&self, p27: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[27] = p27.clone();
        Ok(())
    }

    fn P28(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[28].clone())
    }
    fn SetP28(&self, p28: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[28] = p28.clone();
        Ok(())
    }

    fn P29(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[29].clone())
    }
    fn SetP29(&self, p29: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[29] = p29.clone();
        Ok(())
    }

    fn P30(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[30].clone())
    }
    fn SetP30(&self, p30: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[30] = p30.clone();
        Ok(())
    }

    fn P31(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[31].clone())
    }
    fn SetP31(&self, p31: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[31] = p31.clone();
        Ok(())
    }

    fn P32(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[32].clone())
    }
    fn SetP32(&self, p32: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[32] = p32.clone();
        Ok(())
    }

    fn P33(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[33].clone())
    }
    fn SetP33(&self, p33: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[33] = p33.clone();
        Ok(())
    }

    fn P34(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[34].clone())
    }
    fn SetP34(&self, p34: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[34] = p34.clone();
        Ok(())
    }

    fn P35(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[35].clone())
    }
    fn SetP35(&self, p35: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[35] = p35.clone();
        Ok(())
    }

    fn P36(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[36].clone())
    }
    fn SetP36(&self, p36: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[36] = p36.clone();
        Ok(())
    }

    fn P37(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[37].clone())
    }
    fn SetP37(&self, p37: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[37] = p37.clone();
        Ok(())
    }

    fn P38(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[38].clone())
    }
    fn SetP38(&self, p38: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[38] = p38.clone();
        Ok(())
    }

    fn P39(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[39].clone())
    }
    fn SetP39(&self, p39: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[39] = p39.clone();
        Ok(())
    }

    fn P40(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[40].clone())
    }
    fn SetP40(&self, p40: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[40] = p40.clone();
        Ok(())
    }

    fn P41(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[41].clone())
    }
    fn SetP41(&self, p41: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[41] = p41.clone();
        Ok(())
    }

    fn P42(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[42].clone())
    }
    fn SetP42(&self, p42: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[42] = p42.clone();
        Ok(())
    }

    fn P43(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[43].clone())
    }
    fn SetP43(&self, p43: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[43] = p43.clone();
        Ok(())
    }

    fn P44(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[44].clone())
    }
    fn SetP44(&self, p44: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[44] = p44.clone();
        Ok(())
    }

    fn P45(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[45].clone())
    }
    fn SetP45(&self, p45: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[45] = p45.clone();
        Ok(())
    }

    fn P46(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[46].clone())
    }
    fn SetP46(&self, p46: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[46] = p46.clone();
        Ok(())
    }

    fn P47(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[47].clone())
    }
    fn SetP47(&self, p47: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[47] = p47.clone();
        Ok(())
    }

    fn P48(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[48].clone())
    }
    fn SetP48(&self, p48: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[48] = p48.clone();
        Ok(())
    }

    fn P49(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[49].clone())
    }
    fn SetP49(&self, p49: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[49] = p49.clone();
        Ok(())
    }

    fn P50(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[50].clone())
    }
    fn SetP50(&self, p50: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[50] = p50.clone();
        Ok(())
    }

    fn P51(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[51].clone())
    }
    fn SetP51(&self, p51: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[51] = p51.clone();
        Ok(())
    }

    fn P52(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[52].clone())
    }
    fn SetP52(&self, p52: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[52] = p52.clone();
        Ok(())
    }

    fn P53(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[53].clone())
    }
    fn SetP53(&self, p53: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[53] = p53.clone();
        Ok(())
    }

    fn P54(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[54].clone())
    }
    fn SetP54(&self, p54: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[54] = p54.clone();
        Ok(())
    }

    fn P55(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[55].clone())
    }
    fn SetP55(&self, p55: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[55] = p55.clone();
        Ok(())
    }

    fn P56(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[56].clone())
    }
    fn SetP56(&self, p56: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[56] = p56.clone();
        Ok(())
    }

    fn P57(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[57].clone())
    }
    fn SetP57(&self, p57: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[57] = p57.clone();
        Ok(())
    }

    fn P58(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[58].clone())
    }
    fn SetP58(&self, p58: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[58] = p58.clone();
        Ok(())
    }

    fn P59(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[59].clone())
    }
    fn SetP59(&self, p59: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[59] = p59.clone();
        Ok(())
    }

    fn P60(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[60].clone())
    }
    fn SetP60(&self, p60: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[60] = p60.clone();
        Ok(())
    }

    fn P61(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[61].clone())
    }
    fn SetP61(&self, p61: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[61] = p61.clone();
        Ok(())
    }

    fn P62(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[62].clone())
    }
    fn SetP62(&self, p62: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[62] = p62.clone();
        Ok(())
    }

    fn P63(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[63].clone())
    }
    fn SetP63(&self, p63: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[63] = p63.clone();
        Ok(())
    }

    fn P64(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[64].clone())
    }
    fn SetP64(&self, p64: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[64] = p64.clone();
        Ok(())
    }

    fn P65(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[65].clone())
    }
    fn SetP65(&self, p65: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[65] = p65.clone();
        Ok(())
    }

    fn P66(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[66].clone())
    }
    fn SetP66(&self, p66: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[66] = p66.clone();
        Ok(())
    }

    fn P67(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[67].clone())
    }
    fn SetP67(&self, p67: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[67] = p67.clone();
        Ok(())
    }

    fn P68(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[68].clone())
    }
    fn SetP68(&self, p68: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[68] = p68.clone();
        Ok(())
    }

    fn P69(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[69].clone())
    }
    fn SetP69(&self, p69: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[69] = p69.clone();
        Ok(())
    }

    fn P70(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[70].clone())
    }
    fn SetP70(&self, p70: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[70] = p70.clone();
        Ok(())
    }

    fn P71(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[71].clone())
    }
    fn SetP71(&self, p71: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[71] = p71.clone();
        Ok(())
    }

    fn P72(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[72].clone())
    }
    fn SetP72(&self, p72: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[72] = p72.clone();
        Ok(())
    }

    fn P73(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[73].clone())
    }
    fn SetP73(&self, p73: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[73] = p73.clone();
        Ok(())
    }

    fn P74(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[74].clone())
    }
    fn SetP74(&self, p74: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[74] = p74.clone();
        Ok(())
    }

    fn P75(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[75].clone())
    }
    fn SetP75(&self, p75: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[75] = p75.clone();
        Ok(())
    }

    fn P76(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[76].clone())
    }
    fn SetP76(&self, p76: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[76] = p76.clone();
        Ok(())
    }

    fn P77(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[77].clone())
    }
    fn SetP77(&self, p77: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[77] = p77.clone();
        Ok(())
    }

    fn P78(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[78].clone())
    }
    fn SetP78(&self, p78: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[78] = p78.clone();
        Ok(())
    }

    fn P79(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[79].clone())
    }
    fn SetP79(&self, p79: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[79] = p79.clone();
        Ok(())
    }

    fn P80(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[80].clone())
    }
    fn SetP80(&self, p80: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[80] = p80.clone();
        Ok(())
    }

    fn P81(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[81].clone())
    }
    fn SetP81(&self, p81: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[81] = p81.clone();
        Ok(())
    }

    fn P82(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[82].clone())
    }
    fn SetP82(&self, p82: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[82] = p82.clone();
        Ok(())
    }

    fn P83(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[83].clone())
    }
    fn SetP83(&self, p83: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[83] = p83.clone();
        Ok(())
    }

    fn P84(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[84].clone())
    }
    fn SetP84(&self, p84: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[84] = p84.clone();
        Ok(())
    }

    fn P85(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[85].clone())
    }
    fn SetP85(&self, p85: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[85] = p85.clone();
        Ok(())
    }

    fn P86(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[86].clone())
    }
    fn SetP86(&self, p86: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[86] = p86.clone();
        Ok(())
    }

    fn P87(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[87].clone())
    }
    fn SetP87(&self, p87: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[87] = p87.clone();
        Ok(())
    }

    fn P88(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[88].clone())
    }
    fn SetP88(&self, p88: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[88] = p88.clone();
        Ok(())
    }

    fn P89(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[89].clone())
    }
    fn SetP89(&self, p89: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[89] = p89.clone();
        Ok(())
    }

    fn P90(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[90].clone())
    }
    fn SetP90(&self, p90: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[90] = p90.clone();
        Ok(())
    }

    fn P91(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[91].clone())
    }
    fn SetP91(&self, p91: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[91] = p91.clone();
        Ok(())
    }

    fn P92(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[92].clone())
    }
    fn SetP92(&self, p92: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[92] = p92.clone();
        Ok(())
    }

    fn P93(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[93].clone())
    }
    fn SetP93(&self, p93: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[93] = p93.clone();
        Ok(())
    }

    fn P94(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[94].clone())
    }
    fn SetP94(&self, p94: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[94] = p94.clone();
        Ok(())
    }

    fn P95(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[95].clone())
    }
    fn SetP95(&self, p95: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[95] = p95.clone();
        Ok(())
    }

    fn P96(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[96].clone())
    }
    fn SetP96(&self, p96: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[96] = p96.clone();
        Ok(())
    }

    fn P97(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[97].clone())
    }
    fn SetP97(&self, p97: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[97] = p97.clone();
        Ok(())
    }

    fn P98(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[98].clone())
    }
    fn SetP98(&self, p98: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[98] = p98.clone();
        Ok(())
    }

    fn P99(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[99].clone())
    }
    fn SetP99(&self, p99: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[99] = p99.clone();
        Ok(())
    }

    fn P100(&self) -> Result<HSTRING> {
        let reader = self.0.read().unwrap();
        Ok(reader[100].clone())
    }
    fn SetP100(&self, p100: &HSTRING) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        writer[100] = p100.clone();
        Ok(())
    }
}

#[implement(IActivationFactory)]
struct WriterFactory;

impl IActivationFactory_Impl for WriterFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Writer(RwLock::new(unsafe { std::mem::zeroed() })).into())
    }
}

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    // TODO: this to_string conversion seems wasteful. Should be able to compare
    // these to h!("...") in the match expression.
    let factory: Option<IActivationFactory> = match (*name).to_string().as_str() {
        "Server.Reader" => Some(ReaderFactory.into()),
        "Server.Writer" => Some(WriterFactory.into()),
        _ => None,
    };

    if let Some(factory) = factory {
        *result = std::mem::transmute(factory);
        S_OK
    } else {
        *result = std::ptr::null_mut();
        CLASS_E_CLASSNOTAVAILABLE
    }
}
