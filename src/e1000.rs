/*Ethernet Driver for Intel PRO 1000/MT 
 * Device ID : 0x100E 
 * Vendor ID : 0x8086
 * Found in Qemu architecture.
 * Intel is the vendor.
*/

use peripherals::mycpu::Port;
use driver::{Driver, NetworkDriver};
//use pci::{PciManifest, PortGranter};
use pci::* ; 

/////////////////////MACROS BEGIN //////////////////////////////////////////////

pub static INTEL_VEND  =   0x8086  // Vendor ID for Intel 
pub static E1000_DEV   =   0x100E  // Device ID for the e1000 Qemu, Bochs, and VirtualBox emmulated NICs
pub static E1000_I217  =   0x153A  // Device ID for Intel I217
pub static E1000_82577LM=   0x10EA  // Device ID for Intel 82577LM
 
 
// I have gathered those from different Hobby online operating systems instead of getting them one by one from the manual
 
pub static REG_CTRL     =   0x0000
pub static REG_STATUS   =   0x0008
pub static REG_EEPROM   =   0x0014
pub static REG_CTRL_EXT =   0x0018
pub static REG_IMASK    =   0x00D0
pub static REG_RCTRL    =   0x0100
pub static REG_RXDESCLO =   0x2800
pub static REG_RXDESCHI =   0x2804
pub static REG_RXDESCLEN=   0x2808
pub static REG_RXDESCHEAD = 0x2810
pub static REG_RXDESCTAIL = 0x2818
 
pub static REG_TCTRL    =   0x0400
pub static REG_TXDESCLO =   0x3800
pub static REG_TXDESCHI =   0x3804
pub static REG_TXDESCLEN=   0x3808
pub static REG_TXDESCHEAD = 0x3810
pub static REG_TXDESCTAIL = 0x3818
 
 
pub static REG_RDTR      =   0x2820 // RX Delay Timer Register
pub static REG_RXDCTL    =   0x3828 // RX Descriptor Control
pub static REG_RADV      =   0x282C // RX Int. Absolute Delay Timer
pub static REG_RSRPD     =   0x2C00 // RX Small Packet Detect Interrupt
 
 
 
pub static REG_TIPG      =   0x0410      // Transmit Inter Packet Gap
pub static ECTRL_SLU     =   0x40        //set link up
 
 
pub static RCTL_EN                    =    (1 << 1)    // Receiver Enable
pub static RCTL_SBP                   =    (1 << 2)    // Store Bad Packets
pub static RCTL_UPE                   =    (1 << 3)    // Unicast Promiscuous Enabled
pub static RCTL_MPE                   =    (1 << 4)    // Multicast Promiscuous Enabled
pub static RCTL_LPE                   =    (1 << 5)    // Long Packet Reception Enable
pub static RCTL_LBM_NONE              =     (0 << 6)    // No Loopback
pub static RCTL_LBM_PHY               =     (3 << 6)    // PHY or external SerDesc loopback
pub static RTCL_RDMTS_HALF            =     (0 << 8)    // Free Buffer Threshold is 1/2 of RDLEN
pub static RTCL_RDMTS_QUARTER         =    (1 << 8)    // Free Buffer Threshold is 1/4 of RDLEN
pub static RTCL_RDMTS_EIGHTH          =     (2 << 8)    // Free Buffer Threshold is 1/8 of RDLEN
pub static RCTL_MO_36                 =     (0 << 12)   // Multicast Offset - bits 47:36
pub static RCTL_MO_35                 =    (1 << 12)   // Multicast Offset - bits 46:35
pub static RCTL_MO_34                 =     (2 << 12)   // Multicast Offset - bits 45:34
pub static RCTL_MO_32                 =     (3 << 12)   // Multicast Offset - bits 43:32
pub static RCTL_BAM                   =    (1 << 15)   // Broadcast Accept Mode
pub static RCTL_VFE                   =    (1 << 18)   // VLAN Filter Enable
pub static RCTL_CFIEN                 =    (1 << 19)   // Canonical Form Indicator Enable
pub static RCTL_CFI                   =    (1 << 20)   // Canonical Form Indicator Bit Value
pub static RCTL_DPF                   =    (1 << 22)   // Discard Pause Frames
pub static RCTL_PMCF                  =    (1 << 23)   // Pass MAC Control Frames
pub static RCTL_SECRC                 =    (1 << 26)   // Strip Ethernet CRC
 
// Buffer Sizes
pub static RCTL_BSIZE_256            =    (3 << 16)
pub static RCTL_BSIZE_512            =      (2 << 16)
pub static RCTL_BSIZE_1024            =    (1 << 16)
pub static RCTL_BSIZE_2048           =     (0 << 16)
pub static RCTL_BSIZE_4096           =      ((3 << 16) | (1 << 25))
pub static RCTL_BSIZE_8192           =      ((2 << 16) | (1 << 25))
pub static RCTL_BSIZE_16384          =      ((1 << 16) | (1 << 25))
 
 
// Transmit Command
 
pub static CMD_EOP                    =    (1 << 0)    // End of Packet
pub static CMD_IFCS                   =    (1 << 1)    // Insert FCS
pub static CMD_IC                     =    (1 << 2)    // Insert Checksum
pub static CMD_RS                     =    (1 << 3)    // Report Status
pub static CMD_RPS                    =    (1 << 4)    // Report Packet Sent
pub static CMD_VLE                    =    (1 << 6)    // VLAN Packet Enable
pub static CMD_IDE                    =    (1 << 7)    // Interrupt Delay Enable
 
 
// TCTL Register
 
pub static TCTL_EN                    =    (1 << 1)    // Transmit Enable
pub static TCTL_PSP                   =    (1 << 3)    // Pad Short Packets
pub static TCTL_CT_SHIFT              =     4           // Collision Threshold
pub static TCTL_COLD_SHIFT            =     12          // Collision Distance
pub static TCTL_SWXOFF                =    (1 << 22)   // Software XOFF Transmission
pub static TCTL_RTLC                  =    (1 << 24)   // Re-transmit on Late Collision
 
pub static TSTA_DD                    =    (1 << 0)    // Descriptor Done
pub static TSTA_EC                    =    (1 << 1)    // Excess Collisions
pub static TSTA_LC                    =    (1 << 2)    // Late Collision
pub static LSTA_TU                    =    (1 << 3)    // Transmit Underrun

pub const E1000_NUM_RX_DESC = 32
pub const E1000_NUM_TX_DESC = 8

/////////////////////////MACROS END ///////////////////////////////////////////////

#[packed]
pub struct e1000_rx_desc {
	address : u64 , 
	length : u16, 
	status : u8, '
	checksum : u16, 
	errors : u8, 
	special : u16
}

#[packed]
pub struct e1000_tx_desc {
	address : u64 , 
	length : u16, 
	status : u8, '
	checksum : u16, 
	errors : u8, 
	special : u16,
	cso : u8, 
	cmd: u8, 
	css: u8
}

pub struct e1000 {
	bar_type : u8 , // Type of BAR0
	io_base : u16 , // IO base address
	mem_base : u64 , //MMIO base address
	eeprom_exists : bool, 
	mac : [u8;6], // MAC Address 
	rx_descs : [e1000_rx_desc; E1000_NUM_RX_DESC], 
	tx_descs : [e1000_tx_desc; E1000_NUM_TX_DESC], 
	rx_cur : u16 ,   // Current descriptors that we are using
	tx_cur : u16 ,   // Used in Round robin way. 

}

impl e1000 {

	// Something may go wrong here.
	pub fn write_command(self,p_address: u16, p_value: u16) {
		
		if (self.bar_type == 0){
			println!("This is a memory op! You should panic!");
			loop{} 
		}
		else{
			Port::new(self.io_base).out32(p_address) ; 
			Port::new(self.io_base+4).out32(p_value) ; 
		}
	}

	pub fn read_command(self,p_address: u16) -> u32 {
		if (self.bar_type == 0){
			println!("This is a memory op! You should panic!");
			loop{} 
		}
		else{
			Port::new(self.io_base).out32(p_address) ; 
			return Port::new(self.io_base+4).in32() ; 
		}
	}
	pub fn detect_eeprom(self) -> bool {
		let mut val : u32 = 0 ; 
		self.write_command(REG_EEPROM, 0x1) ; 

		for i in 0..100 {
			if self.eeprom_exists== true {
				break ; 
			}
			let mut val = self.read_command(REG_EEPROM) ; 
			if val&0x10 {
				self.eeprom_exists = true ; 
			}
			else {
				self.eeprom_exists = false ; 
			}
		}
		return self.eeprom_exists ; 
	}
	fn eeprom_read(self,addr: u8) -> u32 {
		let mut data : u16 = 0 ; 
		let mut tmp : u16 = 0 ; 
		if self.eeprom_exists==true {
			self.write_command(REG_EEPROM, (1)|((u32)addr)<<8) ; 
			while( !((tmp = self.read_command(REG_EEPROM)) & (1 << 4)) );
		}
		else{
			self.write_command( REG_EEPROM, (1) | ((u32)(addr) << 2));
            while( !((tmp = self.read_command(REG_EEPROM)) & (1 << 1)) );
		}
		data = (u16)((tmp >> 16) & 0xFFFF);
		return data;
	}


	bool read_mac_addr(self) ->bool {
		if self.eeprom_exists==true {
			let mut temp : u32 ; 
			temp = self.eepromRead(0);
			self.mac[0] = temp &0xff;
	        self.mac[1] = temp >> 8;
	        temp = self.eepromRead(1);
	        self.mac[2] = temp &0xff;
	        self.mac[3] = temp >> 8;
	        temp = self.eepromRead(2);
	        self.mac[4] = temp &0xff;
	        self.mac[5] = temp >> 8;
		}
		else {
			let mem_base_mac_8 = (self.mem_base+0x5400) as *const u8 ; 
			let mem_base_mac_32 = (self.mem_base+0x5400) as *const u32 ; 
			if  mem_base_mac_32[0] != 0 {
	            for(int i = 0; i < 6; i++)
	            {
	                self.mac[i] = mem_base_mac_8[i];
	            }
        	}
        	else return false;
        }
    	return true;
	}
	
	pub fn rxinit(self) {
		// TODO 
	}

	pub fn txinit(self) {
		// TODO 
	}
	pub fn enable_interrupt(self) {
		self.write_command(REG_IMASK,0x1F6DC) ; 
		self.write_command(REG_IMASK,0xff & ~4) ; 
		self.read_command(0xc0) ;
	}

	pub fn init(self) {
		self.bar_type  = 1; 
		//self.mem_base = ?? ; 
		self.io_base = 0xfebc0000 ; 
		self.eeprom_exists = false ; 
	}
	pub fn start(self) -> bool {
		self.detect_eeprom() ; 
		if self.read_mac_addr() == false {
			return false ; 
		}
		println!("{:?}", self.mac) ;
		
		self.write_command(0x5200+i*4, 0) ; 
		self.enable_interrupt() ; 
		self.rxinit() ; 
		self.txinit() ; 
		println!("Ethernet card started!");
		return true ; 


	}
}