#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, Delay,i2c1,prelude::*};



#[entry]
fn main() -> ! {

    let (i2c1, mut delay, mut itm) = aux14::init();
    i2c1.cr2.write(|w| {
        w.start().set_bit();
        w.sadd1().bits(0x3F);
        w.rd_wrn().clear_bit();
        w.nbytes().bits(8);
        w.autoend().clear_bit()
    });
    while i2c1.isr.read().txis().bit_is_clear() {};

        let mut lcd = LiquidCrystal_I2C::new(0x3F, 16, 2,&mut delay, i2c1);
        
        lcd.begin();
        lcd.cursor();
        lcd.blinkon();

    loop {}
}


struct LiquidCrystal_I2C <'a,'b>{
    add : u8,
    cols : u8,
    rows : u8,
    charsize: u8,
    backlightval:u8,
    delay:&'b mut Delay,
    i2c1:&'a i2c1::RegisterBlock
}
impl <'a,'b> LiquidCrystal_I2C <'a,'b>{

    fn begin(&mut self){
        let displayFunction:u8 = 0x00 | 0x08 | 0x00;
        let displaymode:u8 = 0x02 | 0x00;
        self.delay.delay_ms(50u32);
        self.expanderWrite(0x08);
        self.delay.delay_ms(1000u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(4500u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(4500u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(150u32);
        self.write4bits(0x02 << 4);
        self.command(0x20| displayFunction);
        self.display();
        self.clear();
        self.command(0x04 | displaymode);
        self.home();
    }
    fn clear(&mut self){
        self.command(0x01);
        self.delay.delay_us(2000u32);
    }
    fn home(&mut self){
        self.command(0x02);
        self.delay.delay_us(2000u32)
    }
    fn display(&mut self){
        let displaycontrol:u8 = 0x04 | 0x00 | 0x00;
        self.command(0x08 | displaycontrol)

    }
    fn cursor(&mut self){
        let displaycontrol:u8 = 0x04 | 0x02 | 0x00;
        self.command(0x08 | displaycontrol)
    }
    fn blinkon(&mut self){
        let displaycontrol:u8 = 0x04 | 0x02 | 0x01;
        self.command(0x08 | displaycontrol)
    }
    fn command(&mut self,value:u8){
        self.send(value, 0)
    }
    fn send(&mut self,value:u8,mode:u8){
        let highnib:u8 = value&0xf0;
        let lownib:u8 = (value<<4)&0xf0;
        self.write4bits(highnib|mode);
        self.write4bits(lownib|mode);
    }
    fn write4bits(&mut self,value:u8){
        self.expanderWrite(value);
	    self.pulseEnable(value);
    }
    fn expanderWrite(&self,data:u8){
        // self.i2c1.cr2.write(|w| {
        //     w.start().set_bit();
        //     w.sadd1().bits(0x3F);
        //     w.rd_wrn().clear_bit();
        //     w.nbytes().bits(1);
        //     w.autoend().clear_bit()
        // });
        // while self.i2c1.isr.read().txis().bit_is_clear() {};
        self.i2c1.txdr.write(|w| w.txdata().bits( data | 0x08));
        while self.i2c1.isr.read().tc().bit_is_clear() {}
    }

    fn pulseEnable(&mut self,data:u8){
        let EN :u8 = 0b00000100;
        self.expanderWrite(data|EN);
        self.delay.delay_us(1u32);
        self.expanderWrite(data &!EN);
	    self.delay.delay_us(1u32);	
    }
}

impl <'a,'b> LiquidCrystal_I2C <'a,'b>{

    fn new(add:u8,cols:u8,rows:u8,delay:&'b mut Delay, i2c1:&'a i2c1::RegisterBlock )->LiquidCrystal_I2C<'a,'b>{
       
        git init{
            add : add,
            cols : cols,
            rows : rows,
            charsize: 0x00,
            backlightval:0x08,
            delay:delay,
            i2c1:i2c1
        }
    }
}