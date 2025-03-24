use elf::abi::*;

pub fn get_ei_class_meaning(val: u8) -> &'static str {
    match val {
        ELFCLASSNONE => "Invalid class",
        ELFCLASS32 => "32-bit architecture",
        ELFCLASS64 => "64-bit architecture",
        _ => ""
    }
}

pub fn get_ei_data_meaning(val: u8) -> &'static str {
    match val {
        ELFDATANONE => "Unknown data format",
        ELFDATA2LSB => "Two's complement little-endian",
        ELFDATA2MSB => "Two's complement big-endian",
        _ => ""
    }
}

pub fn get_ei_version_meaning(val: u8) -> &'static str {
    match val {
        EV_NONE => "Invalid version",
        EV_CURRENT => "Current version",
        _ => ""
    }
}

pub fn get_ei_osabi_meaning(val: u8) -> &'static str {
    match val {
        ELFOSABI_SYSV => "UNIX System V",
        ELFOSABI_HPUX => "HP-UX",
        ELFOSABI_NETBSD => "NetBSD",
        ELFOSABI_LINUX => "Linux",
        ELFOSABI_SOLARIS => "Solaris",
        ELFOSABI_AIX => "AIX",
        ELFOSABI_IRIX => "IRIX",
        ELFOSABI_FREEBSD => "FreeBSD",
        ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
        ELFOSABI_MODESTO => "Novell Modesto",
        ELFOSABI_OPENBSD => "Open BSD",
        ELFOSABI_OPENVMS => "Open VMS",
        ELFOSABI_NSK => "HP Non-Stop Kernel",
        ELFOSABI_AROS => "Amiga Research OS",
        ELFOSABI_FENIXOS => "FenixOS",
        ELFOSABI_CLOUDABI => "Nuxi CloudABI",
        ELFOSABI_OPENVOS => "Stratus Technologies OpenVOS",
        _ => ""
    }
}
