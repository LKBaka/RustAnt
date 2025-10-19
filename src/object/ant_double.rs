use bigdecimal::num_bigint::Sign;
use bigdecimal::{BigDecimal, Context};
use std::any::Any;
use std::str::FromStr;

use crate::impl_object;
use crate::obj_enum::object::Object;
use crate::object::id_counter::next_id;
use crate::object::object::{DOUBLE, IAntObject, ObjectType};

#[derive(Clone)]
pub struct AntDouble {
    id: usize,
    pub value: BigDecimal,
}

impl From<BigDecimal> for AntDouble {
    fn from(value: BigDecimal) -> Self {
        AntDouble {
            id: next_id(),
            value,
        }
    }
}

impl From<i32> for AntDouble {
    fn from(value: i32) -> Self {
        AntDouble {
            id: next_id(),
            value: BigDecimal::from(value),
        }
    }
}

impl From<i64> for AntDouble {
    fn from(value: i64) -> Self {
        AntDouble {
            id: next_id(),
            value: BigDecimal::from(value),
        }
    }
}

impl From<f64> for AntDouble {
    fn from(value: f64) -> Self {
        AntDouble {
            id: next_id(),
            value: BigDecimal::from_str(&value.to_string()).unwrap(),
        }
    }
}

impl IAntObject for AntDouble {
    fn get_type(&self) -> ObjectType {
        DOUBLE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone())
    }

    fn get_base(&self) -> Option<Object> {
        None
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.normalized().to_string())
    }

    fn equals(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id
            || if other.get_type() == DOUBLE {
                other.as_any().downcast_ref::<AntDouble>().unwrap().value == self.value
            } else if other.get_type() == "INT" {
                // 支持与 AntInt 的比较
                if let Some(int_obj) = other
                    .as_any()
                    .downcast_ref::<crate::object::ant_int::AntInt>()
                {
                    &int_obj.value == &self.value
                } else {
                    false
                }
            } else {
                false
            }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object!(AntDouble);

fn fast_path(d: &BigDecimal) -> bool {
    use bigdecimal::num_bigint::ToBigInt;

    // 负数直接拒绝（sqrt 返回 NaN，与 BigDecimal 行为一致）
    if d.sign() == Sign::Minus {
        return false;
    }

    // 必须无小数位（scale <= 0）
    if !d.is_integer() {
        return false;
    }
    
    // 整数部分位数 <= 53 bit
    let int = d.with_scale(0).to_bigint().expect("already checked");
    int.bits() <= 53
}

#[inline(always)]
pub fn sqrt_big_dec(d: &BigDecimal, ctx: &Context) -> Option<BigDecimal> {
    use bigdecimal::num_bigint::ToBigInt;
    use num_traits::ToPrimitive;
    use num_traits::FromPrimitive;

    const MAX_PREC: u64 = 16;

    let prec = ctx.precision().get();

    // fast path    
    if prec <= MAX_PREC && fast_path(d) {
        // 此时 d 一定是非负整数，且 < 2^53
        let i = d.to_bigint().unwrap();
        let v = i.to_f64()?;      // 必成功
        let r = v.sqrt();         // 硬件指令
        
        // f64 -> BigDecimal 再按用户精度舍入一次
        return BigDecimal::from_f64(r)
            .map(|b| b.with_precision_round(ctx.precision(), ctx.rounding_mode()));
    }

    d.sqrt_with_context(ctx)
}

pub fn sqrt_default(d: &BigDecimal) -> Option<BigDecimal> {
    use std::num::NonZero;

    let ctx = Context::new(NonZero::new(16).unwrap(), bigdecimal::RoundingMode::HalfUp);

    sqrt_big_dec(d, &ctx)
}