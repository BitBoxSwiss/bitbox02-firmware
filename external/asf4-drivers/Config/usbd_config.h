/* Auto-generated config file usbd_config.h */
#ifndef USBD_CONFIG_H
#define USBD_CONFIG_H

// <<< Use Configuration Wizard in Context Menu >>>

// ---- USB Device Stack Core Options ----

// <q> High Speed Support
// <i> Enable high speed specific descriptors support, e.g., DeviceQualifierDescriptor and OtherSpeedConfiguration Descriptor.
// <i> High speed support require descriptors description array on start, for LS/FS and HS support in first and second place.
// <id> usbd_hs_sp
#ifndef CONF_USBD_HS_SP
#define CONF_USBD_HS_SP 0// FIXME - changing to `1` didn't change speed reported by osx
#endif

// ---- USB Device Stack HID generic Options ----

// <e> Enable String Descriptors
// <id> usb_hid_generic_str_en
#ifndef CONF_USB_HID_GENERIC_STR_EN
#define CONF_USB_HID_GENERIC_STR_EN 0
#endif
// <s> Language IDs
// <i> Language IDs in c format, split by comma (E.g., 0x0409 ...)
// <id> usb_hid_generic_langid
#ifndef CONF_USB_HID_GENERIC_LANGID
#define CONF_USB_HID_GENERIC_LANGID "0x0409"
#endif

#ifndef CONF_USB_HID_GENERIC_LANGID_DESC
#define CONF_USB_HID_GENERIC_LANGID_DESC
#endif
// </e>

// <h> HID Generic Device Descriptor

// <o> bcdUSB
// <0x0200=> USB 2.0 version
// <0x0210=> USB 2.1 version
// <id> usb_hid_generic_bcdusb
#ifndef CONF_USB_HID_GENERIC_BCDUSB
#define CONF_USB_HID_GENERIC_BCDUSB 0x200
#endif

// <o> bMaxPackeSize0
// <0x0008=> 8 bytes
// <0x0010=> 16 bytes
// <0x0020=> 32 bytes
// <0x0040=> 64 bytes
// <id> usb_hid_generic_bmaxpksz0
#ifndef CONF_USB_HID_GENERIC_BMAXPKSZ0
#define CONF_USB_HID_GENERIC_BMAXPKSZ0 0x40
#endif

// <o> idVender <0x0000-0xFFFF>
// <id> usb_hid_generic_idvender
#ifndef CONF_USB_HID_GENERIC_IDVENDER
#define CONF_USB_HID_GENERIC_IDVENDER 0x3eb
#endif

// <o> idProduct <0x0000-0xFFFF>
// <id> usb_hid_generic_idproduct
#ifndef CONF_USB_HID_GENERIC_IDPRODUCT
#define CONF_USB_HID_GENERIC_IDPRODUCT 0x2402
#endif

// <o> bcdDevice <0x0000-0xFFFF>
// <id> usb_hid_generic_bcddevice
#ifndef CONF_USB_HID_GENERIC_BCDDEVICE
#define CONF_USB_HID_GENERIC_BCDDEVICE 0x100
#endif

// <e> Enable string descriptor of iManufact
// <id> usb_hid_generic_imanufact_en
#ifndef CONF_USB_HID_GENERIC_IMANUFACT_EN
#define CONF_USB_HID_GENERIC_IMANUFACT_EN 0
#endif

#ifndef CONF_USB_HID_GENERIC_IMANUFACT
#define CONF_USB_HID_GENERIC_IMANUFACT (CONF_USB_HID_GENERIC_IMANUFACT_EN * (CONF_USB_HID_GENERIC_IMANUFACT_EN))
#endif

// <s> Unicode string of iManufact
// <id> usb_hid_generic_imanufact_str
#ifndef CONF_USB_HID_GENERIC_IMANUFACT_STR
#define CONF_USB_HID_GENERIC_IMANUFACT_STR "Atmel"
#endif

#ifndef CONF_USB_HID_GENERIC_IMANUFACT_STR_DESC
#define CONF_USB_HID_GENERIC_IMANUFACT_STR_DESC
#endif

// </h>

// <h> HID Generic Configuration Descriptor

// <e> Enable string descriptor of iConfig
// <id> usb_hid_generic_iconfig_en
#ifndef CONF_USB_HID_GENERIC_ICONFIG_EN
#define CONF_USB_HID_GENERIC_ICONFIG_EN 0
#endif

#ifndef CONF_USB_HID_GENERIC_ICONFIG
#define CONF_USB_HID_GENERIC_ICONFIG                                                                                   \
	(CONF_USB_HID_GENERIC_ICONFIG_EN                                                                                   \
	 * (CONF_USB_HID_GENERIC_IMANUFACT_EN + CONF_USB_HID_GENERIC_IPRODUCT_EN + CONF_USB_HID_GENERIC_ISERIALNUM_EN      \
	    + CONF_USB_HID_GENERIC_ICONFIG_EN))
#endif

// <s> Unicode string of iConfig
// <id> usb_hid_generic_iconfig_str
#ifndef CONF_USB_HID_GENERIC_ICONFIG_STR
#define CONF_USB_HID_GENERIC_ICONFIG_STR ""
#endif

#ifndef CONF_USB_HID_GENERIC_ICONFIG_STR_DESC
#define CONF_USB_HID_GENERIC_ICONFIG_STR_DESC
#endif

//
// <<< end of configuration section >>>

#endif // USBD_CONFIG_H
