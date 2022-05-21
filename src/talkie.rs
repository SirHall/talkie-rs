use crate::data;

#[derive(Debug, Clone)]
pub struct Talkie
{
    synth_period : u8,
    synth_energy : u16,

    synth_k1 : i16,
    synth_k2 : i16,

    synth_k3 :  i8,
    synth_k4 :  i8,
    synth_k5 :  i8,
    synth_k6 :  i8,
    synth_k7 :  i8,
    synth_k8 :  i8,
    synth_k9 :  i8,
    synth_k10 : i8,

    next_pwm :       u8,
    period_counter : u8,
    x0 :             i16,
    x1 :             i16,
    x2 :             i16,
    x3 :             i16,
    x4 :             i16,
    x5 :             i16,
    x6 :             i16,
    x7 :             i16,
    x8 :             i16,
    x9 :             i16,
    x10 :            i16,

    synth_rand : u16,
}

impl Default for Talkie
{
    fn default() -> Self
    {
        Self {
            synth_period : 0,
            synth_energy : 0,

            synth_k1 : 0,
            synth_k2 : 0,

            synth_k3 :  0,
            synth_k4 :  0,
            synth_k5 :  0,
            synth_k6 :  0,
            synth_k7 :  0,
            synth_k8 :  0,
            synth_k9 :  0,
            synth_k10 : 0,

            next_pwm :       0,
            period_counter : 0,
            x0 :             0,
            x1 :             0,
            x2 :             0,
            x3 :             0,
            x4 :             0,
            x5 :             0,
            x6 :             0,
            x7 :             0,
            x8 :             0,
            x9 :             0,
            x10 :            0,

            synth_rand : 1,
        }
    }
}

impl Talkie
{
    // Generates entire sentences worth of text at a time
    // pub fn say(&self, addr : &[u8]) -> Vec<u8> { vec![0] }

    pub fn get_sample() -> i16 { 0 }

    // The ROMs used with the TI speech were serial, not byte wide.
    // Here's a handy routine to flip ROM data which is usually reversed.
    pub fn rev(mut a : u8) -> u8
    {
        // 76543210
        a = (a >> 4) | (a << 4); // Swap in groups of 4
                                 // 32107654
        a = ((a & 0xcc) >> 2) | ((a & 0x33) << 2); // Swap in groups of 2
                                                   // 10325476
        a = ((a & 0xaa) >> 1) | ((a & 0x55) << 1); // Swap bit pairs
                                                   // 01234567
        a
    }

    pub fn getBits(bits : u8, voice_data : &[u8], data_idx : &mut usize, ptr_bit : &mut u8) -> u8
    {
        let mut data : u16 = (Self::rev(voice_data[*data_idx]) as u16) << 8;
        if *ptr_bit + bits > 8
        {
            data |= Self::rev(voice_data[*data_idx + 1]) as u16;
        }
        data <<= *ptr_bit as u16;
        let value : u8 = (data >> (16 - bits)) as u8;
        *ptr_bit += bits;
        if *ptr_bit >= 8
        {
            *ptr_bit -= 8;
            *data_idx += 1;
        }

        value
    }

    pub fn say(&mut self, voice_data : &[u8])
    {
        let mut ptr_bit = 0u8;
        let mut data_idx = 0usize;

        let mut energy = 0u8;

        loop
        {
            let mut repeat = 0u8;

            // Read speech data, processing the variable size frames.

            energy = Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit);
            if energy == 0
            {
                // Energy = 0: rest frame
                self.synth_energy = 0;
            }
            else if energy == 0
            {
                // Energy = 15: stop frame. Silence the synthesiser.
                self.synth_energy = 0;
                self.synth_k1 = 0;
                self.synth_k2 = 0;
                self.synth_k3 = 0;
                self.synth_k4 = 0;
                self.synth_k5 = 0;
                self.synth_k6 = 0;
                self.synth_k7 = 0;
                self.synth_k8 = 0;
                self.synth_k9 = 0;
                self.synth_k10 = 0;
            }
            else
            {
                self.synth_energy = data::tms_energy[energy as usize] as u16;

                repeat = Self::getBits(1, voice_data, &mut data_idx, &mut ptr_bit);

                self.synth_period =
                    data::tms_period[Self::getBits(6, voice_data, &mut data_idx, &mut ptr_bit) as usize];

                // A repeat frame uses the last coefficients
                if repeat == 0
                {
                    // All frames use the first 4 coefficients
                    self.synth_k1 = data::tms_k1[Self::getBits(5, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                    self.synth_k2 = data::tms_k2[Self::getBits(5, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                    self.synth_k3 = data::tms_k3[Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                    self.synth_k4 = data::tms_k4[Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                    if self.synth_period != 0
                    {
                        // Voiced frames use 6 extra coefficients.
                        self.synth_k5 =
                            data::tms_k5[Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                        self.synth_k6 =
                            data::tms_k6[Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                        self.synth_k7 =
                            data::tms_k7[Self::getBits(4, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                        self.synth_k8 =
                            data::tms_k8[Self::getBits(3, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                        self.synth_k9 =
                            data::tms_k9[Self::getBits(3, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                        self.synth_k10 =
                            data::tms_k10[Self::getBits(3, voice_data, &mut data_idx, &mut ptr_bit) as usize];
                    }
                }
            }

            if energy == 0
            {
                break;
            }
        }
    }

    pub fn pwm_gen(&mut self)
    {
        let mut u0 = 0i16;
        let mut u1 = 0i16;
        let mut u2 = 0i16;
        let mut u3 = 0i16;
        let mut u4 = 0i16;
        let mut u5 = 0i16;
        let mut u6 = 0i16;
        let mut u7 = 0i16;
        let mut u8 = 0i16;
        let mut u9 = 0i16;
        let mut u10 = 0i16;

        if self.synth_period != 0
        {
            // Voiced source
            if self.period_counter < self.synth_period
            {
                self.period_counter += 1;
            }
            else
            {
                self.period_counter = 0;
            }
            if self.period_counter < data::CHIRP_SIZE
            {
                u10 = (((data::chirp[self.period_counter as usize] as u32) * (self.synth_energy as u32)) >> 8) as i16;
            }
            else
            {
                u10 = 0;
            }
        }
        else
        {
            // Unvoiced source
            self.synth_rand = (self.synth_rand >> 1) ^ (if (self.synth_rand & 1) > 0 { 0xB800 } else { 0 });
            if (self.synth_rand & 1) > 0
            {
                u10 = self.synth_energy as i16;
            }
            else
            {
                u10 = -(self.synth_energy as i16);
            };
        }

        // Lattice filter forward path
        u9 = u10 - (((self.synth_k10 as i16) * self.x9) >> 7);
        u8 = u9 - (((self.synth_k9 as i16) * self.x8) >> 7);
        u7 = u8 - (((self.synth_k8 as i16) * self.x7) >> 7);
        u6 = u7 - (((self.synth_k7 as i16) * self.x6) >> 7);
        u5 = u6 - (((self.synth_k6 as i16) * self.x5) >> 7);
        u4 = u5 - (((self.synth_k5 as i16) * self.x4) >> 7);
        u3 = u4 - (((self.synth_k4 as i16) * self.x3) >> 7);
        u2 = u3 - (((self.synth_k3 as i16) * self.x2) >> 7);
        u1 = u2 - ((((self.synth_k2 as i32) * (self.x1 as i32)) >> 15) as i16);
        u0 = u1 - ((((self.synth_k1 as i32) * (self.x0 as i32)) >> 15) as i16);

        // Output clamp
        u0 = u0.clamp(-512, 511);

        // Lattice filter reverse path
        self.x9 = self.x8 + (((self.synth_k9 as i16) * u8) >> 7);
        self.x8 = self.x7 + (((self.synth_k8 as i16) * u7) >> 7);
        self.x7 = self.x6 + (((self.synth_k7 as i16) * u6) >> 7);
        self.x6 = self.x5 + (((self.synth_k6 as i16) * u5) >> 7);
        self.x5 = self.x4 + (((self.synth_k5 as i16) * u4) >> 7);
        self.x4 = self.x3 + (((self.synth_k4 as i16) * u3) >> 7);
        self.x3 = self.x2 + (((self.synth_k3 as i16) * u2) >> 7);
        self.x2 = self.x1 + ((((self.synth_k2 as i32) * (u1 as i32)) >> 15) as i16);
        self.x1 = self.x0 + ((((self.synth_k1 as i32) * (u0 as i32)) >> 15) as i16);
        self.x0 = u0;

        self.next_pwm = ((u0 >> 2) + 0x80) as u8;
    }
}
