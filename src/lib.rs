//! ESI file schema.
//!
//! Generated from EtherCATBase.xsd and EtherCATInfo.xsd version 1.19 with david-boles/xsd-parser-rs xsd-parser-cli 693aa0ebc6ad582fee790be19e11b1d8ebae4671
//!
//! Then hand-modified. SlotType and ProfileType are currently not supported due to code generator issues.

use std::str::FromStr;

use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_parser::generator::validator::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, File};

    use crate::EtherCATInfo;

    #[test]
    fn deserializes_device_esi_files() {
        for esi in read_dir("devices").expect("device ESI files not found") {
            let path = esi.unwrap().path();
            println!("Deserializing {}", path.display());
            let _: EtherCATInfo = yaserde::de::from_reader(File::open(path).unwrap()).unwrap();
        }
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct AccessType {
    #[yaserde(attribute, rename = "ReadRestrictions")]
    pub read_restrictions: Option<String>,

    #[yaserde(attribute, rename = "WriteRestrictions")]
    pub write_restrictions: Option<String>,
}

impl Validate for AccessType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ArrayInfoType {
    #[yaserde(rename = "LBound")]
    pub l_bound: xs::Integer,

    #[yaserde(rename = "Elements")]
    pub elements: xs::Integer,
}

impl Validate for ArrayInfoType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct DataTypeType {
    #[yaserde(rename = "Index")]
    pub index: Option<HexDecValue>,

    #[yaserde(rename = "Name")]
    pub name: String,

    #[yaserde(rename = "BaseType")]
    pub base_type: Option<String>,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "BitSize")]
    pub bit_size: i32,

    #[yaserde(rename = "DataTypeTypeChoice")]
    pub data_type_type_choice: data_type_type::DataTypeTypeChoice,

    #[yaserde(rename = "Properties")]
    pub properties: Option<data_type_type::PropertiesType>,

    #[yaserde(rename = "Xml")]
    pub xml: Option<data_type_type::XmlType>,
}

impl Validate for DataTypeType {}

pub mod data_type_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum DataTypeTypeChoice {
        ArrayInfo(Vec<ArrayInfoType>),
        SubItem(Vec<SubItemType>),
        EnumInfo(Vec<EnumInfoType>),
        __Unknown__(String),
    }

    impl Default for DataTypeTypeChoice {
        fn default() -> DataTypeTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for DataTypeTypeChoice {}

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct PropertiesType {
        #[yaserde(rename = "Property")]
        pub property: Vec<PropertyType>,
    }

    impl Validate for PropertiesType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct XmlType {}

    impl Validate for XmlType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct PropertyType {
    #[yaserde(rename = "Name")]
    pub name: String,

    #[yaserde(rename = "Value")]
    pub value: Option<String>,

    #[yaserde(rename = "Desc")]
    pub desc: Option<NameType>,
}

impl Validate for PropertyType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct SubItemType {
    #[yaserde(rename = "SubIdx")]
    pub sub_idx: Option<HexDecValue>,

    #[yaserde(rename = "Name")]
    pub name: String,

    #[yaserde(rename = "DisplayName")]
    pub display_name: Vec<NameType>,

    #[yaserde(rename = "Type")]
    pub _type: String,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "BitSize")]
    pub bit_size: i32,

    #[yaserde(rename = "BitOffs")]
    pub bit_offs: i32,

    #[yaserde(rename = "SubItemTypeChoice")]
    pub sub_item_type_choice: sub_item_type::SubItemTypeChoice,

    #[yaserde(rename = "Flags")]
    pub flags: Option<sub_item_type::FlagsType>,

    #[yaserde(rename = "Property")]
    pub property: Vec<PropertyType>,

    // obsolete
    #[yaserde(rename = "Xml")]
    pub xml: Option<sub_item_type::XmlType>,
}

impl Validate for SubItemType {}

pub mod sub_item_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum SubItemTypeChoice {
        // obsolete
        DefaultString(Option<String>),
        // obsolete
        DefaultData(Option<String>),
        __Unknown__(String),
    }

    impl Default for SubItemTypeChoice {
        fn default() -> SubItemTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for SubItemTypeChoice {}

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct FlagsType {
        #[yaserde(rename = "Access")]
        pub access: Option<flags_type::AccessType>,

        #[yaserde(rename = "Category")]
        pub category: Option<flags_type::CategoryType>,

        #[yaserde(rename = "PdoMapping")]
        pub pdo_mapping: Option<flags_type::PdoMappingType>,

        #[yaserde(rename = "SafetyMapping")]
        pub safety_mapping: Option<flags_type::SafetyMappingType>,

        #[yaserde(rename = "Attribute")]
        pub attribute: Option<HexDecValue>,

        #[yaserde(rename = "Backup")]
        pub backup: Option<i32>,

        #[yaserde(rename = "Setting")]
        pub setting: Option<i32>,
    }

    impl Validate for FlagsType {}

    pub mod flags_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct AccessType {
            #[yaserde(attribute, rename = "ReadRestrictions")]
            pub read_restrictions: Option<String>,

            #[yaserde(attribute, rename = "WriteRestrictions")]
            pub write_restrictions: Option<String>,
        }

        impl Validate for AccessType {}
        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum CategoryType {
            #[yaserde(rename = "m")]
            M,
            #[yaserde(rename = "o")]
            O,
            #[yaserde(rename = "c")]
            C,
            __Unknown__(String),
        }

        impl Default for CategoryType {
            fn default() -> CategoryType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for CategoryType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum PdoMappingType {
            T,
            R,
            #[yaserde(rename = "TR")]
            Tr,
            #[yaserde(rename = "RT")]
            Rt,
            #[yaserde(rename = "t")]
            TLower,
            #[yaserde(rename = "r")]
            RLower,
            #[yaserde(rename = "tr")]
            TrLower,
            #[yaserde(rename = "rt")]
            RtLower,
            __Unknown__(String),
        }

        impl Default for PdoMappingType {
            fn default() -> PdoMappingType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for PdoMappingType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum SafetyMappingType {
            #[yaserde(rename = "si")]
            SiLower,
            #[yaserde(rename = "SI")]
            Si,
            #[yaserde(rename = "so")]
            SoLower,
            #[yaserde(rename = "SO")]
            So,
            #[yaserde(rename = "sio")]
            SioLower,
            #[yaserde(rename = "SIO")]
            Sio,
            #[yaserde(rename = "sp")]
            SpLower,
            #[yaserde(rename = "SP")]
            Sp,
            __Unknown__(String),
        }

        impl Default for SafetyMappingType {
            fn default() -> SafetyMappingType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for SafetyMappingType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct XmlType {}

    impl Validate for XmlType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct EnumInfoType {
    #[yaserde(rename = "Text")]
    pub text: Vec<NameType>,

    #[yaserde(rename = "Enum")]
    pub _enum: HexDecValue,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,
}

impl Validate for EnumInfoType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct DiagnosticsType {
    #[yaserde(rename = "DiagReset")]
    pub diag_reset: Vec<EnumInfoType>,

    #[yaserde(rename = "DiagReaction")]
    pub diag_reaction: Vec<EnumInfoType>,

    #[yaserde(rename = "DiagType")]
    pub diag_type: Vec<EnumInfoType>,

    #[yaserde(rename = "DiagMessage")]
    pub diag_message: Vec<diagnostics_type::DiagMessageType>,
}

impl Validate for DiagnosticsType {}

pub mod diagnostics_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DiagMessageType {
        #[yaserde(rename = "TextId")]
        pub text_id: HexDecValue,

        #[yaserde(rename = "MessageText")]
        pub message_text: Vec<NameType>,

        #[yaserde(rename = "Description")]
        pub description: Vec<NameType>,

        #[yaserde(rename = "Flags")]
        pub flags: Option<diag_message_type::FlagsType>,

        #[yaserde(rename = "CauseRemedy")]
        pub cause_remedy: Vec<diag_message_type::CauseRemedyType>,

        #[yaserde(rename = "Info")]
        pub info: Vec<NameType>,

        #[yaserde(rename = "Hint")]
        pub hint: Vec<NameType>,

        #[yaserde(rename = "URL")]
        pub url: Vec<NameType>,
    }

    impl Validate for DiagMessageType {}

    pub mod diag_message_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct FlagsType {
            #[yaserde(rename = "DiagClass")]
            pub diag_class: Option<flags_type::DiagClassType>,

            #[yaserde(rename = "DiagReset")]
            pub diag_reset: Option<i32>,

            #[yaserde(rename = "DiagReaction")]
            pub diag_reaction: Option<i32>,

            #[yaserde(rename = "DiagType")]
            pub diag_type: Option<i32>,
        }

        impl Validate for FlagsType {}

        pub mod flags_type {
            use super::*;

            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]

            pub enum DiagClassType {
                #[yaserde(rename = "error")]
                Error,
                #[yaserde(rename = "warning")]
                Warning,
                #[yaserde(rename = "information")]
                Information,
                __Unknown__(String),
            }

            impl Default for DiagClassType {
                fn default() -> DiagClassType {
                    Self::__Unknown__("No valid variants".into())
                }
            }

            impl Validate for DiagClassType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct CauseRemedyType {
            #[yaserde(rename = "Cause")]
            pub cause: Vec<cause_remedy_type::CauseType>,

            #[yaserde(rename = "Remedy")]
            pub remedy: Vec<cause_remedy_type::RemedyType>,

            #[yaserde(attribute, rename = "Idx")]
            pub idx: String,
        }

        impl Validate for CauseRemedyType {}

        pub mod cause_remedy_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CauseType {
                #[yaserde(attribute, rename = "LcId")]
                pub lc_id: Option<xs::Integer>,
            }

            impl Validate for CauseType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct RemedyType {
                #[yaserde(attribute, rename = "LcId")]
                pub lc_id: Option<xs::Integer>,
            }

            impl Validate for RemedyType {}
        }
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct NameType {
    #[yaserde(attribute, rename = "LcId")]
    pub lc_id: Option<xs::Integer>,
}

impl Validate for NameType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct DictionaryType {
    #[yaserde(rename = "UnitTypes")]
    pub unit_types: Option<dictionary_type::UnitTypesType>,

    #[yaserde(rename = "DataTypes")]
    pub data_types: Option<dictionary_type::DataTypesType>,

    #[yaserde(rename = "Objects")]
    pub objects: dictionary_type::ObjectsType,
}

impl Validate for DictionaryType {}

pub mod dictionary_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct UnitTypesType {
        #[yaserde(rename = "UnitType")]
        pub unit_type: Vec<UnitTypeType>,
    }

    impl Validate for UnitTypesType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DataTypesType {
        #[yaserde(rename = "DataType")]
        pub data_type: Vec<DataTypeType>,
    }

    impl Validate for DataTypesType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ObjectsType {
        #[yaserde(rename = "Object")]
        pub object: Vec<ObjectType>,
    }

    impl Validate for ObjectsType {}
}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexDecValue(pub String);

impl Validate for HexDecValue {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ObjectType {
    #[yaserde(rename = "Index")]
    pub index: object_type::IndexType,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "Type")]
    pub _type: String,

    #[yaserde(rename = "BitSize")]
    pub bit_size: i32,

    #[yaserde(rename = "Info")]
    pub info: Option<ObjectInfoType>,

    #[yaserde(rename = "Flags")]
    pub flags: Option<object_type::FlagsType>,

    #[yaserde(rename = "Properties")]
    pub properties: Option<object_type::PropertiesType>,

    #[yaserde(rename = "Xml")]
    pub xml: Option<object_type::XmlType>,
}

impl Validate for ObjectType {}

pub mod object_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct IndexType {
        #[yaserde(attribute, rename = "DependOnSlot")]
        pub depend_on_slot: Option<bool>,

        #[yaserde(attribute, rename = "DependOnSlotGroup")]
        pub depend_on_slot_group: Option<bool>,

        #[yaserde(attribute, rename = "OverwrittenByModule")]
        pub overwritten_by_module: Option<bool>,
    }

    impl Validate for IndexType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct FlagsType {
        #[yaserde(rename = "Access")]
        pub access: Option<flags_type::AccessType>,

        #[yaserde(rename = "Category")]
        pub category: Option<flags_type::CategoryType>,

        #[yaserde(rename = "PdoMapping")]
        pub pdo_mapping: Option<flags_type::PdoMappingType>,

        #[yaserde(rename = "SafetyMapping")]
        pub safety_mapping: Option<flags_type::SafetyMappingType>,

        #[yaserde(rename = "Attribute")]
        pub attribute: Option<HexDecValue>,

        // obsolete
        #[yaserde(rename = "Transition")]
        pub transition: Option<flags_type::TransitionType>,

        #[yaserde(rename = "SdoAccess")]
        pub sdo_access: Option<flags_type::SdoAccessType>,

        #[yaserde(rename = "Backup")]
        pub backup: Option<i32>,

        #[yaserde(rename = "Setting")]
        pub setting: Option<i32>,
    }

    impl Validate for FlagsType {}

    pub mod flags_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct AccessType {
            #[yaserde(attribute, rename = "ReadRestrictions")]
            pub read_restrictions: Option<String>,

            #[yaserde(attribute, rename = "WriteRestrictions")]
            pub write_restrictions: Option<String>,
        }

        impl Validate for AccessType {}
        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum CategoryType {
            #[yaserde(rename = "m")]
            M,
            #[yaserde(rename = "o")]
            O,
            #[yaserde(rename = "c")]
            C,
            __Unknown__(String),
        }

        impl Default for CategoryType {
            fn default() -> CategoryType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for CategoryType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum PdoMappingType {
            T,
            R,
            #[yaserde(rename = "TR")]
            Tr,
            #[yaserde(rename = "RT")]
            Rt,
            #[yaserde(rename = "t")]
            TLower,
            #[yaserde(rename = "r")]
            RLower,
            #[yaserde(rename = "tr")]
            TrLower,
            #[yaserde(rename = "rt")]
            RtLower,
            __Unknown__(String),
        }

        impl Default for PdoMappingType {
            fn default() -> PdoMappingType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for PdoMappingType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum SafetyMappingType {
            #[yaserde(rename = "si")]
            SiLower,
            #[yaserde(rename = "SI")]
            Si,
            #[yaserde(rename = "so")]
            SoLower,
            #[yaserde(rename = "SO")]
            So,
            #[yaserde(rename = "sio")]
            SioLower,
            #[yaserde(rename = "SIO")]
            Sio,
            #[yaserde(rename = "sp")]
            SpLower,
            #[yaserde(rename = "SP")]
            Sp,
            __Unknown__(String),
        }

        impl Default for SafetyMappingType {
            fn default() -> SafetyMappingType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for SafetyMappingType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum TransitionType {
            #[yaserde(rename = "IP")]
            Ip,
            #[yaserde(rename = "PS")]
            Ps,
            #[yaserde(rename = "SO")]
            So,
            __Unknown__(String),
        }

        impl Default for TransitionType {
            fn default() -> TransitionType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for TransitionType {}

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum SdoAccessType {
            CompleteAccess,
            SubIndexAccess,
            __Unknown__(String),
        }

        impl Default for SdoAccessType {
            fn default() -> SdoAccessType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for SdoAccessType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct PropertiesType {
        #[yaserde(rename = "Property")]
        pub property: Vec<PropertyType>,
    }

    impl Validate for PropertiesType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct XmlType {}

    impl Validate for XmlType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ObjectInfoType {
    #[yaserde(rename = "ObjectInfoTypeChoice")]
    pub object_info_type_choice: object_info_type::ObjectInfoTypeChoice,

    #[yaserde(rename = "DisplayName")]
    pub display_name: Option<String>,

    #[yaserde(rename = "Unit")]
    pub unit: Option<HexDecValue>,
}

impl Validate for ObjectInfoType {}

pub mod object_info_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum ObjectInfoTypeChoice {
        DefaultString(Option<String>),
        SubItem,
        __Unknown__(String),
    }

    impl Default for ObjectInfoTypeChoice {
        fn default() -> ObjectInfoTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for ObjectInfoTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct EntryType {
    #[yaserde(rename = "Index")]
    pub index: entry_type::IndexType,

    #[yaserde(rename = "SubIndex")]
    pub sub_index: Option<HexDecValue>,

    #[yaserde(rename = "BitLen")]
    pub bit_len: i32,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Comment")]
    pub comment: Option<String>,

    #[yaserde(rename = "DataType")]
    pub data_type: Option<entry_type::DataTypeType>,
}

impl Validate for EntryType {}

pub mod entry_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct IndexType {
        #[yaserde(attribute, rename = "DependOnSlot")]
        pub depend_on_slot: Option<bool>,

        #[yaserde(attribute, rename = "DependOnSlotGroup")]
        pub depend_on_slot_group: Option<bool>,
    }

    impl Validate for IndexType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DataTypeType {
        #[yaserde(attribute, rename = "DScale")]
        pub d_scale: Option<String>,

        #[yaserde(attribute, rename = "SwapData")]
        pub swap_data: Option<String>,
    }

    impl Validate for DataTypeType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct PdoType {
    #[yaserde(rename = "Index")]
    pub index: pdo_type::IndexType,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Exclude")]
    pub exclude: Vec<pdo_type::ExcludeType>,

    #[yaserde(rename = "Entry")]
    pub entry: Vec<pdo_type::EntryType>,

    #[yaserde(rename = "ExcludedSm")]
    pub excluded_sm: Vec<i32>,

    #[yaserde(attribute, rename = "Fixed")]
    pub fixed: Option<bool>,

    #[yaserde(attribute, rename = "Mandatory")]
    pub mandatory: Option<bool>,

    #[yaserde(attribute, rename = "Virtual")]
    pub _virtual: Option<bool>,

    #[yaserde(attribute, rename = "Sm")]
    pub sm: Option<i32>,

    #[yaserde(attribute, rename = "Su")]
    pub su: Option<i32>,

    // obsolete
    #[yaserde(attribute, rename = "PdoOrder")]
    pub pdo_order: Option<i32>,

    #[yaserde(attribute, rename = "OSFac")]
    pub os_fac: Option<i32>,

    #[yaserde(attribute, rename = "OSMin")]
    pub os_min: Option<i32>,

    #[yaserde(attribute, rename = "OSMax")]
    pub os_max: Option<i32>,

    #[yaserde(attribute, rename = "OSIndexInc")]
    pub os_index_inc: Option<i32>,

    #[yaserde(attribute, rename = "OverwrittenByModule")]
    pub overwritten_by_module: Option<bool>,

    #[yaserde(attribute, rename = "SRA_Parameter")]
    pub sra_parameter: Option<bool>,

    #[yaserde(attribute, rename = "SafetyPdoType")]
    pub safety_pdo_type: Option<String>,

    #[yaserde(attribute, rename = "SafetyConnNumber")]
    pub safety_conn_number: Option<i32>,
}

impl Validate for PdoType {}

pub mod pdo_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct IndexType {
        #[yaserde(attribute, rename = "DependOnSlot")]
        pub depend_on_slot: Option<bool>,

        #[yaserde(attribute, rename = "DependOnSlotGroup")]
        pub depend_on_slot_group: Option<bool>,
    }

    impl Validate for IndexType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ExcludeType {
        #[yaserde(attribute, rename = "DependOnSlot")]
        pub depend_on_slot: Option<bool>,

        #[yaserde(attribute, rename = "DependOnSlotGroup")]
        pub depend_on_slot_group: Option<bool>,
    }

    impl Validate for ExcludeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct EntryType {
        #[yaserde(attribute, rename = "Fixed")]
        pub fixed: Option<bool>,

        #[yaserde(attribute, rename = "SafetyConnNumber")]
        pub safety_conn_number: Option<i32>,

        #[yaserde(attribute, rename = "SafetyPdoEntryType")]
        pub safety_pdo_entry_type: Option<String>,

        #[yaserde(rename = "Index")]
        pub index: entry_type::IndexType,

        #[yaserde(rename = "SubIndex")]
        pub sub_index: Option<HexDecValue>,

        #[yaserde(rename = "BitLen")]
        pub bit_len: i32,

        #[yaserde(rename = "Name")]
        pub name: Vec<NameType>,

        #[yaserde(rename = "Comment")]
        pub comment: Option<String>,

        #[yaserde(rename = "DataType")]
        pub data_type: Option<entry_type::DataTypeType>,
    }

    impl Validate for EntryType {}

    pub mod entry_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct IndexType {
            #[yaserde(attribute, rename = "DependOnSlot")]
            pub depend_on_slot: Option<bool>,

            #[yaserde(attribute, rename = "DependOnSlotGroup")]
            pub depend_on_slot_group: Option<bool>,
        }

        impl Validate for IndexType {}
        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct DataTypeType {
            #[yaserde(attribute, rename = "DScale")]
            pub d_scale: Option<String>,

            #[yaserde(attribute, rename = "SwapData")]
            pub swap_data: Option<String>,
        }

        impl Validate for DataTypeType {}
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct VendorSpecificType {}

impl Validate for VendorSpecificType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ProfileType {
    #[yaserde(rename = "ProfileNo")]
    pub profile_no: Option<i32>,

    #[yaserde(rename = "AddInfo")]
    pub add_info: Option<i32>,

    #[yaserde(rename = "SubAddInfo")]
    pub sub_add_info: Option<i32>,

    // #[yaserde(rename = "ProfileTypeChoice")]
    // pub profile_type_choice: profile_type::ProfileTypeChoice,

    // #[yaserde(rename = "ProfileTypeChoice")]
    // pub profile_type_choice: profile_type::ProfileTypeChoice,

    // #[yaserde(rename = "ProfileTypeChoice")]
    // pub profile_type_choice: profile_type::ProfileTypeChoice,
    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,
}

impl Validate for ProfileType {}

pub mod profile_type {
    // use super::*;

    // #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    // #[yaserde()]

    // pub enum ProfileTypeChoice {
    //     ChannelCount(i32),
    //     ChannelInfo,
    //     __Unknown__(String),
    // }

    // impl Default for ProfileTypeChoice {
    //     fn default() -> ProfileTypeChoice {
    //         Self::__Unknown__("No valid variants".into())
    //     }
    // }

    // impl Validate for ProfileTypeChoice {}

    // #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    // #[yaserde()]

    // pub enum ProfileTypeChoice {
    //     DictionaryFile(Option<String>),
    //     Dictionary(Option<DictionaryType>),
    //     __Unknown__(String),
    // }

    // impl Default for ProfileTypeChoice {
    //     fn default() -> ProfileTypeChoice {
    //         Self::__Unknown__("No valid variants".into())
    //     }
    // }

    // impl Validate for ProfileTypeChoice {}

    // #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    // #[yaserde()]

    // pub enum ProfileTypeChoice {
    //     DiagFile(Vec<String>),
    //     DiagMessages(Option<DiagnosticsType>),
    //     __Unknown__(String),
    // }

    // impl Default for ProfileTypeChoice {
    //     fn default() -> ProfileTypeChoice {
    //         Self::__Unknown__("No valid variants".into())
    //     }
    // }

    // impl Validate for ProfileTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct VendorType {
    #[yaserde(rename = "Id")]
    pub id: HexDecValue,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "URL")]
    pub url: Vec<NameType>,

    #[yaserde(rename = "DescriptionURL")]
    pub description_url: Option<String>,

    #[yaserde(rename = "VendorTypeChoice")]
    pub vendor_type_choice: vendor_type::VendorTypeChoice,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,

    // obsolete
    #[yaserde(attribute, rename = "UniqueName")]
    pub unique_name: Option<String>,
}

impl Validate for VendorType {}

pub mod vendor_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum VendorTypeChoice {
        // obsolete
        #[yaserde(rename = "Image16x14")]
        Image16X14(Option<String>),
        #[yaserde(rename = "ImageFile16x14")]
        ImageFile16X14(Option<String>),
        #[yaserde(rename = "ImageData16x14")]
        ImageData16X14(Option<String>),
        __Unknown__(String),
    }

    impl Default for VendorTypeChoice {
        fn default() -> VendorTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for VendorTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ModuleType {
    #[yaserde(rename = "Type")]
    pub _type: module_type::TypeType,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "RxPdo")]
    pub rx_pdo: Vec<PdoType>,

    #[yaserde(rename = "TxPdo")]
    pub tx_pdo: Vec<PdoType>,

    #[yaserde(rename = "SafetyParaMapping")]
    pub safety_para_mapping: Vec<PdoType>,

    #[yaserde(rename = "Mailbox")]
    pub mailbox: Option<module_type::MailboxType>,

    #[yaserde(rename = "Profile")]
    pub profile: Option<ProfileType>,

    #[yaserde(rename = "DcOpModeName")]
    pub dc_op_mode_name: Option<String>,

    #[yaserde(rename = "ModuleTypeChoice")]
    pub module_type_choice: module_type::ModuleTypeChoice,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,
}

impl Validate for ModuleType {}

pub mod module_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct TypeType {
        #[yaserde(attribute, rename = "ModuleIdent")]
        pub module_ident: HexDecValue,

        #[yaserde(attribute, rename = "ModuleClass")]
        pub module_class: Option<String>,

        #[yaserde(attribute, rename = "ModulePdoGroup")]
        pub module_pdo_group: Option<i32>,

        #[yaserde(attribute, rename = "SRA_ParameterSupported")]
        pub sra_parameter_supported: Option<bool>,
    }

    impl Validate for TypeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct MailboxType {
        #[yaserde(rename = "CoE")]
        pub co_e: Option<mailbox_type::CoEType>,
    }

    impl Validate for MailboxType {}

    pub mod mailbox_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct CoEType {
            #[yaserde(rename = "InitCmd")]
            pub init_cmd: Vec<co_e_type::InitCmdType>,

            #[yaserde(attribute, rename = "SdoInfo")]
            pub sdo_info: Option<bool>,

            #[yaserde(attribute, rename = "PdoAssign")]
            pub pdo_assign: Option<bool>,

            #[yaserde(attribute, rename = "PdoConfig")]
            pub pdo_config: Option<bool>,

            #[yaserde(attribute, rename = "PdoUpload")]
            pub pdo_upload: Option<bool>,

            #[yaserde(attribute, rename = "CompleteAccess")]
            pub complete_access: Option<bool>,

            #[yaserde(attribute, rename = "EdsFile")]
            pub eds_file: Option<String>,

            #[yaserde(attribute, rename = "SegmentedSdo")]
            pub segmented_sdo: Option<bool>,

            #[yaserde(attribute, rename = "ModuleOD")]
            pub module_od: Option<bool>,
        }

        impl Validate for CoEType {}

        pub mod co_e_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct InitCmdType {
                #[yaserde(rename = "Transition")]
                pub transition: Vec<init_cmd_type::TransitionType>,

                #[yaserde(rename = "Index")]
                pub index: init_cmd_type::IndexType,

                #[yaserde(rename = "SubIndex")]
                pub sub_index: HexDecValue,

                #[yaserde(rename = "Data")]
                pub data: init_cmd_type::DataType,

                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,

                #[yaserde(attribute, rename = "Fixed")]
                pub fixed: Option<bool>,

                #[yaserde(attribute, rename = "CompleteAccess")]
                pub complete_access: Option<bool>,
            }

            impl Validate for InitCmdType {}

            pub mod init_cmd_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum TransitionType {
                    #[yaserde(rename = "IP")]
                    Ip,
                    #[yaserde(rename = "PS")]
                    Ps,
                    #[yaserde(rename = "SO")]
                    So,
                    #[yaserde(rename = "SP")]
                    Sp,
                    #[yaserde(rename = "OP")]
                    Op,
                    #[yaserde(rename = "OS")]
                    Os,
                    __Unknown__(String),
                }

                impl Default for TransitionType {
                    fn default() -> TransitionType {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for TransitionType {}

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct IndexType {
                    #[yaserde(attribute, rename = "DependOnSlot")]
                    pub depend_on_slot: Option<bool>,

                    #[yaserde(attribute, rename = "DependOnSlotGroup")]
                    pub depend_on_slot_group: Option<bool>,
                }

                impl Validate for IndexType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct DataType {
                    #[yaserde(attribute, rename = "AdaptAutomatically")]
                    pub adapt_automatically: Option<bool>,
                }

                impl Validate for DataType {}
            }
        }
    }

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum ModuleTypeChoice {
        // obsolete
        #[yaserde(rename = "Image16x14")]
        Image16X14(Option<String>),
        #[yaserde(rename = "ImageFile16x14")]
        ImageFile16X14(Option<String>),
        #[yaserde(rename = "ImageData16x14")]
        ImageData16X14(Option<String>),
        __Unknown__(String),
    }

    impl Default for ModuleTypeChoice {
        fn default() -> ModuleTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for ModuleTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct UnitTypeType {
    #[yaserde(rename = "NotationIndex")]
    pub notation_index: HexDecValue,

    #[yaserde(rename = "Index")]
    pub index: Option<HexDecValue>,

    #[yaserde(rename = "Name")]
    pub name: String,

    #[yaserde(rename = "Symbol")]
    pub symbol: String,
}

impl Validate for UnitTypeType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum EtherCATPLoadType {
    SwitchingRegulator,
    Resistor,
    LowDropoutRegulator,
    __Unknown__(String),
}

impl Default for EtherCATPLoadType {
    fn default() -> EtherCATPLoadType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EtherCATPLoadType {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Guid(pub String);

impl Validate for Guid {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct EtherCATInfo {
    #[yaserde(rename = "InfoReference")]
    pub info_reference: Vec<String>,

    #[yaserde(rename = "Vendor")]
    pub vendor: ether_cat_info::VendorType,

    #[yaserde(rename = "Descriptions")]
    pub descriptions: ether_cat_info::DescriptionsType,

    #[yaserde(attribute, rename = "Version")]
    pub version: Option<String>,
}

impl Validate for EtherCATInfo {}

pub mod ether_cat_info {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct VendorType {
        #[yaserde(attribute, rename = "FileVersion")]
        pub file_version: Option<i32>,
    }

    impl Validate for VendorType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DescriptionsType {
        #[yaserde(rename = "Groups")]
        pub groups: descriptions_type::GroupsType,

        #[yaserde(rename = "Devices")]
        pub devices: descriptions_type::DevicesType,

        #[yaserde(rename = "Modules")]
        pub modules: Option<descriptions_type::ModulesType>,
    }

    impl Validate for DescriptionsType {}

    pub mod descriptions_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct GroupsType {
            #[yaserde(rename = "Group")]
            pub group: Vec<groups_type::GroupType>,
        }

        impl Validate for GroupsType {}

        pub mod groups_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct GroupType {
                #[yaserde(attribute, rename = "SortOrder")]
                pub sort_order: Option<i32>,

                #[yaserde(attribute, rename = "ParentGroup")]
                pub parent_group: Option<String>,

                #[yaserde(rename = "Type")]
                pub _type: String,

                #[yaserde(rename = "Name")]
                pub name: Vec<NameType>,

                #[yaserde(rename = "Comment")]
                pub comment: Vec<NameType>,

                #[yaserde(rename = "GroupTypeChoice")]
                pub group_type_choice: group_type::GroupTypeChoice,

                #[yaserde(rename = "VendorSpecific")]
                pub vendor_specific: Option<VendorSpecificType>,
            }

            impl Validate for GroupType {}

            pub mod group_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum GroupTypeChoice {
                    // obsolete
                    #[yaserde(rename = "Image16x14")]
                    Image16X14(Option<String>),
                    #[yaserde(rename = "ImageFile16x14")]
                    ImageFile16X14(Option<String>),
                    #[yaserde(rename = "ImageData16x14")]
                    ImageData16X14(Option<String>),
                    __Unknown__(String),
                }

                impl Default for GroupTypeChoice {
                    fn default() -> GroupTypeChoice {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for GroupTypeChoice {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct DevicesType {
            #[yaserde(rename = "Device")]
            pub device: Vec<devices_type::DeviceType>,
        }

        impl Validate for DevicesType {}

        pub mod devices_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct DeviceType {
                #[yaserde(attribute, rename = "Invisible")]
                pub invisible: Option<bool>,

                #[yaserde(attribute, rename = "Physics")]
                pub physics: PhysicsType,

                #[yaserde(attribute, rename = "Crc32")]
                pub crc_32: Option<HexDecValue>,

                // (SCI)
                #[yaserde(rename = "Sci")]
                pub sci: Option<device_type::SciType>,

                #[yaserde(rename = "Type")]
                pub _type: device_type::TypeType,

                #[yaserde(rename = "HideType")]
                pub hide_type: Vec<device_type::HideTypeType>,

                #[yaserde(rename = "AlternativeType")]
                pub alternative_type: Vec<device_type::AlternativeTypeType>,

                #[yaserde(rename = "SubDevice")]
                pub sub_device: Vec<device_type::SubDeviceType>,

                #[yaserde(rename = "Name")]
                pub name: Vec<NameType>,

                #[yaserde(rename = "Comment")]
                pub comment: Vec<NameType>,

                #[yaserde(rename = "URL")]
                pub url: Vec<NameType>,

                #[yaserde(rename = "Info")]
                pub info: Option<InfoType>,

                #[yaserde(rename = "GroupType")]
                pub group_type: String,

                #[yaserde(rename = "Profile")]
                pub profile: Vec<device_type::ProfileType>,

                #[yaserde(rename = "Fmmu")]
                pub fmmu: Vec<device_type::FmmuType>,

                #[yaserde(rename = "Sm")]
                pub sm: Vec<device_type::SmType>,

                #[yaserde(rename = "Su")]
                pub su: Vec<device_type::SuType>,

                #[yaserde(rename = "RxPdo")]
                pub rx_pdo: Vec<PdoType>,

                #[yaserde(rename = "TxPdo")]
                pub tx_pdo: Vec<PdoType>,

                #[yaserde(rename = "Mailbox")]
                pub mailbox: Option<device_type::MailboxType>,

                #[yaserde(rename = "Dc")]
                pub dc: Option<device_type::DcType>,

                #[yaserde(rename = "Slots")]
                pub slots: Option<device_type::SlotsType>,

                #[yaserde(rename = "ESC")]
                pub esc: Option<device_type::Esctype>,

                #[yaserde(rename = "Eeprom")]
                pub eeprom: Option<device_type::EepromType>,

                #[yaserde(rename = "DeviceTypeChoice")]
                pub device_type_choice: device_type::DeviceTypeChoice,

                #[yaserde(rename = "VendorSpecific")]
                pub vendor_specific: Option<VendorSpecificType>,
            }

            impl Validate for DeviceType {}

            pub mod device_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SciType {
                    #[yaserde(rename = "Name")]
                    pub name: Vec<NameType>,

                    #[yaserde(rename = "Description")]
                    pub description: Vec<NameType>,

                    #[yaserde(rename = "Guid")]
                    pub guid: Guid,

                    #[yaserde(rename = "CreatedBy")]
                    pub created_by: sci_type::CreatedByType,

                    #[yaserde(rename = "TargetSpecific")]
                    pub target_specific: Option<sci_type::TargetSpecificType>,

                    #[yaserde(rename = "VendorSpecific")]
                    pub vendor_specific: Option<VendorSpecificType>,

                    // Version of ETG.2000 on which this SCI is based on
                    #[yaserde(attribute, rename = "SciVersion")]
                    pub sci_version: Option<String>,
                }

                impl Validate for SciType {}

                pub mod sci_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct CreatedByType {
                        #[yaserde(rename = "Company")]
                        pub company: String,

                        #[yaserde(rename = "VendorId")]
                        pub vendor_id: Option<HexDecValue>,

                        #[yaserde(rename = "Tool")]
                        pub tool: Option<created_by_type::ToolType>,
                    }

                    impl Validate for CreatedByType {}

                    pub mod created_by_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ToolType {
                            #[yaserde(attribute, rename = "Version")]
                            pub version: Option<String>,

                            pub base: String,
                        }

                        impl Validate for ToolType {}
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct TargetSpecificType {
                        #[yaserde(rename = "AoeNetId")]
                        pub aoe_net_id: Option<target_specific_type::AoeNetIdType>,

                        #[yaserde(rename = "EoeMacIp")]
                        pub eoe_mac_ip: Option<target_specific_type::EoeMacIpType>,

                        #[yaserde(rename = "DcCycleTime")]
                        pub dc_cycle_time: Option<target_specific_type::DcCycleTimeType>,

                        #[yaserde(rename = "ModuleIdents")]
                        pub module_idents: Option<target_specific_type::ModuleIdentsType>,
                    }

                    impl Validate for TargetSpecificType {}

                    pub mod target_specific_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct AoeNetIdType {
                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for AoeNetIdType {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct EoeMacIpType {
                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for EoeMacIpType {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct DcCycleTimeType {
                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for DcCycleTimeType {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ModuleIdentsType {
                            #[yaserde(rename = "ModuleIdent")]
                            pub module_ident: Vec<module_idents_type::ModuleIdentType>,
                        }

                        impl Validate for ModuleIdentsType {}

                        pub mod module_idents_type {
                            use super::*;

                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct ModuleIdentType {
                                #[yaserde(rename = "SlotNo")]
                                pub slot_no: i32,

                                #[yaserde(rename = "Esi")]
                                pub esi: HexDecValue,

                                #[yaserde(rename = "Sci")]
                                pub sci: HexDecValue,
                            }

                            impl Validate for ModuleIdentType {}
                        }
                    }
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct TypeType {
                    #[yaserde(attribute, rename = "ProductCode")]
                    pub product_code: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "RevisionNo")]
                    pub revision_no: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "SerialNo")]
                    pub serial_no: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "CheckProductCode")]
                    pub check_product_code: Option<String>,

                    #[yaserde(attribute, rename = "CheckRevisionNo")]
                    pub check_revision_no: Option<String>,

                    #[yaserde(attribute, rename = "CheckSerialNo")]
                    pub check_serial_no: Option<String>,

                    // obsolete
                    #[yaserde(attribute, rename = "TcSmClass")]
                    pub tc_sm_class: Option<String>,

                    // obsolete
                    #[yaserde(attribute, rename = "TcCfgModeSafeOp")]
                    pub tc_cfg_mode_safe_op: Option<bool>,

                    #[yaserde(attribute, rename = "UseLrdLwr")]
                    pub use_lrd_lwr: Option<bool>,

                    #[yaserde(attribute, rename = "ModulePdoGroup")]
                    pub module_pdo_group: Option<i32>,

                    // obsolete
                    #[yaserde(attribute, rename = "DownloadModuleList")]
                    pub download_module_list: Option<bool>,

                    #[yaserde(attribute, rename = "ShowHideableSubDevices")]
                    pub show_hideable_sub_devices: Option<bool>,

                    pub base: String,
                }

                impl Validate for TypeType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct HideTypeType {
                    #[yaserde(attribute, rename = "ProductCode")]
                    pub product_code: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "RevisionNo")]
                    pub revision_no: Option<HexDecValue>,

                    // obsolete
                    #[yaserde(attribute, rename = "ProductRevision")]
                    pub product_revision: Option<String>,

                    pub base: String,
                }

                impl Validate for HideTypeType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct AlternativeTypeType {
                    // for future use
                    #[yaserde(attribute, rename = "ProductCode")]
                    pub product_code: Option<HexDecValue>,

                    // for future use
                    #[yaserde(attribute, rename = "RevisionNo")]
                    pub revision_no: Option<HexDecValue>,

                    pub base: String,
                }

                impl Validate for AlternativeTypeType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SubDeviceType {
                    // for future use
                    #[yaserde(attribute, rename = "ProductCode")]
                    pub product_code: Option<HexDecValue>,

                    // for future use
                    #[yaserde(attribute, rename = "RevisionNo")]
                    pub revision_no: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "PreviousDevice")]
                    pub previous_device: Option<i32>,

                    #[yaserde(attribute, rename = "PreviousPortNo")]
                    pub previous_port_no: Option<i32>,

                    #[yaserde(attribute, rename = "Hideable")]
                    pub hideable: Option<bool>,

                    pub base: String,
                }

                impl Validate for SubDeviceType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct ProfileType {
                    // obsolete
                    #[yaserde(attribute, rename = "Channel")]
                    pub channel: Option<i32>,

                    pub base: crate::ProfileType,
                }

                impl Validate for ProfileType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct FmmuType {
                    // obsolete
                    #[yaserde(attribute, rename = "OpOnly")]
                    pub op_only: Option<bool>,

                    #[yaserde(attribute, rename = "Sm")]
                    pub sm: Option<i32>,

                    #[yaserde(attribute, rename = "Su")]
                    pub su: Option<i32>,

                    pub base: String,
                }

                impl Validate for FmmuType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SmType {
                    #[yaserde(attribute, rename = "MinSize")]
                    pub min_size: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "MaxSize")]
                    pub max_size: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "DefaultSize")]
                    pub default_size: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "StartAddress")]
                    pub start_address: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "ControlByte")]
                    pub control_byte: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "Enable")]
                    pub enable: Option<HexDecValue>,

                    // obsolete
                    #[yaserde(attribute, rename = "OneByteMode")]
                    pub one_byte_mode: Option<bool>,

                    #[yaserde(attribute, rename = "Virtual")]
                    pub _virtual: Option<bool>,

                    // obsolete
                    #[yaserde(attribute, rename = "Watchdog")]
                    pub watchdog: Option<bool>,

                    #[yaserde(attribute, rename = "OpOnly")]
                    pub op_only: Option<bool>,

                    // obsolete
                    #[yaserde(attribute, rename = "FixedAssignment")]
                    pub fixed_assignment: Option<bool>,

                    pub base: String,
                }

                impl Validate for SmType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SuType {
                    #[yaserde(attribute, rename = "SeparateSu")]
                    pub separate_su: Option<bool>,

                    #[yaserde(attribute, rename = "SeparateFrame")]
                    pub separate_frame: Option<bool>,

                    // for future use
                    #[yaserde(attribute, rename = "DependOnInputState")]
                    pub depend_on_input_state: Option<bool>,

                    #[yaserde(attribute, rename = "FrameRepeatSupport")]
                    pub frame_repeat_support: Option<bool>,

                    pub base: String,
                }

                impl Validate for SuType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct MailboxType {
                    #[yaserde(rename = "AoE")]
                    pub ao_e: Option<mailbox_type::AoEType>,

                    #[yaserde(rename = "EoE")]
                    pub eo_e: Option<mailbox_type::EoEType>,

                    #[yaserde(rename = "CoE")]
                    pub co_e: Option<mailbox_type::CoEType>,

                    #[yaserde(rename = "FoE")]
                    pub fo_e: Option<mailbox_type::FoEType>,

                    #[yaserde(rename = "SoE")]
                    pub so_e: Option<mailbox_type::SoEType>,

                    #[yaserde(rename = "VoE")]
                    pub vo_e: Option<mailbox_type::VoEType>,

                    #[yaserde(rename = "VendorSpecific")]
                    pub vendor_specific: Option<VendorSpecificType>,

                    #[yaserde(attribute, rename = "DataLinkLayer")]
                    pub data_link_layer: Option<bool>,

                    // for future use
                    #[yaserde(attribute, rename = "RealTimeMode")]
                    pub real_time_mode: Option<bool>,
                }

                impl Validate for MailboxType {}

                pub mod mailbox_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct AoEType {
                        #[yaserde(rename = "InitCmd")]
                        pub init_cmd: Vec<ao_e_type::InitCmdType>,

                        #[yaserde(attribute, rename = "AdsRouter")]
                        pub ads_router: Option<bool>,

                        #[yaserde(attribute, rename = "GenerateOwnNetId")]
                        pub generate_own_net_id: Option<bool>,

                        #[yaserde(attribute, rename = "InitializeOwnNetId")]
                        pub initialize_own_net_id: Option<bool>,
                    }

                    impl Validate for AoEType {}

                    pub mod ao_e_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct InitCmdType {
                            #[yaserde(rename = "Transition")]
                            pub transition: Vec<init_cmd_type::TransitionType>,

                            #[yaserde(rename = "Data")]
                            pub data: String,

                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for InitCmdType {}

                        pub mod init_cmd_type {
                            use super::*;

                            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]

                            pub enum TransitionType {
                                #[yaserde(rename = "IP")]
                                Ip,
                                #[yaserde(rename = "PS")]
                                Ps,
                                #[yaserde(rename = "SO")]
                                So,
                                #[yaserde(rename = "SP")]
                                Sp,
                                #[yaserde(rename = "OP")]
                                Op,
                                #[yaserde(rename = "OS")]
                                Os,
                                __Unknown__(String),
                            }

                            impl Default for TransitionType {
                                fn default() -> TransitionType {
                                    Self::__Unknown__("No valid variants".into())
                                }
                            }

                            impl Validate for TransitionType {}
                        }
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct EoEType {
                        #[yaserde(rename = "InitCmd")]
                        pub init_cmd: Vec<eo_e_type::InitCmdType>,

                        #[yaserde(attribute, rename = "IP")]
                        pub ip: Option<bool>,

                        #[yaserde(attribute, rename = "MAC")]
                        pub mac: Option<bool>,

                        #[yaserde(attribute, rename = "TimeStamp")]
                        pub time_stamp: Option<bool>,
                    }

                    impl Validate for EoEType {}

                    pub mod eo_e_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct InitCmdType {
                            #[yaserde(rename = "Transition")]
                            pub transition: Vec<init_cmd_type::TransitionType>,

                            #[yaserde(rename = "Type")]
                            pub _type: i32,

                            #[yaserde(rename = "Data")]
                            pub data: String,

                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for InitCmdType {}

                        pub mod init_cmd_type {
                            use super::*;

                            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]

                            pub enum TransitionType {
                                #[yaserde(rename = "IP")]
                                Ip,
                                #[yaserde(rename = "PS")]
                                Ps,
                                #[yaserde(rename = "SO")]
                                So,
                                #[yaserde(rename = "SP")]
                                Sp,
                                #[yaserde(rename = "OP")]
                                Op,
                                #[yaserde(rename = "OS")]
                                Os,
                                __Unknown__(String),
                            }

                            impl Default for TransitionType {
                                fn default() -> TransitionType {
                                    Self::__Unknown__("No valid variants".into())
                                }
                            }

                            impl Validate for TransitionType {}
                        }
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct CoEType {
                        // obsolete
                        #[yaserde(rename = "Object")]
                        pub object: Vec<co_e_type::ObjectType>,

                        #[yaserde(rename = "InitCmd")]
                        pub init_cmd: Vec<co_e_type::InitCmdType>,

                        #[yaserde(attribute, rename = "SdoInfo")]
                        pub sdo_info: Option<bool>,

                        #[yaserde(attribute, rename = "PdoAssign")]
                        pub pdo_assign: Option<bool>,

                        #[yaserde(attribute, rename = "PdoConfig")]
                        pub pdo_config: Option<bool>,

                        #[yaserde(attribute, rename = "PdoUpload")]
                        pub pdo_upload: Option<bool>,

                        #[yaserde(attribute, rename = "CompleteAccess")]
                        pub complete_access: Option<bool>,

                        #[yaserde(attribute, rename = "EdsFile")]
                        pub eds_file: Option<String>,

                        // obsolete
                        #[yaserde(attribute, rename = "DS402Channels")]
                        pub ds402_channels: Option<i32>,

                        #[yaserde(attribute, rename = "SegmentedSdo")]
                        pub segmented_sdo: Option<bool>,

                        #[yaserde(attribute, rename = "DiagHistory")]
                        pub diag_history: Option<bool>,

                        #[yaserde(attribute, rename = "SdoUploadWithMaxLength")]
                        pub sdo_upload_with_max_length: Option<bool>,

                        #[yaserde(attribute, rename = "TimeDistribution")]
                        pub time_distribution: Option<bool>,
                    }

                    impl Validate for CoEType {}

                    pub mod co_e_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ObjectType {
                            // obsolete
                            #[yaserde(rename = "Index")]
                            pub index: i32,

                            // obsolete
                            #[yaserde(rename = "SubIndex")]
                            pub sub_index: i32,

                            // obsolete
                            #[yaserde(rename = "Data")]
                            pub data: String,

                            // obsolete
                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,
                        }

                        impl Validate for ObjectType {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct InitCmdType {
                            #[yaserde(rename = "Transition")]
                            pub transition: Vec<init_cmd_type::TransitionType>,

                            #[yaserde(rename = "Index")]
                            pub index: HexDecValue,

                            #[yaserde(rename = "SubIndex")]
                            pub sub_index: HexDecValue,

                            #[yaserde(rename = "Data")]
                            pub data: init_cmd_type::DataType,

                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,

                            // obsolete
                            #[yaserde(attribute, rename = "Fixed")]
                            pub fixed: Option<bool>,

                            #[yaserde(attribute, rename = "CompleteAccess")]
                            pub complete_access: Option<bool>,

                            #[yaserde(attribute, rename = "OverwrittenByModule")]
                            pub overwritten_by_module: Option<bool>,
                        }

                        impl Validate for InitCmdType {}

                        pub mod init_cmd_type {
                            use super::*;

                            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]

                            pub enum TransitionType {
                                #[yaserde(rename = "IP")]
                                Ip,
                                #[yaserde(rename = "PS")]
                                Ps,
                                #[yaserde(rename = "SO")]
                                So,
                                #[yaserde(rename = "SP")]
                                Sp,
                                #[yaserde(rename = "OP")]
                                Op,
                                #[yaserde(rename = "OS")]
                                Os,
                                __Unknown__(String),
                            }

                            impl Default for TransitionType {
                                fn default() -> TransitionType {
                                    Self::__Unknown__("No valid variants".into())
                                }
                            }

                            impl Validate for TransitionType {}

                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct DataType {
                                #[yaserde(attribute, rename = "AdaptAutomatically")]
                                pub adapt_automatically: Option<bool>,

                                pub base: String,
                            }

                            impl Validate for DataType {}
                        }
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct FoEType {}

                    impl Validate for FoEType {}
                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct SoEType {
                        #[yaserde(rename = "InitCmd")]
                        pub init_cmd: Vec<so_e_type::InitCmdType>,

                        #[yaserde(rename = "SoEChoice")]
                        pub so_e_choice: so_e_type::SoEChoice,

                        #[yaserde(attribute, rename = "ChannelCount")]
                        pub channel_count: Option<i32>,

                        #[yaserde(attribute, rename = "DriveFollowsBit3Support")]
                        pub drive_follows_bit_3_support: Option<bool>,
                    }

                    impl Validate for SoEType {}

                    pub mod so_e_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct InitCmdType {
                            #[yaserde(rename = "Transition")]
                            pub transition: Vec<init_cmd_type::TransitionType>,

                            #[yaserde(rename = "IDN")]
                            pub idn: i32,

                            #[yaserde(rename = "Data")]
                            pub data: String,

                            #[yaserde(rename = "Comment")]
                            pub comment: Option<String>,

                            #[yaserde(attribute, rename = "Chn")]
                            pub chn: Option<i32>,
                        }

                        impl Validate for InitCmdType {}

                        pub mod init_cmd_type {
                            use super::*;

                            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]

                            pub enum TransitionType {
                                #[yaserde(rename = "IP")]
                                Ip,
                                #[yaserde(rename = "PS")]
                                Ps,
                                #[yaserde(rename = "SO")]
                                So,
                                #[yaserde(rename = "SP")]
                                Sp,
                                #[yaserde(rename = "OP")]
                                Op,
                                #[yaserde(rename = "OS")]
                                Os,
                                __Unknown__(String),
                            }

                            impl Default for TransitionType {
                                fn default() -> TransitionType {
                                    Self::__Unknown__("No valid variants".into())
                                }
                            }

                            impl Validate for TransitionType {}
                        }

                        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]

                        pub enum SoEChoice {
                            DiagFile(Vec<String>),
                            DiagMessages(Option<DiagnosticsType>),
                            __Unknown__(String),
                        }

                        impl Default for SoEChoice {
                            fn default() -> SoEChoice {
                                Self::__Unknown__("No valid variants".into())
                            }
                        }

                        impl Validate for SoEChoice {}
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct VoEType {}

                    impl Validate for VoEType {}
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct DcType {
                    #[yaserde(rename = "OpMode")]
                    pub op_mode: Vec<dc_type::OpModeType>,

                    #[yaserde(rename = "VendorSpecific")]
                    pub vendor_specific: Option<VendorSpecificType>,

                    #[yaserde(attribute, rename = "UnknownFRMW")]
                    pub unknown_frmw: Option<bool>,

                    #[yaserde(attribute, rename = "Unknown64Bit")]
                    pub unknown_64_bit: Option<bool>,

                    #[yaserde(attribute, rename = "ExternalRefClock")]
                    pub external_ref_clock: Option<bool>,

                    #[yaserde(attribute, rename = "PotentialReferenceClock")]
                    pub potential_reference_clock: Option<bool>,

                    #[yaserde(attribute, rename = "TimeLoopControlOnly")]
                    pub time_loop_control_only: Option<bool>,
                }

                impl Validate for DcType {}

                pub mod dc_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct OpModeType {
                        #[yaserde(rename = "Name")]
                        pub name: String,

                        #[yaserde(rename = "Desc")]
                        pub desc: Option<String>,

                        #[yaserde(rename = "AssignActivate")]
                        pub assign_activate: HexDecValue,

                        #[yaserde(rename = "ActivateAdditional")]
                        pub activate_additional: Option<HexDecValue>,

                        #[yaserde(rename = "CycleTimeSync0")]
                        pub cycle_time_sync_0: Option<op_mode_type::CycleTimeSync0Type>,

                        #[yaserde(rename = "ShiftTimeSync0")]
                        pub shift_time_sync_0: Option<op_mode_type::ShiftTimeSync0Type>,

                        #[yaserde(rename = "CycleTimeSync1")]
                        pub cycle_time_sync_1: Option<op_mode_type::CycleTimeSync1Type>,

                        #[yaserde(rename = "ShiftTimeSync1")]
                        pub shift_time_sync_1: Option<op_mode_type::ShiftTimeSync1Type>,

                        #[yaserde(rename = "CycleTimeSync2")]
                        pub cycle_time_sync_2: Option<op_mode_type::CycleTimeSync2Type>,

                        #[yaserde(rename = "ShiftTimeSync2")]
                        pub shift_time_sync_2: Option<op_mode_type::ShiftTimeSync2Type>,

                        #[yaserde(rename = "CycleTimeSync3")]
                        pub cycle_time_sync_3: Option<op_mode_type::CycleTimeSync3Type>,

                        #[yaserde(rename = "ShiftTimeSync3")]
                        pub shift_time_sync_3: Option<op_mode_type::ShiftTimeSync3Type>,

                        #[yaserde(rename = "Sm")]
                        pub sm: Vec<op_mode_type::SmType>,

                        #[yaserde(rename = "VendorSpecific")]
                        pub vendor_specific: Option<VendorSpecificType>,
                    }

                    impl Validate for OpModeType {}

                    pub mod op_mode_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct CycleTimeSync0Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for CycleTimeSync0Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ShiftTimeSync0Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            #[yaserde(attribute, rename = "Input")]
                            pub input: Option<bool>,

                            #[yaserde(attribute, rename = "OutputDelayTime")]
                            pub output_delay_time: Option<i32>,

                            #[yaserde(attribute, rename = "InputDelayTime")]
                            pub input_delay_time: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for ShiftTimeSync0Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct CycleTimeSync1Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for CycleTimeSync1Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ShiftTimeSync1Type {
                            // for future use
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            #[yaserde(attribute, rename = "Input")]
                            pub input: Option<bool>,

                            #[yaserde(attribute, rename = "OutputDelayTime")]
                            pub output_delay_time: Option<i32>,

                            #[yaserde(attribute, rename = "InputDelayTime")]
                            pub input_delay_time: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for ShiftTimeSync1Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct CycleTimeSync2Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for CycleTimeSync2Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ShiftTimeSync2Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            #[yaserde(attribute, rename = "Input")]
                            pub input: Option<bool>,

                            #[yaserde(attribute, rename = "OutputDelayTime")]
                            pub output_delay_time: Option<i32>,

                            #[yaserde(attribute, rename = "InputDelayTime")]
                            pub input_delay_time: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for ShiftTimeSync2Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct CycleTimeSync3Type {
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for CycleTimeSync3Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ShiftTimeSync3Type {
                            // for future use
                            #[yaserde(attribute, rename = "Factor")]
                            pub factor: Option<i32>,

                            #[yaserde(attribute, rename = "Input")]
                            pub input: Option<bool>,

                            #[yaserde(attribute, rename = "OutputDelayTime")]
                            pub output_delay_time: Option<i32>,

                            #[yaserde(attribute, rename = "InputDelayTime")]
                            pub input_delay_time: Option<i32>,

                            pub base: i32,
                        }

                        impl Validate for ShiftTimeSync3Type {}
                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct SmType {
                            // obsolete
                            #[yaserde(rename = "SyncType")]
                            pub sync_type: Option<i32>,

                            // obsolete
                            #[yaserde(rename = "CycleTime")]
                            pub cycle_time: Option<sm_type::CycleTimeType>,

                            // obsolete
                            #[yaserde(rename = "ShiftTime")]
                            pub shift_time: Option<sm_type::ShiftTimeType>,

                            #[yaserde(rename = "Pdo")]
                            pub pdo: Vec<sm_type::PdoType>,

                            #[yaserde(attribute, rename = "No")]
                            pub no: i32,
                        }

                        impl Validate for SmType {}

                        pub mod sm_type {
                            use super::*;

                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct CycleTimeType {
                                // obsolete
                                #[yaserde(attribute, rename = "Factor")]
                                pub factor: Option<i32>,

                                pub base: i32,
                            }

                            impl Validate for CycleTimeType {}
                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct ShiftTimeType {
                                // obsolete
                                #[yaserde(attribute, rename = "MinAfterSync")]
                                pub min_after_sync: Option<i32>,

                                // obsolete
                                #[yaserde(attribute, rename = "MinBeforeFrame")]
                                pub min_before_frame: Option<i32>,

                                pub base: i32,
                            }

                            impl Validate for ShiftTimeType {}
                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct PdoType {
                                #[yaserde(attribute, rename = "OSFac")]
                                pub os_fac: Option<i32>,

                                pub base: HexDecValue,
                            }

                            impl Validate for PdoType {}
                        }
                    }
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SlotsType {
                    #[yaserde(rename = "Slot")]
                    pub slot: Vec<SlotType>,

                    #[yaserde(rename = "ModuleGroups")]
                    pub module_groups: Option<slots_type::ModuleGroupsType>,

                    #[yaserde(rename = "SlotGroupData")]
                    pub slot_group_data: Vec<slots_type::SlotGroupDataType>,

                    #[yaserde(rename = "ModulePdoGroup")]
                    pub module_pdo_group: Vec<slots_type::ModulePdoGroupType>,

                    #[yaserde(rename = "SlotSelections")]
                    pub slot_selections: Vec<slots_type::SlotSelectionsType>,

                    #[yaserde(attribute, rename = "DownloadModuleIdentList")]
                    pub download_module_ident_list: Option<bool>,

                    #[yaserde(attribute, rename = "DownloadModuleAddressList")]
                    pub download_module_address_list: Option<bool>,

                    #[yaserde(attribute, rename = "DownloadModuleListTransition")]
                    pub download_module_list_transition: Option<String>,

                    #[yaserde(attribute, rename = "MaxSlotCount")]
                    pub max_slot_count: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "MaxSlotGroupCount")]
                    pub max_slot_group_count: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "SlotPdoIncrement")]
                    pub slot_pdo_increment: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "SlotGroupPdoIncrement")]
                    pub slot_group_pdo_increment: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "SlotIndexIncrement")]
                    pub slot_index_increment: Option<HexDecValue>,

                    #[yaserde(attribute, rename = "SlotGroupIndexIncrement")]
                    pub slot_group_index_increment: Option<HexDecValue>,

                    // obsolete
                    #[yaserde(attribute, rename = "IdentifyModuleBy")]
                    pub identify_module_by: Option<String>,
                }

                impl Validate for SlotsType {}

                pub mod slots_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct ModuleGroupsType {
                        #[yaserde(rename = "ModuleGroup")]
                        pub module_group: Vec<module_groups_type::ModuleGroupType>,
                    }

                    impl Validate for ModuleGroupsType {}

                    pub mod module_groups_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ModuleGroupType {
                            #[yaserde(rename = "Name")]
                            pub name: NameType,

                            #[yaserde(rename = "Type")]
                            pub _type: String,

                            #[yaserde(rename = "Class")]
                            pub class: Option<String>,

                            #[yaserde(rename = "ModuleIdent")]
                            pub module_ident: Vec<HexDecValue>,

                            #[yaserde(rename = "ModuleGroupChoice")]
                            pub module_group_choice: module_group_type::ModuleGroupChoice,

                            #[yaserde(attribute, rename = "Id")]
                            pub id: String,
                        }

                        impl Validate for ModuleGroupType {}

                        pub mod module_group_type {
                            use super::*;

                            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]

                            pub enum ModuleGroupChoice {
                                #[yaserde(rename = "ImageFile16x14")]
                                ImageFile16X14(Option<String>),
                                #[yaserde(rename = "ImageData16x14")]
                                ImageData16X14(Option<String>),
                                __Unknown__(String),
                            }

                            impl Default for ModuleGroupChoice {
                                fn default() -> ModuleGroupChoice {
                                    Self::__Unknown__("No valid variants".into())
                                }
                            }

                            impl Validate for ModuleGroupChoice {}
                        }
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct SlotGroupDataType {
                        #[yaserde(rename = "Name")]
                        pub name: Vec<NameType>,

                        #[yaserde(rename = "ModuleGroups")]
                        pub module_groups: Option<slot_group_data_type::ModuleGroupsType>,

                        #[yaserde(rename = "SlotGroupDataChoice")]
                        pub slot_group_data_choice: slot_group_data_type::SlotGroupDataChoice,

                        #[yaserde(attribute, rename = "SlotGroup")]
                        pub slot_group: HexDecValue,
                    }

                    impl Validate for SlotGroupDataType {}

                    pub mod slot_group_data_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ModuleGroupsType {
                            #[yaserde(rename = "Id")]
                            pub id: Vec<String>,
                        }

                        impl Validate for ModuleGroupsType {}
                        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]

                        pub enum SlotGroupDataChoice {
                            #[yaserde(rename = "ImageFile16x14")]
                            ImageFile16X14(Option<String>),
                            #[yaserde(rename = "ImageData16x14")]
                            ImageData16X14(Option<String>),
                            __Unknown__(String),
                        }

                        impl Default for SlotGroupDataChoice {
                            fn default() -> SlotGroupDataChoice {
                                Self::__Unknown__("No valid variants".into())
                            }
                        }

                        impl Validate for SlotGroupDataChoice {}
                    }

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct ModulePdoGroupType {
                        #[yaserde(attribute, rename = "Alignment")]
                        pub alignment: Option<i32>,

                        #[yaserde(attribute, rename = "RxPdo")]
                        pub rx_pdo: Option<HexDecValue>,

                        #[yaserde(attribute, rename = "TxPdo")]
                        pub tx_pdo: Option<HexDecValue>,

                        pub base: String,
                    }

                    impl Validate for ModulePdoGroupType {}
                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct SlotSelectionsType {
                        #[yaserde(rename = "Name")]
                        pub name: NameType,

                        #[yaserde(rename = "ModuleIdent")]
                        pub module_ident: Vec<HexDecValue>,
                    }

                    impl Validate for SlotSelectionsType {}
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct Esctype {
                    #[yaserde(rename = "Reg0108")]
                    pub reg_0108: Option<HexDecValue>,

                    #[yaserde(rename = "Reg0400")]
                    pub reg_0400: Option<HexDecValue>,

                    #[yaserde(rename = "Reg0410")]
                    pub reg_0410: Option<HexDecValue>,

                    #[yaserde(rename = "Reg0420")]
                    pub reg_0420: Option<HexDecValue>,

                    #[yaserde(rename = "VendorSpecific")]
                    pub vendor_specific: Option<VendorSpecificType>,
                }

                impl Validate for Esctype {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct EepromType {
                    #[yaserde(attribute, rename = "AssignToPdi")]
                    pub assign_to_pdi: Option<bool>,

                    pub base: crate::EepromType,
                }

                impl Validate for EepromType {}
                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum DeviceTypeChoice {
                    // obsolete
                    #[yaserde(rename = "Image16x14")]
                    Image16X14(Option<String>),
                    #[yaserde(rename = "ImageFile16x14")]
                    ImageFile16X14(Option<String>),
                    #[yaserde(rename = "ImageData16x14")]
                    ImageData16X14(Option<String>),
                    __Unknown__(String),
                }

                impl Default for DeviceTypeChoice {
                    fn default() -> DeviceTypeChoice {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for DeviceTypeChoice {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ModulesType {
            #[yaserde(rename = "Module")]
            pub module: Vec<modules_type::ModuleType>,
        }

        impl Validate for ModulesType {}

        pub mod modules_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ModuleType {
                #[yaserde(attribute, rename = "Crc32")]
                pub crc_32: Option<HexDecValue>,
            }

            impl Validate for ModuleType {}
        }
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct DeviceType {
    // (SCI)
    #[yaserde(rename = "Sci")]
    pub sci: Option<device_type::SciType>,

    #[yaserde(rename = "Type")]
    pub _type: device_type::TypeType,

    #[yaserde(rename = "HideType")]
    pub hide_type: Vec<device_type::HideTypeType>,

    #[yaserde(rename = "AlternativeType")]
    pub alternative_type: Vec<device_type::AlternativeTypeType>,

    #[yaserde(rename = "SubDevice")]
    pub sub_device: Vec<device_type::SubDeviceType>,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "URL")]
    pub url: Vec<NameType>,

    #[yaserde(rename = "Info")]
    pub info: Option<InfoType>,

    #[yaserde(rename = "GroupType")]
    pub group_type: String,

    #[yaserde(rename = "Profile")]
    pub profile: Vec<device_type::ProfileType>,

    #[yaserde(rename = "Fmmu")]
    pub fmmu: Vec<device_type::FmmuType>,

    #[yaserde(rename = "Sm")]
    pub sm: Vec<device_type::SmType>,

    #[yaserde(rename = "Su")]
    pub su: Vec<device_type::SuType>,

    #[yaserde(rename = "RxPdo")]
    pub rx_pdo: Vec<PdoType>,

    #[yaserde(rename = "TxPdo")]
    pub tx_pdo: Vec<PdoType>,

    #[yaserde(rename = "Mailbox")]
    pub mailbox: Option<device_type::MailboxType>,

    #[yaserde(rename = "Dc")]
    pub dc: Option<device_type::DcType>,

    #[yaserde(rename = "Slots")]
    pub slots: Option<device_type::SlotsType>,

    #[yaserde(rename = "ESC")]
    pub esc: Option<device_type::Esctype>,

    #[yaserde(rename = "Eeprom")]
    pub eeprom: Option<device_type::EepromType>,

    #[yaserde(rename = "DeviceTypeChoice")]
    pub device_type_choice: device_type::DeviceTypeChoice,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,
}

impl Validate for DeviceType {}

pub mod device_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SciType {
        #[yaserde(rename = "Name")]
        pub name: Vec<NameType>,

        #[yaserde(rename = "Description")]
        pub description: Vec<NameType>,

        #[yaserde(rename = "Guid")]
        pub guid: Guid,

        #[yaserde(rename = "CreatedBy")]
        pub created_by: sci_type::CreatedByType,

        #[yaserde(rename = "TargetSpecific")]
        pub target_specific: Option<sci_type::TargetSpecificType>,

        #[yaserde(rename = "VendorSpecific")]
        pub vendor_specific: Option<VendorSpecificType>,

        // Version of ETG.2000 on which this SCI is based on
        #[yaserde(attribute, rename = "SciVersion")]
        pub sci_version: Option<String>,
    }

    impl Validate for SciType {}

    pub mod sci_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct CreatedByType {
            #[yaserde(rename = "Company")]
            pub company: String,

            #[yaserde(rename = "VendorId")]
            pub vendor_id: Option<HexDecValue>,

            #[yaserde(rename = "Tool")]
            pub tool: Option<created_by_type::ToolType>,
        }

        impl Validate for CreatedByType {}

        pub mod created_by_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ToolType {
                #[yaserde(attribute, rename = "Version")]
                pub version: Option<String>,
            }

            impl Validate for ToolType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TargetSpecificType {
            #[yaserde(rename = "AoeNetId")]
            pub aoe_net_id: Option<target_specific_type::AoeNetIdType>,

            #[yaserde(rename = "EoeMacIp")]
            pub eoe_mac_ip: Option<target_specific_type::EoeMacIpType>,

            #[yaserde(rename = "DcCycleTime")]
            pub dc_cycle_time: Option<target_specific_type::DcCycleTimeType>,

            #[yaserde(rename = "ModuleIdents")]
            pub module_idents: Option<target_specific_type::ModuleIdentsType>,
        }

        impl Validate for TargetSpecificType {}

        pub mod target_specific_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct AoeNetIdType {
                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for AoeNetIdType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct EoeMacIpType {
                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for EoeMacIpType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct DcCycleTimeType {
                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for DcCycleTimeType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ModuleIdentsType {
                #[yaserde(rename = "ModuleIdent")]
                pub module_ident: Vec<module_idents_type::ModuleIdentType>,
            }

            impl Validate for ModuleIdentsType {}

            pub mod module_idents_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct ModuleIdentType {
                    #[yaserde(rename = "SlotNo")]
                    pub slot_no: i32,

                    #[yaserde(rename = "Esi")]
                    pub esi: HexDecValue,

                    #[yaserde(rename = "Sci")]
                    pub sci: HexDecValue,
                }

                impl Validate for ModuleIdentType {}
            }
        }
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct TypeType {
        #[yaserde(attribute, rename = "ProductCode")]
        pub product_code: Option<HexDecValue>,

        #[yaserde(attribute, rename = "RevisionNo")]
        pub revision_no: Option<HexDecValue>,

        #[yaserde(attribute, rename = "SerialNo")]
        pub serial_no: Option<HexDecValue>,

        #[yaserde(attribute, rename = "CheckProductCode")]
        pub check_product_code: Option<String>,

        #[yaserde(attribute, rename = "CheckRevisionNo")]
        pub check_revision_no: Option<String>,

        #[yaserde(attribute, rename = "CheckSerialNo")]
        pub check_serial_no: Option<String>,

        // obsolete
        #[yaserde(attribute, rename = "TcSmClass")]
        pub tc_sm_class: Option<String>,

        // obsolete
        #[yaserde(attribute, rename = "TcCfgModeSafeOp")]
        pub tc_cfg_mode_safe_op: Option<bool>,

        #[yaserde(attribute, rename = "UseLrdLwr")]
        pub use_lrd_lwr: Option<bool>,

        #[yaserde(attribute, rename = "ModulePdoGroup")]
        pub module_pdo_group: Option<i32>,

        // obsolete
        #[yaserde(attribute, rename = "DownloadModuleList")]
        pub download_module_list: Option<bool>,

        #[yaserde(attribute, rename = "ShowHideableSubDevices")]
        pub show_hideable_sub_devices: Option<bool>,
    }

    impl Validate for TypeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct HideTypeType {
        #[yaserde(attribute, rename = "ProductCode")]
        pub product_code: Option<HexDecValue>,

        #[yaserde(attribute, rename = "RevisionNo")]
        pub revision_no: Option<HexDecValue>,

        // obsolete
        #[yaserde(attribute, rename = "ProductRevision")]
        pub product_revision: Option<String>,
    }

    impl Validate for HideTypeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct AlternativeTypeType {
        // for future use
        #[yaserde(attribute, rename = "ProductCode")]
        pub product_code: Option<HexDecValue>,

        // for future use
        #[yaserde(attribute, rename = "RevisionNo")]
        pub revision_no: Option<HexDecValue>,
    }

    impl Validate for AlternativeTypeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SubDeviceType {
        // for future use
        #[yaserde(attribute, rename = "ProductCode")]
        pub product_code: Option<HexDecValue>,

        // for future use
        #[yaserde(attribute, rename = "RevisionNo")]
        pub revision_no: Option<HexDecValue>,

        #[yaserde(attribute, rename = "PreviousDevice")]
        pub previous_device: Option<i32>,

        #[yaserde(attribute, rename = "PreviousPortNo")]
        pub previous_port_no: Option<i32>,

        #[yaserde(attribute, rename = "Hideable")]
        pub hideable: Option<bool>,
    }

    impl Validate for SubDeviceType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ProfileType {
        // obsolete
        #[yaserde(attribute, rename = "Channel")]
        pub channel: Option<i32>,
    }

    impl Validate for ProfileType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct FmmuType {
        // obsolete
        #[yaserde(attribute, rename = "OpOnly")]
        pub op_only: Option<bool>,

        #[yaserde(attribute, rename = "Sm")]
        pub sm: Option<i32>,

        #[yaserde(attribute, rename = "Su")]
        pub su: Option<i32>,
    }

    impl Validate for FmmuType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SmType {
        #[yaserde(attribute, rename = "MinSize")]
        pub min_size: Option<HexDecValue>,

        #[yaserde(attribute, rename = "MaxSize")]
        pub max_size: Option<HexDecValue>,

        #[yaserde(attribute, rename = "DefaultSize")]
        pub default_size: Option<HexDecValue>,

        #[yaserde(attribute, rename = "StartAddress")]
        pub start_address: Option<HexDecValue>,

        #[yaserde(attribute, rename = "ControlByte")]
        pub control_byte: Option<HexDecValue>,

        #[yaserde(attribute, rename = "Enable")]
        pub enable: Option<HexDecValue>,

        // obsolete
        #[yaserde(attribute, rename = "OneByteMode")]
        pub one_byte_mode: Option<bool>,

        #[yaserde(attribute, rename = "Virtual")]
        pub _virtual: Option<bool>,

        // obsolete
        #[yaserde(attribute, rename = "Watchdog")]
        pub watchdog: Option<bool>,

        #[yaserde(attribute, rename = "OpOnly")]
        pub op_only: Option<bool>,

        // obsolete
        #[yaserde(attribute, rename = "FixedAssignment")]
        pub fixed_assignment: Option<bool>,
    }

    impl Validate for SmType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SuType {
        #[yaserde(attribute, rename = "SeparateSu")]
        pub separate_su: Option<bool>,

        #[yaserde(attribute, rename = "SeparateFrame")]
        pub separate_frame: Option<bool>,

        // for future use
        #[yaserde(attribute, rename = "DependOnInputState")]
        pub depend_on_input_state: Option<bool>,

        #[yaserde(attribute, rename = "FrameRepeatSupport")]
        pub frame_repeat_support: Option<bool>,
    }

    impl Validate for SuType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct MailboxType {
        #[yaserde(rename = "AoE")]
        pub ao_e: Option<mailbox_type::AoEType>,

        #[yaserde(rename = "EoE")]
        pub eo_e: Option<mailbox_type::EoEType>,

        #[yaserde(rename = "CoE")]
        pub co_e: Option<mailbox_type::CoEType>,

        #[yaserde(rename = "FoE")]
        pub fo_e: Option<mailbox_type::FoEType>,

        #[yaserde(rename = "SoE")]
        pub so_e: Option<mailbox_type::SoEType>,

        #[yaserde(rename = "VoE")]
        pub vo_e: Option<mailbox_type::VoEType>,

        #[yaserde(rename = "VendorSpecific")]
        pub vendor_specific: Option<VendorSpecificType>,

        #[yaserde(attribute, rename = "DataLinkLayer")]
        pub data_link_layer: Option<bool>,

        // for future use
        #[yaserde(attribute, rename = "RealTimeMode")]
        pub real_time_mode: Option<bool>,
    }

    impl Validate for MailboxType {}

    pub mod mailbox_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct AoEType {
            #[yaserde(rename = "InitCmd")]
            pub init_cmd: Vec<ao_e_type::InitCmdType>,

            #[yaserde(attribute, rename = "AdsRouter")]
            pub ads_router: Option<bool>,

            #[yaserde(attribute, rename = "GenerateOwnNetId")]
            pub generate_own_net_id: Option<bool>,

            #[yaserde(attribute, rename = "InitializeOwnNetId")]
            pub initialize_own_net_id: Option<bool>,
        }

        impl Validate for AoEType {}

        pub mod ao_e_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct InitCmdType {
                #[yaserde(rename = "Transition")]
                pub transition: Vec<init_cmd_type::TransitionType>,

                #[yaserde(rename = "Data")]
                pub data: String,

                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for InitCmdType {}

            pub mod init_cmd_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum TransitionType {
                    #[yaserde(rename = "IP")]
                    Ip,
                    #[yaserde(rename = "PS")]
                    Ps,
                    #[yaserde(rename = "SO")]
                    So,
                    #[yaserde(rename = "SP")]
                    Sp,
                    #[yaserde(rename = "OP")]
                    Op,
                    #[yaserde(rename = "OS")]
                    Os,
                    __Unknown__(String),
                }

                impl Default for TransitionType {
                    fn default() -> TransitionType {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for TransitionType {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct EoEType {
            #[yaserde(rename = "InitCmd")]
            pub init_cmd: Vec<eo_e_type::InitCmdType>,

            #[yaserde(attribute, rename = "IP")]
            pub ip: Option<bool>,

            #[yaserde(attribute, rename = "MAC")]
            pub mac: Option<bool>,

            #[yaserde(attribute, rename = "TimeStamp")]
            pub time_stamp: Option<bool>,
        }

        impl Validate for EoEType {}

        pub mod eo_e_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct InitCmdType {
                #[yaserde(rename = "Transition")]
                pub transition: Vec<init_cmd_type::TransitionType>,

                #[yaserde(rename = "Type")]
                pub _type: i32,

                #[yaserde(rename = "Data")]
                pub data: String,

                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for InitCmdType {}

            pub mod init_cmd_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum TransitionType {
                    #[yaserde(rename = "IP")]
                    Ip,
                    #[yaserde(rename = "PS")]
                    Ps,
                    #[yaserde(rename = "SO")]
                    So,
                    #[yaserde(rename = "SP")]
                    Sp,
                    #[yaserde(rename = "OP")]
                    Op,
                    #[yaserde(rename = "OS")]
                    Os,
                    __Unknown__(String),
                }

                impl Default for TransitionType {
                    fn default() -> TransitionType {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for TransitionType {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct CoEType {
            // obsolete
            #[yaserde(rename = "Object")]
            pub object: Vec<co_e_type::ObjectType>,

            #[yaserde(rename = "InitCmd")]
            pub init_cmd: Vec<co_e_type::InitCmdType>,

            #[yaserde(attribute, rename = "SdoInfo")]
            pub sdo_info: Option<bool>,

            #[yaserde(attribute, rename = "PdoAssign")]
            pub pdo_assign: Option<bool>,

            #[yaserde(attribute, rename = "PdoConfig")]
            pub pdo_config: Option<bool>,

            #[yaserde(attribute, rename = "PdoUpload")]
            pub pdo_upload: Option<bool>,

            #[yaserde(attribute, rename = "CompleteAccess")]
            pub complete_access: Option<bool>,

            #[yaserde(attribute, rename = "EdsFile")]
            pub eds_file: Option<String>,

            // obsolete
            #[yaserde(attribute, rename = "DS402Channels")]
            pub ds402_channels: Option<i32>,

            #[yaserde(attribute, rename = "SegmentedSdo")]
            pub segmented_sdo: Option<bool>,

            #[yaserde(attribute, rename = "DiagHistory")]
            pub diag_history: Option<bool>,

            #[yaserde(attribute, rename = "SdoUploadWithMaxLength")]
            pub sdo_upload_with_max_length: Option<bool>,

            #[yaserde(attribute, rename = "TimeDistribution")]
            pub time_distribution: Option<bool>,
        }

        impl Validate for CoEType {}

        pub mod co_e_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ObjectType {
                // obsolete
                #[yaserde(rename = "Index")]
                pub index: i32,

                // obsolete
                #[yaserde(rename = "SubIndex")]
                pub sub_index: i32,

                // obsolete
                #[yaserde(rename = "Data")]
                pub data: String,

                // obsolete
                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,
            }

            impl Validate for ObjectType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct InitCmdType {
                #[yaserde(rename = "Transition")]
                pub transition: Vec<init_cmd_type::TransitionType>,

                #[yaserde(rename = "Index")]
                pub index: HexDecValue,

                #[yaserde(rename = "SubIndex")]
                pub sub_index: HexDecValue,

                #[yaserde(rename = "Data")]
                pub data: init_cmd_type::DataType,

                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,

                // obsolete
                #[yaserde(attribute, rename = "Fixed")]
                pub fixed: Option<bool>,

                #[yaserde(attribute, rename = "CompleteAccess")]
                pub complete_access: Option<bool>,

                #[yaserde(attribute, rename = "OverwrittenByModule")]
                pub overwritten_by_module: Option<bool>,
            }

            impl Validate for InitCmdType {}

            pub mod init_cmd_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum TransitionType {
                    #[yaserde(rename = "IP")]
                    Ip,
                    #[yaserde(rename = "PS")]
                    Ps,
                    #[yaserde(rename = "SO")]
                    So,
                    #[yaserde(rename = "SP")]
                    Sp,
                    #[yaserde(rename = "OP")]
                    Op,
                    #[yaserde(rename = "OS")]
                    Os,
                    __Unknown__(String),
                }

                impl Default for TransitionType {
                    fn default() -> TransitionType {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for TransitionType {}

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct DataType {
                    #[yaserde(attribute, rename = "AdaptAutomatically")]
                    pub adapt_automatically: Option<bool>,
                }

                impl Validate for DataType {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct FoEType {}

        impl Validate for FoEType {}
        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SoEType {
            #[yaserde(rename = "InitCmd")]
            pub init_cmd: Vec<so_e_type::InitCmdType>,

            #[yaserde(rename = "SoEChoice")]
            pub so_e_choice: so_e_type::SoEChoice,

            #[yaserde(attribute, rename = "ChannelCount")]
            pub channel_count: Option<i32>,

            #[yaserde(attribute, rename = "DriveFollowsBit3Support")]
            pub drive_follows_bit_3_support: Option<bool>,
        }

        impl Validate for SoEType {}

        pub mod so_e_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct InitCmdType {
                #[yaserde(rename = "Transition")]
                pub transition: Vec<init_cmd_type::TransitionType>,

                #[yaserde(rename = "IDN")]
                pub idn: i32,

                #[yaserde(rename = "Data")]
                pub data: String,

                #[yaserde(rename = "Comment")]
                pub comment: Option<String>,

                #[yaserde(attribute, rename = "Chn")]
                pub chn: Option<i32>,
            }

            impl Validate for InitCmdType {}

            pub mod init_cmd_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum TransitionType {
                    #[yaserde(rename = "IP")]
                    Ip,
                    #[yaserde(rename = "PS")]
                    Ps,
                    #[yaserde(rename = "SO")]
                    So,
                    #[yaserde(rename = "SP")]
                    Sp,
                    #[yaserde(rename = "OP")]
                    Op,
                    #[yaserde(rename = "OS")]
                    Os,
                    __Unknown__(String),
                }

                impl Default for TransitionType {
                    fn default() -> TransitionType {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for TransitionType {}
            }

            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]

            pub enum SoEChoice {
                DiagFile(Vec<String>),
                DiagMessages(Option<DiagnosticsType>),
                __Unknown__(String),
            }

            impl Default for SoEChoice {
                fn default() -> SoEChoice {
                    Self::__Unknown__("No valid variants".into())
                }
            }

            impl Validate for SoEChoice {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct VoEType {}

        impl Validate for VoEType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DcType {
        #[yaserde(rename = "OpMode")]
        pub op_mode: Vec<dc_type::OpModeType>,

        #[yaserde(rename = "VendorSpecific")]
        pub vendor_specific: Option<VendorSpecificType>,

        #[yaserde(attribute, rename = "UnknownFRMW")]
        pub unknown_frmw: Option<bool>,

        #[yaserde(attribute, rename = "Unknown64Bit")]
        pub unknown_64_bit: Option<bool>,

        #[yaserde(attribute, rename = "ExternalRefClock")]
        pub external_ref_clock: Option<bool>,

        #[yaserde(attribute, rename = "PotentialReferenceClock")]
        pub potential_reference_clock: Option<bool>,

        #[yaserde(attribute, rename = "TimeLoopControlOnly")]
        pub time_loop_control_only: Option<bool>,
    }

    impl Validate for DcType {}

    pub mod dc_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct OpModeType {
            #[yaserde(rename = "Name")]
            pub name: String,

            #[yaserde(rename = "Desc")]
            pub desc: Option<String>,

            #[yaserde(rename = "AssignActivate")]
            pub assign_activate: HexDecValue,

            #[yaserde(rename = "ActivateAdditional")]
            pub activate_additional: Option<HexDecValue>,

            #[yaserde(rename = "CycleTimeSync0")]
            pub cycle_time_sync_0: Option<op_mode_type::CycleTimeSync0Type>,

            #[yaserde(rename = "ShiftTimeSync0")]
            pub shift_time_sync_0: Option<op_mode_type::ShiftTimeSync0Type>,

            #[yaserde(rename = "CycleTimeSync1")]
            pub cycle_time_sync_1: Option<op_mode_type::CycleTimeSync1Type>,

            #[yaserde(rename = "ShiftTimeSync1")]
            pub shift_time_sync_1: Option<op_mode_type::ShiftTimeSync1Type>,

            #[yaserde(rename = "CycleTimeSync2")]
            pub cycle_time_sync_2: Option<op_mode_type::CycleTimeSync2Type>,

            #[yaserde(rename = "ShiftTimeSync2")]
            pub shift_time_sync_2: Option<op_mode_type::ShiftTimeSync2Type>,

            #[yaserde(rename = "CycleTimeSync3")]
            pub cycle_time_sync_3: Option<op_mode_type::CycleTimeSync3Type>,

            #[yaserde(rename = "ShiftTimeSync3")]
            pub shift_time_sync_3: Option<op_mode_type::ShiftTimeSync3Type>,

            #[yaserde(rename = "Sm")]
            pub sm: Vec<op_mode_type::SmType>,

            #[yaserde(rename = "VendorSpecific")]
            pub vendor_specific: Option<VendorSpecificType>,
        }

        impl Validate for OpModeType {}

        pub mod op_mode_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CycleTimeSync0Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,
            }

            impl Validate for CycleTimeSync0Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ShiftTimeSync0Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,

                #[yaserde(attribute, rename = "Input")]
                pub input: Option<bool>,

                #[yaserde(attribute, rename = "OutputDelayTime")]
                pub output_delay_time: Option<i32>,

                #[yaserde(attribute, rename = "InputDelayTime")]
                pub input_delay_time: Option<i32>,
            }

            impl Validate for ShiftTimeSync0Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CycleTimeSync1Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,
            }

            impl Validate for CycleTimeSync1Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ShiftTimeSync1Type {
                // for future use
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,

                #[yaserde(attribute, rename = "Input")]
                pub input: Option<bool>,

                #[yaserde(attribute, rename = "OutputDelayTime")]
                pub output_delay_time: Option<i32>,

                #[yaserde(attribute, rename = "InputDelayTime")]
                pub input_delay_time: Option<i32>,
            }

            impl Validate for ShiftTimeSync1Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CycleTimeSync2Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,
            }

            impl Validate for CycleTimeSync2Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ShiftTimeSync2Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,

                #[yaserde(attribute, rename = "Input")]
                pub input: Option<bool>,

                #[yaserde(attribute, rename = "OutputDelayTime")]
                pub output_delay_time: Option<i32>,

                #[yaserde(attribute, rename = "InputDelayTime")]
                pub input_delay_time: Option<i32>,
            }

            impl Validate for ShiftTimeSync2Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CycleTimeSync3Type {
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,
            }

            impl Validate for CycleTimeSync3Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ShiftTimeSync3Type {
                // for future use
                #[yaserde(attribute, rename = "Factor")]
                pub factor: Option<i32>,

                #[yaserde(attribute, rename = "Input")]
                pub input: Option<bool>,

                #[yaserde(attribute, rename = "OutputDelayTime")]
                pub output_delay_time: Option<i32>,

                #[yaserde(attribute, rename = "InputDelayTime")]
                pub input_delay_time: Option<i32>,
            }

            impl Validate for ShiftTimeSync3Type {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SmType {
                // obsolete
                #[yaserde(rename = "SyncType")]
                pub sync_type: Option<i32>,

                // obsolete
                #[yaserde(rename = "CycleTime")]
                pub cycle_time: Option<sm_type::CycleTimeType>,

                // obsolete
                #[yaserde(rename = "ShiftTime")]
                pub shift_time: Option<sm_type::ShiftTimeType>,

                #[yaserde(rename = "Pdo")]
                pub pdo: Vec<sm_type::PdoType>,

                #[yaserde(attribute, rename = "No")]
                pub no: i32,
            }

            impl Validate for SmType {}

            pub mod sm_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct CycleTimeType {
                    // obsolete
                    #[yaserde(attribute, rename = "Factor")]
                    pub factor: Option<i32>,
                }

                impl Validate for CycleTimeType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct ShiftTimeType {
                    // obsolete
                    #[yaserde(attribute, rename = "MinAfterSync")]
                    pub min_after_sync: Option<i32>,

                    // obsolete
                    #[yaserde(attribute, rename = "MinBeforeFrame")]
                    pub min_before_frame: Option<i32>,
                }

                impl Validate for ShiftTimeType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct PdoType {
                    #[yaserde(attribute, rename = "OSFac")]
                    pub os_fac: Option<i32>,
                }

                impl Validate for PdoType {}
            }
        }
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SlotsType {
        #[yaserde(rename = "Slot")]
        pub slot: Vec<SlotType>,

        #[yaserde(rename = "ModuleGroups")]
        pub module_groups: Option<slots_type::ModuleGroupsType>,

        #[yaserde(rename = "SlotGroupData")]
        pub slot_group_data: Vec<slots_type::SlotGroupDataType>,

        #[yaserde(rename = "ModulePdoGroup")]
        pub module_pdo_group: Vec<slots_type::ModulePdoGroupType>,

        #[yaserde(rename = "SlotSelections")]
        pub slot_selections: Vec<slots_type::SlotSelectionsType>,

        #[yaserde(attribute, rename = "DownloadModuleIdentList")]
        pub download_module_ident_list: Option<bool>,

        #[yaserde(attribute, rename = "DownloadModuleAddressList")]
        pub download_module_address_list: Option<bool>,

        #[yaserde(attribute, rename = "DownloadModuleListTransition")]
        pub download_module_list_transition: Option<String>,

        #[yaserde(attribute, rename = "MaxSlotCount")]
        pub max_slot_count: Option<HexDecValue>,

        #[yaserde(attribute, rename = "MaxSlotGroupCount")]
        pub max_slot_group_count: Option<HexDecValue>,

        #[yaserde(attribute, rename = "SlotPdoIncrement")]
        pub slot_pdo_increment: Option<HexDecValue>,

        #[yaserde(attribute, rename = "SlotGroupPdoIncrement")]
        pub slot_group_pdo_increment: Option<HexDecValue>,

        #[yaserde(attribute, rename = "SlotIndexIncrement")]
        pub slot_index_increment: Option<HexDecValue>,

        #[yaserde(attribute, rename = "SlotGroupIndexIncrement")]
        pub slot_group_index_increment: Option<HexDecValue>,

        // obsolete
        #[yaserde(attribute, rename = "IdentifyModuleBy")]
        pub identify_module_by: Option<String>,
    }

    impl Validate for SlotsType {}

    pub mod slots_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ModuleGroupsType {
            #[yaserde(rename = "ModuleGroup")]
            pub module_group: Vec<module_groups_type::ModuleGroupType>,
        }

        impl Validate for ModuleGroupsType {}

        pub mod module_groups_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ModuleGroupType {
                #[yaserde(rename = "Name")]
                pub name: NameType,

                #[yaserde(rename = "Type")]
                pub _type: String,

                #[yaserde(rename = "Class")]
                pub class: Option<String>,

                #[yaserde(rename = "ModuleIdent")]
                pub module_ident: Vec<HexDecValue>,

                #[yaserde(rename = "ModuleGroupChoice")]
                pub module_group_choice: module_group_type::ModuleGroupChoice,

                #[yaserde(attribute, rename = "Id")]
                pub id: String,
            }

            impl Validate for ModuleGroupType {}

            pub mod module_group_type {
                use super::*;

                #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]

                pub enum ModuleGroupChoice {
                    #[yaserde(rename = "ImageFile16x14")]
                    ImageFile16X14(Option<String>),
                    #[yaserde(rename = "ImageData16x14")]
                    ImageData16X14(Option<String>),
                    __Unknown__(String),
                }

                impl Default for ModuleGroupChoice {
                    fn default() -> ModuleGroupChoice {
                        Self::__Unknown__("No valid variants".into())
                    }
                }

                impl Validate for ModuleGroupChoice {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SlotGroupDataType {
            #[yaserde(rename = "Name")]
            pub name: Vec<NameType>,

            #[yaserde(rename = "ModuleGroups")]
            pub module_groups: Option<slot_group_data_type::ModuleGroupsType>,

            #[yaserde(rename = "SlotGroupDataChoice")]
            pub slot_group_data_choice: slot_group_data_type::SlotGroupDataChoice,

            #[yaserde(attribute, rename = "SlotGroup")]
            pub slot_group: HexDecValue,
        }

        impl Validate for SlotGroupDataType {}

        pub mod slot_group_data_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ModuleGroupsType {
                #[yaserde(rename = "Id")]
                pub id: Vec<String>,
            }

            impl Validate for ModuleGroupsType {}
            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]

            pub enum SlotGroupDataChoice {
                #[yaserde(rename = "ImageFile16x14")]
                ImageFile16X14(Option<String>),
                #[yaserde(rename = "ImageData16x14")]
                ImageData16X14(Option<String>),
                __Unknown__(String),
            }

            impl Default for SlotGroupDataChoice {
                fn default() -> SlotGroupDataChoice {
                    Self::__Unknown__("No valid variants".into())
                }
            }

            impl Validate for SlotGroupDataChoice {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ModulePdoGroupType {
            #[yaserde(attribute, rename = "Alignment")]
            pub alignment: Option<i32>,

            #[yaserde(attribute, rename = "RxPdo")]
            pub rx_pdo: Option<HexDecValue>,

            #[yaserde(attribute, rename = "TxPdo")]
            pub tx_pdo: Option<HexDecValue>,
        }

        impl Validate for ModulePdoGroupType {}
        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SlotSelectionsType {
            #[yaserde(rename = "Name")]
            pub name: NameType,

            #[yaserde(rename = "ModuleIdent")]
            pub module_ident: Vec<HexDecValue>,
        }

        impl Validate for SlotSelectionsType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct Esctype {
        #[yaserde(rename = "Reg0108")]
        pub reg_0108: Option<HexDecValue>,

        #[yaserde(rename = "Reg0400")]
        pub reg_0400: Option<HexDecValue>,

        #[yaserde(rename = "Reg0410")]
        pub reg_0410: Option<HexDecValue>,

        #[yaserde(rename = "Reg0420")]
        pub reg_0420: Option<HexDecValue>,

        #[yaserde(rename = "VendorSpecific")]
        pub vendor_specific: Option<VendorSpecificType>,
    }

    impl Validate for Esctype {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct EepromType {
        #[yaserde(attribute, rename = "AssignToPdi")]
        pub assign_to_pdi: Option<bool>,

        #[yaserde(rename = "EepromTypeChoice")]
        pub eeprom_type_choice: eeprom_type::EepromTypeChoice,

        #[yaserde(rename = "VendorSpecific")]
        pub vendor_specific: Option<VendorSpecificType>,
    }

    impl Validate for EepromType {}

    pub mod eeprom_type {
        use super::*;

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum EepromTypeChoice {
            Data(String),
            __Unknown__(String),
        }

        impl Default for EepromTypeChoice {
            fn default() -> EepromTypeChoice {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for EepromTypeChoice {}
    }

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum DeviceTypeChoice {
        // obsolete
        #[yaserde(rename = "Image16x14")]
        Image16X14(Option<String>),
        #[yaserde(rename = "ImageFile16x14")]
        ImageFile16X14(Option<String>),
        #[yaserde(rename = "ImageData16x14")]
        ImageData16X14(Option<String>),
        __Unknown__(String),
    }

    impl Default for DeviceTypeChoice {
        fn default() -> DeviceTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for DeviceTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct GroupType {
    #[yaserde(rename = "Type")]
    pub _type: String,

    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    #[yaserde(rename = "Comment")]
    pub comment: Vec<NameType>,

    #[yaserde(rename = "GroupTypeChoice")]
    pub group_type_choice: group_type::GroupTypeChoice,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,
}

impl Validate for GroupType {}

pub mod group_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum GroupTypeChoice {
        // obsolete
        #[yaserde(rename = "Image16x14")]
        Image16X14(Option<String>),
        #[yaserde(rename = "ImageFile16x14")]
        ImageFile16X14(Option<String>),
        #[yaserde(rename = "ImageData16x14")]
        ImageData16X14(Option<String>),
        __Unknown__(String),
    }

    impl Default for GroupTypeChoice {
        fn default() -> GroupTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for GroupTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct EepromType {
    #[yaserde(rename = "EepromTypeChoice")]
    pub eeprom_type_choice: eeprom_type::EepromTypeChoice,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,
}

impl Validate for EepromType {}

pub mod eeprom_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum EepromTypeChoice {
        Data(String),
        __Unknown__(String),
    }

    impl Default for EepromTypeChoice {
        fn default() -> EepromTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for EepromTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct InfoType {
    #[yaserde(rename = "Electrical")]
    pub electrical: Option<info_type::ElectricalType>,

    #[yaserde(rename = "StateMachine")]
    pub state_machine: Option<info_type::StateMachineType>,

    #[yaserde(rename = "Mailbox")]
    pub mailbox: Option<info_type::MailboxType>,

    #[yaserde(rename = "EtherCATController")]
    pub ether_cat_controller: Option<info_type::EtherCATControllerType>,

    #[yaserde(rename = "Port")]
    pub port: Vec<info_type::PortType>,

    #[yaserde(rename = "ExecutionUnit")]
    pub execution_unit: Vec<info_type::ExecutionUnitType>,

    #[yaserde(rename = "VendorSpecific")]
    pub vendor_specific: Option<VendorSpecificType>,

    // obsolete
    #[yaserde(rename = "StationAliasSupported")]
    pub station_alias_supported: Option<info_type::StationAliasSupportedType>,

    #[yaserde(rename = "IdentificationAdo")]
    pub identification_ado: Option<HexDecValue>,

    #[yaserde(rename = "IdentificationReg134")]
    pub identification_reg_134: Option<bool>,

    // for future use
    #[yaserde(rename = "DeviceFeature")]
    pub device_feature: Vec<info_type::DeviceFeatureType>,
}

impl Validate for InfoType {}

pub mod info_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ElectricalType {
        // Set Value = 0, in case of only EtherCATp is needed
        #[yaserde(rename = "EBusCurrent")]
        pub e_bus_current: i32,

        #[yaserde(rename = "EtherCATp")]
        pub ether_ca_tp: Option<electrical_type::EtherCATpType>,
    }

    impl Validate for ElectricalType {}

    pub mod electrical_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct EtherCATpType {
            #[yaserde(rename = "Device")]
            pub device: ether_ca_tp_type::DeviceType,

            // Externe Power Einspeisung
            #[yaserde(rename = "PowerSupply")]
            pub power_supply: Option<ether_ca_tp_type::PowerSupplyType>,
        }

        impl Validate for EtherCATpType {}

        pub mod ether_ca_tp_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct DeviceType {
                #[yaserde(rename = "Us")]
                pub us: device_type::UsType,

                #[yaserde(rename = "Up")]
                pub up: Option<device_type::UpType>,
            }

            impl Validate for DeviceType {}

            pub mod device_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct UsType {
                    // If power comes from PowerSupply instead of Port 0
                    #[yaserde(rename = "PowerSupply")]
                    pub power_supply: Option<bool>,

                    // wenn nicht angegeben, wird die MinVoltage von 20,4 V angenommen
                    #[yaserde(rename = "MinVoltage")]
                    pub min_voltage: Option<us_type::MinVoltageType>,

                    // max. Stromverbrauch bei 24V
                    #[yaserde(rename = "Current")]
                    pub current: Option<us_type::CurrentType>,

                    // falls vorhanden wird Us nach aussen geführt
                    #[yaserde(rename = "External")]
                    pub external: Option<us_type::ExternalType>,
                }

                impl Validate for UsType {}

                pub mod us_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
                    pub struct MinVoltageType(pub f64);

                    impl Validate for MinVoltageType {}
                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct CurrentType {
                        // load characteristics
                        #[yaserde(attribute, rename = "Type")]
                        pub _type: Option<EtherCATPLoadType>,
                    }

                    impl Validate for CurrentType {}
                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct ExternalType {
                        #[yaserde(rename = "Channel")]
                        pub channel: Vec<external_type::ChannelType>,
                    }

                    impl Validate for ExternalType {}

                    pub mod external_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ChannelType {
                            #[yaserde(rename = "Name")]
                            pub name: Vec<NameType>,

                            // muss angegeben werden wenn die Spannung ungleich Us ist , e.g. 5V.
                            // Falls nichts angegeben wird, wird Us untranfsormiert ausgegeben.
                            #[yaserde(rename = "Voltage")]
                            pub voltage: Option<channel_type::VoltageType>,
                        }

                        impl Validate for ChannelType {}

                        pub mod channel_type {
                            use super::*;

                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct VoltageType {
                                // load characteristics (default_ switching regulator)
                                #[yaserde(attribute, rename = "Type")]
                                pub _type: Option<EtherCATPLoadType>,
                            }

                            impl Validate for VoltageType {}
                        }
                    }
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct UpType {
                    // If power comes from PowerSupply instead of Port 0
                    #[yaserde(rename = "PowerSupply")]
                    pub power_supply: Option<bool>,

                    // wenn nicht angegeben, wird die MinVoltage von 20,4 V angenommen
                    #[yaserde(rename = "MinVoltage")]
                    pub min_voltage: Option<f64>,

                    // max. Stromverbrauch bei 24V
                    #[yaserde(rename = "Current")]
                    pub current: Option<up_type::CurrentType>,

                    // falls vorhanden wird Up nach aussen geführt
                    #[yaserde(rename = "External")]
                    pub external: Option<up_type::ExternalType>,
                }

                impl Validate for UpType {}

                pub mod up_type {
                    use super::*;

                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct CurrentType {
                        // load characteristics
                        #[yaserde(attribute, rename = "Type")]
                        pub _type: Option<EtherCATPLoadType>,
                    }

                    impl Validate for CurrentType {}
                    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]
                    pub struct ExternalType {
                        #[yaserde(rename = "Channel")]
                        pub channel: Vec<external_type::ChannelType>,
                    }

                    impl Validate for ExternalType {}

                    pub mod external_type {
                        use super::*;

                        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                        #[yaserde()]
                        pub struct ChannelType {
                            #[yaserde(rename = "Name")]
                            pub name: Vec<NameType>,

                            // muss angegeben werden wenn die Spannung ungleich Up ist , e.g. 5V.
                            // Falls nichts angegeben wird, wird Us untranfsormiert ausgegeben.
                            #[yaserde(rename = "Voltage")]
                            pub voltage: Option<channel_type::VoltageType>,
                        }

                        impl Validate for ChannelType {}

                        pub mod channel_type {
                            use super::*;

                            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                            #[yaserde()]
                            pub struct VoltageType {
                                // load characteristics (default_ switching regulator)
                                #[yaserde(attribute, rename = "Type")]
                                pub _type: Option<EtherCATPLoadType>,
                            }

                            impl Validate for VoltageType {}
                        }
                    }
                }
            }

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct PowerSupplyType {
                #[yaserde(rename = "Us")]
                pub us: Option<power_supply_type::UsType>,

                #[yaserde(rename = "Up")]
                pub up: Option<power_supply_type::UpType>,
            }

            impl Validate for PowerSupplyType {}

            pub mod power_supply_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct UsType {
                    #[yaserde(rename = "MaxCurrent")]
                    pub max_current: f64,
                }

                impl Validate for UsType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct UpType {
                    #[yaserde(rename = "MaxCurrent")]
                    pub max_current: f64,
                }

                impl Validate for UpType {}
            }
        }
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct StateMachineType {
        #[yaserde(rename = "Timeout")]
        pub timeout: Option<state_machine_type::TimeoutType>,

        #[yaserde(rename = "Behavior")]
        pub behavior: Option<state_machine_type::BehaviorType>,
    }

    impl Validate for StateMachineType {}

    pub mod state_machine_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TimeoutType {
            #[yaserde(rename = "PreopTimeout")]
            pub preop_timeout: i32,

            #[yaserde(rename = "SafeopOpTimeout")]
            pub safeop_op_timeout: i32,

            #[yaserde(rename = "BackToInitTimeout")]
            pub back_to_init_timeout: i32,

            #[yaserde(rename = "BackToSafeopTimeout")]
            pub back_to_safeop_timeout: i32,
        }

        impl Validate for TimeoutType {}
        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct BehaviorType {
            #[yaserde(attribute, rename = "StartToInit")]
            pub start_to_init: Option<bool>,

            #[yaserde(attribute, rename = "StartToPreop")]
            pub start_to_preop: Option<bool>,

            #[yaserde(attribute, rename = "StartToSafeop")]
            pub start_to_safeop: Option<bool>,

            #[yaserde(attribute, rename = "StartToSafeopNoSync")]
            pub start_to_safeop_no_sync: Option<bool>,
        }

        impl Validate for BehaviorType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct MailboxType {
        #[yaserde(rename = "Timeout")]
        pub timeout: mailbox_type::TimeoutType,
    }

    impl Validate for MailboxType {}

    pub mod mailbox_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TimeoutType {
            #[yaserde(rename = "RequestTimeout")]
            pub request_timeout: i32,

            #[yaserde(rename = "ResponseTimeout")]
            pub response_timeout: i32,
        }

        impl Validate for TimeoutType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct EtherCATControllerType {
        #[yaserde(rename = "DpramSize")]
        pub dpram_size: Option<i32>,

        #[yaserde(rename = "SmCount")]
        pub sm_count: Option<i32>,

        #[yaserde(rename = "FmmuCount")]
        pub fmmu_count: Option<i32>,

        #[yaserde(rename = "DcSyncCount")]
        pub dc_sync_count: Option<i32>,

        #[yaserde(rename = "DcLatchCount")]
        pub dc_latch_count: Option<i32>,
    }

    impl Validate for EtherCATControllerType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct PortType {
        #[yaserde(rename = "Type")]
        pub _type: port_type::TypeType,

        #[yaserde(rename = "Connector")]
        pub connector: Vec<port_type::ConnectorType>,

        #[yaserde(rename = "Label")]
        pub label: Option<String>,

        // For future use
        #[yaserde(rename = "RxDelay")]
        pub rx_delay: Option<i32>,

        // For future use
        #[yaserde(rename = "TxDelay")]
        pub tx_delay: Option<i32>,

        #[yaserde(rename = "PhysicalPhyAddr")]
        pub physical_phy_addr: Option<i32>,

        #[yaserde(rename = "EtherCATp")]
        pub ether_ca_tp: Option<port_type::EtherCATpType>,
    }

    impl Validate for PortType {}

    pub mod port_type {
        use super::*;

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum TypeType {
            #[yaserde(rename = "MII")]
            Mii,
            #[yaserde(rename = "EBUS")]
            Ebus,
            #[yaserde(rename = "NONE")]
            None,
            __Unknown__(String),
        }

        impl Default for TypeType {
            fn default() -> TypeType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for TypeType {}

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ConnectorType {
            #[yaserde(attribute, rename = "VendorId")]
            pub vendor_id: Option<HexDecValue>,
        }

        impl Validate for ConnectorType {}
        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct EtherCATpType {
            #[yaserde(rename = "Us")]
            pub us: Option<ether_ca_tp_type::UsType>,

            #[yaserde(rename = "Up")]
            pub up: Option<ether_ca_tp_type::UpType>,
        }

        impl Validate for EtherCATpType {}

        pub mod ether_ca_tp_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct UsType {
                // If power comes from PowerSupply instead of Port 0
                #[yaserde(rename = "PowerSupply")]
                pub power_supply: Option<bool>,

                // Default 3A
                #[yaserde(rename = "MaxCurrent")]
                pub max_current: Option<f64>,

                #[yaserde(rename = "Resistance")]
                pub resistance: f64,
            }

            impl Validate for UsType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct UpType {
                // If power comes from PowerSupply instead of Port 0
                #[yaserde(rename = "PowerSupply")]
                pub power_supply: Option<bool>,

                // Default 3A
                #[yaserde(rename = "MaxCurrent")]
                pub max_current: Option<f64>,

                #[yaserde(rename = "Resistance")]
                pub resistance: f64,
            }

            impl Validate for UpType {}
        }
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ExecutionUnitType {
        #[yaserde(rename = "Type")]
        pub _type: execution_unit_type::TypeType,

        // For future use
        #[yaserde(rename = "RxDelay")]
        pub rx_delay: Option<i32>,

        // For future use
        #[yaserde(rename = "TxDelay")]
        pub tx_delay: Option<i32>,
    }

    impl Validate for ExecutionUnitType {}

    pub mod execution_unit_type {
        use super::*;

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]

        pub enum TypeType {
            #[yaserde(rename = "PRIMARY")]
            Primary,
            #[yaserde(rename = "SECONDARY")]
            Secondary,
            #[yaserde(rename = "NONE")]
            None,
            __Unknown__(String),
        }

        impl Default for TypeType {
            fn default() -> TypeType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for TypeType {}
    }

    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct StationAliasSupportedType(pub String);

    impl Validate for StationAliasSupportedType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct DeviceFeatureType {
        // for future use
        #[yaserde(rename = "Name")]
        pub name: String,

        // for future use
        #[yaserde(rename = "Value")]
        pub value: Option<String>,

        // for future use
        #[yaserde(rename = "Description")]
        pub description: Option<String>,

        // for future use
        #[yaserde(rename = "Register")]
        pub register: Vec<device_feature_type::RegisterType>,
    }

    impl Validate for DeviceFeatureType {}

    pub mod device_feature_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct RegisterType {
            // for future use;
            // in bytes
            #[yaserde(rename = "StartAddress")]
            pub start_address: i32,

            // for future use;
            // in bytes
            #[yaserde(rename = "Length")]
            pub length: i32,

            // for future use
            #[yaserde(rename = "BitMask")]
            pub bit_mask: Option<HexDecValue>,
        }

        impl Validate for RegisterType {}
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PhysicsType(pub String);

impl Validate for PhysicsType {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct SlotType {
    #[yaserde(rename = "Name")]
    pub name: Vec<NameType>,

    // #[yaserde(rename = "SlotTypeChoice")]
    // pub slot_type_choice: slot_type::SlotTypeChoice,

    // #[yaserde(rename = "SlotTypeChoice")]
    // pub slot_type_choice: slot_type::SlotTypeChoice,
    #[yaserde(attribute, rename = "SlotGroup")]
    pub slot_group: Option<HexDecValue>,

    #[yaserde(attribute, rename = "MinInstances")]
    pub min_instances: HexDecValue,

    #[yaserde(attribute, rename = "MaxInstances")]
    pub max_instances: HexDecValue,

    #[yaserde(attribute, rename = "SlotPdoIncrement")]
    pub slot_pdo_increment: Option<HexDecValue>,

    #[yaserde(attribute, rename = "SlotGroupPdoIncrement")]
    pub slot_group_pdo_increment: Option<HexDecValue>,

    #[yaserde(attribute, rename = "SlotIndexIncrement")]
    pub slot_index_increment: Option<HexDecValue>,

    #[yaserde(attribute, rename = "SlotGroupIndexIncrement")]
    pub slot_group_index_increment: Option<HexDecValue>,

    #[yaserde(attribute, rename = "TreeView")]
    pub tree_view: Option<String>,

    #[yaserde(attribute, rename = "Default")]
    pub default: Option<HexDecValue>,

    // automatically plugged when no other module is plugged to this slot
    #[yaserde(attribute, rename = "Fallback")]
    pub fallback: Option<HexDecValue>,
}

impl Validate for SlotType {}

pub mod slot_type {
    // use super::*;

    // #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    // #[yaserde()]

    // pub enum SlotTypeChoice {
    //     ModuleIdent,
    //     ModuleClass,
    //     __Unknown__(String),
    // }

    // impl Default for SlotTypeChoice {
    //     fn default() -> SlotTypeChoice {
    //         Self::__Unknown__("No valid variants".into())
    //     }
    // }

    // impl Validate for SlotTypeChoice {}

    // #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    // #[yaserde()]

    // pub enum SlotTypeChoice {
    //     // obsolete
    //     #[yaserde(rename = "Image16x14")]
    //     Image16X14(Option<String>),
    //     #[yaserde(rename = "ImageFile16x14")]
    //     ImageFile16X14(Option<String>),
    //     #[yaserde(rename = "ImageData16x14")]
    //     ImageData16X14(Option<String>),
    //     __Unknown__(String),
    // }

    // impl Default for SlotTypeChoice {
    //     fn default() -> SlotTypeChoice {
    //         Self::__Unknown__("No valid variants".into())
    //     }
    // }

    // impl Validate for SlotTypeChoice {}
}
