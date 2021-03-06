#[doc = "Reader of register GPIO8_CTRL"]
pub type R = crate::R<u32, super::GPIO8_CTRL>;
#[doc = "Writer for register GPIO8_CTRL"]
pub type W = crate::W<u32, super::GPIO8_CTRL>;
#[doc = "Register GPIO8_CTRL `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::GPIO8_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQOVER_A {
    #[doc = "0: don't invert the interrupt"]
    NORMAL = 0,
    #[doc = "1: invert the interrupt"]
    INVERT = 1,
    #[doc = "2: drive interrupt low"]
    LOW = 2,
    #[doc = "3: drive interrupt high"]
    HIGH = 3,
}
impl From<IRQOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQOVER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IRQOVER`"]
pub type IRQOVER_R = crate::R<u8, IRQOVER_A>;
impl IRQOVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQOVER_A {
        match self.bits {
            0 => IRQOVER_A::NORMAL,
            1 => IRQOVER_A::INVERT,
            2 => IRQOVER_A::LOW,
            3 => IRQOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRQOVER_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == IRQOVER_A::INVERT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IRQOVER_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IRQOVER_A::HIGH
    }
}
#[doc = "Write proxy for field `IRQOVER`"]
pub struct IRQOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQOVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQOVER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "don't invert the interrupt"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRQOVER_A::NORMAL)
    }
    #[doc = "invert the interrupt"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(IRQOVER_A::INVERT)
    }
    #[doc = "drive interrupt low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IRQOVER_A::LOW)
    }
    #[doc = "drive interrupt high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IRQOVER_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INOVER_A {
    #[doc = "0: don't invert the peri input"]
    NORMAL = 0,
    #[doc = "1: invert the peri input"]
    INVERT = 1,
    #[doc = "2: drive peri input low"]
    LOW = 2,
    #[doc = "3: drive peri input high"]
    HIGH = 3,
}
impl From<INOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: INOVER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INOVER`"]
pub type INOVER_R = crate::R<u8, INOVER_A>;
impl INOVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INOVER_A {
        match self.bits {
            0 => INOVER_A::NORMAL,
            1 => INOVER_A::INVERT,
            2 => INOVER_A::LOW,
            3 => INOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == INOVER_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == INOVER_A::INVERT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INOVER_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INOVER_A::HIGH
    }
}
#[doc = "Write proxy for field `INOVER`"]
pub struct INOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> INOVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INOVER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "don't invert the peri input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(INOVER_A::NORMAL)
    }
    #[doc = "invert the peri input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(INOVER_A::INVERT)
    }
    #[doc = "drive peri input low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(INOVER_A::LOW)
    }
    #[doc = "drive peri input high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(INOVER_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OEOVER_A {
    #[doc = "0: drive output enable from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "1: drive output enable from inverse of peripheral signal selected by funcsel"]
    INVERT = 1,
    #[doc = "2: disable output"]
    DISABLE = 2,
    #[doc = "3: enable output"]
    ENABLE = 3,
}
impl From<OEOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: OEOVER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OEOVER`"]
pub type OEOVER_R = crate::R<u8, OEOVER_A>;
impl OEOVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEOVER_A {
        match self.bits {
            0 => OEOVER_A::NORMAL,
            1 => OEOVER_A::INVERT,
            2 => OEOVER_A::DISABLE,
            3 => OEOVER_A::ENABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OEOVER_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OEOVER_A::INVERT
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OEOVER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OEOVER_A::ENABLE
    }
}
#[doc = "Write proxy for field `OEOVER`"]
pub struct OEOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OEOVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEOVER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OEOVER_A::NORMAL)
    }
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(OEOVER_A::INVERT)
    }
    #[doc = "disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OEOVER_A::DISABLE)
    }
    #[doc = "enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OEOVER_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTOVER_A {
    #[doc = "0: drive output from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "1: drive output from inverse of peripheral signal selected by funcsel"]
    INVERT = 1,
    #[doc = "2: drive output low"]
    LOW = 2,
    #[doc = "3: drive output high"]
    HIGH = 3,
}
impl From<OUTOVER_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTOVER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTOVER`"]
pub type OUTOVER_R = crate::R<u8, OUTOVER_A>;
impl OUTOVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTOVER_A {
        match self.bits {
            0 => OUTOVER_A::NORMAL,
            1 => OUTOVER_A::INVERT,
            2 => OUTOVER_A::LOW,
            3 => OUTOVER_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OUTOVER_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OUTOVER_A::INVERT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTOVER_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTOVER_A::HIGH
    }
}
#[doc = "Write proxy for field `OUTOVER`"]
pub struct OUTOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTOVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTOVER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "drive output from peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OUTOVER_A::NORMAL)
    }
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(OUTOVER_A::INVERT)
    }
    #[doc = "drive output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUTOVER_A::LOW)
    }
    #[doc = "drive output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUTOVER_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "0-31 -> selects pin function according to the gpio table\\n 31 == NULL\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNCSEL_A {
    #[doc = "1: `1`"]
    SPI1_RX = 1,
    #[doc = "2: `10`"]
    UART1_TX = 2,
    #[doc = "3: `11`"]
    I2C0_SDA = 3,
    #[doc = "4: `100`"]
    PWM_A_4 = 4,
    #[doc = "5: `101`"]
    SIO_8 = 5,
    #[doc = "6: `110`"]
    PIO0_8 = 6,
    #[doc = "7: `111`"]
    PIO1_8 = 7,
    #[doc = "8: `1000`"]
    USB_MUXING_EXTPHY_RCV = 8,
    #[doc = "9: `1001`"]
    USB_MUXING_VBUS_EN = 9,
    #[doc = "31: `11111`"]
    NULL = 31,
}
impl From<FUNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNCSEL`"]
pub type FUNCSEL_R = crate::R<u8, FUNCSEL_A>;
impl FUNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FUNCSEL_A::SPI1_RX),
            2 => Val(FUNCSEL_A::UART1_TX),
            3 => Val(FUNCSEL_A::I2C0_SDA),
            4 => Val(FUNCSEL_A::PWM_A_4),
            5 => Val(FUNCSEL_A::SIO_8),
            6 => Val(FUNCSEL_A::PIO0_8),
            7 => Val(FUNCSEL_A::PIO1_8),
            8 => Val(FUNCSEL_A::USB_MUXING_EXTPHY_RCV),
            9 => Val(FUNCSEL_A::USB_MUXING_VBUS_EN),
            31 => Val(FUNCSEL_A::NULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_RX`"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == FUNCSEL_A::SPI1_RX
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == FUNCSEL_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `I2C0_SDA`"]
    #[inline(always)]
    pub fn is_i2c0_sda(&self) -> bool {
        *self == FUNCSEL_A::I2C0_SDA
    }
    #[doc = "Checks if the value of the field is `PWM_A_4`"]
    #[inline(always)]
    pub fn is_pwm_a_4(&self) -> bool {
        *self == FUNCSEL_A::PWM_A_4
    }
    #[doc = "Checks if the value of the field is `SIO_8`"]
    #[inline(always)]
    pub fn is_sio_8(&self) -> bool {
        *self == FUNCSEL_A::SIO_8
    }
    #[doc = "Checks if the value of the field is `PIO0_8`"]
    #[inline(always)]
    pub fn is_pio0_8(&self) -> bool {
        *self == FUNCSEL_A::PIO0_8
    }
    #[doc = "Checks if the value of the field is `PIO1_8`"]
    #[inline(always)]
    pub fn is_pio1_8(&self) -> bool {
        *self == FUNCSEL_A::PIO1_8
    }
    #[doc = "Checks if the value of the field is `USB_MUXING_EXTPHY_RCV`"]
    #[inline(always)]
    pub fn is_usb_muxing_extphy_rcv(&self) -> bool {
        *self == FUNCSEL_A::USB_MUXING_EXTPHY_RCV
    }
    #[doc = "Checks if the value of the field is `USB_MUXING_VBUS_EN`"]
    #[inline(always)]
    pub fn is_usb_muxing_vbus_en(&self) -> bool {
        *self == FUNCSEL_A::USB_MUXING_VBUS_EN
    }
    #[doc = "Checks if the value of the field is `NULL`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == FUNCSEL_A::NULL
    }
}
#[doc = "Write proxy for field `FUNCSEL`"]
pub struct FUNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut W {
        self.variant(FUNCSEL_A::SPI1_RX)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(FUNCSEL_A::UART1_TX)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2c0_sda(self) -> &'a mut W {
        self.variant(FUNCSEL_A::I2C0_SDA)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm_a_4(self) -> &'a mut W {
        self.variant(FUNCSEL_A::PWM_A_4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sio_8(self) -> &'a mut W {
        self.variant(FUNCSEL_A::SIO_8)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pio0_8(self) -> &'a mut W {
        self.variant(FUNCSEL_A::PIO0_8)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pio1_8(self) -> &'a mut W {
        self.variant(FUNCSEL_A::PIO1_8)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn usb_muxing_extphy_rcv(self) -> &'a mut W {
        self.variant(FUNCSEL_A::USB_MUXING_EXTPHY_RCV)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn usb_muxing_vbus_en(self) -> &'a mut W {
        self.variant(FUNCSEL_A::USB_MUXING_VBUS_EN)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn null(self) -> &'a mut W {
        self.variant(FUNCSEL_A::NULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn irqover(&self) -> IRQOVER_R {
        IRQOVER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn inover(&self) -> INOVER_R {
        INOVER_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn oeover(&self) -> OEOVER_R {
        OEOVER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn outover(&self) -> OUTOVER_R {
        OUTOVER_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - 0-31 -> selects pin function according to the gpio table\\n 31 == NULL"]
    #[inline(always)]
    pub fn funcsel(&self) -> FUNCSEL_R {
        FUNCSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn irqover(&mut self) -> IRQOVER_W {
        IRQOVER_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn inover(&mut self) -> INOVER_W {
        INOVER_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn oeover(&mut self) -> OEOVER_W {
        OEOVER_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn outover(&mut self) -> OUTOVER_W {
        OUTOVER_W { w: self }
    }
    #[doc = "Bits 0:4 - 0-31 -> selects pin function according to the gpio table\\n 31 == NULL"]
    #[inline(always)]
    pub fn funcsel(&mut self) -> FUNCSEL_W {
        FUNCSEL_W { w: self }
    }
}
