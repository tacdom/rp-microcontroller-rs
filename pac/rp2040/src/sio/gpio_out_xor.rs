#[doc = "Reader of register GPIO_OUT_XOR"]
pub type R = crate::R<u32, super::GPIO_OUT_XOR>;
#[doc = "Writer for register GPIO_OUT_XOR"]
pub type W = crate::W<u32, super::GPIO_OUT_XOR>;
#[doc = "Register GPIO_OUT_XOR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_OUT_XOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_OUT_XOR`"]
pub type GPIO_OUT_XOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPIO_OUT_XOR`"]
pub struct GPIO_OUT_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`"]
    #[inline(always)]
    pub fn gpio_out_xor(&self) -> GPIO_OUT_XOR_R {
        GPIO_OUT_XOR_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`"]
    #[inline(always)]
    pub fn gpio_out_xor(&mut self) -> GPIO_OUT_XOR_W {
        GPIO_OUT_XOR_W { w: self }
    }
}
