// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0
//
// This file is auto-generated. Do not modify.
// To regenerate the file, execute the following command:
// cargo run --manifest-path=dbgen/Cargo.toml | rustfmt > src/db.rs

//! A Database of all IANA-registered Media Types

#![allow(missing_docs, clippy::panic)]

use crate::Essence;

/// `application/1d-interleaved-parityfec`
pub const APPLICATION_1D_INTERLEAVED_PARITYFEC: Essence<&'static str> =
    match Essence::new_const("application/1d-interleaved-parityfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/1d-interleaved-parityfec"),
    };

/// `application/3gpdash-qoe-report+xml`
pub const APPLICATION_3GPDASH_QOE_REPORT_XML: Essence<&'static str> =
    match Essence::new_const("application/3gpdash-qoe-report+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/3gpdash-qoe-report+xml"),
    };

/// `application/3gppHal+json`
pub const APPLICATION_3GPPHAL_JSON: Essence<&'static str> =
    match Essence::new_const("application/3gppHal+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/3gppHal+json"),
    };

/// `application/3gppHalForms+json`
pub const APPLICATION_3GPPHALFORMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/3gppHalForms+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/3gppHalForms+json"),
    };

/// `application/3gpp-ims+xml`
pub const APPLICATION_3GPP_IMS_XML: Essence<&'static str> =
    match Essence::new_const("application/3gpp-ims+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/3gpp-ims+xml"),
    };

/// `application/A2L`
pub const APPLICATION_A2L: Essence<&'static str> = match Essence::new_const("application/A2L") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/A2L"),
};

/// `application/ace+cbor`
pub const APPLICATION_ACE_CBOR: Essence<&'static str> =
    match Essence::new_const("application/ace+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ace+cbor"),
    };

/// `application/ace+json`
pub const APPLICATION_ACE_JSON: Essence<&'static str> =
    match Essence::new_const("application/ace+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ace+json"),
    };

/// `application/activemessage`
pub const APPLICATION_ACTIVEMESSAGE: Essence<&'static str> =
    match Essence::new_const("application/activemessage") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/activemessage"),
    };

/// `application/activity+json`
pub const APPLICATION_ACTIVITY_JSON: Essence<&'static str> =
    match Essence::new_const("application/activity+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/activity+json"),
    };

/// `application/aif+cbor`
pub const APPLICATION_AIF_CBOR: Essence<&'static str> =
    match Essence::new_const("application/aif+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/aif+cbor"),
    };

/// `application/aif+json`
pub const APPLICATION_AIF_JSON: Essence<&'static str> =
    match Essence::new_const("application/aif+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/aif+json"),
    };

/// `application/alto-cdni+json`
pub const APPLICATION_ALTO_CDNI_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-cdni+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-cdni+json"),
    };

/// `application/alto-cdnifilter+json`
pub const APPLICATION_ALTO_CDNIFILTER_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-cdnifilter+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-cdnifilter+json"),
    };

/// `application/alto-costmap+json`
pub const APPLICATION_ALTO_COSTMAP_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-costmap+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-costmap+json"),
    };

/// `application/alto-costmapfilter+json`
pub const APPLICATION_ALTO_COSTMAPFILTER_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-costmapfilter+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-costmapfilter+json"),
    };

/// `application/alto-directory+json`
pub const APPLICATION_ALTO_DIRECTORY_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-directory+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-directory+json"),
    };

/// `application/alto-endpointprop+json`
pub const APPLICATION_ALTO_ENDPOINTPROP_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-endpointprop+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-endpointprop+json"),
    };

/// `application/alto-endpointpropparams+json`
pub const APPLICATION_ALTO_ENDPOINTPROPPARAMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-endpointpropparams+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-endpointpropparams+json"),
    };

/// `application/alto-endpointcost+json`
pub const APPLICATION_ALTO_ENDPOINTCOST_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-endpointcost+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-endpointcost+json"),
    };

/// `application/alto-endpointcostparams+json`
pub const APPLICATION_ALTO_ENDPOINTCOSTPARAMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-endpointcostparams+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-endpointcostparams+json"),
    };

/// `application/alto-error+json`
pub const APPLICATION_ALTO_ERROR_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-error+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-error+json"),
    };

/// `application/alto-networkmapfilter+json`
pub const APPLICATION_ALTO_NETWORKMAPFILTER_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-networkmapfilter+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-networkmapfilter+json"),
    };

/// `application/alto-networkmap+json`
pub const APPLICATION_ALTO_NETWORKMAP_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-networkmap+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-networkmap+json"),
    };

/// `application/alto-propmap+json`
pub const APPLICATION_ALTO_PROPMAP_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-propmap+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-propmap+json"),
    };

/// `application/alto-propmapparams+json`
pub const APPLICATION_ALTO_PROPMAPPARAMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-propmapparams+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-propmapparams+json"),
    };

/// `application/alto-updatestreamcontrol+json`
pub const APPLICATION_ALTO_UPDATESTREAMCONTROL_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-updatestreamcontrol+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-updatestreamcontrol+json"),
    };

/// `application/alto-updatestreamparams+json`
pub const APPLICATION_ALTO_UPDATESTREAMPARAMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/alto-updatestreamparams+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/alto-updatestreamparams+json"),
    };

/// `application/AML`
pub const APPLICATION_AML: Essence<&'static str> = match Essence::new_const("application/AML") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/AML"),
};

/// `application/andrew-inset`
pub const APPLICATION_ANDREW_INSET: Essence<&'static str> =
    match Essence::new_const("application/andrew-inset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/andrew-inset"),
    };

/// `application/applefile`
pub const APPLICATION_APPLEFILE: Essence<&'static str> =
    match Essence::new_const("application/applefile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/applefile"),
    };

/// `application/at+jwt`
pub const APPLICATION_AT_JWT: Essence<&'static str> = match Essence::new_const("application/at+jwt")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/at+jwt"),
};

/// `application/ATF`
pub const APPLICATION_ATF: Essence<&'static str> = match Essence::new_const("application/ATF") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ATF"),
};

/// `application/ATFX`
pub const APPLICATION_ATFX: Essence<&'static str> = match Essence::new_const("application/ATFX") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ATFX"),
};

/// `application/atom+xml`
pub const APPLICATION_ATOM_XML: Essence<&'static str> =
    match Essence::new_const("application/atom+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atom+xml"),
    };

/// `application/atomcat+xml`
pub const APPLICATION_ATOMCAT_XML: Essence<&'static str> =
    match Essence::new_const("application/atomcat+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atomcat+xml"),
    };

/// `application/atomdeleted+xml`
pub const APPLICATION_ATOMDELETED_XML: Essence<&'static str> =
    match Essence::new_const("application/atomdeleted+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atomdeleted+xml"),
    };

/// `application/atomicmail`
pub const APPLICATION_ATOMICMAIL: Essence<&'static str> =
    match Essence::new_const("application/atomicmail") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atomicmail"),
    };

/// `application/atomsvc+xml`
pub const APPLICATION_ATOMSVC_XML: Essence<&'static str> =
    match Essence::new_const("application/atomsvc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atomsvc+xml"),
    };

/// `application/atsc-dwd+xml`
pub const APPLICATION_ATSC_DWD_XML: Essence<&'static str> =
    match Essence::new_const("application/atsc-dwd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atsc-dwd+xml"),
    };

/// `application/atsc-dynamic-event-message`
pub const APPLICATION_ATSC_DYNAMIC_EVENT_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/atsc-dynamic-event-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atsc-dynamic-event-message"),
    };

/// `application/atsc-held+xml`
pub const APPLICATION_ATSC_HELD_XML: Essence<&'static str> =
    match Essence::new_const("application/atsc-held+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atsc-held+xml"),
    };

/// `application/atsc-rdt+json`
pub const APPLICATION_ATSC_RDT_JSON: Essence<&'static str> =
    match Essence::new_const("application/atsc-rdt+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atsc-rdt+json"),
    };

/// `application/atsc-rsat+xml`
pub const APPLICATION_ATSC_RSAT_XML: Essence<&'static str> =
    match Essence::new_const("application/atsc-rsat+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/atsc-rsat+xml"),
    };

/// `application/ATXML`
pub const APPLICATION_ATXML: Essence<&'static str> = match Essence::new_const("application/ATXML") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ATXML"),
};

/// `application/auth-policy+xml`
pub const APPLICATION_AUTH_POLICY_XML: Essence<&'static str> =
    match Essence::new_const("application/auth-policy+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/auth-policy+xml"),
    };

/// `application/bacnet-xdd+zip`
pub const APPLICATION_BACNET_XDD_ZIP: Essence<&'static str> =
    match Essence::new_const("application/bacnet-xdd+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/bacnet-xdd+zip"),
    };

/// `application/batch-SMTP`
pub const APPLICATION_BATCH_SMTP: Essence<&'static str> =
    match Essence::new_const("application/batch-SMTP") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/batch-SMTP"),
    };

/// `application/beep+xml`
pub const APPLICATION_BEEP_XML: Essence<&'static str> =
    match Essence::new_const("application/beep+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/beep+xml"),
    };

/// `application/calendar+json`
pub const APPLICATION_CALENDAR_JSON: Essence<&'static str> =
    match Essence::new_const("application/calendar+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/calendar+json"),
    };

/// `application/calendar+xml`
pub const APPLICATION_CALENDAR_XML: Essence<&'static str> =
    match Essence::new_const("application/calendar+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/calendar+xml"),
    };

/// `application/call-completion`
pub const APPLICATION_CALL_COMPLETION: Essence<&'static str> =
    match Essence::new_const("application/call-completion") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/call-completion"),
    };

/// `application/CALS-1840`
pub const APPLICATION_CALS_1840: Essence<&'static str> =
    match Essence::new_const("application/CALS-1840") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/CALS-1840"),
    };

/// `application/captive+json`
pub const APPLICATION_CAPTIVE_JSON: Essence<&'static str> =
    match Essence::new_const("application/captive+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/captive+json"),
    };

/// `application/cbor`
pub const APPLICATION_CBOR: Essence<&'static str> = match Essence::new_const("application/cbor") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cbor"),
};

/// `application/cbor-seq`
pub const APPLICATION_CBOR_SEQ: Essence<&'static str> =
    match Essence::new_const("application/cbor-seq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cbor-seq"),
    };

/// `application/cccex`
pub const APPLICATION_CCCEX: Essence<&'static str> = match Essence::new_const("application/cccex") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cccex"),
};

/// `application/ccmp+xml`
pub const APPLICATION_CCMP_XML: Essence<&'static str> =
    match Essence::new_const("application/ccmp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ccmp+xml"),
    };

/// `application/ccxml+xml`
pub const APPLICATION_CCXML_XML: Essence<&'static str> =
    match Essence::new_const("application/ccxml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ccxml+xml"),
    };

/// `application/cda+xml`
pub const APPLICATION_CDA_XML: Essence<&'static str> =
    match Essence::new_const("application/cda+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cda+xml"),
    };

/// `application/CDFX+XML`
pub const APPLICATION_CDFX_XML: Essence<&'static str> =
    match Essence::new_const("application/CDFX+XML") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/CDFX+XML"),
    };

/// `application/cdmi-capability`
pub const APPLICATION_CDMI_CAPABILITY: Essence<&'static str> =
    match Essence::new_const("application/cdmi-capability") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cdmi-capability"),
    };

/// `application/cdmi-container`
pub const APPLICATION_CDMI_CONTAINER: Essence<&'static str> =
    match Essence::new_const("application/cdmi-container") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cdmi-container"),
    };

/// `application/cdmi-domain`
pub const APPLICATION_CDMI_DOMAIN: Essence<&'static str> =
    match Essence::new_const("application/cdmi-domain") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cdmi-domain"),
    };

/// `application/cdmi-object`
pub const APPLICATION_CDMI_OBJECT: Essence<&'static str> =
    match Essence::new_const("application/cdmi-object") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cdmi-object"),
    };

/// `application/cdmi-queue`
pub const APPLICATION_CDMI_QUEUE: Essence<&'static str> =
    match Essence::new_const("application/cdmi-queue") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cdmi-queue"),
    };

/// `application/cdni`
pub const APPLICATION_CDNI: Essence<&'static str> = match Essence::new_const("application/cdni") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cdni"),
};

/// `application/CEA`
pub const APPLICATION_CEA: Essence<&'static str> = match Essence::new_const("application/CEA") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/CEA"),
};

/// `application/cea-2018+xml`
pub const APPLICATION_CEA_2018_XML: Essence<&'static str> =
    match Essence::new_const("application/cea-2018+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cea-2018+xml"),
    };

/// `application/cellml+xml`
pub const APPLICATION_CELLML_XML: Essence<&'static str> =
    match Essence::new_const("application/cellml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cellml+xml"),
    };

/// `application/cfw`
pub const APPLICATION_CFW: Essence<&'static str> = match Essence::new_const("application/cfw") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cfw"),
};

/// `application/city+json`
pub const APPLICATION_CITY_JSON: Essence<&'static str> =
    match Essence::new_const("application/city+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/city+json"),
    };

/// `application/clr`
pub const APPLICATION_CLR: Essence<&'static str> = match Essence::new_const("application/clr") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/clr"),
};

/// `application/clue_info+xml`
pub const APPLICATION_CLUE_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/clue_info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/clue_info+xml"),
    };

/// `application/clue+xml`
pub const APPLICATION_CLUE_XML: Essence<&'static str> =
    match Essence::new_const("application/clue+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/clue+xml"),
    };

/// `application/cms`
pub const APPLICATION_CMS: Essence<&'static str> = match Essence::new_const("application/cms") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cms"),
};

/// `application/cnrp+xml`
pub const APPLICATION_CNRP_XML: Essence<&'static str> =
    match Essence::new_const("application/cnrp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cnrp+xml"),
    };

/// `application/coap-group+json`
pub const APPLICATION_COAP_GROUP_JSON: Essence<&'static str> =
    match Essence::new_const("application/coap-group+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/coap-group+json"),
    };

/// `application/coap-payload`
pub const APPLICATION_COAP_PAYLOAD: Essence<&'static str> =
    match Essence::new_const("application/coap-payload") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/coap-payload"),
    };

/// `application/commonground`
pub const APPLICATION_COMMONGROUND: Essence<&'static str> =
    match Essence::new_const("application/commonground") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/commonground"),
    };

/// `application/conference-info+xml`
pub const APPLICATION_CONFERENCE_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/conference-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/conference-info+xml"),
    };

/// `application/cpl+xml`
pub const APPLICATION_CPL_XML: Essence<&'static str> =
    match Essence::new_const("application/cpl+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cpl+xml"),
    };

/// `application/cose`
pub const APPLICATION_COSE: Essence<&'static str> = match Essence::new_const("application/cose") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cose"),
};

/// `application/cose-key`
pub const APPLICATION_COSE_KEY: Essence<&'static str> =
    match Essence::new_const("application/cose-key") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cose-key"),
    };

/// `application/cose-key-set`
pub const APPLICATION_COSE_KEY_SET: Essence<&'static str> =
    match Essence::new_const("application/cose-key-set") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cose-key-set"),
    };

/// `application/csrattrs`
pub const APPLICATION_CSRATTRS: Essence<&'static str> =
    match Essence::new_const("application/csrattrs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/csrattrs"),
    };

/// `application/csta+xml`
pub const APPLICATION_CSTA_XML: Essence<&'static str> =
    match Essence::new_const("application/csta+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/csta+xml"),
    };

/// `application/CSTAdata+xml`
pub const APPLICATION_CSTADATA_XML: Essence<&'static str> =
    match Essence::new_const("application/CSTAdata+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/CSTAdata+xml"),
    };

/// `application/csvm+json`
pub const APPLICATION_CSVM_JSON: Essence<&'static str> =
    match Essence::new_const("application/csvm+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/csvm+json"),
    };

/// `application/cwt`
pub const APPLICATION_CWT: Essence<&'static str> = match Essence::new_const("application/cwt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/cwt"),
};

/// `application/cybercash`
pub const APPLICATION_CYBERCASH: Essence<&'static str> =
    match Essence::new_const("application/cybercash") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/cybercash"),
    };

/// `application/dash+xml`
pub const APPLICATION_DASH_XML: Essence<&'static str> =
    match Essence::new_const("application/dash+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dash+xml"),
    };

/// `application/dash-patch+xml`
pub const APPLICATION_DASH_PATCH_XML: Essence<&'static str> =
    match Essence::new_const("application/dash-patch+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dash-patch+xml"),
    };

/// `application/dashdelta`
pub const APPLICATION_DASHDELTA: Essence<&'static str> =
    match Essence::new_const("application/dashdelta") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dashdelta"),
    };

/// `application/davmount+xml`
pub const APPLICATION_DAVMOUNT_XML: Essence<&'static str> =
    match Essence::new_const("application/davmount+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/davmount+xml"),
    };

/// `application/dca-rft`
pub const APPLICATION_DCA_RFT: Essence<&'static str> =
    match Essence::new_const("application/dca-rft") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dca-rft"),
    };

/// `application/DCD`
pub const APPLICATION_DCD: Essence<&'static str> = match Essence::new_const("application/DCD") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/DCD"),
};

/// `application/dec-dx`
pub const APPLICATION_DEC_DX: Essence<&'static str> = match Essence::new_const("application/dec-dx")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/dec-dx"),
};

/// `application/dialog-info+xml`
pub const APPLICATION_DIALOG_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/dialog-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dialog-info+xml"),
    };

/// `application/dicom`
pub const APPLICATION_DICOM: Essence<&'static str> = match Essence::new_const("application/dicom") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/dicom"),
};

/// `application/dicom+json`
pub const APPLICATION_DICOM_JSON: Essence<&'static str> =
    match Essence::new_const("application/dicom+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dicom+json"),
    };

/// `application/dicom+xml`
pub const APPLICATION_DICOM_XML: Essence<&'static str> =
    match Essence::new_const("application/dicom+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dicom+xml"),
    };

/// `application/DII`
pub const APPLICATION_DII: Essence<&'static str> = match Essence::new_const("application/DII") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/DII"),
};

/// `application/DIT`
pub const APPLICATION_DIT: Essence<&'static str> = match Essence::new_const("application/DIT") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/DIT"),
};

/// `application/dns`
pub const APPLICATION_DNS: Essence<&'static str> = match Essence::new_const("application/dns") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/dns"),
};

/// `application/dns+json`
pub const APPLICATION_DNS_JSON: Essence<&'static str> =
    match Essence::new_const("application/dns+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dns+json"),
    };

/// `application/dns-message`
pub const APPLICATION_DNS_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/dns-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dns-message"),
    };

/// `application/dots+cbor`
pub const APPLICATION_DOTS_CBOR: Essence<&'static str> =
    match Essence::new_const("application/dots+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dots+cbor"),
    };

/// `application/dskpp+xml`
pub const APPLICATION_DSKPP_XML: Essence<&'static str> =
    match Essence::new_const("application/dskpp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dskpp+xml"),
    };

/// `application/dssc+der`
pub const APPLICATION_DSSC_DER: Essence<&'static str> =
    match Essence::new_const("application/dssc+der") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dssc+der"),
    };

/// `application/dssc+xml`
pub const APPLICATION_DSSC_XML: Essence<&'static str> =
    match Essence::new_const("application/dssc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/dssc+xml"),
    };

/// `application/dvcs`
pub const APPLICATION_DVCS: Essence<&'static str> = match Essence::new_const("application/dvcs") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/dvcs"),
};

/// `application/ecmascript`
pub const APPLICATION_ECMASCRIPT: Essence<&'static str> =
    match Essence::new_const("application/ecmascript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ecmascript"),
    };

/// `application/EDI-consent`
pub const APPLICATION_EDI_CONSENT: Essence<&'static str> =
    match Essence::new_const("application/EDI-consent") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EDI-consent"),
    };

/// `application/EDIFACT`
pub const APPLICATION_EDIFACT: Essence<&'static str> =
    match Essence::new_const("application/EDIFACT") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EDIFACT"),
    };

/// `application/EDI-X12`
pub const APPLICATION_EDI_X12: Essence<&'static str> =
    match Essence::new_const("application/EDI-X12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EDI-X12"),
    };

/// `application/efi`
pub const APPLICATION_EFI: Essence<&'static str> = match Essence::new_const("application/efi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/efi"),
};

/// `application/elm+json`
pub const APPLICATION_ELM_JSON: Essence<&'static str> =
    match Essence::new_const("application/elm+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/elm+json"),
    };

/// `application/elm+xml`
pub const APPLICATION_ELM_XML: Essence<&'static str> =
    match Essence::new_const("application/elm+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/elm+xml"),
    };

/// `application/EmergencyCallData.cap+xml`
pub const APPLICATION_EMERGENCYCALLDATA_CAP_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.cap+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.cap+xml"),
    };

/// `application/EmergencyCallData.Comment+xml`
pub const APPLICATION_EMERGENCYCALLDATA_COMMENT_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.Comment+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.Comment+xml"),
    };

/// `application/EmergencyCallData.Control+xml`
pub const APPLICATION_EMERGENCYCALLDATA_CONTROL_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.Control+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.Control+xml"),
    };

/// `application/EmergencyCallData.DeviceInfo+xml`
pub const APPLICATION_EMERGENCYCALLDATA_DEVICEINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.DeviceInfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.DeviceInfo+xml"),
    };

/// `application/EmergencyCallData.eCall.MSD`
pub const APPLICATION_EMERGENCYCALLDATA_ECALL_MSD: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.eCall.MSD") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.eCall.MSD"),
    };

/// `application/EmergencyCallData.ProviderInfo+xml`
pub const APPLICATION_EMERGENCYCALLDATA_PROVIDERINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.ProviderInfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.ProviderInfo+xml"),
    };

/// `application/EmergencyCallData.ServiceInfo+xml`
pub const APPLICATION_EMERGENCYCALLDATA_SERVICEINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.ServiceInfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.ServiceInfo+xml"),
    };

/// `application/EmergencyCallData.SubscriberInfo+xml`
pub const APPLICATION_EMERGENCYCALLDATA_SUBSCRIBERINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.SubscriberInfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.SubscriberInfo+xml"),
    };

/// `application/EmergencyCallData.VEDS+xml`
pub const APPLICATION_EMERGENCYCALLDATA_VEDS_XML: Essence<&'static str> =
    match Essence::new_const("application/EmergencyCallData.VEDS+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/EmergencyCallData.VEDS+xml"),
    };

/// `application/emma+xml`
pub const APPLICATION_EMMA_XML: Essence<&'static str> =
    match Essence::new_const("application/emma+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/emma+xml"),
    };

/// `application/emotionml+xml`
pub const APPLICATION_EMOTIONML_XML: Essence<&'static str> =
    match Essence::new_const("application/emotionml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/emotionml+xml"),
    };

/// `application/encaprtp`
pub const APPLICATION_ENCAPRTP: Essence<&'static str> =
    match Essence::new_const("application/encaprtp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/encaprtp"),
    };

/// `application/epp+xml`
pub const APPLICATION_EPP_XML: Essence<&'static str> =
    match Essence::new_const("application/epp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/epp+xml"),
    };

/// `application/epub+zip`
pub const APPLICATION_EPUB_ZIP: Essence<&'static str> =
    match Essence::new_const("application/epub+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/epub+zip"),
    };

/// `application/eshop`
pub const APPLICATION_ESHOP: Essence<&'static str> = match Essence::new_const("application/eshop") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/eshop"),
};

/// `application/example`
pub const APPLICATION_EXAMPLE: Essence<&'static str> =
    match Essence::new_const("application/example") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/example"),
    };

/// `application/exi`
pub const APPLICATION_EXI: Essence<&'static str> = match Essence::new_const("application/exi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/exi"),
};

/// `application/expect-ct-report+json`
pub const APPLICATION_EXPECT_CT_REPORT_JSON: Essence<&'static str> =
    match Essence::new_const("application/expect-ct-report+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/expect-ct-report+json"),
    };

/// `application/express`
pub const APPLICATION_EXPRESS: Essence<&'static str> =
    match Essence::new_const("application/express") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/express"),
    };

/// `application/fastinfoset`
pub const APPLICATION_FASTINFOSET: Essence<&'static str> =
    match Essence::new_const("application/fastinfoset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/fastinfoset"),
    };

/// `application/fastsoap`
pub const APPLICATION_FASTSOAP: Essence<&'static str> =
    match Essence::new_const("application/fastsoap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/fastsoap"),
    };

/// `application/fdf`
pub const APPLICATION_FDF: Essence<&'static str> = match Essence::new_const("application/fdf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/fdf"),
};

/// `application/fdt+xml`
pub const APPLICATION_FDT_XML: Essence<&'static str> =
    match Essence::new_const("application/fdt+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/fdt+xml"),
    };

/// `application/fhir+json`
pub const APPLICATION_FHIR_JSON: Essence<&'static str> =
    match Essence::new_const("application/fhir+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/fhir+json"),
    };

/// `application/fhir+xml`
pub const APPLICATION_FHIR_XML: Essence<&'static str> =
    match Essence::new_const("application/fhir+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/fhir+xml"),
    };

/// `application/fits`
pub const APPLICATION_FITS: Essence<&'static str> = match Essence::new_const("application/fits") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/fits"),
};

/// `application/flexfec`
pub const APPLICATION_FLEXFEC: Essence<&'static str> =
    match Essence::new_const("application/flexfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/flexfec"),
    };

/// `application/font-tdpfr`
pub const APPLICATION_FONT_TDPFR: Essence<&'static str> =
    match Essence::new_const("application/font-tdpfr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/font-tdpfr"),
    };

/// `application/framework-attributes+xml`
pub const APPLICATION_FRAMEWORK_ATTRIBUTES_XML: Essence<&'static str> =
    match Essence::new_const("application/framework-attributes+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/framework-attributes+xml"),
    };

/// `application/geo+json`
pub const APPLICATION_GEO_JSON: Essence<&'static str> =
    match Essence::new_const("application/geo+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/geo+json"),
    };

/// `application/geo+json-seq`
pub const APPLICATION_GEO_JSON_SEQ: Essence<&'static str> =
    match Essence::new_const("application/geo+json-seq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/geo+json-seq"),
    };

/// `application/geopackage+sqlite3`
pub const APPLICATION_GEOPACKAGE_SQLITE3: Essence<&'static str> =
    match Essence::new_const("application/geopackage+sqlite3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/geopackage+sqlite3"),
    };

/// `application/geoxacml+xml`
pub const APPLICATION_GEOXACML_XML: Essence<&'static str> =
    match Essence::new_const("application/geoxacml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/geoxacml+xml"),
    };

/// `application/gltf-buffer`
pub const APPLICATION_GLTF_BUFFER: Essence<&'static str> =
    match Essence::new_const("application/gltf-buffer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/gltf-buffer"),
    };

/// `application/gml+xml`
pub const APPLICATION_GML_XML: Essence<&'static str> =
    match Essence::new_const("application/gml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/gml+xml"),
    };

/// `application/gzip`
pub const APPLICATION_GZIP: Essence<&'static str> = match Essence::new_const("application/gzip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/gzip"),
};

/// `application/H224`
pub const APPLICATION_H224: Essence<&'static str> = match Essence::new_const("application/H224") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/H224"),
};

/// `application/held+xml`
pub const APPLICATION_HELD_XML: Essence<&'static str> =
    match Essence::new_const("application/held+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/held+xml"),
    };

/// `application/hl7v2+xml`
pub const APPLICATION_HL7V2_XML: Essence<&'static str> =
    match Essence::new_const("application/hl7v2+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/hl7v2+xml"),
    };

/// `application/http`
pub const APPLICATION_HTTP: Essence<&'static str> = match Essence::new_const("application/http") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/http"),
};

/// `application/hyperstudio`
pub const APPLICATION_HYPERSTUDIO: Essence<&'static str> =
    match Essence::new_const("application/hyperstudio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/hyperstudio"),
    };

/// `application/ibe-key-request+xml`
pub const APPLICATION_IBE_KEY_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/ibe-key-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ibe-key-request+xml"),
    };

/// `application/ibe-pkg-reply+xml`
pub const APPLICATION_IBE_PKG_REPLY_XML: Essence<&'static str> =
    match Essence::new_const("application/ibe-pkg-reply+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ibe-pkg-reply+xml"),
    };

/// `application/ibe-pp-data`
pub const APPLICATION_IBE_PP_DATA: Essence<&'static str> =
    match Essence::new_const("application/ibe-pp-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ibe-pp-data"),
    };

/// `application/iges`
pub const APPLICATION_IGES: Essence<&'static str> = match Essence::new_const("application/iges") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/iges"),
};

/// `application/im-iscomposing+xml`
pub const APPLICATION_IM_ISCOMPOSING_XML: Essence<&'static str> =
    match Essence::new_const("application/im-iscomposing+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/im-iscomposing+xml"),
    };

/// `application/index`
pub const APPLICATION_INDEX: Essence<&'static str> = match Essence::new_const("application/index") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/index"),
};

/// `application/index.cmd`
pub const APPLICATION_INDEX_CMD: Essence<&'static str> =
    match Essence::new_const("application/index.cmd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/index.cmd"),
    };

/// `application/index.obj`
pub const APPLICATION_INDEX_OBJ: Essence<&'static str> =
    match Essence::new_const("application/index.obj") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/index.obj"),
    };

/// `application/index.response`
pub const APPLICATION_INDEX_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/index.response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/index.response"),
    };

/// `application/index.vnd`
pub const APPLICATION_INDEX_VND: Essence<&'static str> =
    match Essence::new_const("application/index.vnd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/index.vnd"),
    };

/// `application/inkml+xml`
pub const APPLICATION_INKML_XML: Essence<&'static str> =
    match Essence::new_const("application/inkml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/inkml+xml"),
    };

/// `application/IOTP`
pub const APPLICATION_IOTP: Essence<&'static str> = match Essence::new_const("application/IOTP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/IOTP"),
};

/// `application/ipfix`
pub const APPLICATION_IPFIX: Essence<&'static str> = match Essence::new_const("application/ipfix") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ipfix"),
};

/// `application/ipp`
pub const APPLICATION_IPP: Essence<&'static str> = match Essence::new_const("application/ipp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ipp"),
};

/// `application/ISUP`
pub const APPLICATION_ISUP: Essence<&'static str> = match Essence::new_const("application/ISUP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ISUP"),
};

/// `application/its+xml`
pub const APPLICATION_ITS_XML: Essence<&'static str> =
    match Essence::new_const("application/its+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/its+xml"),
    };

/// `application/javascript`
pub const APPLICATION_JAVASCRIPT: Essence<&'static str> =
    match Essence::new_const("application/javascript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/javascript"),
    };

/// `application/jf2feed+json`
pub const APPLICATION_JF2FEED_JSON: Essence<&'static str> =
    match Essence::new_const("application/jf2feed+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jf2feed+json"),
    };

/// `application/jose`
pub const APPLICATION_JOSE: Essence<&'static str> = match Essence::new_const("application/jose") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/jose"),
};

/// `application/jose+json`
pub const APPLICATION_JOSE_JSON: Essence<&'static str> =
    match Essence::new_const("application/jose+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jose+json"),
    };

/// `application/jrd+json`
pub const APPLICATION_JRD_JSON: Essence<&'static str> =
    match Essence::new_const("application/jrd+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jrd+json"),
    };

/// `application/jscalendar+json`
pub const APPLICATION_JSCALENDAR_JSON: Essence<&'static str> =
    match Essence::new_const("application/jscalendar+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jscalendar+json"),
    };

/// `application/json`
pub const APPLICATION_JSON: Essence<&'static str> = match Essence::new_const("application/json") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/json"),
};

/// `application/json-patch+json`
pub const APPLICATION_JSON_PATCH_JSON: Essence<&'static str> =
    match Essence::new_const("application/json-patch+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/json-patch+json"),
    };

/// `application/json-seq`
pub const APPLICATION_JSON_SEQ: Essence<&'static str> =
    match Essence::new_const("application/json-seq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/json-seq"),
    };

/// `application/jwk+json`
pub const APPLICATION_JWK_JSON: Essence<&'static str> =
    match Essence::new_const("application/jwk+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jwk+json"),
    };

/// `application/jwk-set+json`
pub const APPLICATION_JWK_SET_JSON: Essence<&'static str> =
    match Essence::new_const("application/jwk-set+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/jwk-set+json"),
    };

/// `application/jwt`
pub const APPLICATION_JWT: Essence<&'static str> = match Essence::new_const("application/jwt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/jwt"),
};

/// `application/kpml-request+xml`
pub const APPLICATION_KPML_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/kpml-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/kpml-request+xml"),
    };

/// `application/kpml-response+xml`
pub const APPLICATION_KPML_RESPONSE_XML: Essence<&'static str> =
    match Essence::new_const("application/kpml-response+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/kpml-response+xml"),
    };

/// `application/ld+json`
pub const APPLICATION_LD_JSON: Essence<&'static str> =
    match Essence::new_const("application/ld+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ld+json"),
    };

/// `application/lgr+xml`
pub const APPLICATION_LGR_XML: Essence<&'static str> =
    match Essence::new_const("application/lgr+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/lgr+xml"),
    };

/// `application/link-format`
pub const APPLICATION_LINK_FORMAT: Essence<&'static str> =
    match Essence::new_const("application/link-format") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/link-format"),
    };

/// `application/load-control+xml`
pub const APPLICATION_LOAD_CONTROL_XML: Essence<&'static str> =
    match Essence::new_const("application/load-control+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/load-control+xml"),
    };

/// `application/lost+xml`
pub const APPLICATION_LOST_XML: Essence<&'static str> =
    match Essence::new_const("application/lost+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/lost+xml"),
    };

/// `application/lostsync+xml`
pub const APPLICATION_LOSTSYNC_XML: Essence<&'static str> =
    match Essence::new_const("application/lostsync+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/lostsync+xml"),
    };

/// `application/lpf+zip`
pub const APPLICATION_LPF_ZIP: Essence<&'static str> =
    match Essence::new_const("application/lpf+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/lpf+zip"),
    };

/// `application/LXF`
pub const APPLICATION_LXF: Essence<&'static str> = match Essence::new_const("application/LXF") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/LXF"),
};

/// `application/mac-binhex40`
pub const APPLICATION_MAC_BINHEX40: Essence<&'static str> =
    match Essence::new_const("application/mac-binhex40") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mac-binhex40"),
    };

/// `application/macwriteii`
pub const APPLICATION_MACWRITEII: Essence<&'static str> =
    match Essence::new_const("application/macwriteii") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/macwriteii"),
    };

/// `application/mads+xml`
pub const APPLICATION_MADS_XML: Essence<&'static str> =
    match Essence::new_const("application/mads+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mads+xml"),
    };

/// `application/manifest+json`
pub const APPLICATION_MANIFEST_JSON: Essence<&'static str> =
    match Essence::new_const("application/manifest+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/manifest+json"),
    };

/// `application/marc`
pub const APPLICATION_MARC: Essence<&'static str> = match Essence::new_const("application/marc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/marc"),
};

/// `application/marcxml+xml`
pub const APPLICATION_MARCXML_XML: Essence<&'static str> =
    match Essence::new_const("application/marcxml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/marcxml+xml"),
    };

/// `application/mathematica`
pub const APPLICATION_MATHEMATICA: Essence<&'static str> =
    match Essence::new_const("application/mathematica") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mathematica"),
    };

/// `application/mathml+xml`
pub const APPLICATION_MATHML_XML: Essence<&'static str> =
    match Essence::new_const("application/mathml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mathml+xml"),
    };

/// `application/mathml-content+xml`
pub const APPLICATION_MATHML_CONTENT_XML: Essence<&'static str> =
    match Essence::new_const("application/mathml-content+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mathml-content+xml"),
    };

/// `application/mathml-presentation+xml`
pub const APPLICATION_MATHML_PRESENTATION_XML: Essence<&'static str> =
    match Essence::new_const("application/mathml-presentation+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mathml-presentation+xml"),
    };

/// `application/mbms-associated-procedure-description+xml`
pub const APPLICATION_MBMS_ASSOCIATED_PROCEDURE_DESCRIPTION_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-associated-procedure-description+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-associated-procedure-description+xml"),
    };

/// `application/mbms-deregister+xml`
pub const APPLICATION_MBMS_DEREGISTER_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-deregister+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-deregister+xml"),
    };

/// `application/mbms-envelope+xml`
pub const APPLICATION_MBMS_ENVELOPE_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-envelope+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-envelope+xml"),
    };

/// `application/mbms-msk-response+xml`
pub const APPLICATION_MBMS_MSK_RESPONSE_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-msk-response+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-msk-response+xml"),
    };

/// `application/mbms-msk+xml`
pub const APPLICATION_MBMS_MSK_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-msk+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-msk+xml"),
    };

/// `application/mbms-protection-description+xml`
pub const APPLICATION_MBMS_PROTECTION_DESCRIPTION_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-protection-description+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-protection-description+xml"),
    };

/// `application/mbms-reception-report+xml`
pub const APPLICATION_MBMS_RECEPTION_REPORT_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-reception-report+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-reception-report+xml"),
    };

/// `application/mbms-register-response+xml`
pub const APPLICATION_MBMS_REGISTER_RESPONSE_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-register-response+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-register-response+xml"),
    };

/// `application/mbms-register+xml`
pub const APPLICATION_MBMS_REGISTER_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-register+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-register+xml"),
    };

/// `application/mbms-schedule+xml`
pub const APPLICATION_MBMS_SCHEDULE_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-schedule+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-schedule+xml"),
    };

/// `application/mbms-user-service-description+xml`
pub const APPLICATION_MBMS_USER_SERVICE_DESCRIPTION_XML: Essence<&'static str> =
    match Essence::new_const("application/mbms-user-service-description+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mbms-user-service-description+xml"),
    };

/// `application/mbox`
pub const APPLICATION_MBOX: Essence<&'static str> = match Essence::new_const("application/mbox") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mbox"),
};

/// `application/media_control+xml`
pub const APPLICATION_MEDIA_CONTROL_XML: Essence<&'static str> =
    match Essence::new_const("application/media_control+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/media_control+xml"),
    };

/// `application/media-policy-dataset+xml`
pub const APPLICATION_MEDIA_POLICY_DATASET_XML: Essence<&'static str> =
    match Essence::new_const("application/media-policy-dataset+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/media-policy-dataset+xml"),
    };

/// `application/mediaservercontrol+xml`
pub const APPLICATION_MEDIASERVERCONTROL_XML: Essence<&'static str> =
    match Essence::new_const("application/mediaservercontrol+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mediaservercontrol+xml"),
    };

/// `application/merge-patch+json`
pub const APPLICATION_MERGE_PATCH_JSON: Essence<&'static str> =
    match Essence::new_const("application/merge-patch+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/merge-patch+json"),
    };

/// `application/metalink4+xml`
pub const APPLICATION_METALINK4_XML: Essence<&'static str> =
    match Essence::new_const("application/metalink4+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/metalink4+xml"),
    };

/// `application/mets+xml`
pub const APPLICATION_METS_XML: Essence<&'static str> =
    match Essence::new_const("application/mets+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mets+xml"),
    };

/// `application/MF4`
pub const APPLICATION_MF4: Essence<&'static str> = match Essence::new_const("application/MF4") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/MF4"),
};

/// `application/mikey`
pub const APPLICATION_MIKEY: Essence<&'static str> = match Essence::new_const("application/mikey") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mikey"),
};

/// `application/mipc`
pub const APPLICATION_MIPC: Essence<&'static str> = match Essence::new_const("application/mipc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mipc"),
};

/// `application/missing-blocks+cbor-seq`
pub const APPLICATION_MISSING_BLOCKS_CBOR_SEQ: Essence<&'static str> =
    match Essence::new_const("application/missing-blocks+cbor-seq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/missing-blocks+cbor-seq"),
    };

/// `application/mmt-aei+xml`
pub const APPLICATION_MMT_AEI_XML: Essence<&'static str> =
    match Essence::new_const("application/mmt-aei+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mmt-aei+xml"),
    };

/// `application/mmt-usd+xml`
pub const APPLICATION_MMT_USD_XML: Essence<&'static str> =
    match Essence::new_const("application/mmt-usd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mmt-usd+xml"),
    };

/// `application/mods+xml`
pub const APPLICATION_MODS_XML: Essence<&'static str> =
    match Essence::new_const("application/mods+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mods+xml"),
    };

/// `application/moss-keys`
pub const APPLICATION_MOSS_KEYS: Essence<&'static str> =
    match Essence::new_const("application/moss-keys") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/moss-keys"),
    };

/// `application/moss-signature`
pub const APPLICATION_MOSS_SIGNATURE: Essence<&'static str> =
    match Essence::new_const("application/moss-signature") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/moss-signature"),
    };

/// `application/mosskey-data`
pub const APPLICATION_MOSSKEY_DATA: Essence<&'static str> =
    match Essence::new_const("application/mosskey-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mosskey-data"),
    };

/// `application/mosskey-request`
pub const APPLICATION_MOSSKEY_REQUEST: Essence<&'static str> =
    match Essence::new_const("application/mosskey-request") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mosskey-request"),
    };

/// `application/mp21`
pub const APPLICATION_MP21: Essence<&'static str> = match Essence::new_const("application/mp21") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mp21"),
};

/// `application/mp4`
pub const APPLICATION_MP4: Essence<&'static str> = match Essence::new_const("application/mp4") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mp4"),
};

/// `application/mpeg4-generic`
pub const APPLICATION_MPEG4_GENERIC: Essence<&'static str> =
    match Essence::new_const("application/mpeg4-generic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mpeg4-generic"),
    };

/// `application/mpeg4-iod`
pub const APPLICATION_MPEG4_IOD: Essence<&'static str> =
    match Essence::new_const("application/mpeg4-iod") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mpeg4-iod"),
    };

/// `application/mpeg4-iod-xmt`
pub const APPLICATION_MPEG4_IOD_XMT: Essence<&'static str> =
    match Essence::new_const("application/mpeg4-iod-xmt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mpeg4-iod-xmt"),
    };

/// `application/mrb-consumer+xml`
pub const APPLICATION_MRB_CONSUMER_XML: Essence<&'static str> =
    match Essence::new_const("application/mrb-consumer+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mrb-consumer+xml"),
    };

/// `application/mrb-publish+xml`
pub const APPLICATION_MRB_PUBLISH_XML: Essence<&'static str> =
    match Essence::new_const("application/mrb-publish+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mrb-publish+xml"),
    };

/// `application/msc-ivr+xml`
pub const APPLICATION_MSC_IVR_XML: Essence<&'static str> =
    match Essence::new_const("application/msc-ivr+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/msc-ivr+xml"),
    };

/// `application/msc-mixer+xml`
pub const APPLICATION_MSC_MIXER_XML: Essence<&'static str> =
    match Essence::new_const("application/msc-mixer+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/msc-mixer+xml"),
    };

/// `application/msword`
pub const APPLICATION_MSWORD: Essence<&'static str> = match Essence::new_const("application/msword")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/msword"),
};

/// `application/mud+json`
pub const APPLICATION_MUD_JSON: Essence<&'static str> =
    match Essence::new_const("application/mud+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/mud+json"),
    };

/// `application/multipart-core`
pub const APPLICATION_MULTIPART_CORE: Essence<&'static str> =
    match Essence::new_const("application/multipart-core") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/multipart-core"),
    };

/// `application/mxf`
pub const APPLICATION_MXF: Essence<&'static str> = match Essence::new_const("application/mxf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/mxf"),
};

/// `application/n-quads`
pub const APPLICATION_N_QUADS: Essence<&'static str> =
    match Essence::new_const("application/n-quads") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/n-quads"),
    };

/// `application/n-triples`
pub const APPLICATION_N_TRIPLES: Essence<&'static str> =
    match Essence::new_const("application/n-triples") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/n-triples"),
    };

/// `application/nasdata`
pub const APPLICATION_NASDATA: Essence<&'static str> =
    match Essence::new_const("application/nasdata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/nasdata"),
    };

/// `application/news-checkgroups`
pub const APPLICATION_NEWS_CHECKGROUPS: Essence<&'static str> =
    match Essence::new_const("application/news-checkgroups") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/news-checkgroups"),
    };

/// `application/news-groupinfo`
pub const APPLICATION_NEWS_GROUPINFO: Essence<&'static str> =
    match Essence::new_const("application/news-groupinfo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/news-groupinfo"),
    };

/// `application/news-transmission`
pub const APPLICATION_NEWS_TRANSMISSION: Essence<&'static str> =
    match Essence::new_const("application/news-transmission") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/news-transmission"),
    };

/// `application/nlsml+xml`
pub const APPLICATION_NLSML_XML: Essence<&'static str> =
    match Essence::new_const("application/nlsml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/nlsml+xml"),
    };

/// `application/node`
pub const APPLICATION_NODE: Essence<&'static str> = match Essence::new_const("application/node") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/node"),
};

/// `application/nss`
pub const APPLICATION_NSS: Essence<&'static str> = match Essence::new_const("application/nss") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/nss"),
};

/// `application/oauth-authz-req+jwt`
pub const APPLICATION_OAUTH_AUTHZ_REQ_JWT: Essence<&'static str> =
    match Essence::new_const("application/oauth-authz-req+jwt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/oauth-authz-req+jwt"),
    };

/// `application/oblivious-dns-message`
pub const APPLICATION_OBLIVIOUS_DNS_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/oblivious-dns-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/oblivious-dns-message"),
    };

/// `application/ocsp-request`
pub const APPLICATION_OCSP_REQUEST: Essence<&'static str> =
    match Essence::new_const("application/ocsp-request") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ocsp-request"),
    };

/// `application/ocsp-response`
pub const APPLICATION_OCSP_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/ocsp-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ocsp-response"),
    };

/// `application/octet-stream`
pub const APPLICATION_OCTET_STREAM: Essence<&'static str> =
    match Essence::new_const("application/octet-stream") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/octet-stream"),
    };

/// `application/ODA`
pub const APPLICATION_ODA: Essence<&'static str> = match Essence::new_const("application/ODA") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ODA"),
};

/// `application/odm+xml`
pub const APPLICATION_ODM_XML: Essence<&'static str> =
    match Essence::new_const("application/odm+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/odm+xml"),
    };

/// `application/ODX`
pub const APPLICATION_ODX: Essence<&'static str> = match Essence::new_const("application/ODX") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ODX"),
};

/// `application/oebps-package+xml`
pub const APPLICATION_OEBPS_PACKAGE_XML: Essence<&'static str> =
    match Essence::new_const("application/oebps-package+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/oebps-package+xml"),
    };

/// `application/ogg`
pub const APPLICATION_OGG: Essence<&'static str> = match Essence::new_const("application/ogg") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ogg"),
};

/// `application/opc-nodeset+xml`
pub const APPLICATION_OPC_NODESET_XML: Essence<&'static str> =
    match Essence::new_const("application/opc-nodeset+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/opc-nodeset+xml"),
    };

/// `application/oscore`
pub const APPLICATION_OSCORE: Essence<&'static str> = match Essence::new_const("application/oscore")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/oscore"),
};

/// `application/oxps`
pub const APPLICATION_OXPS: Essence<&'static str> = match Essence::new_const("application/oxps") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/oxps"),
};

/// `application/p21`
pub const APPLICATION_P21: Essence<&'static str> = match Essence::new_const("application/p21") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/p21"),
};

/// `application/p21+zip`
pub const APPLICATION_P21_ZIP: Essence<&'static str> =
    match Essence::new_const("application/p21+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/p21+zip"),
    };

/// `application/p2p-overlay+xml`
pub const APPLICATION_P2P_OVERLAY_XML: Essence<&'static str> =
    match Essence::new_const("application/p2p-overlay+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/p2p-overlay+xml"),
    };

/// `application/parityfec`
pub const APPLICATION_PARITYFEC: Essence<&'static str> =
    match Essence::new_const("application/parityfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/parityfec"),
    };

/// `application/passport`
pub const APPLICATION_PASSPORT: Essence<&'static str> =
    match Essence::new_const("application/passport") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/passport"),
    };

/// `application/patch-ops-error+xml`
pub const APPLICATION_PATCH_OPS_ERROR_XML: Essence<&'static str> =
    match Essence::new_const("application/patch-ops-error+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/patch-ops-error+xml"),
    };

/// `application/pdf`
pub const APPLICATION_PDF: Essence<&'static str> = match Essence::new_const("application/pdf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/pdf"),
};

/// `application/PDX`
pub const APPLICATION_PDX: Essence<&'static str> = match Essence::new_const("application/PDX") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/PDX"),
};

/// `application/pem-certificate-chain`
pub const APPLICATION_PEM_CERTIFICATE_CHAIN: Essence<&'static str> =
    match Essence::new_const("application/pem-certificate-chain") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pem-certificate-chain"),
    };

/// `application/pgp-encrypted`
pub const APPLICATION_PGP_ENCRYPTED: Essence<&'static str> =
    match Essence::new_const("application/pgp-encrypted") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pgp-encrypted"),
    };

/// `application/pgp-keys`
pub const APPLICATION_PGP_KEYS: Essence<&'static str> =
    match Essence::new_const("application/pgp-keys") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pgp-keys"),
    };

/// `application/pgp-signature`
pub const APPLICATION_PGP_SIGNATURE: Essence<&'static str> =
    match Essence::new_const("application/pgp-signature") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pgp-signature"),
    };

/// `application/pidf-diff+xml`
pub const APPLICATION_PIDF_DIFF_XML: Essence<&'static str> =
    match Essence::new_const("application/pidf-diff+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pidf-diff+xml"),
    };

/// `application/pidf+xml`
pub const APPLICATION_PIDF_XML: Essence<&'static str> =
    match Essence::new_const("application/pidf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pidf+xml"),
    };

/// `application/pkcs10`
pub const APPLICATION_PKCS10: Essence<&'static str> = match Essence::new_const("application/pkcs10")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/pkcs10"),
};

/// `application/pkcs7-mime`
pub const APPLICATION_PKCS7_MIME: Essence<&'static str> =
    match Essence::new_const("application/pkcs7-mime") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkcs7-mime"),
    };

/// `application/pkcs7-signature`
pub const APPLICATION_PKCS7_SIGNATURE: Essence<&'static str> =
    match Essence::new_const("application/pkcs7-signature") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkcs7-signature"),
    };

/// `application/pkcs8`
pub const APPLICATION_PKCS8: Essence<&'static str> = match Essence::new_const("application/pkcs8") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/pkcs8"),
};

/// `application/pkcs8-encrypted`
pub const APPLICATION_PKCS8_ENCRYPTED: Essence<&'static str> =
    match Essence::new_const("application/pkcs8-encrypted") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkcs8-encrypted"),
    };

/// `application/pkcs12`
pub const APPLICATION_PKCS12: Essence<&'static str> = match Essence::new_const("application/pkcs12")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/pkcs12"),
};

/// `application/pkix-attr-cert`
pub const APPLICATION_PKIX_ATTR_CERT: Essence<&'static str> =
    match Essence::new_const("application/pkix-attr-cert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkix-attr-cert"),
    };

/// `application/pkix-cert`
pub const APPLICATION_PKIX_CERT: Essence<&'static str> =
    match Essence::new_const("application/pkix-cert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkix-cert"),
    };

/// `application/pkix-crl`
pub const APPLICATION_PKIX_CRL: Essence<&'static str> =
    match Essence::new_const("application/pkix-crl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkix-crl"),
    };

/// `application/pkix-pkipath`
pub const APPLICATION_PKIX_PKIPATH: Essence<&'static str> =
    match Essence::new_const("application/pkix-pkipath") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkix-pkipath"),
    };

/// `application/pkixcmp`
pub const APPLICATION_PKIXCMP: Essence<&'static str> =
    match Essence::new_const("application/pkixcmp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pkixcmp"),
    };

/// `application/pls+xml`
pub const APPLICATION_PLS_XML: Essence<&'static str> =
    match Essence::new_const("application/pls+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pls+xml"),
    };

/// `application/poc-settings+xml`
pub const APPLICATION_POC_SETTINGS_XML: Essence<&'static str> =
    match Essence::new_const("application/poc-settings+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/poc-settings+xml"),
    };

/// `application/postscript`
pub const APPLICATION_POSTSCRIPT: Essence<&'static str> =
    match Essence::new_const("application/postscript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/postscript"),
    };

/// `application/ppsp-tracker+json`
pub const APPLICATION_PPSP_TRACKER_JSON: Essence<&'static str> =
    match Essence::new_const("application/ppsp-tracker+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ppsp-tracker+json"),
    };

/// `application/problem+json`
pub const APPLICATION_PROBLEM_JSON: Essence<&'static str> =
    match Essence::new_const("application/problem+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/problem+json"),
    };

/// `application/problem+xml`
pub const APPLICATION_PROBLEM_XML: Essence<&'static str> =
    match Essence::new_const("application/problem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/problem+xml"),
    };

/// `application/provenance+xml`
pub const APPLICATION_PROVENANCE_XML: Essence<&'static str> =
    match Essence::new_const("application/provenance+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/provenance+xml"),
    };

/// `application/prs.alvestrand.titrax-sheet`
pub const APPLICATION_PRS_ALVESTRAND_TITRAX_SHEET: Essence<&'static str> =
    match Essence::new_const("application/prs.alvestrand.titrax-sheet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.alvestrand.titrax-sheet"),
    };

/// `application/prs.cww`
pub const APPLICATION_PRS_CWW: Essence<&'static str> =
    match Essence::new_const("application/prs.cww") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.cww"),
    };

/// `application/prs.cyn`
pub const APPLICATION_PRS_CYN: Essence<&'static str> =
    match Essence::new_const("application/prs.cyn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.cyn"),
    };

/// `application/prs.hpub+zip`
pub const APPLICATION_PRS_HPUB_ZIP: Essence<&'static str> =
    match Essence::new_const("application/prs.hpub+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.hpub+zip"),
    };

/// `application/prs.nprend`
pub const APPLICATION_PRS_NPREND: Essence<&'static str> =
    match Essence::new_const("application/prs.nprend") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.nprend"),
    };

/// `application/prs.plucker`
pub const APPLICATION_PRS_PLUCKER: Essence<&'static str> =
    match Essence::new_const("application/prs.plucker") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.plucker"),
    };

/// `application/prs.rdf-xml-crypt`
pub const APPLICATION_PRS_RDF_XML_CRYPT: Essence<&'static str> =
    match Essence::new_const("application/prs.rdf-xml-crypt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.rdf-xml-crypt"),
    };

/// `application/prs.xsf+xml`
pub const APPLICATION_PRS_XSF_XML: Essence<&'static str> =
    match Essence::new_const("application/prs.xsf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/prs.xsf+xml"),
    };

/// `application/pskc+xml`
pub const APPLICATION_PSKC_XML: Essence<&'static str> =
    match Essence::new_const("application/pskc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pskc+xml"),
    };

/// `application/pvd+json`
pub const APPLICATION_PVD_JSON: Essence<&'static str> =
    match Essence::new_const("application/pvd+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/pvd+json"),
    };

/// `application/rdf+xml`
pub const APPLICATION_RDF_XML: Essence<&'static str> =
    match Essence::new_const("application/rdf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rdf+xml"),
    };

/// `application/route-apd+xml`
pub const APPLICATION_ROUTE_APD_XML: Essence<&'static str> =
    match Essence::new_const("application/route-apd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/route-apd+xml"),
    };

/// `application/route-s-tsid+xml`
pub const APPLICATION_ROUTE_S_TSID_XML: Essence<&'static str> =
    match Essence::new_const("application/route-s-tsid+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/route-s-tsid+xml"),
    };

/// `application/route-usd+xml`
pub const APPLICATION_ROUTE_USD_XML: Essence<&'static str> =
    match Essence::new_const("application/route-usd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/route-usd+xml"),
    };

/// `application/QSIG`
pub const APPLICATION_QSIG: Essence<&'static str> = match Essence::new_const("application/QSIG") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/QSIG"),
};

/// `application/raptorfec`
pub const APPLICATION_RAPTORFEC: Essence<&'static str> =
    match Essence::new_const("application/raptorfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/raptorfec"),
    };

/// `application/rdap+json`
pub const APPLICATION_RDAP_JSON: Essence<&'static str> =
    match Essence::new_const("application/rdap+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rdap+json"),
    };

/// `application/reginfo+xml`
pub const APPLICATION_REGINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/reginfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/reginfo+xml"),
    };

/// `application/relax-ng-compact-syntax`
pub const APPLICATION_RELAX_NG_COMPACT_SYNTAX: Essence<&'static str> =
    match Essence::new_const("application/relax-ng-compact-syntax") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/relax-ng-compact-syntax"),
    };

/// `application/remote-printing`
pub const APPLICATION_REMOTE_PRINTING: Essence<&'static str> =
    match Essence::new_const("application/remote-printing") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/remote-printing"),
    };

/// `application/reputon+json`
pub const APPLICATION_REPUTON_JSON: Essence<&'static str> =
    match Essence::new_const("application/reputon+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/reputon+json"),
    };

/// `application/resource-lists-diff+xml`
pub const APPLICATION_RESOURCE_LISTS_DIFF_XML: Essence<&'static str> =
    match Essence::new_const("application/resource-lists-diff+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/resource-lists-diff+xml"),
    };

/// `application/resource-lists+xml`
pub const APPLICATION_RESOURCE_LISTS_XML: Essence<&'static str> =
    match Essence::new_const("application/resource-lists+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/resource-lists+xml"),
    };

/// `application/rfc+xml`
pub const APPLICATION_RFC_XML: Essence<&'static str> =
    match Essence::new_const("application/rfc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rfc+xml"),
    };

/// `application/riscos`
pub const APPLICATION_RISCOS: Essence<&'static str> = match Essence::new_const("application/riscos")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/riscos"),
};

/// `application/rlmi+xml`
pub const APPLICATION_RLMI_XML: Essence<&'static str> =
    match Essence::new_const("application/rlmi+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rlmi+xml"),
    };

/// `application/rls-services+xml`
pub const APPLICATION_RLS_SERVICES_XML: Essence<&'static str> =
    match Essence::new_const("application/rls-services+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rls-services+xml"),
    };

/// `application/rpki-ghostbusters`
pub const APPLICATION_RPKI_GHOSTBUSTERS: Essence<&'static str> =
    match Essence::new_const("application/rpki-ghostbusters") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rpki-ghostbusters"),
    };

/// `application/rpki-manifest`
pub const APPLICATION_RPKI_MANIFEST: Essence<&'static str> =
    match Essence::new_const("application/rpki-manifest") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rpki-manifest"),
    };

/// `application/rpki-publication`
pub const APPLICATION_RPKI_PUBLICATION: Essence<&'static str> =
    match Essence::new_const("application/rpki-publication") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rpki-publication"),
    };

/// `application/rpki-roa`
pub const APPLICATION_RPKI_ROA: Essence<&'static str> =
    match Essence::new_const("application/rpki-roa") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rpki-roa"),
    };

/// `application/rpki-updown`
pub const APPLICATION_RPKI_UPDOWN: Essence<&'static str> =
    match Essence::new_const("application/rpki-updown") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rpki-updown"),
    };

/// `application/rtf`
pub const APPLICATION_RTF: Essence<&'static str> = match Essence::new_const("application/rtf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/rtf"),
};

/// `application/rtploopback`
pub const APPLICATION_RTPLOOPBACK: Essence<&'static str> =
    match Essence::new_const("application/rtploopback") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/rtploopback"),
    };

/// `application/rtx`
pub const APPLICATION_RTX: Essence<&'static str> = match Essence::new_const("application/rtx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/rtx"),
};

/// `application/samlassertion+xml`
pub const APPLICATION_SAMLASSERTION_XML: Essence<&'static str> =
    match Essence::new_const("application/samlassertion+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/samlassertion+xml"),
    };

/// `application/samlmetadata+xml`
pub const APPLICATION_SAMLMETADATA_XML: Essence<&'static str> =
    match Essence::new_const("application/samlmetadata+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/samlmetadata+xml"),
    };

/// `application/sarif-external-properties+json`
pub const APPLICATION_SARIF_EXTERNAL_PROPERTIES_JSON: Essence<&'static str> =
    match Essence::new_const("application/sarif-external-properties+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sarif-external-properties+json"),
    };

/// `application/sarif+json`
pub const APPLICATION_SARIF_JSON: Essence<&'static str> =
    match Essence::new_const("application/sarif+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sarif+json"),
    };

/// `application/sbe`
pub const APPLICATION_SBE: Essence<&'static str> = match Essence::new_const("application/sbe") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/sbe"),
};

/// `application/sbml+xml`
pub const APPLICATION_SBML_XML: Essence<&'static str> =
    match Essence::new_const("application/sbml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sbml+xml"),
    };

/// `application/scaip+xml`
pub const APPLICATION_SCAIP_XML: Essence<&'static str> =
    match Essence::new_const("application/scaip+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scaip+xml"),
    };

/// `application/scim+json`
pub const APPLICATION_SCIM_JSON: Essence<&'static str> =
    match Essence::new_const("application/scim+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scim+json"),
    };

/// `application/scvp-cv-request`
pub const APPLICATION_SCVP_CV_REQUEST: Essence<&'static str> =
    match Essence::new_const("application/scvp-cv-request") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scvp-cv-request"),
    };

/// `application/scvp-cv-response`
pub const APPLICATION_SCVP_CV_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/scvp-cv-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scvp-cv-response"),
    };

/// `application/scvp-vp-request`
pub const APPLICATION_SCVP_VP_REQUEST: Essence<&'static str> =
    match Essence::new_const("application/scvp-vp-request") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scvp-vp-request"),
    };

/// `application/scvp-vp-response`
pub const APPLICATION_SCVP_VP_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/scvp-vp-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/scvp-vp-response"),
    };

/// `application/sdp`
pub const APPLICATION_SDP: Essence<&'static str> = match Essence::new_const("application/sdp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/sdp"),
};

/// `application/secevent+jwt`
pub const APPLICATION_SECEVENT_JWT: Essence<&'static str> =
    match Essence::new_const("application/secevent+jwt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/secevent+jwt"),
    };

/// `application/senml-etch+cbor`
pub const APPLICATION_SENML_ETCH_CBOR: Essence<&'static str> =
    match Essence::new_const("application/senml-etch+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml-etch+cbor"),
    };

/// `application/senml-etch+json`
pub const APPLICATION_SENML_ETCH_JSON: Essence<&'static str> =
    match Essence::new_const("application/senml-etch+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml-etch+json"),
    };

/// `application/senml-exi`
pub const APPLICATION_SENML_EXI: Essence<&'static str> =
    match Essence::new_const("application/senml-exi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml-exi"),
    };

/// `application/senml+cbor`
pub const APPLICATION_SENML_CBOR: Essence<&'static str> =
    match Essence::new_const("application/senml+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml+cbor"),
    };

/// `application/senml+json`
pub const APPLICATION_SENML_JSON: Essence<&'static str> =
    match Essence::new_const("application/senml+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml+json"),
    };

/// `application/senml+xml`
pub const APPLICATION_SENML_XML: Essence<&'static str> =
    match Essence::new_const("application/senml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/senml+xml"),
    };

/// `application/sensml-exi`
pub const APPLICATION_SENSML_EXI: Essence<&'static str> =
    match Essence::new_const("application/sensml-exi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sensml-exi"),
    };

/// `application/sensml+cbor`
pub const APPLICATION_SENSML_CBOR: Essence<&'static str> =
    match Essence::new_const("application/sensml+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sensml+cbor"),
    };

/// `application/sensml+json`
pub const APPLICATION_SENSML_JSON: Essence<&'static str> =
    match Essence::new_const("application/sensml+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sensml+json"),
    };

/// `application/sensml+xml`
pub const APPLICATION_SENSML_XML: Essence<&'static str> =
    match Essence::new_const("application/sensml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sensml+xml"),
    };

/// `application/sep-exi`
pub const APPLICATION_SEP_EXI: Essence<&'static str> =
    match Essence::new_const("application/sep-exi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sep-exi"),
    };

/// `application/sep+xml`
pub const APPLICATION_SEP_XML: Essence<&'static str> =
    match Essence::new_const("application/sep+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sep+xml"),
    };

/// `application/session-info`
pub const APPLICATION_SESSION_INFO: Essence<&'static str> =
    match Essence::new_const("application/session-info") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/session-info"),
    };

/// `application/set-payment`
pub const APPLICATION_SET_PAYMENT: Essence<&'static str> =
    match Essence::new_const("application/set-payment") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/set-payment"),
    };

/// `application/set-payment-initiation`
pub const APPLICATION_SET_PAYMENT_INITIATION: Essence<&'static str> =
    match Essence::new_const("application/set-payment-initiation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/set-payment-initiation"),
    };

/// `application/set-registration`
pub const APPLICATION_SET_REGISTRATION: Essence<&'static str> =
    match Essence::new_const("application/set-registration") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/set-registration"),
    };

/// `application/set-registration-initiation`
pub const APPLICATION_SET_REGISTRATION_INITIATION: Essence<&'static str> =
    match Essence::new_const("application/set-registration-initiation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/set-registration-initiation"),
    };

/// `application/SGML`
pub const APPLICATION_SGML: Essence<&'static str> = match Essence::new_const("application/SGML") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/SGML"),
};

/// `application/sgml-open-catalog`
pub const APPLICATION_SGML_OPEN_CATALOG: Essence<&'static str> =
    match Essence::new_const("application/sgml-open-catalog") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sgml-open-catalog"),
    };

/// `application/shf+xml`
pub const APPLICATION_SHF_XML: Essence<&'static str> =
    match Essence::new_const("application/shf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/shf+xml"),
    };

/// `application/sieve`
pub const APPLICATION_SIEVE: Essence<&'static str> = match Essence::new_const("application/sieve") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/sieve"),
};

/// `application/simple-filter+xml`
pub const APPLICATION_SIMPLE_FILTER_XML: Essence<&'static str> =
    match Essence::new_const("application/simple-filter+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/simple-filter+xml"),
    };

/// `application/simple-message-summary`
pub const APPLICATION_SIMPLE_MESSAGE_SUMMARY: Essence<&'static str> =
    match Essence::new_const("application/simple-message-summary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/simple-message-summary"),
    };

/// `application/simpleSymbolContainer`
pub const APPLICATION_SIMPLESYMBOLCONTAINER: Essence<&'static str> =
    match Essence::new_const("application/simpleSymbolContainer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/simpleSymbolContainer"),
    };

/// `application/sipc`
pub const APPLICATION_SIPC: Essence<&'static str> = match Essence::new_const("application/sipc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/sipc"),
};

/// `application/slate`
pub const APPLICATION_SLATE: Essence<&'static str> = match Essence::new_const("application/slate") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/slate"),
};

/// `application/smil`
pub const APPLICATION_SMIL: Essence<&'static str> = match Essence::new_const("application/smil") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/smil"),
};

/// `application/smil+xml`
pub const APPLICATION_SMIL_XML: Essence<&'static str> =
    match Essence::new_const("application/smil+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/smil+xml"),
    };

/// `application/smpte336m`
pub const APPLICATION_SMPTE336M: Essence<&'static str> =
    match Essence::new_const("application/smpte336m") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/smpte336m"),
    };

/// `application/soap+fastinfoset`
pub const APPLICATION_SOAP_FASTINFOSET: Essence<&'static str> =
    match Essence::new_const("application/soap+fastinfoset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/soap+fastinfoset"),
    };

/// `application/soap+xml`
pub const APPLICATION_SOAP_XML: Essence<&'static str> =
    match Essence::new_const("application/soap+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/soap+xml"),
    };

/// `application/sparql-query`
pub const APPLICATION_SPARQL_QUERY: Essence<&'static str> =
    match Essence::new_const("application/sparql-query") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sparql-query"),
    };

/// `application/spdx+json`
pub const APPLICATION_SPDX_JSON: Essence<&'static str> =
    match Essence::new_const("application/spdx+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/spdx+json"),
    };

/// `application/sparql-results+xml`
pub const APPLICATION_SPARQL_RESULTS_XML: Essence<&'static str> =
    match Essence::new_const("application/sparql-results+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sparql-results+xml"),
    };

/// `application/spirits-event+xml`
pub const APPLICATION_SPIRITS_EVENT_XML: Essence<&'static str> =
    match Essence::new_const("application/spirits-event+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/spirits-event+xml"),
    };

/// `application/sql`
pub const APPLICATION_SQL: Essence<&'static str> = match Essence::new_const("application/sql") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/sql"),
};

/// `application/srgs`
pub const APPLICATION_SRGS: Essence<&'static str> = match Essence::new_const("application/srgs") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/srgs"),
};

/// `application/srgs+xml`
pub const APPLICATION_SRGS_XML: Essence<&'static str> =
    match Essence::new_const("application/srgs+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/srgs+xml"),
    };

/// `application/sru+xml`
pub const APPLICATION_SRU_XML: Essence<&'static str> =
    match Essence::new_const("application/sru+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/sru+xml"),
    };

/// `application/ssml+xml`
pub const APPLICATION_SSML_XML: Essence<&'static str> =
    match Essence::new_const("application/ssml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ssml+xml"),
    };

/// `application/stix+json`
pub const APPLICATION_STIX_JSON: Essence<&'static str> =
    match Essence::new_const("application/stix+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/stix+json"),
    };

/// `application/swid+xml`
pub const APPLICATION_SWID_XML: Essence<&'static str> =
    match Essence::new_const("application/swid+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/swid+xml"),
    };

/// `application/tamp-apex-update`
pub const APPLICATION_TAMP_APEX_UPDATE: Essence<&'static str> =
    match Essence::new_const("application/tamp-apex-update") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-apex-update"),
    };

/// `application/tamp-apex-update-confirm`
pub const APPLICATION_TAMP_APEX_UPDATE_CONFIRM: Essence<&'static str> =
    match Essence::new_const("application/tamp-apex-update-confirm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-apex-update-confirm"),
    };

/// `application/tamp-community-update`
pub const APPLICATION_TAMP_COMMUNITY_UPDATE: Essence<&'static str> =
    match Essence::new_const("application/tamp-community-update") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-community-update"),
    };

/// `application/tamp-community-update-confirm`
pub const APPLICATION_TAMP_COMMUNITY_UPDATE_CONFIRM: Essence<&'static str> =
    match Essence::new_const("application/tamp-community-update-confirm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-community-update-confirm"),
    };

/// `application/tamp-error`
pub const APPLICATION_TAMP_ERROR: Essence<&'static str> =
    match Essence::new_const("application/tamp-error") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-error"),
    };

/// `application/tamp-sequence-adjust`
pub const APPLICATION_TAMP_SEQUENCE_ADJUST: Essence<&'static str> =
    match Essence::new_const("application/tamp-sequence-adjust") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-sequence-adjust"),
    };

/// `application/tamp-sequence-adjust-confirm`
pub const APPLICATION_TAMP_SEQUENCE_ADJUST_CONFIRM: Essence<&'static str> =
    match Essence::new_const("application/tamp-sequence-adjust-confirm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-sequence-adjust-confirm"),
    };

/// `application/tamp-status-query`
pub const APPLICATION_TAMP_STATUS_QUERY: Essence<&'static str> =
    match Essence::new_const("application/tamp-status-query") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-status-query"),
    };

/// `application/tamp-status-response`
pub const APPLICATION_TAMP_STATUS_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/tamp-status-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-status-response"),
    };

/// `application/tamp-update`
pub const APPLICATION_TAMP_UPDATE: Essence<&'static str> =
    match Essence::new_const("application/tamp-update") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-update"),
    };

/// `application/tamp-update-confirm`
pub const APPLICATION_TAMP_UPDATE_CONFIRM: Essence<&'static str> =
    match Essence::new_const("application/tamp-update-confirm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tamp-update-confirm"),
    };

/// `application/taxii+json`
pub const APPLICATION_TAXII_JSON: Essence<&'static str> =
    match Essence::new_const("application/taxii+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/taxii+json"),
    };

/// `application/td+json`
pub const APPLICATION_TD_JSON: Essence<&'static str> =
    match Essence::new_const("application/td+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/td+json"),
    };

/// `application/tei+xml`
pub const APPLICATION_TEI_XML: Essence<&'static str> =
    match Essence::new_const("application/tei+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tei+xml"),
    };

/// `application/TETRA_ISI`
pub const APPLICATION_TETRA_ISI: Essence<&'static str> =
    match Essence::new_const("application/TETRA_ISI") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/TETRA_ISI"),
    };

/// `application/thraud+xml`
pub const APPLICATION_THRAUD_XML: Essence<&'static str> =
    match Essence::new_const("application/thraud+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/thraud+xml"),
    };

/// `application/timestamp-query`
pub const APPLICATION_TIMESTAMP_QUERY: Essence<&'static str> =
    match Essence::new_const("application/timestamp-query") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/timestamp-query"),
    };

/// `application/timestamp-reply`
pub const APPLICATION_TIMESTAMP_REPLY: Essence<&'static str> =
    match Essence::new_const("application/timestamp-reply") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/timestamp-reply"),
    };

/// `application/timestamped-data`
pub const APPLICATION_TIMESTAMPED_DATA: Essence<&'static str> =
    match Essence::new_const("application/timestamped-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/timestamped-data"),
    };

/// `application/tlsrpt+gzip`
pub const APPLICATION_TLSRPT_GZIP: Essence<&'static str> =
    match Essence::new_const("application/tlsrpt+gzip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tlsrpt+gzip"),
    };

/// `application/tlsrpt+json`
pub const APPLICATION_TLSRPT_JSON: Essence<&'static str> =
    match Essence::new_const("application/tlsrpt+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tlsrpt+json"),
    };

/// `application/tnauthlist`
pub const APPLICATION_TNAUTHLIST: Essence<&'static str> =
    match Essence::new_const("application/tnauthlist") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tnauthlist"),
    };

/// `application/token-introspection+jwt`
pub const APPLICATION_TOKEN_INTROSPECTION_JWT: Essence<&'static str> =
    match Essence::new_const("application/token-introspection+jwt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/token-introspection+jwt"),
    };

/// `application/trickle-ice-sdpfrag`
pub const APPLICATION_TRICKLE_ICE_SDPFRAG: Essence<&'static str> =
    match Essence::new_const("application/trickle-ice-sdpfrag") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/trickle-ice-sdpfrag"),
    };

/// `application/trig`
pub const APPLICATION_TRIG: Essence<&'static str> = match Essence::new_const("application/trig") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/trig"),
};

/// `application/ttml+xml`
pub const APPLICATION_TTML_XML: Essence<&'static str> =
    match Essence::new_const("application/ttml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/ttml+xml"),
    };

/// `application/tve-trigger`
pub const APPLICATION_TVE_TRIGGER: Essence<&'static str> =
    match Essence::new_const("application/tve-trigger") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tve-trigger"),
    };

/// `application/tzif`
pub const APPLICATION_TZIF: Essence<&'static str> = match Essence::new_const("application/tzif") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/tzif"),
};

/// `application/tzif-leap`
pub const APPLICATION_TZIF_LEAP: Essence<&'static str> =
    match Essence::new_const("application/tzif-leap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/tzif-leap"),
    };

/// `application/ulpfec`
pub const APPLICATION_ULPFEC: Essence<&'static str> = match Essence::new_const("application/ulpfec")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/ulpfec"),
};

/// `application/urc-grpsheet+xml`
pub const APPLICATION_URC_GRPSHEET_XML: Essence<&'static str> =
    match Essence::new_const("application/urc-grpsheet+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/urc-grpsheet+xml"),
    };

/// `application/urc-ressheet+xml`
pub const APPLICATION_URC_RESSHEET_XML: Essence<&'static str> =
    match Essence::new_const("application/urc-ressheet+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/urc-ressheet+xml"),
    };

/// `application/urc-targetdesc+xml`
pub const APPLICATION_URC_TARGETDESC_XML: Essence<&'static str> =
    match Essence::new_const("application/urc-targetdesc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/urc-targetdesc+xml"),
    };

/// `application/urc-uisocketdesc+xml`
pub const APPLICATION_URC_UISOCKETDESC_XML: Essence<&'static str> =
    match Essence::new_const("application/urc-uisocketdesc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/urc-uisocketdesc+xml"),
    };

/// `application/vcard+json`
pub const APPLICATION_VCARD_JSON: Essence<&'static str> =
    match Essence::new_const("application/vcard+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vcard+json"),
    };

/// `application/vcard+xml`
pub const APPLICATION_VCARD_XML: Essence<&'static str> =
    match Essence::new_const("application/vcard+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vcard+xml"),
    };

/// `application/vemmi`
pub const APPLICATION_VEMMI: Essence<&'static str> = match Essence::new_const("application/vemmi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vemmi"),
};

/// `application/vnd.1000minds.decision-model+xml`
pub const APPLICATION_VND_1000MINDS_DECISION_MODEL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.1000minds.decision-model+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.1000minds.decision-model+xml"),
    };

/// `application/vnd.3gpp.5gnas`
pub const APPLICATION_VND_3GPP_5GNAS: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.5gnas") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.5gnas"),
    };

/// `application/vnd.3gpp.access-transfer-events+xml`
pub const APPLICATION_VND_3GPP_ACCESS_TRANSFER_EVENTS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.access-transfer-events+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.access-transfer-events+xml"),
    };

/// `application/vnd.3gpp.bsf+xml`
pub const APPLICATION_VND_3GPP_BSF_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.bsf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.bsf+xml"),
    };

/// `application/vnd.3gpp.GMOP+xml`
pub const APPLICATION_VND_3GPP_GMOP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.GMOP+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.GMOP+xml"),
    };

/// `application/vnd.3gpp.gtpc`
pub const APPLICATION_VND_3GPP_GTPC: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.gtpc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.gtpc"),
    };

/// `application/vnd.3gpp.interworking-data`
pub const APPLICATION_VND_3GPP_INTERWORKING_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.interworking-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.interworking-data"),
    };

/// `application/vnd.3gpp.lpp`
pub const APPLICATION_VND_3GPP_LPP: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.lpp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.lpp"),
    };

/// `application/vnd.3gpp.mc-signalling-ear`
pub const APPLICATION_VND_3GPP_MC_SIGNALLING_EAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mc-signalling-ear") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mc-signalling-ear"),
    };

/// `application/vnd.3gpp.mcdata-affiliation-command+xml`
pub const APPLICATION_VND_3GPP_MCDATA_AFFILIATION_COMMAND_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-affiliation-command+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-affiliation-command+xml"),
    };

/// `application/vnd.3gpp.mcdata-info+xml`
pub const APPLICATION_VND_3GPP_MCDATA_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-info+xml"),
    };

/// `application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml`
pub const APPLICATION_VND_3GPP_MCDATA_MSGSTORE_CTRL_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml"),
    };

/// `application/vnd.3gpp.mcdata-payload`
pub const APPLICATION_VND_3GPP_MCDATA_PAYLOAD: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-payload") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-payload"),
    };

/// `application/vnd.3gpp.mcdata-regroup+xml`
pub const APPLICATION_VND_3GPP_MCDATA_REGROUP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-regroup+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-regroup+xml"),
    };

/// `application/vnd.3gpp.mcdata-service-config+xml`
pub const APPLICATION_VND_3GPP_MCDATA_SERVICE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-service-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-service-config+xml"),
    };

/// `application/vnd.3gpp.mcdata-signalling`
pub const APPLICATION_VND_3GPP_MCDATA_SIGNALLING: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-signalling") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-signalling"),
    };

/// `application/vnd.3gpp.mcdata-ue-config+xml`
pub const APPLICATION_VND_3GPP_MCDATA_UE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-ue-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-ue-config+xml"),
    };

/// `application/vnd.3gpp.mcdata-user-profile+xml`
pub const APPLICATION_VND_3GPP_MCDATA_USER_PROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcdata-user-profile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcdata-user-profile+xml"),
    };

/// `application/vnd.3gpp.mcptt-affiliation-command+xml`
pub const APPLICATION_VND_3GPP_MCPTT_AFFILIATION_COMMAND_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-affiliation-command+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-affiliation-command+xml"),
    };

/// `application/vnd.3gpp.mcptt-floor-request+xml`
pub const APPLICATION_VND_3GPP_MCPTT_FLOOR_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-floor-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-floor-request+xml"),
    };

/// `application/vnd.3gpp.mcptt-info+xml`
pub const APPLICATION_VND_3GPP_MCPTT_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-info+xml"),
    };

/// `application/vnd.3gpp.mcptt-location-info+xml`
pub const APPLICATION_VND_3GPP_MCPTT_LOCATION_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-location-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-location-info+xml"),
    };

/// `application/vnd.3gpp.mcptt-mbms-usage-info+xml`
pub const APPLICATION_VND_3GPP_MCPTT_MBMS_USAGE_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-mbms-usage-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-mbms-usage-info+xml"),
    };

/// `application/vnd.3gpp.mcptt-service-config+xml`
pub const APPLICATION_VND_3GPP_MCPTT_SERVICE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-service-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-service-config+xml"),
    };

/// `application/vnd.3gpp.mcptt-signed+xml`
pub const APPLICATION_VND_3GPP_MCPTT_SIGNED_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-signed+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-signed+xml"),
    };

/// `application/vnd.3gpp.mcptt-ue-config+xml`
pub const APPLICATION_VND_3GPP_MCPTT_UE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-ue-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-ue-config+xml"),
    };

/// `application/vnd.3gpp.mcptt-ue-init-config+xml`
pub const APPLICATION_VND_3GPP_MCPTT_UE_INIT_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-ue-init-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-ue-init-config+xml"),
    };

/// `application/vnd.3gpp.mcptt-user-profile+xml`
pub const APPLICATION_VND_3GPP_MCPTT_USER_PROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcptt-user-profile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcptt-user-profile+xml"),
    };

/// `application/vnd.3gpp.mcvideo-affiliation-command+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_AFFILIATION_COMMAND_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-affiliation-command+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-affiliation-command+xml"),
    };

/// `application/vnd.3gpp.mcvideo-affiliation-info+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_AFFILIATION_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-affiliation-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-affiliation-info+xml"),
    };

/// `application/vnd.3gpp.mcvideo-info+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-info+xml"),
    };

/// `application/vnd.3gpp.mcvideo-location-info+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_LOCATION_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-location-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-location-info+xml"),
    };

/// `application/vnd.3gpp.mcvideo-mbms-usage-info+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_MBMS_USAGE_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-mbms-usage-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-mbms-usage-info+xml"),
    };

/// `application/vnd.3gpp.mcvideo-service-config+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_SERVICE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-service-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-service-config+xml"),
    };

/// `application/vnd.3gpp.mcvideo-transmission-request+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_TRANSMISSION_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-transmission-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-transmission-request+xml"),
    };

/// `application/vnd.3gpp.mcvideo-ue-config+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_UE_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-ue-config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-ue-config+xml"),
    };

/// `application/vnd.3gpp.mcvideo-user-profile+xml`
pub const APPLICATION_VND_3GPP_MCVIDEO_USER_PROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mcvideo-user-profile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mcvideo-user-profile+xml"),
    };

/// `application/vnd.3gpp.mid-call+xml`
pub const APPLICATION_VND_3GPP_MID_CALL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.mid-call+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.mid-call+xml"),
    };

/// `application/vnd.3gpp.ngap`
pub const APPLICATION_VND_3GPP_NGAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.ngap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.ngap"),
    };

/// `application/vnd.3gpp.pfcp`
pub const APPLICATION_VND_3GPP_PFCP: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.pfcp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.pfcp"),
    };

/// `application/vnd.3gpp.pic-bw-large`
pub const APPLICATION_VND_3GPP_PIC_BW_LARGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.pic-bw-large") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.pic-bw-large"),
    };

/// `application/vnd.3gpp.pic-bw-small`
pub const APPLICATION_VND_3GPP_PIC_BW_SMALL: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.pic-bw-small") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.pic-bw-small"),
    };

/// `application/vnd.3gpp.pic-bw-var`
pub const APPLICATION_VND_3GPP_PIC_BW_VAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.pic-bw-var") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.pic-bw-var"),
    };

/// `application/vnd.3gpp-prose-pc3ch+xml`
pub const APPLICATION_VND_3GPP_PROSE_PC3CH_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp-prose-pc3ch+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp-prose-pc3ch+xml"),
    };

/// `application/vnd.3gpp-prose+xml`
pub const APPLICATION_VND_3GPP_PROSE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp-prose+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp-prose+xml"),
    };

/// `application/vnd.3gpp.s1ap`
pub const APPLICATION_VND_3GPP_S1AP: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.s1ap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.s1ap"),
    };

/// `application/vnd.3gpp.sms`
pub const APPLICATION_VND_3GPP_SMS: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.sms") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.sms"),
    };

/// `application/vnd.3gpp.sms+xml`
pub const APPLICATION_VND_3GPP_SMS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.sms+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.sms+xml"),
    };

/// `application/vnd.3gpp.srvcc-ext+xml`
pub const APPLICATION_VND_3GPP_SRVCC_EXT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.srvcc-ext+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.srvcc-ext+xml"),
    };

/// `application/vnd.3gpp.SRVCC-info+xml`
pub const APPLICATION_VND_3GPP_SRVCC_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.SRVCC-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.SRVCC-info+xml"),
    };

/// `application/vnd.3gpp.state-and-event-info+xml`
pub const APPLICATION_VND_3GPP_STATE_AND_EVENT_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.state-and-event-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.state-and-event-info+xml"),
    };

/// `application/vnd.3gpp.ussd+xml`
pub const APPLICATION_VND_3GPP_USSD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp.ussd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp.ussd+xml"),
    };

/// `application/vnd.3gpp-v2x-local-service-information`
pub const APPLICATION_VND_3GPP_V2X_LOCAL_SERVICE_INFORMATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp-v2x-local-service-information") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp-v2x-local-service-information"),
    };

/// `application/vnd.3gpp2.bcmcsinfo+xml`
pub const APPLICATION_VND_3GPP2_BCMCSINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp2.bcmcsinfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp2.bcmcsinfo+xml"),
    };

/// `application/vnd.3gpp2.sms`
pub const APPLICATION_VND_3GPP2_SMS: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp2.sms") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp2.sms"),
    };

/// `application/vnd.3gpp2.tcap`
pub const APPLICATION_VND_3GPP2_TCAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.3gpp2.tcap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3gpp2.tcap"),
    };

/// `application/vnd.3lightssoftware.imagescal`
pub const APPLICATION_VND_3LIGHTSSOFTWARE_IMAGESCAL: Essence<&'static str> =
    match Essence::new_const("application/vnd.3lightssoftware.imagescal") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3lightssoftware.imagescal"),
    };

/// `application/vnd.3M.Post-it-Notes`
pub const APPLICATION_VND_3M_POST_IT_NOTES: Essence<&'static str> =
    match Essence::new_const("application/vnd.3M.Post-it-Notes") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.3M.Post-it-Notes"),
    };

/// `application/vnd.accpac.simply.aso`
pub const APPLICATION_VND_ACCPAC_SIMPLY_ASO: Essence<&'static str> =
    match Essence::new_const("application/vnd.accpac.simply.aso") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.accpac.simply.aso"),
    };

/// `application/vnd.accpac.simply.imp`
pub const APPLICATION_VND_ACCPAC_SIMPLY_IMP: Essence<&'static str> =
    match Essence::new_const("application/vnd.accpac.simply.imp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.accpac.simply.imp"),
    };

/// `application/vnd.acucobol`
pub const APPLICATION_VND_ACUCOBOL: Essence<&'static str> =
    match Essence::new_const("application/vnd.acucobol") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.acucobol"),
    };

/// `application/vnd.acucorp`
pub const APPLICATION_VND_ACUCORP: Essence<&'static str> =
    match Essence::new_const("application/vnd.acucorp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.acucorp"),
    };

/// `application/vnd.adobe.flash.movie`
pub const APPLICATION_VND_ADOBE_FLASH_MOVIE: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.flash.movie") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.flash.movie"),
    };

/// `application/vnd.adobe.formscentral.fcdt`
pub const APPLICATION_VND_ADOBE_FORMSCENTRAL_FCDT: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.formscentral.fcdt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.formscentral.fcdt"),
    };

/// `application/vnd.adobe.fxp`
pub const APPLICATION_VND_ADOBE_FXP: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.fxp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.fxp"),
    };

/// `application/vnd.adobe.partial-upload`
pub const APPLICATION_VND_ADOBE_PARTIAL_UPLOAD: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.partial-upload") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.partial-upload"),
    };

/// `application/vnd.adobe.xdp+xml`
pub const APPLICATION_VND_ADOBE_XDP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.xdp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.xdp+xml"),
    };

/// `application/vnd.adobe.xfdf`
pub const APPLICATION_VND_ADOBE_XFDF: Essence<&'static str> =
    match Essence::new_const("application/vnd.adobe.xfdf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.adobe.xfdf"),
    };

/// `application/vnd.aether.imp`
pub const APPLICATION_VND_AETHER_IMP: Essence<&'static str> =
    match Essence::new_const("application/vnd.aether.imp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.aether.imp"),
    };

/// `application/vnd.afpc.afplinedata`
pub const APPLICATION_VND_AFPC_AFPLINEDATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.afplinedata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.afplinedata"),
    };

/// `application/vnd.afpc.afplinedata-pagedef`
pub const APPLICATION_VND_AFPC_AFPLINEDATA_PAGEDEF: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.afplinedata-pagedef") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.afplinedata-pagedef"),
    };

/// `application/vnd.afpc.cmoca-cmresource`
pub const APPLICATION_VND_AFPC_CMOCA_CMRESOURCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.cmoca-cmresource") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.cmoca-cmresource"),
    };

/// `application/vnd.afpc.foca-charset`
pub const APPLICATION_VND_AFPC_FOCA_CHARSET: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.foca-charset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.foca-charset"),
    };

/// `application/vnd.afpc.foca-codedfont`
pub const APPLICATION_VND_AFPC_FOCA_CODEDFONT: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.foca-codedfont") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.foca-codedfont"),
    };

/// `application/vnd.afpc.foca-codepage`
pub const APPLICATION_VND_AFPC_FOCA_CODEPAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.foca-codepage") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.foca-codepage"),
    };

/// `application/vnd.afpc.modca`
pub const APPLICATION_VND_AFPC_MODCA: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca"),
    };

/// `application/vnd.afpc.modca-cmtable`
pub const APPLICATION_VND_AFPC_MODCA_CMTABLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-cmtable") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-cmtable"),
    };

/// `application/vnd.afpc.modca-formdef`
pub const APPLICATION_VND_AFPC_MODCA_FORMDEF: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-formdef") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-formdef"),
    };

/// `application/vnd.afpc.modca-mediummap`
pub const APPLICATION_VND_AFPC_MODCA_MEDIUMMAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-mediummap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-mediummap"),
    };

/// `application/vnd.afpc.modca-objectcontainer`
pub const APPLICATION_VND_AFPC_MODCA_OBJECTCONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-objectcontainer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-objectcontainer"),
    };

/// `application/vnd.afpc.modca-overlay`
pub const APPLICATION_VND_AFPC_MODCA_OVERLAY: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-overlay") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-overlay"),
    };

/// `application/vnd.afpc.modca-pagesegment`
pub const APPLICATION_VND_AFPC_MODCA_PAGESEGMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.afpc.modca-pagesegment") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.afpc.modca-pagesegment"),
    };

/// `application/vnd.age`
pub const APPLICATION_VND_AGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.age") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.age"),
    };

/// `application/vnd.ah-barcode`
pub const APPLICATION_VND_AH_BARCODE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ah-barcode") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ah-barcode"),
    };

/// `application/vnd.ahead.space`
pub const APPLICATION_VND_AHEAD_SPACE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ahead.space") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ahead.space"),
    };

/// `application/vnd.airzip.filesecure.azf`
pub const APPLICATION_VND_AIRZIP_FILESECURE_AZF: Essence<&'static str> =
    match Essence::new_const("application/vnd.airzip.filesecure.azf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.airzip.filesecure.azf"),
    };

/// `application/vnd.airzip.filesecure.azs`
pub const APPLICATION_VND_AIRZIP_FILESECURE_AZS: Essence<&'static str> =
    match Essence::new_const("application/vnd.airzip.filesecure.azs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.airzip.filesecure.azs"),
    };

/// `application/vnd.amadeus+json`
pub const APPLICATION_VND_AMADEUS_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.amadeus+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.amadeus+json"),
    };

/// `application/vnd.amazon.mobi8-ebook`
pub const APPLICATION_VND_AMAZON_MOBI8_EBOOK: Essence<&'static str> =
    match Essence::new_const("application/vnd.amazon.mobi8-ebook") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.amazon.mobi8-ebook"),
    };

/// `application/vnd.americandynamics.acc`
pub const APPLICATION_VND_AMERICANDYNAMICS_ACC: Essence<&'static str> =
    match Essence::new_const("application/vnd.americandynamics.acc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.americandynamics.acc"),
    };

/// `application/vnd.amiga.ami`
pub const APPLICATION_VND_AMIGA_AMI: Essence<&'static str> =
    match Essence::new_const("application/vnd.amiga.ami") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.amiga.ami"),
    };

/// `application/vnd.amundsen.maze+xml`
pub const APPLICATION_VND_AMUNDSEN_MAZE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.amundsen.maze+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.amundsen.maze+xml"),
    };

/// `application/vnd.android.ota`
pub const APPLICATION_VND_ANDROID_OTA: Essence<&'static str> =
    match Essence::new_const("application/vnd.android.ota") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.android.ota"),
    };

/// `application/vnd.anki`
pub const APPLICATION_VND_ANKI: Essence<&'static str> =
    match Essence::new_const("application/vnd.anki") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.anki"),
    };

/// `application/vnd.anser-web-certificate-issue-initiation`
pub const APPLICATION_VND_ANSER_WEB_CERTIFICATE_ISSUE_INITIATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.anser-web-certificate-issue-initiation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.anser-web-certificate-issue-initiation"),
    };

/// `application/vnd.antix.game-component`
pub const APPLICATION_VND_ANTIX_GAME_COMPONENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.antix.game-component") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.antix.game-component"),
    };

/// `application/vnd.apache.arrow.file`
pub const APPLICATION_VND_APACHE_ARROW_FILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.apache.arrow.file") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apache.arrow.file"),
    };

/// `application/vnd.apache.arrow.stream`
pub const APPLICATION_VND_APACHE_ARROW_STREAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.apache.arrow.stream") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apache.arrow.stream"),
    };

/// `application/vnd.apache.thrift.binary`
pub const APPLICATION_VND_APACHE_THRIFT_BINARY: Essence<&'static str> =
    match Essence::new_const("application/vnd.apache.thrift.binary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apache.thrift.binary"),
    };

/// `application/vnd.apache.thrift.compact`
pub const APPLICATION_VND_APACHE_THRIFT_COMPACT: Essence<&'static str> =
    match Essence::new_const("application/vnd.apache.thrift.compact") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apache.thrift.compact"),
    };

/// `application/vnd.apache.thrift.json`
pub const APPLICATION_VND_APACHE_THRIFT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.apache.thrift.json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apache.thrift.json"),
    };

/// `application/vnd.api+json`
pub const APPLICATION_VND_API_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.api+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.api+json"),
    };

/// `application/vnd.aplextor.warrp+json`
pub const APPLICATION_VND_APLEXTOR_WARRP_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.aplextor.warrp+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.aplextor.warrp+json"),
    };

/// `application/vnd.apothekende.reservation+json`
pub const APPLICATION_VND_APOTHEKENDE_RESERVATION_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.apothekende.reservation+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apothekende.reservation+json"),
    };

/// `application/vnd.apple.installer+xml`
pub const APPLICATION_VND_APPLE_INSTALLER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.apple.installer+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apple.installer+xml"),
    };

/// `application/vnd.apple.keynote`
pub const APPLICATION_VND_APPLE_KEYNOTE: Essence<&'static str> =
    match Essence::new_const("application/vnd.apple.keynote") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apple.keynote"),
    };

/// `application/vnd.apple.mpegurl`
pub const APPLICATION_VND_APPLE_MPEGURL: Essence<&'static str> =
    match Essence::new_const("application/vnd.apple.mpegurl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apple.mpegurl"),
    };

/// `application/vnd.apple.numbers`
pub const APPLICATION_VND_APPLE_NUMBERS: Essence<&'static str> =
    match Essence::new_const("application/vnd.apple.numbers") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apple.numbers"),
    };

/// `application/vnd.apple.pages`
pub const APPLICATION_VND_APPLE_PAGES: Essence<&'static str> =
    match Essence::new_const("application/vnd.apple.pages") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.apple.pages"),
    };

/// `application/vnd.arastra.swi`
pub const APPLICATION_VND_ARASTRA_SWI: Essence<&'static str> =
    match Essence::new_const("application/vnd.arastra.swi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.arastra.swi"),
    };

/// `application/vnd.aristanetworks.swi`
pub const APPLICATION_VND_ARISTANETWORKS_SWI: Essence<&'static str> =
    match Essence::new_const("application/vnd.aristanetworks.swi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.aristanetworks.swi"),
    };

/// `application/vnd.artisan+json`
pub const APPLICATION_VND_ARTISAN_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.artisan+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.artisan+json"),
    };

/// `application/vnd.artsquare`
pub const APPLICATION_VND_ARTSQUARE: Essence<&'static str> =
    match Essence::new_const("application/vnd.artsquare") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.artsquare"),
    };

/// `application/vnd.astraea-software.iota`
pub const APPLICATION_VND_ASTRAEA_SOFTWARE_IOTA: Essence<&'static str> =
    match Essence::new_const("application/vnd.astraea-software.iota") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.astraea-software.iota"),
    };

/// `application/vnd.audiograph`
pub const APPLICATION_VND_AUDIOGRAPH: Essence<&'static str> =
    match Essence::new_const("application/vnd.audiograph") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.audiograph"),
    };

/// `application/vnd.autopackage`
pub const APPLICATION_VND_AUTOPACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.autopackage") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.autopackage"),
    };

/// `application/vnd.avalon+json`
pub const APPLICATION_VND_AVALON_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.avalon+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.avalon+json"),
    };

/// `application/vnd.avistar+xml`
pub const APPLICATION_VND_AVISTAR_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.avistar+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.avistar+xml"),
    };

/// `application/vnd.balsamiq.bmml+xml`
pub const APPLICATION_VND_BALSAMIQ_BMML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.balsamiq.bmml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.balsamiq.bmml+xml"),
    };

/// `application/vnd.banana-accounting`
pub const APPLICATION_VND_BANANA_ACCOUNTING: Essence<&'static str> =
    match Essence::new_const("application/vnd.banana-accounting") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.banana-accounting"),
    };

/// `application/vnd.bbf.usp.error`
pub const APPLICATION_VND_BBF_USP_ERROR: Essence<&'static str> =
    match Essence::new_const("application/vnd.bbf.usp.error") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bbf.usp.error"),
    };

/// `application/vnd.bbf.usp.msg`
pub const APPLICATION_VND_BBF_USP_MSG: Essence<&'static str> =
    match Essence::new_const("application/vnd.bbf.usp.msg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bbf.usp.msg"),
    };

/// `application/vnd.bbf.usp.msg+json`
pub const APPLICATION_VND_BBF_USP_MSG_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.bbf.usp.msg+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bbf.usp.msg+json"),
    };

/// `application/vnd.balsamiq.bmpr`
pub const APPLICATION_VND_BALSAMIQ_BMPR: Essence<&'static str> =
    match Essence::new_const("application/vnd.balsamiq.bmpr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.balsamiq.bmpr"),
    };

/// `application/vnd.bekitzur-stech+json`
pub const APPLICATION_VND_BEKITZUR_STECH_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.bekitzur-stech+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bekitzur-stech+json"),
    };

/// `application/vnd.belightsoft.lhzd+zip`
pub const APPLICATION_VND_BELIGHTSOFT_LHZD_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.belightsoft.lhzd+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.belightsoft.lhzd+zip"),
    };

/// `application/vnd.bint.med-content`
pub const APPLICATION_VND_BINT_MED_CONTENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.bint.med-content") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bint.med-content"),
    };

/// `application/vnd.biopax.rdf+xml`
pub const APPLICATION_VND_BIOPAX_RDF_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.biopax.rdf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.biopax.rdf+xml"),
    };

/// `application/vnd.blink-idb-value-wrapper`
pub const APPLICATION_VND_BLINK_IDB_VALUE_WRAPPER: Essence<&'static str> =
    match Essence::new_const("application/vnd.blink-idb-value-wrapper") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.blink-idb-value-wrapper"),
    };

/// `application/vnd.blueice.multipass`
pub const APPLICATION_VND_BLUEICE_MULTIPASS: Essence<&'static str> =
    match Essence::new_const("application/vnd.blueice.multipass") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.blueice.multipass"),
    };

/// `application/vnd.bluetooth.ep.oob`
pub const APPLICATION_VND_BLUETOOTH_EP_OOB: Essence<&'static str> =
    match Essence::new_const("application/vnd.bluetooth.ep.oob") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bluetooth.ep.oob"),
    };

/// `application/vnd.bluetooth.le.oob`
pub const APPLICATION_VND_BLUETOOTH_LE_OOB: Essence<&'static str> =
    match Essence::new_const("application/vnd.bluetooth.le.oob") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bluetooth.le.oob"),
    };

/// `application/vnd.bmi`
pub const APPLICATION_VND_BMI: Essence<&'static str> =
    match Essence::new_const("application/vnd.bmi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bmi"),
    };

/// `application/vnd.bpf`
pub const APPLICATION_VND_BPF: Essence<&'static str> =
    match Essence::new_const("application/vnd.bpf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bpf"),
    };

/// `application/vnd.bpf3`
pub const APPLICATION_VND_BPF3: Essence<&'static str> =
    match Essence::new_const("application/vnd.bpf3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.bpf3"),
    };

/// `application/vnd.businessobjects`
pub const APPLICATION_VND_BUSINESSOBJECTS: Essence<&'static str> =
    match Essence::new_const("application/vnd.businessobjects") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.businessobjects"),
    };

/// `application/vnd.byu.uapi+json`
pub const APPLICATION_VND_BYU_UAPI_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.byu.uapi+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.byu.uapi+json"),
    };

/// `application/vnd.cab-jscript`
pub const APPLICATION_VND_CAB_JSCRIPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.cab-jscript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cab-jscript"),
    };

/// `application/vnd.canon-cpdl`
pub const APPLICATION_VND_CANON_CPDL: Essence<&'static str> =
    match Essence::new_const("application/vnd.canon-cpdl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.canon-cpdl"),
    };

/// `application/vnd.canon-lips`
pub const APPLICATION_VND_CANON_LIPS: Essence<&'static str> =
    match Essence::new_const("application/vnd.canon-lips") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.canon-lips"),
    };

/// `application/vnd.capasystems-pg+json`
pub const APPLICATION_VND_CAPASYSTEMS_PG_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.capasystems-pg+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.capasystems-pg+json"),
    };

/// `application/vnd.cendio.thinlinc.clientconf`
pub const APPLICATION_VND_CENDIO_THINLINC_CLIENTCONF: Essence<&'static str> =
    match Essence::new_const("application/vnd.cendio.thinlinc.clientconf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cendio.thinlinc.clientconf"),
    };

/// `application/vnd.century-systems.tcp_stream`
pub const APPLICATION_VND_CENTURY_SYSTEMS_TCP_STREAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.century-systems.tcp_stream") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.century-systems.tcp_stream"),
    };

/// `application/vnd.chemdraw+xml`
pub const APPLICATION_VND_CHEMDRAW_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.chemdraw+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.chemdraw+xml"),
    };

/// `application/vnd.chess-pgn`
pub const APPLICATION_VND_CHESS_PGN: Essence<&'static str> =
    match Essence::new_const("application/vnd.chess-pgn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.chess-pgn"),
    };

/// `application/vnd.chipnuts.karaoke-mmd`
pub const APPLICATION_VND_CHIPNUTS_KARAOKE_MMD: Essence<&'static str> =
    match Essence::new_const("application/vnd.chipnuts.karaoke-mmd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.chipnuts.karaoke-mmd"),
    };

/// `application/vnd.ciedi`
pub const APPLICATION_VND_CIEDI: Essence<&'static str> =
    match Essence::new_const("application/vnd.ciedi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ciedi"),
    };

/// `application/vnd.cinderella`
pub const APPLICATION_VND_CINDERELLA: Essence<&'static str> =
    match Essence::new_const("application/vnd.cinderella") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cinderella"),
    };

/// `application/vnd.cirpack.isdn-ext`
pub const APPLICATION_VND_CIRPACK_ISDN_EXT: Essence<&'static str> =
    match Essence::new_const("application/vnd.cirpack.isdn-ext") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cirpack.isdn-ext"),
    };

/// `application/vnd.citationstyles.style+xml`
pub const APPLICATION_VND_CITATIONSTYLES_STYLE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.citationstyles.style+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.citationstyles.style+xml"),
    };

/// `application/vnd.claymore`
pub const APPLICATION_VND_CLAYMORE: Essence<&'static str> =
    match Essence::new_const("application/vnd.claymore") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.claymore"),
    };

/// `application/vnd.cloanto.rp9`
pub const APPLICATION_VND_CLOANTO_RP9: Essence<&'static str> =
    match Essence::new_const("application/vnd.cloanto.rp9") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cloanto.rp9"),
    };

/// `application/vnd.clonk.c4group`
pub const APPLICATION_VND_CLONK_C4GROUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.clonk.c4group") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.clonk.c4group"),
    };

/// `application/vnd.cluetrust.cartomobile-config`
pub const APPLICATION_VND_CLUETRUST_CARTOMOBILE_CONFIG: Essence<&'static str> =
    match Essence::new_const("application/vnd.cluetrust.cartomobile-config") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cluetrust.cartomobile-config"),
    };

/// `application/vnd.cluetrust.cartomobile-config-pkg`
pub const APPLICATION_VND_CLUETRUST_CARTOMOBILE_CONFIG_PKG: Essence<&'static str> =
    match Essence::new_const("application/vnd.cluetrust.cartomobile-config-pkg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cluetrust.cartomobile-config-pkg"),
    };

/// `application/vnd.coffeescript`
pub const APPLICATION_VND_COFFEESCRIPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.coffeescript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.coffeescript"),
    };

/// `application/vnd.collabio.xodocuments.document`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_DOCUMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.document") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.document"),
    };

/// `application/vnd.collabio.xodocuments.document-template`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_DOCUMENT_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.document-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.document-template"),
    };

/// `application/vnd.collabio.xodocuments.presentation`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_PRESENTATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.presentation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.presentation"),
    };

/// `application/vnd.collabio.xodocuments.presentation-template`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_PRESENTATION_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.presentation-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.presentation-template"),
    };

/// `application/vnd.collabio.xodocuments.spreadsheet`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_SPREADSHEET: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.spreadsheet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.spreadsheet"),
    };

/// `application/vnd.collabio.xodocuments.spreadsheet-template`
pub const APPLICATION_VND_COLLABIO_XODOCUMENTS_SPREADSHEET_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.collabio.xodocuments.spreadsheet-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collabio.xodocuments.spreadsheet-template"),
    };

/// `application/vnd.collection.doc+json`
pub const APPLICATION_VND_COLLECTION_DOC_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.collection.doc+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collection.doc+json"),
    };

/// `application/vnd.collection+json`
pub const APPLICATION_VND_COLLECTION_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.collection+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collection+json"),
    };

/// `application/vnd.collection.next+json`
pub const APPLICATION_VND_COLLECTION_NEXT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.collection.next+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.collection.next+json"),
    };

/// `application/vnd.comicbook-rar`
pub const APPLICATION_VND_COMICBOOK_RAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.comicbook-rar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.comicbook-rar"),
    };

/// `application/vnd.comicbook+zip`
pub const APPLICATION_VND_COMICBOOK_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.comicbook+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.comicbook+zip"),
    };

/// `application/vnd.commerce-battelle`
pub const APPLICATION_VND_COMMERCE_BATTELLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.commerce-battelle") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.commerce-battelle"),
    };

/// `application/vnd.commonspace`
pub const APPLICATION_VND_COMMONSPACE: Essence<&'static str> =
    match Essence::new_const("application/vnd.commonspace") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.commonspace"),
    };

/// `application/vnd.coreos.ignition+json`
pub const APPLICATION_VND_COREOS_IGNITION_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.coreos.ignition+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.coreos.ignition+json"),
    };

/// `application/vnd.cosmocaller`
pub const APPLICATION_VND_COSMOCALLER: Essence<&'static str> =
    match Essence::new_const("application/vnd.cosmocaller") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cosmocaller"),
    };

/// `application/vnd.contact.cmsg`
pub const APPLICATION_VND_CONTACT_CMSG: Essence<&'static str> =
    match Essence::new_const("application/vnd.contact.cmsg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.contact.cmsg"),
    };

/// `application/vnd.crick.clicker`
pub const APPLICATION_VND_CRICK_CLICKER: Essence<&'static str> =
    match Essence::new_const("application/vnd.crick.clicker") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crick.clicker"),
    };

/// `application/vnd.crick.clicker.keyboard`
pub const APPLICATION_VND_CRICK_CLICKER_KEYBOARD: Essence<&'static str> =
    match Essence::new_const("application/vnd.crick.clicker.keyboard") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crick.clicker.keyboard"),
    };

/// `application/vnd.crick.clicker.palette`
pub const APPLICATION_VND_CRICK_CLICKER_PALETTE: Essence<&'static str> =
    match Essence::new_const("application/vnd.crick.clicker.palette") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crick.clicker.palette"),
    };

/// `application/vnd.crick.clicker.template`
pub const APPLICATION_VND_CRICK_CLICKER_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.crick.clicker.template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crick.clicker.template"),
    };

/// `application/vnd.crick.clicker.wordbank`
pub const APPLICATION_VND_CRICK_CLICKER_WORDBANK: Essence<&'static str> =
    match Essence::new_const("application/vnd.crick.clicker.wordbank") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crick.clicker.wordbank"),
    };

/// `application/vnd.criticaltools.wbs+xml`
pub const APPLICATION_VND_CRITICALTOOLS_WBS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.criticaltools.wbs+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.criticaltools.wbs+xml"),
    };

/// `application/vnd.cryptii.pipe+json`
pub const APPLICATION_VND_CRYPTII_PIPE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.cryptii.pipe+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cryptii.pipe+json"),
    };

/// `application/vnd.crypto-shade-file`
pub const APPLICATION_VND_CRYPTO_SHADE_FILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.crypto-shade-file") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.crypto-shade-file"),
    };

/// `application/vnd.cryptomator.encrypted`
pub const APPLICATION_VND_CRYPTOMATOR_ENCRYPTED: Essence<&'static str> =
    match Essence::new_const("application/vnd.cryptomator.encrypted") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cryptomator.encrypted"),
    };

/// `application/vnd.cryptomator.vault`
pub const APPLICATION_VND_CRYPTOMATOR_VAULT: Essence<&'static str> =
    match Essence::new_const("application/vnd.cryptomator.vault") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cryptomator.vault"),
    };

/// `application/vnd.ctc-posml`
pub const APPLICATION_VND_CTC_POSML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ctc-posml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ctc-posml"),
    };

/// `application/vnd.ctct.ws+xml`
pub const APPLICATION_VND_CTCT_WS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ctct.ws+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ctct.ws+xml"),
    };

/// `application/vnd.cups-pdf`
pub const APPLICATION_VND_CUPS_PDF: Essence<&'static str> =
    match Essence::new_const("application/vnd.cups-pdf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cups-pdf"),
    };

/// `application/vnd.cups-postscript`
pub const APPLICATION_VND_CUPS_POSTSCRIPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.cups-postscript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cups-postscript"),
    };

/// `application/vnd.cups-ppd`
pub const APPLICATION_VND_CUPS_PPD: Essence<&'static str> =
    match Essence::new_const("application/vnd.cups-ppd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cups-ppd"),
    };

/// `application/vnd.cups-raster`
pub const APPLICATION_VND_CUPS_RASTER: Essence<&'static str> =
    match Essence::new_const("application/vnd.cups-raster") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cups-raster"),
    };

/// `application/vnd.cups-raw`
pub const APPLICATION_VND_CUPS_RAW: Essence<&'static str> =
    match Essence::new_const("application/vnd.cups-raw") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cups-raw"),
    };

/// `application/vnd.curl`
pub const APPLICATION_VND_CURL: Essence<&'static str> =
    match Essence::new_const("application/vnd.curl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.curl"),
    };

/// `application/vnd.cyan.dean.root+xml`
pub const APPLICATION_VND_CYAN_DEAN_ROOT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.cyan.dean.root+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cyan.dean.root+xml"),
    };

/// `application/vnd.cybank`
pub const APPLICATION_VND_CYBANK: Essence<&'static str> =
    match Essence::new_const("application/vnd.cybank") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cybank"),
    };

/// `application/vnd.cyclonedx+json`
pub const APPLICATION_VND_CYCLONEDX_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.cyclonedx+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cyclonedx+json"),
    };

/// `application/vnd.cyclonedx+xml`
pub const APPLICATION_VND_CYCLONEDX_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.cyclonedx+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.cyclonedx+xml"),
    };

/// `application/vnd.d2l.coursepackage1p0+zip`
pub const APPLICATION_VND_D2L_COURSEPACKAGE1P0_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.d2l.coursepackage1p0+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.d2l.coursepackage1p0+zip"),
    };

/// `application/vnd.d3m-dataset`
pub const APPLICATION_VND_D3M_DATASET: Essence<&'static str> =
    match Essence::new_const("application/vnd.d3m-dataset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.d3m-dataset"),
    };

/// `application/vnd.d3m-problem`
pub const APPLICATION_VND_D3M_PROBLEM: Essence<&'static str> =
    match Essence::new_const("application/vnd.d3m-problem") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.d3m-problem"),
    };

/// `application/vnd.dart`
pub const APPLICATION_VND_DART: Essence<&'static str> =
    match Essence::new_const("application/vnd.dart") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dart"),
    };

/// `application/vnd.data-vision.rdz`
pub const APPLICATION_VND_DATA_VISION_RDZ: Essence<&'static str> =
    match Essence::new_const("application/vnd.data-vision.rdz") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.data-vision.rdz"),
    };

/// `application/vnd.datapackage+json`
pub const APPLICATION_VND_DATAPACKAGE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.datapackage+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.datapackage+json"),
    };

/// `application/vnd.dataresource+json`
pub const APPLICATION_VND_DATARESOURCE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.dataresource+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dataresource+json"),
    };

/// `application/vnd.dbf`
pub const APPLICATION_VND_DBF: Essence<&'static str> =
    match Essence::new_const("application/vnd.dbf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dbf"),
    };

/// `application/vnd.debian.binary-package`
pub const APPLICATION_VND_DEBIAN_BINARY_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.debian.binary-package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.debian.binary-package"),
    };

/// `application/vnd.dece.data`
pub const APPLICATION_VND_DECE_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.dece.data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dece.data"),
    };

/// `application/vnd.dece.ttml+xml`
pub const APPLICATION_VND_DECE_TTML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dece.ttml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dece.ttml+xml"),
    };

/// `application/vnd.dece.unspecified`
pub const APPLICATION_VND_DECE_UNSPECIFIED: Essence<&'static str> =
    match Essence::new_const("application/vnd.dece.unspecified") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dece.unspecified"),
    };

/// `application/vnd.dece.zip`
pub const APPLICATION_VND_DECE_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.dece.zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dece.zip"),
    };

/// `application/vnd.denovo.fcselayout-link`
pub const APPLICATION_VND_DENOVO_FCSELAYOUT_LINK: Essence<&'static str> =
    match Essence::new_const("application/vnd.denovo.fcselayout-link") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.denovo.fcselayout-link"),
    };

/// `application/vnd.desmume.movie`
pub const APPLICATION_VND_DESMUME_MOVIE: Essence<&'static str> =
    match Essence::new_const("application/vnd.desmume.movie") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.desmume.movie"),
    };

/// `application/vnd.dir-bi.plate-dl-nosuffix`
pub const APPLICATION_VND_DIR_BI_PLATE_DL_NOSUFFIX: Essence<&'static str> =
    match Essence::new_const("application/vnd.dir-bi.plate-dl-nosuffix") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dir-bi.plate-dl-nosuffix"),
    };

/// `application/vnd.dm.delegation+xml`
pub const APPLICATION_VND_DM_DELEGATION_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dm.delegation+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dm.delegation+xml"),
    };

/// `application/vnd.dna`
pub const APPLICATION_VND_DNA: Essence<&'static str> =
    match Essence::new_const("application/vnd.dna") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dna"),
    };

/// `application/vnd.document+json`
pub const APPLICATION_VND_DOCUMENT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.document+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.document+json"),
    };

/// `application/vnd.dolby.mobile.1`
pub const APPLICATION_VND_DOLBY_MOBILE_1: Essence<&'static str> =
    match Essence::new_const("application/vnd.dolby.mobile.1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dolby.mobile.1"),
    };

/// `application/vnd.dolby.mobile.2`
pub const APPLICATION_VND_DOLBY_MOBILE_2: Essence<&'static str> =
    match Essence::new_const("application/vnd.dolby.mobile.2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dolby.mobile.2"),
    };

/// `application/vnd.doremir.scorecloud-binary-document`
pub const APPLICATION_VND_DOREMIR_SCORECLOUD_BINARY_DOCUMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.doremir.scorecloud-binary-document") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.doremir.scorecloud-binary-document"),
    };

/// `application/vnd.dpgraph`
pub const APPLICATION_VND_DPGRAPH: Essence<&'static str> =
    match Essence::new_const("application/vnd.dpgraph") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dpgraph"),
    };

/// `application/vnd.dreamfactory`
pub const APPLICATION_VND_DREAMFACTORY: Essence<&'static str> =
    match Essence::new_const("application/vnd.dreamfactory") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dreamfactory"),
    };

/// `application/vnd.drive+json`
pub const APPLICATION_VND_DRIVE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.drive+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.drive+json"),
    };

/// `application/vnd.dtg.local`
pub const APPLICATION_VND_DTG_LOCAL: Essence<&'static str> =
    match Essence::new_const("application/vnd.dtg.local") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dtg.local"),
    };

/// `application/vnd.dtg.local.flash`
pub const APPLICATION_VND_DTG_LOCAL_FLASH: Essence<&'static str> =
    match Essence::new_const("application/vnd.dtg.local.flash") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dtg.local.flash"),
    };

/// `application/vnd.dtg.local.html`
pub const APPLICATION_VND_DTG_LOCAL_HTML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dtg.local.html") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dtg.local.html"),
    };

/// `application/vnd.dvb.ait`
pub const APPLICATION_VND_DVB_AIT: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ait") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ait"),
    };

/// `application/vnd.dvb.dvbisl+xml`
pub const APPLICATION_VND_DVB_DVBISL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.dvbisl+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.dvbisl+xml"),
    };

/// `application/vnd.dvb.dvbj`
pub const APPLICATION_VND_DVB_DVBJ: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.dvbj") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.dvbj"),
    };

/// `application/vnd.dvb.esgcontainer`
pub const APPLICATION_VND_DVB_ESGCONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.esgcontainer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.esgcontainer"),
    };

/// `application/vnd.dvb.ipdcdftnotifaccess`
pub const APPLICATION_VND_DVB_IPDCDFTNOTIFACCESS: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ipdcdftnotifaccess") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ipdcdftnotifaccess"),
    };

/// `application/vnd.dvb.ipdcesgaccess`
pub const APPLICATION_VND_DVB_IPDCESGACCESS: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ipdcesgaccess") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ipdcesgaccess"),
    };

/// `application/vnd.dvb.ipdcesgaccess2`
pub const APPLICATION_VND_DVB_IPDCESGACCESS2: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ipdcesgaccess2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ipdcesgaccess2"),
    };

/// `application/vnd.dvb.ipdcesgpdd`
pub const APPLICATION_VND_DVB_IPDCESGPDD: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ipdcesgpdd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ipdcesgpdd"),
    };

/// `application/vnd.dvb.ipdcroaming`
pub const APPLICATION_VND_DVB_IPDCROAMING: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.ipdcroaming") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.ipdcroaming"),
    };

/// `application/vnd.dvb.iptv.alfec-base`
pub const APPLICATION_VND_DVB_IPTV_ALFEC_BASE: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.iptv.alfec-base") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.iptv.alfec-base"),
    };

/// `application/vnd.dvb.iptv.alfec-enhancement`
pub const APPLICATION_VND_DVB_IPTV_ALFEC_ENHANCEMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.iptv.alfec-enhancement") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.iptv.alfec-enhancement"),
    };

/// `application/vnd.dvb.notif-aggregate-root+xml`
pub const APPLICATION_VND_DVB_NOTIF_AGGREGATE_ROOT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-aggregate-root+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-aggregate-root+xml"),
    };

/// `application/vnd.dvb.notif-container+xml`
pub const APPLICATION_VND_DVB_NOTIF_CONTAINER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-container+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-container+xml"),
    };

/// `application/vnd.dvb.notif-generic+xml`
pub const APPLICATION_VND_DVB_NOTIF_GENERIC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-generic+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-generic+xml"),
    };

/// `application/vnd.dvb.notif-ia-msglist+xml`
pub const APPLICATION_VND_DVB_NOTIF_IA_MSGLIST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-ia-msglist+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-ia-msglist+xml"),
    };

/// `application/vnd.dvb.notif-ia-registration-request+xml`
pub const APPLICATION_VND_DVB_NOTIF_IA_REGISTRATION_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-ia-registration-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-ia-registration-request+xml"),
    };

/// `application/vnd.dvb.notif-ia-registration-response+xml`
pub const APPLICATION_VND_DVB_NOTIF_IA_REGISTRATION_RESPONSE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-ia-registration-response+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-ia-registration-response+xml"),
    };

/// `application/vnd.dvb.notif-init+xml`
pub const APPLICATION_VND_DVB_NOTIF_INIT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.notif-init+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.notif-init+xml"),
    };

/// `application/vnd.dvb.pfr`
pub const APPLICATION_VND_DVB_PFR: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.pfr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.pfr"),
    };

/// `application/vnd.dvb.service`
pub const APPLICATION_VND_DVB_SERVICE: Essence<&'static str> =
    match Essence::new_const("application/vnd.dvb.service") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dvb.service"),
    };

/// `application/vnd.dxr`
pub const APPLICATION_VND_DXR: Essence<&'static str> =
    match Essence::new_const("application/vnd.dxr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dxr"),
    };

/// `application/vnd.dynageo`
pub const APPLICATION_VND_DYNAGEO: Essence<&'static str> =
    match Essence::new_const("application/vnd.dynageo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dynageo"),
    };

/// `application/vnd.dzr`
pub const APPLICATION_VND_DZR: Essence<&'static str> =
    match Essence::new_const("application/vnd.dzr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.dzr"),
    };

/// `application/vnd.easykaraoke.cdgdownload`
pub const APPLICATION_VND_EASYKARAOKE_CDGDOWNLOAD: Essence<&'static str> =
    match Essence::new_const("application/vnd.easykaraoke.cdgdownload") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.easykaraoke.cdgdownload"),
    };

/// `application/vnd.ecip.rlp`
pub const APPLICATION_VND_ECIP_RLP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecip.rlp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecip.rlp"),
    };

/// `application/vnd.ecdis-update`
pub const APPLICATION_VND_ECDIS_UPDATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecdis-update") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecdis-update"),
    };

/// `application/vnd.eclipse.ditto+json`
pub const APPLICATION_VND_ECLIPSE_DITTO_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.eclipse.ditto+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.eclipse.ditto+json"),
    };

/// `application/vnd.ecowin.chart`
pub const APPLICATION_VND_ECOWIN_CHART: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.chart") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.chart"),
    };

/// `application/vnd.ecowin.filerequest`
pub const APPLICATION_VND_ECOWIN_FILEREQUEST: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.filerequest") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.filerequest"),
    };

/// `application/vnd.ecowin.fileupdate`
pub const APPLICATION_VND_ECOWIN_FILEUPDATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.fileupdate") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.fileupdate"),
    };

/// `application/vnd.ecowin.series`
pub const APPLICATION_VND_ECOWIN_SERIES: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.series") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.series"),
    };

/// `application/vnd.ecowin.seriesrequest`
pub const APPLICATION_VND_ECOWIN_SERIESREQUEST: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.seriesrequest") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.seriesrequest"),
    };

/// `application/vnd.ecowin.seriesupdate`
pub const APPLICATION_VND_ECOWIN_SERIESUPDATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ecowin.seriesupdate") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ecowin.seriesupdate"),
    };

/// `application/vnd.efi.img`
pub const APPLICATION_VND_EFI_IMG: Essence<&'static str> =
    match Essence::new_const("application/vnd.efi.img") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.efi.img"),
    };

/// `application/vnd.efi.iso`
pub const APPLICATION_VND_EFI_ISO: Essence<&'static str> =
    match Essence::new_const("application/vnd.efi.iso") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.efi.iso"),
    };

/// `application/vnd.emclient.accessrequest+xml`
pub const APPLICATION_VND_EMCLIENT_ACCESSREQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.emclient.accessrequest+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.emclient.accessrequest+xml"),
    };

/// `application/vnd.enliven`
pub const APPLICATION_VND_ENLIVEN: Essence<&'static str> =
    match Essence::new_const("application/vnd.enliven") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.enliven"),
    };

/// `application/vnd.enphase.envoy`
pub const APPLICATION_VND_ENPHASE_ENVOY: Essence<&'static str> =
    match Essence::new_const("application/vnd.enphase.envoy") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.enphase.envoy"),
    };

/// `application/vnd.eprints.data+xml`
pub const APPLICATION_VND_EPRINTS_DATA_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.eprints.data+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.eprints.data+xml"),
    };

/// `application/vnd.epson.esf`
pub const APPLICATION_VND_EPSON_ESF: Essence<&'static str> =
    match Essence::new_const("application/vnd.epson.esf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.epson.esf"),
    };

/// `application/vnd.epson.msf`
pub const APPLICATION_VND_EPSON_MSF: Essence<&'static str> =
    match Essence::new_const("application/vnd.epson.msf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.epson.msf"),
    };

/// `application/vnd.epson.quickanime`
pub const APPLICATION_VND_EPSON_QUICKANIME: Essence<&'static str> =
    match Essence::new_const("application/vnd.epson.quickanime") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.epson.quickanime"),
    };

/// `application/vnd.epson.salt`
pub const APPLICATION_VND_EPSON_SALT: Essence<&'static str> =
    match Essence::new_const("application/vnd.epson.salt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.epson.salt"),
    };

/// `application/vnd.epson.ssf`
pub const APPLICATION_VND_EPSON_SSF: Essence<&'static str> =
    match Essence::new_const("application/vnd.epson.ssf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.epson.ssf"),
    };

/// `application/vnd.ericsson.quickcall`
pub const APPLICATION_VND_ERICSSON_QUICKCALL: Essence<&'static str> =
    match Essence::new_const("application/vnd.ericsson.quickcall") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ericsson.quickcall"),
    };

/// `application/vnd.espass-espass+zip`
pub const APPLICATION_VND_ESPASS_ESPASS_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.espass-espass+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.espass-espass+zip"),
    };

/// `application/vnd.eszigno3+xml`
pub const APPLICATION_VND_ESZIGNO3_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.eszigno3+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.eszigno3+xml"),
    };

/// `application/vnd.etsi.aoc+xml`
pub const APPLICATION_VND_ETSI_AOC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.aoc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.aoc+xml"),
    };

/// `application/vnd.etsi.asic-s+zip`
pub const APPLICATION_VND_ETSI_ASIC_S_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.asic-s+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.asic-s+zip"),
    };

/// `application/vnd.etsi.asic-e+zip`
pub const APPLICATION_VND_ETSI_ASIC_E_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.asic-e+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.asic-e+zip"),
    };

/// `application/vnd.etsi.cug+xml`
pub const APPLICATION_VND_ETSI_CUG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.cug+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.cug+xml"),
    };

/// `application/vnd.etsi.iptvcommand+xml`
pub const APPLICATION_VND_ETSI_IPTVCOMMAND_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvcommand+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvcommand+xml"),
    };

/// `application/vnd.etsi.iptvdiscovery+xml`
pub const APPLICATION_VND_ETSI_IPTVDISCOVERY_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvdiscovery+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvdiscovery+xml"),
    };

/// `application/vnd.etsi.iptvprofile+xml`
pub const APPLICATION_VND_ETSI_IPTVPROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvprofile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvprofile+xml"),
    };

/// `application/vnd.etsi.iptvsad-bc+xml`
pub const APPLICATION_VND_ETSI_IPTVSAD_BC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvsad-bc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvsad-bc+xml"),
    };

/// `application/vnd.etsi.iptvsad-cod+xml`
pub const APPLICATION_VND_ETSI_IPTVSAD_COD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvsad-cod+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvsad-cod+xml"),
    };

/// `application/vnd.etsi.iptvsad-npvr+xml`
pub const APPLICATION_VND_ETSI_IPTVSAD_NPVR_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvsad-npvr+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvsad-npvr+xml"),
    };

/// `application/vnd.etsi.iptvservice+xml`
pub const APPLICATION_VND_ETSI_IPTVSERVICE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvservice+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvservice+xml"),
    };

/// `application/vnd.etsi.iptvsync+xml`
pub const APPLICATION_VND_ETSI_IPTVSYNC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvsync+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvsync+xml"),
    };

/// `application/vnd.etsi.iptvueprofile+xml`
pub const APPLICATION_VND_ETSI_IPTVUEPROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.iptvueprofile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.iptvueprofile+xml"),
    };

/// `application/vnd.etsi.mcid+xml`
pub const APPLICATION_VND_ETSI_MCID_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.mcid+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.mcid+xml"),
    };

/// `application/vnd.etsi.mheg5`
pub const APPLICATION_VND_ETSI_MHEG5: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.mheg5") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.mheg5"),
    };

/// `application/vnd.etsi.overload-control-policy-dataset+xml`
pub const APPLICATION_VND_ETSI_OVERLOAD_CONTROL_POLICY_DATASET_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.overload-control-policy-dataset+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.overload-control-policy-dataset+xml"),
    };

/// `application/vnd.etsi.pstn+xml`
pub const APPLICATION_VND_ETSI_PSTN_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.pstn+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.pstn+xml"),
    };

/// `application/vnd.etsi.sci+xml`
pub const APPLICATION_VND_ETSI_SCI_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.sci+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.sci+xml"),
    };

/// `application/vnd.etsi.simservs+xml`
pub const APPLICATION_VND_ETSI_SIMSERVS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.simservs+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.simservs+xml"),
    };

/// `application/vnd.etsi.timestamp-token`
pub const APPLICATION_VND_ETSI_TIMESTAMP_TOKEN: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.timestamp-token") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.timestamp-token"),
    };

/// `application/vnd.etsi.tsl+xml`
pub const APPLICATION_VND_ETSI_TSL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.tsl+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.tsl+xml"),
    };

/// `application/vnd.etsi.tsl.der`
pub const APPLICATION_VND_ETSI_TSL_DER: Essence<&'static str> =
    match Essence::new_const("application/vnd.etsi.tsl.der") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.etsi.tsl.der"),
    };

/// `application/vnd.eu.kasparian.car+json`
pub const APPLICATION_VND_EU_KASPARIAN_CAR_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.eu.kasparian.car+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.eu.kasparian.car+json"),
    };

/// `application/vnd.eudora.data`
pub const APPLICATION_VND_EUDORA_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.eudora.data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.eudora.data"),
    };

/// `application/vnd.evolv.ecig.profile`
pub const APPLICATION_VND_EVOLV_ECIG_PROFILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.evolv.ecig.profile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.evolv.ecig.profile"),
    };

/// `application/vnd.evolv.ecig.settings`
pub const APPLICATION_VND_EVOLV_ECIG_SETTINGS: Essence<&'static str> =
    match Essence::new_const("application/vnd.evolv.ecig.settings") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.evolv.ecig.settings"),
    };

/// `application/vnd.evolv.ecig.theme`
pub const APPLICATION_VND_EVOLV_ECIG_THEME: Essence<&'static str> =
    match Essence::new_const("application/vnd.evolv.ecig.theme") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.evolv.ecig.theme"),
    };

/// `application/vnd.exstream-empower+zip`
pub const APPLICATION_VND_EXSTREAM_EMPOWER_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.exstream-empower+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.exstream-empower+zip"),
    };

/// `application/vnd.exstream-package`
pub const APPLICATION_VND_EXSTREAM_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.exstream-package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.exstream-package"),
    };

/// `application/vnd.ezpix-album`
pub const APPLICATION_VND_EZPIX_ALBUM: Essence<&'static str> =
    match Essence::new_const("application/vnd.ezpix-album") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ezpix-album"),
    };

/// `application/vnd.ezpix-package`
pub const APPLICATION_VND_EZPIX_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ezpix-package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ezpix-package"),
    };

/// `application/vnd.f-secure.mobile`
pub const APPLICATION_VND_F_SECURE_MOBILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.f-secure.mobile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.f-secure.mobile"),
    };

/// `application/vnd.fastcopy-disk-image`
pub const APPLICATION_VND_FASTCOPY_DISK_IMAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.fastcopy-disk-image") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fastcopy-disk-image"),
    };

/// `application/vnd.familysearch.gedcom+zip`
pub const APPLICATION_VND_FAMILYSEARCH_GEDCOM_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.familysearch.gedcom+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.familysearch.gedcom+zip"),
    };

/// `application/vnd.fdsn.mseed`
pub const APPLICATION_VND_FDSN_MSEED: Essence<&'static str> =
    match Essence::new_const("application/vnd.fdsn.mseed") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fdsn.mseed"),
    };

/// `application/vnd.fdsn.seed`
pub const APPLICATION_VND_FDSN_SEED: Essence<&'static str> =
    match Essence::new_const("application/vnd.fdsn.seed") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fdsn.seed"),
    };

/// `application/vnd.ffsns`
pub const APPLICATION_VND_FFSNS: Essence<&'static str> =
    match Essence::new_const("application/vnd.ffsns") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ffsns"),
    };

/// `application/vnd.ficlab.flb+zip`
pub const APPLICATION_VND_FICLAB_FLB_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ficlab.flb+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ficlab.flb+zip"),
    };

/// `application/vnd.filmit.zfc`
pub const APPLICATION_VND_FILMIT_ZFC: Essence<&'static str> =
    match Essence::new_const("application/vnd.filmit.zfc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.filmit.zfc"),
    };

/// `application/vnd.fints`
pub const APPLICATION_VND_FINTS: Essence<&'static str> =
    match Essence::new_const("application/vnd.fints") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fints"),
    };

/// `application/vnd.firemonkeys.cloudcell`
pub const APPLICATION_VND_FIREMONKEYS_CLOUDCELL: Essence<&'static str> =
    match Essence::new_const("application/vnd.firemonkeys.cloudcell") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.firemonkeys.cloudcell"),
    };

/// `application/vnd.FloGraphIt`
pub const APPLICATION_VND_FLOGRAPHIT: Essence<&'static str> =
    match Essence::new_const("application/vnd.FloGraphIt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.FloGraphIt"),
    };

/// `application/vnd.fluxtime.clip`
pub const APPLICATION_VND_FLUXTIME_CLIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.fluxtime.clip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fluxtime.clip"),
    };

/// `application/vnd.font-fontforge-sfd`
pub const APPLICATION_VND_FONT_FONTFORGE_SFD: Essence<&'static str> =
    match Essence::new_const("application/vnd.font-fontforge-sfd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.font-fontforge-sfd"),
    };

/// `application/vnd.framemaker`
pub const APPLICATION_VND_FRAMEMAKER: Essence<&'static str> =
    match Essence::new_const("application/vnd.framemaker") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.framemaker"),
    };

/// `application/vnd.frogans.fnc`
pub const APPLICATION_VND_FROGANS_FNC: Essence<&'static str> =
    match Essence::new_const("application/vnd.frogans.fnc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.frogans.fnc"),
    };

/// `application/vnd.frogans.ltf`
pub const APPLICATION_VND_FROGANS_LTF: Essence<&'static str> =
    match Essence::new_const("application/vnd.frogans.ltf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.frogans.ltf"),
    };

/// `application/vnd.fsc.weblaunch`
pub const APPLICATION_VND_FSC_WEBLAUNCH: Essence<&'static str> =
    match Essence::new_const("application/vnd.fsc.weblaunch") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fsc.weblaunch"),
    };

/// `application/vnd.fujifilm.fb.docuworks`
pub const APPLICATION_VND_FUJIFILM_FB_DOCUWORKS: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujifilm.fb.docuworks") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujifilm.fb.docuworks"),
    };

/// `application/vnd.fujifilm.fb.docuworks.binder`
pub const APPLICATION_VND_FUJIFILM_FB_DOCUWORKS_BINDER: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujifilm.fb.docuworks.binder") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujifilm.fb.docuworks.binder"),
    };

/// `application/vnd.fujifilm.fb.docuworks.container`
pub const APPLICATION_VND_FUJIFILM_FB_DOCUWORKS_CONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujifilm.fb.docuworks.container") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujifilm.fb.docuworks.container"),
    };

/// `application/vnd.fujifilm.fb.jfi+xml`
pub const APPLICATION_VND_FUJIFILM_FB_JFI_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujifilm.fb.jfi+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujifilm.fb.jfi+xml"),
    };

/// `application/vnd.fujitsu.oasys`
pub const APPLICATION_VND_FUJITSU_OASYS: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujitsu.oasys") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujitsu.oasys"),
    };

/// `application/vnd.fujitsu.oasys2`
pub const APPLICATION_VND_FUJITSU_OASYS2: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujitsu.oasys2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujitsu.oasys2"),
    };

/// `application/vnd.fujitsu.oasys3`
pub const APPLICATION_VND_FUJITSU_OASYS3: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujitsu.oasys3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujitsu.oasys3"),
    };

/// `application/vnd.fujitsu.oasysgp`
pub const APPLICATION_VND_FUJITSU_OASYSGP: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujitsu.oasysgp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujitsu.oasysgp"),
    };

/// `application/vnd.fujitsu.oasysprs`
pub const APPLICATION_VND_FUJITSU_OASYSPRS: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujitsu.oasysprs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujitsu.oasysprs"),
    };

/// `application/vnd.fujixerox.ART4`
pub const APPLICATION_VND_FUJIXEROX_ART4: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.ART4") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.ART4"),
    };

/// `application/vnd.fujixerox.ART-EX`
pub const APPLICATION_VND_FUJIXEROX_ART_EX: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.ART-EX") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.ART-EX"),
    };

/// `application/vnd.fujixerox.ddd`
pub const APPLICATION_VND_FUJIXEROX_DDD: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.ddd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.ddd"),
    };

/// `application/vnd.fujixerox.docuworks`
pub const APPLICATION_VND_FUJIXEROX_DOCUWORKS: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.docuworks") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.docuworks"),
    };

/// `application/vnd.fujixerox.docuworks.binder`
pub const APPLICATION_VND_FUJIXEROX_DOCUWORKS_BINDER: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.docuworks.binder") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.docuworks.binder"),
    };

/// `application/vnd.fujixerox.docuworks.container`
pub const APPLICATION_VND_FUJIXEROX_DOCUWORKS_CONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.docuworks.container") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.docuworks.container"),
    };

/// `application/vnd.fujixerox.HBPL`
pub const APPLICATION_VND_FUJIXEROX_HBPL: Essence<&'static str> =
    match Essence::new_const("application/vnd.fujixerox.HBPL") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fujixerox.HBPL"),
    };

/// `application/vnd.fut-misnet`
pub const APPLICATION_VND_FUT_MISNET: Essence<&'static str> =
    match Essence::new_const("application/vnd.fut-misnet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fut-misnet"),
    };

/// `application/vnd.futoin+cbor`
pub const APPLICATION_VND_FUTOIN_CBOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.futoin+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.futoin+cbor"),
    };

/// `application/vnd.futoin+json`
pub const APPLICATION_VND_FUTOIN_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.futoin+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.futoin+json"),
    };

/// `application/vnd.fuzzysheet`
pub const APPLICATION_VND_FUZZYSHEET: Essence<&'static str> =
    match Essence::new_const("application/vnd.fuzzysheet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.fuzzysheet"),
    };

/// `application/vnd.genomatix.tuxedo`
pub const APPLICATION_VND_GENOMATIX_TUXEDO: Essence<&'static str> =
    match Essence::new_const("application/vnd.genomatix.tuxedo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.genomatix.tuxedo"),
    };

/// `application/vnd.genozip`
pub const APPLICATION_VND_GENOZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.genozip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.genozip"),
    };

/// `application/vnd.gentics.grd+json`
pub const APPLICATION_VND_GENTICS_GRD_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.gentics.grd+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gentics.grd+json"),
    };

/// `application/vnd.geo+json`
pub const APPLICATION_VND_GEO_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.geo+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geo+json"),
    };

/// `application/vnd.geocube+xml`
pub const APPLICATION_VND_GEOCUBE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.geocube+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geocube+xml"),
    };

/// `application/vnd.geogebra.file`
pub const APPLICATION_VND_GEOGEBRA_FILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.geogebra.file") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geogebra.file"),
    };

/// `application/vnd.geogebra.slides`
pub const APPLICATION_VND_GEOGEBRA_SLIDES: Essence<&'static str> =
    match Essence::new_const("application/vnd.geogebra.slides") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geogebra.slides"),
    };

/// `application/vnd.geogebra.tool`
pub const APPLICATION_VND_GEOGEBRA_TOOL: Essence<&'static str> =
    match Essence::new_const("application/vnd.geogebra.tool") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geogebra.tool"),
    };

/// `application/vnd.geometry-explorer`
pub const APPLICATION_VND_GEOMETRY_EXPLORER: Essence<&'static str> =
    match Essence::new_const("application/vnd.geometry-explorer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geometry-explorer"),
    };

/// `application/vnd.geonext`
pub const APPLICATION_VND_GEONEXT: Essence<&'static str> =
    match Essence::new_const("application/vnd.geonext") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geonext"),
    };

/// `application/vnd.geoplan`
pub const APPLICATION_VND_GEOPLAN: Essence<&'static str> =
    match Essence::new_const("application/vnd.geoplan") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geoplan"),
    };

/// `application/vnd.geospace`
pub const APPLICATION_VND_GEOSPACE: Essence<&'static str> =
    match Essence::new_const("application/vnd.geospace") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.geospace"),
    };

/// `application/vnd.gerber`
pub const APPLICATION_VND_GERBER: Essence<&'static str> =
    match Essence::new_const("application/vnd.gerber") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gerber"),
    };

/// `application/vnd.globalplatform.card-content-mgt`
pub const APPLICATION_VND_GLOBALPLATFORM_CARD_CONTENT_MGT: Essence<&'static str> =
    match Essence::new_const("application/vnd.globalplatform.card-content-mgt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.globalplatform.card-content-mgt"),
    };

/// `application/vnd.globalplatform.card-content-mgt-response`
pub const APPLICATION_VND_GLOBALPLATFORM_CARD_CONTENT_MGT_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/vnd.globalplatform.card-content-mgt-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.globalplatform.card-content-mgt-response"),
    };

/// `application/vnd.gnu.taler.exchange+json`
pub const APPLICATION_VND_GNU_TALER_EXCHANGE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.gnu.taler.exchange+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gnu.taler.exchange+json"),
    };

/// `application/vnd.gnu.taler.merchant+json`
pub const APPLICATION_VND_GNU_TALER_MERCHANT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.gnu.taler.merchant+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gnu.taler.merchant+json"),
    };

/// `application/vnd.google-earth.kml+xml`
pub const APPLICATION_VND_GOOGLE_EARTH_KML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.google-earth.kml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.google-earth.kml+xml"),
    };

/// `application/vnd.google-earth.kmz`
pub const APPLICATION_VND_GOOGLE_EARTH_KMZ: Essence<&'static str> =
    match Essence::new_const("application/vnd.google-earth.kmz") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.google-earth.kmz"),
    };

/// `application/vnd.gov.sk.e-form+xml`
pub const APPLICATION_VND_GOV_SK_E_FORM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.gov.sk.e-form+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gov.sk.e-form+xml"),
    };

/// `application/vnd.gov.sk.e-form+zip`
pub const APPLICATION_VND_GOV_SK_E_FORM_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.gov.sk.e-form+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gov.sk.e-form+zip"),
    };

/// `application/vnd.gov.sk.xmldatacontainer+xml`
pub const APPLICATION_VND_GOV_SK_XMLDATACONTAINER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.gov.sk.xmldatacontainer+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gov.sk.xmldatacontainer+xml"),
    };

/// `application/vnd.grafeq`
pub const APPLICATION_VND_GRAFEQ: Essence<&'static str> =
    match Essence::new_const("application/vnd.grafeq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.grafeq"),
    };

/// `application/vnd.gridmp`
pub const APPLICATION_VND_GRIDMP: Essence<&'static str> =
    match Essence::new_const("application/vnd.gridmp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.gridmp"),
    };

/// `application/vnd.groove-account`
pub const APPLICATION_VND_GROOVE_ACCOUNT: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-account") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-account"),
    };

/// `application/vnd.groove-help`
pub const APPLICATION_VND_GROOVE_HELP: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-help") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-help"),
    };

/// `application/vnd.groove-identity-message`
pub const APPLICATION_VND_GROOVE_IDENTITY_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-identity-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-identity-message"),
    };

/// `application/vnd.groove-injector`
pub const APPLICATION_VND_GROOVE_INJECTOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-injector") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-injector"),
    };

/// `application/vnd.groove-tool-message`
pub const APPLICATION_VND_GROOVE_TOOL_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-tool-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-tool-message"),
    };

/// `application/vnd.groove-tool-template`
pub const APPLICATION_VND_GROOVE_TOOL_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-tool-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-tool-template"),
    };

/// `application/vnd.groove-vcard`
pub const APPLICATION_VND_GROOVE_VCARD: Essence<&'static str> =
    match Essence::new_const("application/vnd.groove-vcard") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.groove-vcard"),
    };

/// `application/vnd.hal+json`
pub const APPLICATION_VND_HAL_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.hal+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hal+json"),
    };

/// `application/vnd.hal+xml`
pub const APPLICATION_VND_HAL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.hal+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hal+xml"),
    };

/// `application/vnd.HandHeld-Entertainment+xml`
pub const APPLICATION_VND_HANDHELD_ENTERTAINMENT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.HandHeld-Entertainment+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.HandHeld-Entertainment+xml"),
    };

/// `application/vnd.hbci`
pub const APPLICATION_VND_HBCI: Essence<&'static str> =
    match Essence::new_const("application/vnd.hbci") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hbci"),
    };

/// `application/vnd.hc+json`
pub const APPLICATION_VND_HC_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.hc+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hc+json"),
    };

/// `application/vnd.hcl-bireports`
pub const APPLICATION_VND_HCL_BIREPORTS: Essence<&'static str> =
    match Essence::new_const("application/vnd.hcl-bireports") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hcl-bireports"),
    };

/// `application/vnd.hdt`
pub const APPLICATION_VND_HDT: Essence<&'static str> =
    match Essence::new_const("application/vnd.hdt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hdt"),
    };

/// `application/vnd.heroku+json`
pub const APPLICATION_VND_HEROKU_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.heroku+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.heroku+json"),
    };

/// `application/vnd.hhe.lesson-player`
pub const APPLICATION_VND_HHE_LESSON_PLAYER: Essence<&'static str> =
    match Essence::new_const("application/vnd.hhe.lesson-player") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hhe.lesson-player"),
    };

/// `application/vnd.hp-HPGL`
pub const APPLICATION_VND_HP_HPGL: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-HPGL") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-HPGL"),
    };

/// `application/vnd.hp-hpid`
pub const APPLICATION_VND_HP_HPID: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-hpid") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-hpid"),
    };

/// `application/vnd.hp-hps`
pub const APPLICATION_VND_HP_HPS: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-hps") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-hps"),
    };

/// `application/vnd.hp-jlyt`
pub const APPLICATION_VND_HP_JLYT: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-jlyt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-jlyt"),
    };

/// `application/vnd.hp-PCL`
pub const APPLICATION_VND_HP_PCL: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-PCL") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-PCL"),
    };

/// `application/vnd.hp-PCLXL`
pub const APPLICATION_VND_HP_PCLXL: Essence<&'static str> =
    match Essence::new_const("application/vnd.hp-PCLXL") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hp-PCLXL"),
    };

/// `application/vnd.httphone`
pub const APPLICATION_VND_HTTPHONE: Essence<&'static str> =
    match Essence::new_const("application/vnd.httphone") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.httphone"),
    };

/// `application/vnd.hydrostatix.sof-data`
pub const APPLICATION_VND_HYDROSTATIX_SOF_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.hydrostatix.sof-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hydrostatix.sof-data"),
    };

/// `application/vnd.hyper-item+json`
pub const APPLICATION_VND_HYPER_ITEM_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.hyper-item+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hyper-item+json"),
    };

/// `application/vnd.hyper+json`
pub const APPLICATION_VND_HYPER_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.hyper+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hyper+json"),
    };

/// `application/vnd.hyperdrive+json`
pub const APPLICATION_VND_HYPERDRIVE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.hyperdrive+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hyperdrive+json"),
    };

/// `application/vnd.hzn-3d-crossword`
pub const APPLICATION_VND_HZN_3D_CROSSWORD: Essence<&'static str> =
    match Essence::new_const("application/vnd.hzn-3d-crossword") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.hzn-3d-crossword"),
    };

/// `application/vnd.ibm.afplinedata`
pub const APPLICATION_VND_IBM_AFPLINEDATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.afplinedata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.afplinedata"),
    };

/// `application/vnd.ibm.electronic-media`
pub const APPLICATION_VND_IBM_ELECTRONIC_MEDIA: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.electronic-media") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.electronic-media"),
    };

/// `application/vnd.ibm.MiniPay`
pub const APPLICATION_VND_IBM_MINIPAY: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.MiniPay") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.MiniPay"),
    };

/// `application/vnd.ibm.modcap`
pub const APPLICATION_VND_IBM_MODCAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.modcap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.modcap"),
    };

/// `application/vnd.ibm.rights-management`
pub const APPLICATION_VND_IBM_RIGHTS_MANAGEMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.rights-management") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.rights-management"),
    };

/// `application/vnd.ibm.secure-container`
pub const APPLICATION_VND_IBM_SECURE_CONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.ibm.secure-container") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ibm.secure-container"),
    };

/// `application/vnd.iccprofile`
pub const APPLICATION_VND_ICCPROFILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.iccprofile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iccprofile"),
    };

/// `application/vnd.ieee.1905`
pub const APPLICATION_VND_IEEE_1905: Essence<&'static str> =
    match Essence::new_const("application/vnd.ieee.1905") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ieee.1905"),
    };

/// `application/vnd.igloader`
pub const APPLICATION_VND_IGLOADER: Essence<&'static str> =
    match Essence::new_const("application/vnd.igloader") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.igloader"),
    };

/// `application/vnd.imagemeter.folder+zip`
pub const APPLICATION_VND_IMAGEMETER_FOLDER_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.imagemeter.folder+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.imagemeter.folder+zip"),
    };

/// `application/vnd.imagemeter.image+zip`
pub const APPLICATION_VND_IMAGEMETER_IMAGE_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.imagemeter.image+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.imagemeter.image+zip"),
    };

/// `application/vnd.immervision-ivp`
pub const APPLICATION_VND_IMMERVISION_IVP: Essence<&'static str> =
    match Essence::new_const("application/vnd.immervision-ivp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.immervision-ivp"),
    };

/// `application/vnd.immervision-ivu`
pub const APPLICATION_VND_IMMERVISION_IVU: Essence<&'static str> =
    match Essence::new_const("application/vnd.immervision-ivu") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.immervision-ivu"),
    };

/// `application/vnd.ims.imsccv1p1`
pub const APPLICATION_VND_IMS_IMSCCV1P1: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.imsccv1p1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.imsccv1p1"),
    };

/// `application/vnd.ims.imsccv1p2`
pub const APPLICATION_VND_IMS_IMSCCV1P2: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.imsccv1p2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.imsccv1p2"),
    };

/// `application/vnd.ims.imsccv1p3`
pub const APPLICATION_VND_IMS_IMSCCV1P3: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.imsccv1p3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.imsccv1p3"),
    };

/// `application/vnd.ims.lis.v2.result+json`
pub const APPLICATION_VND_IMS_LIS_V2_RESULT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lis.v2.result+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lis.v2.result+json"),
    };

/// `application/vnd.ims.lti.v2.toolconsumerprofile+json`
pub const APPLICATION_VND_IMS_LTI_V2_TOOLCONSUMERPROFILE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lti.v2.toolconsumerprofile+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lti.v2.toolconsumerprofile+json"),
    };

/// `application/vnd.ims.lti.v2.toolproxy.id+json`
pub const APPLICATION_VND_IMS_LTI_V2_TOOLPROXY_ID_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lti.v2.toolproxy.id+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lti.v2.toolproxy.id+json"),
    };

/// `application/vnd.ims.lti.v2.toolproxy+json`
pub const APPLICATION_VND_IMS_LTI_V2_TOOLPROXY_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lti.v2.toolproxy+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lti.v2.toolproxy+json"),
    };

/// `application/vnd.ims.lti.v2.toolsettings+json`
pub const APPLICATION_VND_IMS_LTI_V2_TOOLSETTINGS_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lti.v2.toolsettings+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lti.v2.toolsettings+json"),
    };

/// `application/vnd.ims.lti.v2.toolsettings.simple+json`
pub const APPLICATION_VND_IMS_LTI_V2_TOOLSETTINGS_SIMPLE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.ims.lti.v2.toolsettings.simple+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ims.lti.v2.toolsettings.simple+json"),
    };

/// `application/vnd.informedcontrol.rms+xml`
pub const APPLICATION_VND_INFORMEDCONTROL_RMS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.informedcontrol.rms+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.informedcontrol.rms+xml"),
    };

/// `application/vnd.infotech.project`
pub const APPLICATION_VND_INFOTECH_PROJECT: Essence<&'static str> =
    match Essence::new_const("application/vnd.infotech.project") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.infotech.project"),
    };

/// `application/vnd.infotech.project+xml`
pub const APPLICATION_VND_INFOTECH_PROJECT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.infotech.project+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.infotech.project+xml"),
    };

/// `application/vnd.informix-visionary`
pub const APPLICATION_VND_INFORMIX_VISIONARY: Essence<&'static str> =
    match Essence::new_const("application/vnd.informix-visionary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.informix-visionary"),
    };

/// `application/vnd.innopath.wamp.notification`
pub const APPLICATION_VND_INNOPATH_WAMP_NOTIFICATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.innopath.wamp.notification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.innopath.wamp.notification"),
    };

/// `application/vnd.insors.igm`
pub const APPLICATION_VND_INSORS_IGM: Essence<&'static str> =
    match Essence::new_const("application/vnd.insors.igm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.insors.igm"),
    };

/// `application/vnd.intercon.formnet`
pub const APPLICATION_VND_INTERCON_FORMNET: Essence<&'static str> =
    match Essence::new_const("application/vnd.intercon.formnet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intercon.formnet"),
    };

/// `application/vnd.intergeo`
pub const APPLICATION_VND_INTERGEO: Essence<&'static str> =
    match Essence::new_const("application/vnd.intergeo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intergeo"),
    };

/// `application/vnd.intertrust.digibox`
pub const APPLICATION_VND_INTERTRUST_DIGIBOX: Essence<&'static str> =
    match Essence::new_const("application/vnd.intertrust.digibox") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intertrust.digibox"),
    };

/// `application/vnd.intertrust.nncp`
pub const APPLICATION_VND_INTERTRUST_NNCP: Essence<&'static str> =
    match Essence::new_const("application/vnd.intertrust.nncp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intertrust.nncp"),
    };

/// `application/vnd.intu.qbo`
pub const APPLICATION_VND_INTU_QBO: Essence<&'static str> =
    match Essence::new_const("application/vnd.intu.qbo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intu.qbo"),
    };

/// `application/vnd.intu.qfx`
pub const APPLICATION_VND_INTU_QFX: Essence<&'static str> =
    match Essence::new_const("application/vnd.intu.qfx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.intu.qfx"),
    };

/// `application/vnd.ipld.car`
pub const APPLICATION_VND_IPLD_CAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.ipld.car") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ipld.car"),
    };

/// `application/vnd.ipld.raw`
pub const APPLICATION_VND_IPLD_RAW: Essence<&'static str> =
    match Essence::new_const("application/vnd.ipld.raw") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ipld.raw"),
    };

/// `application/vnd.iptc.g2.catalogitem+xml`
pub const APPLICATION_VND_IPTC_G2_CATALOGITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.catalogitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.catalogitem+xml"),
    };

/// `application/vnd.iptc.g2.conceptitem+xml`
pub const APPLICATION_VND_IPTC_G2_CONCEPTITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.conceptitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.conceptitem+xml"),
    };

/// `application/vnd.iptc.g2.knowledgeitem+xml`
pub const APPLICATION_VND_IPTC_G2_KNOWLEDGEITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.knowledgeitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.knowledgeitem+xml"),
    };

/// `application/vnd.iptc.g2.newsitem+xml`
pub const APPLICATION_VND_IPTC_G2_NEWSITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.newsitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.newsitem+xml"),
    };

/// `application/vnd.iptc.g2.newsmessage+xml`
pub const APPLICATION_VND_IPTC_G2_NEWSMESSAGE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.newsmessage+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.newsmessage+xml"),
    };

/// `application/vnd.iptc.g2.packageitem+xml`
pub const APPLICATION_VND_IPTC_G2_PACKAGEITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.packageitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.packageitem+xml"),
    };

/// `application/vnd.iptc.g2.planningitem+xml`
pub const APPLICATION_VND_IPTC_G2_PLANNINGITEM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.iptc.g2.planningitem+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iptc.g2.planningitem+xml"),
    };

/// `application/vnd.ipunplugged.rcprofile`
pub const APPLICATION_VND_IPUNPLUGGED_RCPROFILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ipunplugged.rcprofile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ipunplugged.rcprofile"),
    };

/// `application/vnd.irepository.package+xml`
pub const APPLICATION_VND_IREPOSITORY_PACKAGE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.irepository.package+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.irepository.package+xml"),
    };

/// `application/vnd.is-xpr`
pub const APPLICATION_VND_IS_XPR: Essence<&'static str> =
    match Essence::new_const("application/vnd.is-xpr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.is-xpr"),
    };

/// `application/vnd.isac.fcs`
pub const APPLICATION_VND_ISAC_FCS: Essence<&'static str> =
    match Essence::new_const("application/vnd.isac.fcs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.isac.fcs"),
    };

/// `application/vnd.jam`
pub const APPLICATION_VND_JAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.jam") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.jam"),
    };

/// `application/vnd.iso11783-10+zip`
pub const APPLICATION_VND_ISO11783_10_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.iso11783-10+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.iso11783-10+zip"),
    };

/// `application/vnd.japannet-directory-service`
pub const APPLICATION_VND_JAPANNET_DIRECTORY_SERVICE: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-directory-service") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-directory-service"),
    };

/// `application/vnd.japannet-jpnstore-wakeup`
pub const APPLICATION_VND_JAPANNET_JPNSTORE_WAKEUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-jpnstore-wakeup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-jpnstore-wakeup"),
    };

/// `application/vnd.japannet-payment-wakeup`
pub const APPLICATION_VND_JAPANNET_PAYMENT_WAKEUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-payment-wakeup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-payment-wakeup"),
    };

/// `application/vnd.japannet-registration`
pub const APPLICATION_VND_JAPANNET_REGISTRATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-registration") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-registration"),
    };

/// `application/vnd.japannet-registration-wakeup`
pub const APPLICATION_VND_JAPANNET_REGISTRATION_WAKEUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-registration-wakeup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-registration-wakeup"),
    };

/// `application/vnd.japannet-setstore-wakeup`
pub const APPLICATION_VND_JAPANNET_SETSTORE_WAKEUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-setstore-wakeup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-setstore-wakeup"),
    };

/// `application/vnd.japannet-verification`
pub const APPLICATION_VND_JAPANNET_VERIFICATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-verification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-verification"),
    };

/// `application/vnd.japannet-verification-wakeup`
pub const APPLICATION_VND_JAPANNET_VERIFICATION_WAKEUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.japannet-verification-wakeup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.japannet-verification-wakeup"),
    };

/// `application/vnd.jcp.javame.midlet-rms`
pub const APPLICATION_VND_JCP_JAVAME_MIDLET_RMS: Essence<&'static str> =
    match Essence::new_const("application/vnd.jcp.javame.midlet-rms") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.jcp.javame.midlet-rms"),
    };

/// `application/vnd.jisp`
pub const APPLICATION_VND_JISP: Essence<&'static str> =
    match Essence::new_const("application/vnd.jisp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.jisp"),
    };

/// `application/vnd.joost.joda-archive`
pub const APPLICATION_VND_JOOST_JODA_ARCHIVE: Essence<&'static str> =
    match Essence::new_const("application/vnd.joost.joda-archive") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.joost.joda-archive"),
    };

/// `application/vnd.jsk.isdn-ngn`
pub const APPLICATION_VND_JSK_ISDN_NGN: Essence<&'static str> =
    match Essence::new_const("application/vnd.jsk.isdn-ngn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.jsk.isdn-ngn"),
    };

/// `application/vnd.kahootz`
pub const APPLICATION_VND_KAHOOTZ: Essence<&'static str> =
    match Essence::new_const("application/vnd.kahootz") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kahootz"),
    };

/// `application/vnd.kde.karbon`
pub const APPLICATION_VND_KDE_KARBON: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.karbon") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.karbon"),
    };

/// `application/vnd.kde.kchart`
pub const APPLICATION_VND_KDE_KCHART: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kchart") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kchart"),
    };

/// `application/vnd.kde.kformula`
pub const APPLICATION_VND_KDE_KFORMULA: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kformula") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kformula"),
    };

/// `application/vnd.kde.kivio`
pub const APPLICATION_VND_KDE_KIVIO: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kivio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kivio"),
    };

/// `application/vnd.kde.kontour`
pub const APPLICATION_VND_KDE_KONTOUR: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kontour") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kontour"),
    };

/// `application/vnd.kde.kpresenter`
pub const APPLICATION_VND_KDE_KPRESENTER: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kpresenter") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kpresenter"),
    };

/// `application/vnd.kde.kspread`
pub const APPLICATION_VND_KDE_KSPREAD: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kspread") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kspread"),
    };

/// `application/vnd.kde.kword`
pub const APPLICATION_VND_KDE_KWORD: Essence<&'static str> =
    match Essence::new_const("application/vnd.kde.kword") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kde.kword"),
    };

/// `application/vnd.kenameaapp`
pub const APPLICATION_VND_KENAMEAAPP: Essence<&'static str> =
    match Essence::new_const("application/vnd.kenameaapp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kenameaapp"),
    };

/// `application/vnd.kidspiration`
pub const APPLICATION_VND_KIDSPIRATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.kidspiration") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kidspiration"),
    };

/// `application/vnd.Kinar`
pub const APPLICATION_VND_KINAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.Kinar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Kinar"),
    };

/// `application/vnd.koan`
pub const APPLICATION_VND_KOAN: Essence<&'static str> =
    match Essence::new_const("application/vnd.koan") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.koan"),
    };

/// `application/vnd.kodak-descriptor`
pub const APPLICATION_VND_KODAK_DESCRIPTOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.kodak-descriptor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.kodak-descriptor"),
    };

/// `application/vnd.las`
pub const APPLICATION_VND_LAS: Essence<&'static str> =
    match Essence::new_const("application/vnd.las") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.las"),
    };

/// `application/vnd.las.las+json`
pub const APPLICATION_VND_LAS_LAS_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.las.las+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.las.las+json"),
    };

/// `application/vnd.las.las+xml`
pub const APPLICATION_VND_LAS_LAS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.las.las+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.las.las+xml"),
    };

/// `application/vnd.laszip`
pub const APPLICATION_VND_LASZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.laszip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.laszip"),
    };

/// `application/vnd.leap+json`
pub const APPLICATION_VND_LEAP_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.leap+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.leap+json"),
    };

/// `application/vnd.liberty-request+xml`
pub const APPLICATION_VND_LIBERTY_REQUEST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.liberty-request+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.liberty-request+xml"),
    };

/// `application/vnd.llamagraphics.life-balance.desktop`
pub const APPLICATION_VND_LLAMAGRAPHICS_LIFE_BALANCE_DESKTOP: Essence<&'static str> =
    match Essence::new_const("application/vnd.llamagraphics.life-balance.desktop") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.llamagraphics.life-balance.desktop"),
    };

/// `application/vnd.llamagraphics.life-balance.exchange+xml`
pub const APPLICATION_VND_LLAMAGRAPHICS_LIFE_BALANCE_EXCHANGE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.llamagraphics.life-balance.exchange+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.llamagraphics.life-balance.exchange+xml"),
    };

/// `application/vnd.logipipe.circuit+zip`
pub const APPLICATION_VND_LOGIPIPE_CIRCUIT_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.logipipe.circuit+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.logipipe.circuit+zip"),
    };

/// `application/vnd.loom`
pub const APPLICATION_VND_LOOM: Essence<&'static str> =
    match Essence::new_const("application/vnd.loom") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.loom"),
    };

/// `application/vnd.lotus-1-2-3`
pub const APPLICATION_VND_LOTUS_1_2_3: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-1-2-3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-1-2-3"),
    };

/// `application/vnd.lotus-approach`
pub const APPLICATION_VND_LOTUS_APPROACH: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-approach") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-approach"),
    };

/// `application/vnd.lotus-freelance`
pub const APPLICATION_VND_LOTUS_FREELANCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-freelance") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-freelance"),
    };

/// `application/vnd.lotus-notes`
pub const APPLICATION_VND_LOTUS_NOTES: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-notes") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-notes"),
    };

/// `application/vnd.lotus-organizer`
pub const APPLICATION_VND_LOTUS_ORGANIZER: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-organizer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-organizer"),
    };

/// `application/vnd.lotus-screencam`
pub const APPLICATION_VND_LOTUS_SCREENCAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-screencam") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-screencam"),
    };

/// `application/vnd.lotus-wordpro`
pub const APPLICATION_VND_LOTUS_WORDPRO: Essence<&'static str> =
    match Essence::new_const("application/vnd.lotus-wordpro") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.lotus-wordpro"),
    };

/// `application/vnd.macports.portpkg`
pub const APPLICATION_VND_MACPORTS_PORTPKG: Essence<&'static str> =
    match Essence::new_const("application/vnd.macports.portpkg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.macports.portpkg"),
    };

/// `application/vnd.mapbox-vector-tile`
pub const APPLICATION_VND_MAPBOX_VECTOR_TILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.mapbox-vector-tile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mapbox-vector-tile"),
    };

/// `application/vnd.marlin.drm.actiontoken+xml`
pub const APPLICATION_VND_MARLIN_DRM_ACTIONTOKEN_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.marlin.drm.actiontoken+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.marlin.drm.actiontoken+xml"),
    };

/// `application/vnd.marlin.drm.conftoken+xml`
pub const APPLICATION_VND_MARLIN_DRM_CONFTOKEN_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.marlin.drm.conftoken+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.marlin.drm.conftoken+xml"),
    };

/// `application/vnd.marlin.drm.license+xml`
pub const APPLICATION_VND_MARLIN_DRM_LICENSE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.marlin.drm.license+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.marlin.drm.license+xml"),
    };

/// `application/vnd.marlin.drm.mdcf`
pub const APPLICATION_VND_MARLIN_DRM_MDCF: Essence<&'static str> =
    match Essence::new_const("application/vnd.marlin.drm.mdcf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.marlin.drm.mdcf"),
    };

/// `application/vnd.mason+json`
pub const APPLICATION_VND_MASON_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.mason+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mason+json"),
    };

/// `application/vnd.maxar.archive.3tz+zip`
pub const APPLICATION_VND_MAXAR_ARCHIVE_3TZ_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.maxar.archive.3tz+zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.maxar.archive.3tz+zip"),
    };

/// `application/vnd.maxmind.maxmind-db`
pub const APPLICATION_VND_MAXMIND_MAXMIND_DB: Essence<&'static str> =
    match Essence::new_const("application/vnd.maxmind.maxmind-db") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.maxmind.maxmind-db"),
    };

/// `application/vnd.mcd`
pub const APPLICATION_VND_MCD: Essence<&'static str> =
    match Essence::new_const("application/vnd.mcd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mcd"),
    };

/// `application/vnd.medcalcdata`
pub const APPLICATION_VND_MEDCALCDATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.medcalcdata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.medcalcdata"),
    };

/// `application/vnd.mediastation.cdkey`
pub const APPLICATION_VND_MEDIASTATION_CDKEY: Essence<&'static str> =
    match Essence::new_const("application/vnd.mediastation.cdkey") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mediastation.cdkey"),
    };

/// `application/vnd.meridian-slingshot`
pub const APPLICATION_VND_MERIDIAN_SLINGSHOT: Essence<&'static str> =
    match Essence::new_const("application/vnd.meridian-slingshot") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.meridian-slingshot"),
    };

/// `application/vnd.MFER`
pub const APPLICATION_VND_MFER: Essence<&'static str> =
    match Essence::new_const("application/vnd.MFER") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.MFER"),
    };

/// `application/vnd.mfmp`
pub const APPLICATION_VND_MFMP: Essence<&'static str> =
    match Essence::new_const("application/vnd.mfmp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mfmp"),
    };

/// `application/vnd.micro+json`
pub const APPLICATION_VND_MICRO_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.micro+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.micro+json"),
    };

/// `application/vnd.micrografx.flo`
pub const APPLICATION_VND_MICROGRAFX_FLO: Essence<&'static str> =
    match Essence::new_const("application/vnd.micrografx.flo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.micrografx.flo"),
    };

/// `application/vnd.micrografx.igx`
pub const APPLICATION_VND_MICROGRAFX_IGX: Essence<&'static str> =
    match Essence::new_const("application/vnd.micrografx.igx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.micrografx.igx"),
    };

/// `application/vnd.microsoft.portable-executable`
pub const APPLICATION_VND_MICROSOFT_PORTABLE_EXECUTABLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.microsoft.portable-executable") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.microsoft.portable-executable"),
    };

/// `application/vnd.microsoft.windows.thumbnail-cache`
pub const APPLICATION_VND_MICROSOFT_WINDOWS_THUMBNAIL_CACHE: Essence<&'static str> =
    match Essence::new_const("application/vnd.microsoft.windows.thumbnail-cache") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.microsoft.windows.thumbnail-cache"),
    };

/// `application/vnd.miele+json`
pub const APPLICATION_VND_MIELE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.miele+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.miele+json"),
    };

/// `application/vnd.mif`
pub const APPLICATION_VND_MIF: Essence<&'static str> =
    match Essence::new_const("application/vnd.mif") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mif"),
    };

/// `application/vnd.minisoft-hp3000-save`
pub const APPLICATION_VND_MINISOFT_HP3000_SAVE: Essence<&'static str> =
    match Essence::new_const("application/vnd.minisoft-hp3000-save") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.minisoft-hp3000-save"),
    };

/// `application/vnd.mitsubishi.misty-guard.trustweb`
pub const APPLICATION_VND_MITSUBISHI_MISTY_GUARD_TRUSTWEB: Essence<&'static str> =
    match Essence::new_const("application/vnd.mitsubishi.misty-guard.trustweb") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mitsubishi.misty-guard.trustweb"),
    };

/// `application/vnd.Mobius.DAF`
pub const APPLICATION_VND_MOBIUS_DAF: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.DAF") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.DAF"),
    };

/// `application/vnd.Mobius.DIS`
pub const APPLICATION_VND_MOBIUS_DIS: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.DIS") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.DIS"),
    };

/// `application/vnd.Mobius.MBK`
pub const APPLICATION_VND_MOBIUS_MBK: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.MBK") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.MBK"),
    };

/// `application/vnd.Mobius.MQY`
pub const APPLICATION_VND_MOBIUS_MQY: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.MQY") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.MQY"),
    };

/// `application/vnd.Mobius.MSL`
pub const APPLICATION_VND_MOBIUS_MSL: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.MSL") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.MSL"),
    };

/// `application/vnd.Mobius.PLC`
pub const APPLICATION_VND_MOBIUS_PLC: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.PLC") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.PLC"),
    };

/// `application/vnd.Mobius.TXF`
pub const APPLICATION_VND_MOBIUS_TXF: Essence<&'static str> =
    match Essence::new_const("application/vnd.Mobius.TXF") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Mobius.TXF"),
    };

/// `application/vnd.mophun.application`
pub const APPLICATION_VND_MOPHUN_APPLICATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.mophun.application") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mophun.application"),
    };

/// `application/vnd.mophun.certificate`
pub const APPLICATION_VND_MOPHUN_CERTIFICATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.mophun.certificate") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mophun.certificate"),
    };

/// `application/vnd.motorola.flexsuite`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite"),
    };

/// `application/vnd.motorola.flexsuite.adsi`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_ADSI: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.adsi") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.adsi"),
    };

/// `application/vnd.motorola.flexsuite.fis`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_FIS: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.fis") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.fis"),
    };

/// `application/vnd.motorola.flexsuite.gotap`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_GOTAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.gotap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.gotap"),
    };

/// `application/vnd.motorola.flexsuite.kmr`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_KMR: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.kmr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.kmr"),
    };

/// `application/vnd.motorola.flexsuite.ttc`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_TTC: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.ttc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.ttc"),
    };

/// `application/vnd.motorola.flexsuite.wem`
pub const APPLICATION_VND_MOTOROLA_FLEXSUITE_WEM: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.flexsuite.wem") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.flexsuite.wem"),
    };

/// `application/vnd.motorola.iprm`
pub const APPLICATION_VND_MOTOROLA_IPRM: Essence<&'static str> =
    match Essence::new_const("application/vnd.motorola.iprm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.motorola.iprm"),
    };

/// `application/vnd.mozilla.xul+xml`
pub const APPLICATION_VND_MOZILLA_XUL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.mozilla.xul+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mozilla.xul+xml"),
    };

/// `application/vnd.ms-artgalry`
pub const APPLICATION_VND_MS_ARTGALRY: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-artgalry") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-artgalry"),
    };

/// `application/vnd.ms-asf`
pub const APPLICATION_VND_MS_ASF: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-asf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-asf"),
    };

/// `application/vnd.ms-cab-compressed`
pub const APPLICATION_VND_MS_CAB_COMPRESSED: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-cab-compressed") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-cab-compressed"),
    };

/// `application/vnd.ms-3mfdocument`
pub const APPLICATION_VND_MS_3MFDOCUMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-3mfdocument") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-3mfdocument"),
    };

/// `application/vnd.ms-excel`
pub const APPLICATION_VND_MS_EXCEL: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-excel") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-excel"),
    };

/// `application/vnd.ms-excel.addin.macroEnabled.12`
pub const APPLICATION_VND_MS_EXCEL_ADDIN_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-excel.addin.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-excel.addin.macroEnabled.12"),
    };

/// `application/vnd.ms-excel.sheet.binary.macroEnabled.12`
pub const APPLICATION_VND_MS_EXCEL_SHEET_BINARY_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-excel.sheet.binary.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-excel.sheet.binary.macroEnabled.12"),
    };

/// `application/vnd.ms-excel.sheet.macroEnabled.12`
pub const APPLICATION_VND_MS_EXCEL_SHEET_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-excel.sheet.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-excel.sheet.macroEnabled.12"),
    };

/// `application/vnd.ms-excel.template.macroEnabled.12`
pub const APPLICATION_VND_MS_EXCEL_TEMPLATE_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-excel.template.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-excel.template.macroEnabled.12"),
    };

/// `application/vnd.ms-fontobject`
pub const APPLICATION_VND_MS_FONTOBJECT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-fontobject") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-fontobject"),
    };

/// `application/vnd.ms-htmlhelp`
pub const APPLICATION_VND_MS_HTMLHELP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-htmlhelp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-htmlhelp"),
    };

/// `application/vnd.ms-ims`
pub const APPLICATION_VND_MS_IMS: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-ims") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-ims"),
    };

/// `application/vnd.ms-lrm`
pub const APPLICATION_VND_MS_LRM: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-lrm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-lrm"),
    };

/// `application/vnd.ms-office.activeX+xml`
pub const APPLICATION_VND_MS_OFFICE_ACTIVEX_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-office.activeX+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-office.activeX+xml"),
    };

/// `application/vnd.ms-officetheme`
pub const APPLICATION_VND_MS_OFFICETHEME: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-officetheme") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-officetheme"),
    };

/// `application/vnd.ms-playready.initiator+xml`
pub const APPLICATION_VND_MS_PLAYREADY_INITIATOR_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-playready.initiator+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-playready.initiator+xml"),
    };

/// `application/vnd.ms-powerpoint`
pub const APPLICATION_VND_MS_POWERPOINT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint"),
    };

/// `application/vnd.ms-powerpoint.addin.macroEnabled.12`
pub const APPLICATION_VND_MS_POWERPOINT_ADDIN_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint.addin.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint.addin.macroEnabled.12"),
    };

/// `application/vnd.ms-powerpoint.presentation.macroEnabled.12`
pub const APPLICATION_VND_MS_POWERPOINT_PRESENTATION_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint.presentation.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint.presentation.macroEnabled.12"),
    };

/// `application/vnd.ms-powerpoint.slide.macroEnabled.12`
pub const APPLICATION_VND_MS_POWERPOINT_SLIDE_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint.slide.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint.slide.macroEnabled.12"),
    };

/// `application/vnd.ms-powerpoint.slideshow.macroEnabled.12`
pub const APPLICATION_VND_MS_POWERPOINT_SLIDESHOW_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint.slideshow.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint.slideshow.macroEnabled.12"),
    };

/// `application/vnd.ms-powerpoint.template.macroEnabled.12`
pub const APPLICATION_VND_MS_POWERPOINT_TEMPLATE_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-powerpoint.template.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-powerpoint.template.macroEnabled.12"),
    };

/// `application/vnd.ms-PrintDeviceCapabilities+xml`
pub const APPLICATION_VND_MS_PRINTDEVICECAPABILITIES_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-PrintDeviceCapabilities+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-PrintDeviceCapabilities+xml"),
    };

/// `application/vnd.ms-PrintSchemaTicket+xml`
pub const APPLICATION_VND_MS_PRINTSCHEMATICKET_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-PrintSchemaTicket+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-PrintSchemaTicket+xml"),
    };

/// `application/vnd.ms-project`
pub const APPLICATION_VND_MS_PROJECT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-project") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-project"),
    };

/// `application/vnd.ms-tnef`
pub const APPLICATION_VND_MS_TNEF: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-tnef") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-tnef"),
    };

/// `application/vnd.ms-windows.devicepairing`
pub const APPLICATION_VND_MS_WINDOWS_DEVICEPAIRING: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-windows.devicepairing") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-windows.devicepairing"),
    };

/// `application/vnd.ms-windows.nwprinting.oob`
pub const APPLICATION_VND_MS_WINDOWS_NWPRINTING_OOB: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-windows.nwprinting.oob") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-windows.nwprinting.oob"),
    };

/// `application/vnd.ms-windows.printerpairing`
pub const APPLICATION_VND_MS_WINDOWS_PRINTERPAIRING: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-windows.printerpairing") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-windows.printerpairing"),
    };

/// `application/vnd.ms-windows.wsd.oob`
pub const APPLICATION_VND_MS_WINDOWS_WSD_OOB: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-windows.wsd.oob") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-windows.wsd.oob"),
    };

/// `application/vnd.ms-wmdrm.lic-chlg-req`
pub const APPLICATION_VND_MS_WMDRM_LIC_CHLG_REQ: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-wmdrm.lic-chlg-req") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-wmdrm.lic-chlg-req"),
    };

/// `application/vnd.ms-wmdrm.lic-resp`
pub const APPLICATION_VND_MS_WMDRM_LIC_RESP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-wmdrm.lic-resp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-wmdrm.lic-resp"),
    };

/// `application/vnd.ms-wmdrm.meter-chlg-req`
pub const APPLICATION_VND_MS_WMDRM_METER_CHLG_REQ: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-wmdrm.meter-chlg-req") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-wmdrm.meter-chlg-req"),
    };

/// `application/vnd.ms-wmdrm.meter-resp`
pub const APPLICATION_VND_MS_WMDRM_METER_RESP: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-wmdrm.meter-resp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-wmdrm.meter-resp"),
    };

/// `application/vnd.ms-word.document.macroEnabled.12`
pub const APPLICATION_VND_MS_WORD_DOCUMENT_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-word.document.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-word.document.macroEnabled.12"),
    };

/// `application/vnd.ms-word.template.macroEnabled.12`
pub const APPLICATION_VND_MS_WORD_TEMPLATE_MACROENABLED_12: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-word.template.macroEnabled.12") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-word.template.macroEnabled.12"),
    };

/// `application/vnd.ms-works`
pub const APPLICATION_VND_MS_WORKS: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-works") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-works"),
    };

/// `application/vnd.ms-wpl`
pub const APPLICATION_VND_MS_WPL: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-wpl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-wpl"),
    };

/// `application/vnd.ms-xpsdocument`
pub const APPLICATION_VND_MS_XPSDOCUMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.ms-xpsdocument") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ms-xpsdocument"),
    };

/// `application/vnd.msa-disk-image`
pub const APPLICATION_VND_MSA_DISK_IMAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.msa-disk-image") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.msa-disk-image"),
    };

/// `application/vnd.mseq`
pub const APPLICATION_VND_MSEQ: Essence<&'static str> =
    match Essence::new_const("application/vnd.mseq") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mseq"),
    };

/// `application/vnd.msign`
pub const APPLICATION_VND_MSIGN: Essence<&'static str> =
    match Essence::new_const("application/vnd.msign") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.msign"),
    };

/// `application/vnd.multiad.creator`
pub const APPLICATION_VND_MULTIAD_CREATOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.multiad.creator") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.multiad.creator"),
    };

/// `application/vnd.multiad.creator.cif`
pub const APPLICATION_VND_MULTIAD_CREATOR_CIF: Essence<&'static str> =
    match Essence::new_const("application/vnd.multiad.creator.cif") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.multiad.creator.cif"),
    };

/// `application/vnd.musician`
pub const APPLICATION_VND_MUSICIAN: Essence<&'static str> =
    match Essence::new_const("application/vnd.musician") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.musician"),
    };

/// `application/vnd.music-niff`
pub const APPLICATION_VND_MUSIC_NIFF: Essence<&'static str> =
    match Essence::new_const("application/vnd.music-niff") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.music-niff"),
    };

/// `application/vnd.muvee.style`
pub const APPLICATION_VND_MUVEE_STYLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.muvee.style") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.muvee.style"),
    };

/// `application/vnd.mynfc`
pub const APPLICATION_VND_MYNFC: Essence<&'static str> =
    match Essence::new_const("application/vnd.mynfc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.mynfc"),
    };

/// `application/vnd.nacamar.ybrid+json`
pub const APPLICATION_VND_NACAMAR_YBRID_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.nacamar.ybrid+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nacamar.ybrid+json"),
    };

/// `application/vnd.ncd.control`
pub const APPLICATION_VND_NCD_CONTROL: Essence<&'static str> =
    match Essence::new_const("application/vnd.ncd.control") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ncd.control"),
    };

/// `application/vnd.ncd.reference`
pub const APPLICATION_VND_NCD_REFERENCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ncd.reference") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ncd.reference"),
    };

/// `application/vnd.nearst.inv+json`
pub const APPLICATION_VND_NEARST_INV_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.nearst.inv+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nearst.inv+json"),
    };

/// `application/vnd.nebumind.line`
pub const APPLICATION_VND_NEBUMIND_LINE: Essence<&'static str> =
    match Essence::new_const("application/vnd.nebumind.line") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nebumind.line"),
    };

/// `application/vnd.nervana`
pub const APPLICATION_VND_NERVANA: Essence<&'static str> =
    match Essence::new_const("application/vnd.nervana") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nervana"),
    };

/// `application/vnd.netfpx`
pub const APPLICATION_VND_NETFPX: Essence<&'static str> =
    match Essence::new_const("application/vnd.netfpx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.netfpx"),
    };

/// `application/vnd.neurolanguage.nlu`
pub const APPLICATION_VND_NEUROLANGUAGE_NLU: Essence<&'static str> =
    match Essence::new_const("application/vnd.neurolanguage.nlu") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.neurolanguage.nlu"),
    };

/// `application/vnd.nimn`
pub const APPLICATION_VND_NIMN: Essence<&'static str> =
    match Essence::new_const("application/vnd.nimn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nimn"),
    };

/// `application/vnd.nintendo.snes.rom`
pub const APPLICATION_VND_NINTENDO_SNES_ROM: Essence<&'static str> =
    match Essence::new_const("application/vnd.nintendo.snes.rom") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nintendo.snes.rom"),
    };

/// `application/vnd.nintendo.nitro.rom`
pub const APPLICATION_VND_NINTENDO_NITRO_ROM: Essence<&'static str> =
    match Essence::new_const("application/vnd.nintendo.nitro.rom") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nintendo.nitro.rom"),
    };

/// `application/vnd.nitf`
pub const APPLICATION_VND_NITF: Essence<&'static str> =
    match Essence::new_const("application/vnd.nitf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nitf"),
    };

/// `application/vnd.noblenet-directory`
pub const APPLICATION_VND_NOBLENET_DIRECTORY: Essence<&'static str> =
    match Essence::new_const("application/vnd.noblenet-directory") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.noblenet-directory"),
    };

/// `application/vnd.noblenet-sealer`
pub const APPLICATION_VND_NOBLENET_SEALER: Essence<&'static str> =
    match Essence::new_const("application/vnd.noblenet-sealer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.noblenet-sealer"),
    };

/// `application/vnd.noblenet-web`
pub const APPLICATION_VND_NOBLENET_WEB: Essence<&'static str> =
    match Essence::new_const("application/vnd.noblenet-web") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.noblenet-web"),
    };

/// `application/vnd.nokia.catalogs`
pub const APPLICATION_VND_NOKIA_CATALOGS: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.catalogs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.catalogs"),
    };

/// `application/vnd.nokia.conml+wbxml`
pub const APPLICATION_VND_NOKIA_CONML_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.conml+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.conml+wbxml"),
    };

/// `application/vnd.nokia.conml+xml`
pub const APPLICATION_VND_NOKIA_CONML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.conml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.conml+xml"),
    };

/// `application/vnd.nokia.iptv.config+xml`
pub const APPLICATION_VND_NOKIA_IPTV_CONFIG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.iptv.config+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.iptv.config+xml"),
    };

/// `application/vnd.nokia.iSDS-radio-presets`
pub const APPLICATION_VND_NOKIA_ISDS_RADIO_PRESETS: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.iSDS-radio-presets") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.iSDS-radio-presets"),
    };

/// `application/vnd.nokia.landmark+wbxml`
pub const APPLICATION_VND_NOKIA_LANDMARK_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.landmark+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.landmark+wbxml"),
    };

/// `application/vnd.nokia.landmark+xml`
pub const APPLICATION_VND_NOKIA_LANDMARK_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.landmark+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.landmark+xml"),
    };

/// `application/vnd.nokia.landmarkcollection+xml`
pub const APPLICATION_VND_NOKIA_LANDMARKCOLLECTION_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.landmarkcollection+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.landmarkcollection+xml"),
    };

/// `application/vnd.nokia.ncd`
pub const APPLICATION_VND_NOKIA_NCD: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.ncd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.ncd"),
    };

/// `application/vnd.nokia.n-gage.ac+xml`
pub const APPLICATION_VND_NOKIA_N_GAGE_AC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.n-gage.ac+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.n-gage.ac+xml"),
    };

/// `application/vnd.nokia.n-gage.data`
pub const APPLICATION_VND_NOKIA_N_GAGE_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.n-gage.data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.n-gage.data"),
    };

/// `application/vnd.nokia.n-gage.symbian.install`
pub const APPLICATION_VND_NOKIA_N_GAGE_SYMBIAN_INSTALL: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.n-gage.symbian.install") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.n-gage.symbian.install"),
    };

/// `application/vnd.nokia.pcd+wbxml`
pub const APPLICATION_VND_NOKIA_PCD_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.pcd+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.pcd+wbxml"),
    };

/// `application/vnd.nokia.pcd+xml`
pub const APPLICATION_VND_NOKIA_PCD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.pcd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.pcd+xml"),
    };

/// `application/vnd.nokia.radio-preset`
pub const APPLICATION_VND_NOKIA_RADIO_PRESET: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.radio-preset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.radio-preset"),
    };

/// `application/vnd.nokia.radio-presets`
pub const APPLICATION_VND_NOKIA_RADIO_PRESETS: Essence<&'static str> =
    match Essence::new_const("application/vnd.nokia.radio-presets") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.nokia.radio-presets"),
    };

/// `application/vnd.novadigm.EDM`
pub const APPLICATION_VND_NOVADIGM_EDM: Essence<&'static str> =
    match Essence::new_const("application/vnd.novadigm.EDM") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.novadigm.EDM"),
    };

/// `application/vnd.novadigm.EDX`
pub const APPLICATION_VND_NOVADIGM_EDX: Essence<&'static str> =
    match Essence::new_const("application/vnd.novadigm.EDX") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.novadigm.EDX"),
    };

/// `application/vnd.novadigm.EXT`
pub const APPLICATION_VND_NOVADIGM_EXT: Essence<&'static str> =
    match Essence::new_const("application/vnd.novadigm.EXT") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.novadigm.EXT"),
    };

/// `application/vnd.ntt-local.content-share`
pub const APPLICATION_VND_NTT_LOCAL_CONTENT_SHARE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ntt-local.content-share") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ntt-local.content-share"),
    };

/// `application/vnd.ntt-local.file-transfer`
pub const APPLICATION_VND_NTT_LOCAL_FILE_TRANSFER: Essence<&'static str> =
    match Essence::new_const("application/vnd.ntt-local.file-transfer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ntt-local.file-transfer"),
    };

/// `application/vnd.ntt-local.ogw_remote-access`
pub const APPLICATION_VND_NTT_LOCAL_OGW_REMOTE_ACCESS: Essence<&'static str> =
    match Essence::new_const("application/vnd.ntt-local.ogw_remote-access") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ntt-local.ogw_remote-access"),
    };

/// `application/vnd.ntt-local.sip-ta_remote`
pub const APPLICATION_VND_NTT_LOCAL_SIP_TA_REMOTE: Essence<&'static str> =
    match Essence::new_const("application/vnd.ntt-local.sip-ta_remote") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ntt-local.sip-ta_remote"),
    };

/// `application/vnd.ntt-local.sip-ta_tcp_stream`
pub const APPLICATION_VND_NTT_LOCAL_SIP_TA_TCP_STREAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.ntt-local.sip-ta_tcp_stream") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ntt-local.sip-ta_tcp_stream"),
    };

/// `application/vnd.oasis.opendocument.chart`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_CHART: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.chart") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.chart"),
    };

/// `application/vnd.oasis.opendocument.chart-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_CHART_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.chart-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.chart-template"),
    };

/// `application/vnd.oasis.opendocument.database`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_DATABASE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.database") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.database"),
    };

/// `application/vnd.oasis.opendocument.formula`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.formula") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.formula"),
    };

/// `application/vnd.oasis.opendocument.formula-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_FORMULA_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.formula-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.formula-template"),
    };

/// `application/vnd.oasis.opendocument.graphics`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.graphics") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.graphics"),
    };

/// `application/vnd.oasis.opendocument.graphics-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.graphics-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.graphics-template"),
    };

/// `application/vnd.oasis.opendocument.image`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_IMAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.image") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.image"),
    };

/// `application/vnd.oasis.opendocument.image-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_IMAGE_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.image-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.image-template"),
    };

/// `application/vnd.oasis.opendocument.presentation`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.presentation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.presentation"),
    };

/// `application/vnd.oasis.opendocument.presentation-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.presentation-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.presentation-template"),
    };

/// `application/vnd.oasis.opendocument.spreadsheet`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.spreadsheet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.spreadsheet"),
    };

/// `application/vnd.oasis.opendocument.spreadsheet-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.spreadsheet-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.spreadsheet-template"),
    };

/// `application/vnd.oasis.opendocument.text`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.text") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.text"),
    };

/// `application/vnd.oasis.opendocument.text-master`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_MASTER: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.text-master") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.text-master"),
    };

/// `application/vnd.oasis.opendocument.text-template`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.text-template") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.text-template"),
    };

/// `application/vnd.oasis.opendocument.text-web`
pub const APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT_WEB: Essence<&'static str> =
    match Essence::new_const("application/vnd.oasis.opendocument.text-web") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oasis.opendocument.text-web"),
    };

/// `application/vnd.obn`
pub const APPLICATION_VND_OBN: Essence<&'static str> =
    match Essence::new_const("application/vnd.obn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.obn"),
    };

/// `application/vnd.ocf+cbor`
pub const APPLICATION_VND_OCF_CBOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.ocf+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ocf+cbor"),
    };

/// `application/vnd.oci.image.manifest.v1+json`
pub const APPLICATION_VND_OCI_IMAGE_MANIFEST_V1_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.oci.image.manifest.v1+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oci.image.manifest.v1+json"),
    };

/// `application/vnd.oftn.l10n+json`
pub const APPLICATION_VND_OFTN_L10N_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.oftn.l10n+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oftn.l10n+json"),
    };

/// `application/vnd.oipf.contentaccessdownload+xml`
pub const APPLICATION_VND_OIPF_CONTENTACCESSDOWNLOAD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.contentaccessdownload+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.contentaccessdownload+xml"),
    };

/// `application/vnd.oipf.contentaccessstreaming+xml`
pub const APPLICATION_VND_OIPF_CONTENTACCESSSTREAMING_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.contentaccessstreaming+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.contentaccessstreaming+xml"),
    };

/// `application/vnd.oipf.cspg-hexbinary`
pub const APPLICATION_VND_OIPF_CSPG_HEXBINARY: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.cspg-hexbinary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.cspg-hexbinary"),
    };

/// `application/vnd.oipf.dae.svg+xml`
pub const APPLICATION_VND_OIPF_DAE_SVG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.dae.svg+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.dae.svg+xml"),
    };

/// `application/vnd.oipf.dae.xhtml+xml`
pub const APPLICATION_VND_OIPF_DAE_XHTML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.dae.xhtml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.dae.xhtml+xml"),
    };

/// `application/vnd.oipf.mippvcontrolmessage+xml`
pub const APPLICATION_VND_OIPF_MIPPVCONTROLMESSAGE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.mippvcontrolmessage+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.mippvcontrolmessage+xml"),
    };

/// `application/vnd.oipf.pae.gem`
pub const APPLICATION_VND_OIPF_PAE_GEM: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.pae.gem") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.pae.gem"),
    };

/// `application/vnd.oipf.spdiscovery+xml`
pub const APPLICATION_VND_OIPF_SPDISCOVERY_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.spdiscovery+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.spdiscovery+xml"),
    };

/// `application/vnd.oipf.spdlist+xml`
pub const APPLICATION_VND_OIPF_SPDLIST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.spdlist+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.spdlist+xml"),
    };

/// `application/vnd.oipf.ueprofile+xml`
pub const APPLICATION_VND_OIPF_UEPROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.ueprofile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.ueprofile+xml"),
    };

/// `application/vnd.oipf.userprofile+xml`
pub const APPLICATION_VND_OIPF_USERPROFILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oipf.userprofile+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oipf.userprofile+xml"),
    };

/// `application/vnd.olpc-sugar`
pub const APPLICATION_VND_OLPC_SUGAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.olpc-sugar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.olpc-sugar"),
    };

/// `application/vnd.oma.bcast.associated-procedure-parameter+xml`
pub const APPLICATION_VND_OMA_BCAST_ASSOCIATED_PROCEDURE_PARAMETER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.associated-procedure-parameter+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.associated-procedure-parameter+xml"),
    };

/// `application/vnd.oma.bcast.drm-trigger+xml`
pub const APPLICATION_VND_OMA_BCAST_DRM_TRIGGER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.drm-trigger+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.drm-trigger+xml"),
    };

/// `application/vnd.oma.bcast.imd+xml`
pub const APPLICATION_VND_OMA_BCAST_IMD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.imd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.imd+xml"),
    };

/// `application/vnd.oma.bcast.ltkm`
pub const APPLICATION_VND_OMA_BCAST_LTKM: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.ltkm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.ltkm"),
    };

/// `application/vnd.oma.bcast.notification+xml`
pub const APPLICATION_VND_OMA_BCAST_NOTIFICATION_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.notification+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.notification+xml"),
    };

/// `application/vnd.oma.bcast.provisioningtrigger`
pub const APPLICATION_VND_OMA_BCAST_PROVISIONINGTRIGGER: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.provisioningtrigger") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.provisioningtrigger"),
    };

/// `application/vnd.oma.bcast.sgboot`
pub const APPLICATION_VND_OMA_BCAST_SGBOOT: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.sgboot") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.sgboot"),
    };

/// `application/vnd.oma.bcast.sgdd+xml`
pub const APPLICATION_VND_OMA_BCAST_SGDD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.sgdd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.sgdd+xml"),
    };

/// `application/vnd.oma.bcast.sgdu`
pub const APPLICATION_VND_OMA_BCAST_SGDU: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.sgdu") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.sgdu"),
    };

/// `application/vnd.oma.bcast.simple-symbol-container`
pub const APPLICATION_VND_OMA_BCAST_SIMPLE_SYMBOL_CONTAINER: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.simple-symbol-container") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.simple-symbol-container"),
    };

/// `application/vnd.oma.bcast.smartcard-trigger+xml`
pub const APPLICATION_VND_OMA_BCAST_SMARTCARD_TRIGGER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.smartcard-trigger+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.smartcard-trigger+xml"),
    };

/// `application/vnd.oma.bcast.sprov+xml`
pub const APPLICATION_VND_OMA_BCAST_SPROV_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.sprov+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.sprov+xml"),
    };

/// `application/vnd.oma.bcast.stkm`
pub const APPLICATION_VND_OMA_BCAST_STKM: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.bcast.stkm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.bcast.stkm"),
    };

/// `application/vnd.oma.cab-address-book+xml`
pub const APPLICATION_VND_OMA_CAB_ADDRESS_BOOK_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.cab-address-book+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.cab-address-book+xml"),
    };

/// `application/vnd.oma.cab-feature-handler+xml`
pub const APPLICATION_VND_OMA_CAB_FEATURE_HANDLER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.cab-feature-handler+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.cab-feature-handler+xml"),
    };

/// `application/vnd.oma.cab-pcc+xml`
pub const APPLICATION_VND_OMA_CAB_PCC_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.cab-pcc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.cab-pcc+xml"),
    };

/// `application/vnd.oma.cab-subs-invite+xml`
pub const APPLICATION_VND_OMA_CAB_SUBS_INVITE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.cab-subs-invite+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.cab-subs-invite+xml"),
    };

/// `application/vnd.oma.cab-user-prefs+xml`
pub const APPLICATION_VND_OMA_CAB_USER_PREFS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.cab-user-prefs+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.cab-user-prefs+xml"),
    };

/// `application/vnd.oma.dcd`
pub const APPLICATION_VND_OMA_DCD: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.dcd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.dcd"),
    };

/// `application/vnd.oma.dcdc`
pub const APPLICATION_VND_OMA_DCDC: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.dcdc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.dcdc"),
    };

/// `application/vnd.oma.dd2+xml`
pub const APPLICATION_VND_OMA_DD2_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.dd2+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.dd2+xml"),
    };

/// `application/vnd.oma.drm.risd+xml`
pub const APPLICATION_VND_OMA_DRM_RISD_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.drm.risd+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.drm.risd+xml"),
    };

/// `application/vnd.oma.group-usage-list+xml`
pub const APPLICATION_VND_OMA_GROUP_USAGE_LIST_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.group-usage-list+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.group-usage-list+xml"),
    };

/// `application/vnd.oma.lwm2m+cbor`
pub const APPLICATION_VND_OMA_LWM2M_CBOR: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.lwm2m+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.lwm2m+cbor"),
    };

/// `application/vnd.oma.lwm2m+json`
pub const APPLICATION_VND_OMA_LWM2M_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.lwm2m+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.lwm2m+json"),
    };

/// `application/vnd.oma.lwm2m+tlv`
pub const APPLICATION_VND_OMA_LWM2M_TLV: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.lwm2m+tlv") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.lwm2m+tlv"),
    };

/// `application/vnd.oma.pal+xml`
pub const APPLICATION_VND_OMA_PAL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.pal+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.pal+xml"),
    };

/// `application/vnd.oma.poc.detailed-progress-report+xml`
pub const APPLICATION_VND_OMA_POC_DETAILED_PROGRESS_REPORT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.poc.detailed-progress-report+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.poc.detailed-progress-report+xml"),
    };

/// `application/vnd.oma.poc.final-report+xml`
pub const APPLICATION_VND_OMA_POC_FINAL_REPORT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.poc.final-report+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.poc.final-report+xml"),
    };

/// `application/vnd.oma.poc.groups+xml`
pub const APPLICATION_VND_OMA_POC_GROUPS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.poc.groups+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.poc.groups+xml"),
    };

/// `application/vnd.oma.poc.invocation-descriptor+xml`
pub const APPLICATION_VND_OMA_POC_INVOCATION_DESCRIPTOR_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.poc.invocation-descriptor+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.poc.invocation-descriptor+xml"),
    };

/// `application/vnd.oma.poc.optimized-progress-report+xml`
pub const APPLICATION_VND_OMA_POC_OPTIMIZED_PROGRESS_REPORT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.poc.optimized-progress-report+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.poc.optimized-progress-report+xml"),
    };

/// `application/vnd.oma.push`
pub const APPLICATION_VND_OMA_PUSH: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.push") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.push"),
    };

/// `application/vnd.oma.scidm.messages+xml`
pub const APPLICATION_VND_OMA_SCIDM_MESSAGES_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.scidm.messages+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.scidm.messages+xml"),
    };

/// `application/vnd.oma.xcap-directory+xml`
pub const APPLICATION_VND_OMA_XCAP_DIRECTORY_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma.xcap-directory+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma.xcap-directory+xml"),
    };

/// `application/vnd.omads-email+xml`
pub const APPLICATION_VND_OMADS_EMAIL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.omads-email+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.omads-email+xml"),
    };

/// `application/vnd.omads-file+xml`
pub const APPLICATION_VND_OMADS_FILE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.omads-file+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.omads-file+xml"),
    };

/// `application/vnd.omads-folder+xml`
pub const APPLICATION_VND_OMADS_FOLDER_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.omads-folder+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.omads-folder+xml"),
    };

/// `application/vnd.omaloc-supl-init`
pub const APPLICATION_VND_OMALOC_SUPL_INIT: Essence<&'static str> =
    match Essence::new_const("application/vnd.omaloc-supl-init") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.omaloc-supl-init"),
    };

/// `application/vnd.oma-scws-config`
pub const APPLICATION_VND_OMA_SCWS_CONFIG: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma-scws-config") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma-scws-config"),
    };

/// `application/vnd.oma-scws-http-request`
pub const APPLICATION_VND_OMA_SCWS_HTTP_REQUEST: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma-scws-http-request") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma-scws-http-request"),
    };

/// `application/vnd.oma-scws-http-response`
pub const APPLICATION_VND_OMA_SCWS_HTTP_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/vnd.oma-scws-http-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oma-scws-http-response"),
    };

/// `application/vnd.onepager`
pub const APPLICATION_VND_ONEPAGER: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepager") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepager"),
    };

/// `application/vnd.onepagertamp`
pub const APPLICATION_VND_ONEPAGERTAMP: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepagertamp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepagertamp"),
    };

/// `application/vnd.onepagertamx`
pub const APPLICATION_VND_ONEPAGERTAMX: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepagertamx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepagertamx"),
    };

/// `application/vnd.onepagertat`
pub const APPLICATION_VND_ONEPAGERTAT: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepagertat") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepagertat"),
    };

/// `application/vnd.onepagertatp`
pub const APPLICATION_VND_ONEPAGERTATP: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepagertatp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepagertatp"),
    };

/// `application/vnd.onepagertatx`
pub const APPLICATION_VND_ONEPAGERTATX: Essence<&'static str> =
    match Essence::new_const("application/vnd.onepagertatx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onepagertatx"),
    };

/// `application/vnd.onvif.metadata`
pub const APPLICATION_VND_ONVIF_METADATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.onvif.metadata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.onvif.metadata"),
    };

/// `application/vnd.openblox.game-binary`
pub const APPLICATION_VND_OPENBLOX_GAME_BINARY: Essence<&'static str> =
    match Essence::new_const("application/vnd.openblox.game-binary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openblox.game-binary"),
    };

/// `application/vnd.openblox.game+xml`
pub const APPLICATION_VND_OPENBLOX_GAME_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openblox.game+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openblox.game+xml"),
    };

/// `application/vnd.openeye.oeb`
pub const APPLICATION_VND_OPENEYE_OEB: Essence<&'static str> =
    match Essence::new_const("application/vnd.openeye.oeb") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openeye.oeb"),
    };

/// `application/vnd.openstreetmap.data+xml`
pub const APPLICATION_VND_OPENSTREETMAP_DATA_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openstreetmap.data+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openstreetmap.data+xml"),
    };

/// `application/vnd.opentimestamps.ots`
pub const APPLICATION_VND_OPENTIMESTAMPS_OTS: Essence<&'static str> =
    match Essence::new_const("application/vnd.opentimestamps.ots") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.opentimestamps.ots"),
    };

/// `application/vnd.openxmlformats-officedocument.custom-properties+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOM_PROPERTIES_XML: Essence<
    &'static str,
> = match Essence::new_const("application/vnd.openxmlformats-officedocument.custom-properties+xml")
{
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.custom-properties+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.customXmlProperties+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOMXMLPROPERTIES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.customXmlProperties+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.customXmlProperties+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.drawing+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWING_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.drawing+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.drawing+xml"),
    };

/// `application/vnd.openxmlformats-officedocument.drawingml.chart+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHART_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.drawingml.chart+xml") {
        Ok(essence) => essence,
        Err(..) => {
            panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.chart+xml")
        }
    };

/// `application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHARTSHAPES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMCOLORS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMDATA_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMLAYOUT_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMSTYLE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.extended-properties+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_EXTENDED_PROPERTIES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.extended-properties+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.extended-properties+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTAUTHORS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.comments+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.comments+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_HANDOUTMASTER_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESMASTER_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESSLIDE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.presentation`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.presentation")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION_MAIN_XML: Essence<&'static str> = match Essence::new_const("application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.presProps+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESPROPS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slide`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE: Essence<
    &'static str,
> = match Essence::new_const("application/vnd.openxmlformats-officedocument.presentationml.slide") {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.slide")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slide+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.slide+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDELAYOUT_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEMASTER_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slideshow`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.slideshow")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW_MAIN_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEUPDATEINFO_XML:
    Essence<&'static str> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TABLESTYLES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.tags+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TAGS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.tags+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.template`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.template",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.presentationml.template")
    }
};

/// `application/vnd.openxmlformats-officedocument.presentationml.template.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE_MAIN_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.template.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_VIEWPROPS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CALCCHAIN_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CHARTSHEET_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_COMMENTS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CONNECTIONS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_DIALOGSHEET_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_EXTERNALLINK_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHEDEFINITION_XML: Essence<&'static str> = match Essence::new_const("application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHERECORDS_XML: Essence<&'static str> = match Essence::new_const("application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTTABLE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_QUERYTABLE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONHEADERS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONLOG_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHAREDSTRINGS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet") {
        Ok(essence) => essence,
        Err(..) => {
            panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        }
    };

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET_MAIN_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEETMETADATA_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_STYLES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLESINGLECELLS_XML:
    Essence<&'static str> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.template`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE: Essence<
    &'static str,
> = match Essence::new_const("application/vnd.openxmlformats-officedocument.spreadsheetml.template")
{
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.template")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE_MAIN_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_USERNAMES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_VOLATILEDEPENDENCIES_XML: Essence<&'static str> = match Essence::new_const("application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"),
};

/// `application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_WORKSHEET_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.theme+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEME_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.theme+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.theme+xml"),
    };

/// `application/vnd.openxmlformats-officedocument.themeOverride+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEMEOVERRIDE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.themeOverride+xml") {
        Ok(essence) => essence,
        Err(..) => {
            panic!("invalid: application/vnd.openxmlformats-officedocument.themeOverride+xml")
        }
    };

/// `application/vnd.openxmlformats-officedocument.vmlDrawing`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_VMLDRAWING: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-officedocument.vmlDrawing") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.vmlDrawing"),
    };

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_COMMENTS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.document")
    }
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_GLOSSARY_XML: Essence<&'static str> = match Essence::new_const("application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_MAIN_XML:
    Essence<&'static str> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_ENDNOTES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FONTTABLE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTER_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTNOTES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_NUMBERING_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_SETTINGS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_STYLES_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml")
    }
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.template`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.template")
    }
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE_MAIN_XML:
    Essence<&'static str> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml"
    ),
};

/// `application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_WEBSETTINGS_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
) {
    Ok(essence) => essence,
    Err(..) => panic!(
        "invalid: application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
    ),
};

/// `application/vnd.openxmlformats-package.core-properties+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_PACKAGE_CORE_PROPERTIES_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-package.core-properties+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openxmlformats-package.core-properties+xml"),
    };

/// `application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_PACKAGE_DIGITAL_SIGNATURE_XMLSIGNATURE_XML: Essence<
    &'static str,
> = match Essence::new_const(
    "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
) {
    Ok(essence) => essence,
    Err(..) => {
        panic!("invalid: application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml")
    }
};

/// `application/vnd.openxmlformats-package.relationships+xml`
pub const APPLICATION_VND_OPENXMLFORMATS_PACKAGE_RELATIONSHIPS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.openxmlformats-package.relationships+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.openxmlformats-package.relationships+xml"),
    };

/// `application/vnd.oracle.resource+json`
pub const APPLICATION_VND_ORACLE_RESOURCE_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.oracle.resource+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oracle.resource+json"),
    };

/// `application/vnd.orange.indata`
pub const APPLICATION_VND_ORANGE_INDATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.orange.indata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.orange.indata"),
    };

/// `application/vnd.osa.netdeploy`
pub const APPLICATION_VND_OSA_NETDEPLOY: Essence<&'static str> =
    match Essence::new_const("application/vnd.osa.netdeploy") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.osa.netdeploy"),
    };

/// `application/vnd.osgeo.mapguide.package`
pub const APPLICATION_VND_OSGEO_MAPGUIDE_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.osgeo.mapguide.package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.osgeo.mapguide.package"),
    };

/// `application/vnd.osgi.bundle`
pub const APPLICATION_VND_OSGI_BUNDLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.osgi.bundle") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.osgi.bundle"),
    };

/// `application/vnd.osgi.dp`
pub const APPLICATION_VND_OSGI_DP: Essence<&'static str> =
    match Essence::new_const("application/vnd.osgi.dp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.osgi.dp"),
    };

/// `application/vnd.osgi.subsystem`
pub const APPLICATION_VND_OSGI_SUBSYSTEM: Essence<&'static str> =
    match Essence::new_const("application/vnd.osgi.subsystem") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.osgi.subsystem"),
    };

/// `application/vnd.otps.ct-kip+xml`
pub const APPLICATION_VND_OTPS_CT_KIP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.otps.ct-kip+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.otps.ct-kip+xml"),
    };

/// `application/vnd.oxli.countgraph`
pub const APPLICATION_VND_OXLI_COUNTGRAPH: Essence<&'static str> =
    match Essence::new_const("application/vnd.oxli.countgraph") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.oxli.countgraph"),
    };

/// `application/vnd.pagerduty+json`
pub const APPLICATION_VND_PAGERDUTY_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.pagerduty+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pagerduty+json"),
    };

/// `application/vnd.palm`
pub const APPLICATION_VND_PALM: Essence<&'static str> =
    match Essence::new_const("application/vnd.palm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.palm"),
    };

/// `application/vnd.panoply`
pub const APPLICATION_VND_PANOPLY: Essence<&'static str> =
    match Essence::new_const("application/vnd.panoply") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.panoply"),
    };

/// `application/vnd.paos.xml`
pub const APPLICATION_VND_PAOS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.paos.xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.paos.xml"),
    };

/// `application/vnd.patentdive`
pub const APPLICATION_VND_PATENTDIVE: Essence<&'static str> =
    match Essence::new_const("application/vnd.patentdive") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.patentdive"),
    };

/// `application/vnd.patientecommsdoc`
pub const APPLICATION_VND_PATIENTECOMMSDOC: Essence<&'static str> =
    match Essence::new_const("application/vnd.patientecommsdoc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.patientecommsdoc"),
    };

/// `application/vnd.pawaafile`
pub const APPLICATION_VND_PAWAAFILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.pawaafile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pawaafile"),
    };

/// `application/vnd.pcos`
pub const APPLICATION_VND_PCOS: Essence<&'static str> =
    match Essence::new_const("application/vnd.pcos") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pcos"),
    };

/// `application/vnd.pg.format`
pub const APPLICATION_VND_PG_FORMAT: Essence<&'static str> =
    match Essence::new_const("application/vnd.pg.format") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pg.format"),
    };

/// `application/vnd.pg.osasli`
pub const APPLICATION_VND_PG_OSASLI: Essence<&'static str> =
    match Essence::new_const("application/vnd.pg.osasli") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pg.osasli"),
    };

/// `application/vnd.piaccess.application-licence`
pub const APPLICATION_VND_PIACCESS_APPLICATION_LICENCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.piaccess.application-licence") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.piaccess.application-licence"),
    };

/// `application/vnd.picsel`
pub const APPLICATION_VND_PICSEL: Essence<&'static str> =
    match Essence::new_const("application/vnd.picsel") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.picsel"),
    };

/// `application/vnd.pmi.widget`
pub const APPLICATION_VND_PMI_WIDGET: Essence<&'static str> =
    match Essence::new_const("application/vnd.pmi.widget") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pmi.widget"),
    };

/// `application/vnd.poc.group-advertisement+xml`
pub const APPLICATION_VND_POC_GROUP_ADVERTISEMENT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.poc.group-advertisement+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.poc.group-advertisement+xml"),
    };

/// `application/vnd.pocketlearn`
pub const APPLICATION_VND_POCKETLEARN: Essence<&'static str> =
    match Essence::new_const("application/vnd.pocketlearn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pocketlearn"),
    };

/// `application/vnd.powerbuilder6`
pub const APPLICATION_VND_POWERBUILDER6: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder6") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder6"),
    };

/// `application/vnd.powerbuilder6-s`
pub const APPLICATION_VND_POWERBUILDER6_S: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder6-s") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder6-s"),
    };

/// `application/vnd.powerbuilder7`
pub const APPLICATION_VND_POWERBUILDER7: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder7") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder7"),
    };

/// `application/vnd.powerbuilder75`
pub const APPLICATION_VND_POWERBUILDER75: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder75") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder75"),
    };

/// `application/vnd.powerbuilder75-s`
pub const APPLICATION_VND_POWERBUILDER75_S: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder75-s") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder75-s"),
    };

/// `application/vnd.powerbuilder7-s`
pub const APPLICATION_VND_POWERBUILDER7_S: Essence<&'static str> =
    match Essence::new_const("application/vnd.powerbuilder7-s") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.powerbuilder7-s"),
    };

/// `application/vnd.preminet`
pub const APPLICATION_VND_PREMINET: Essence<&'static str> =
    match Essence::new_const("application/vnd.preminet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.preminet"),
    };

/// `application/vnd.previewsystems.box`
pub const APPLICATION_VND_PREVIEWSYSTEMS_BOX: Essence<&'static str> =
    match Essence::new_const("application/vnd.previewsystems.box") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.previewsystems.box"),
    };

/// `application/vnd.proteus.magazine`
pub const APPLICATION_VND_PROTEUS_MAGAZINE: Essence<&'static str> =
    match Essence::new_const("application/vnd.proteus.magazine") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.proteus.magazine"),
    };

/// `application/vnd.psfs`
pub const APPLICATION_VND_PSFS: Essence<&'static str> =
    match Essence::new_const("application/vnd.psfs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.psfs"),
    };

/// `application/vnd.publishare-delta-tree`
pub const APPLICATION_VND_PUBLISHARE_DELTA_TREE: Essence<&'static str> =
    match Essence::new_const("application/vnd.publishare-delta-tree") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.publishare-delta-tree"),
    };

/// `application/vnd.pvi.ptid1`
pub const APPLICATION_VND_PVI_PTID1: Essence<&'static str> =
    match Essence::new_const("application/vnd.pvi.ptid1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pvi.ptid1"),
    };

/// `application/vnd.pwg-multiplexed`
pub const APPLICATION_VND_PWG_MULTIPLEXED: Essence<&'static str> =
    match Essence::new_const("application/vnd.pwg-multiplexed") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pwg-multiplexed"),
    };

/// `application/vnd.pwg-xhtml-print+xml`
pub const APPLICATION_VND_PWG_XHTML_PRINT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.pwg-xhtml-print+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.pwg-xhtml-print+xml"),
    };

/// `application/vnd.qualcomm.brew-app-res`
pub const APPLICATION_VND_QUALCOMM_BREW_APP_RES: Essence<&'static str> =
    match Essence::new_const("application/vnd.qualcomm.brew-app-res") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.qualcomm.brew-app-res"),
    };

/// `application/vnd.quarantainenet`
pub const APPLICATION_VND_QUARANTAINENET: Essence<&'static str> =
    match Essence::new_const("application/vnd.quarantainenet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.quarantainenet"),
    };

/// `application/vnd.Quark.QuarkXPress`
pub const APPLICATION_VND_QUARK_QUARKXPRESS: Essence<&'static str> =
    match Essence::new_const("application/vnd.Quark.QuarkXPress") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.Quark.QuarkXPress"),
    };

/// `application/vnd.quobject-quoxdocument`
pub const APPLICATION_VND_QUOBJECT_QUOXDOCUMENT: Essence<&'static str> =
    match Essence::new_const("application/vnd.quobject-quoxdocument") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.quobject-quoxdocument"),
    };

/// `application/vnd.radisys.moml+xml`
pub const APPLICATION_VND_RADISYS_MOML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.moml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.moml+xml"),
    };

/// `application/vnd.radisys.msml-audit-conf+xml`
pub const APPLICATION_VND_RADISYS_MSML_AUDIT_CONF_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-audit-conf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-audit-conf+xml"),
    };

/// `application/vnd.radisys.msml-audit-conn+xml`
pub const APPLICATION_VND_RADISYS_MSML_AUDIT_CONN_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-audit-conn+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-audit-conn+xml"),
    };

/// `application/vnd.radisys.msml-audit-dialog+xml`
pub const APPLICATION_VND_RADISYS_MSML_AUDIT_DIALOG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-audit-dialog+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-audit-dialog+xml"),
    };

/// `application/vnd.radisys.msml-audit-stream+xml`
pub const APPLICATION_VND_RADISYS_MSML_AUDIT_STREAM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-audit-stream+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-audit-stream+xml"),
    };

/// `application/vnd.radisys.msml-audit+xml`
pub const APPLICATION_VND_RADISYS_MSML_AUDIT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-audit+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-audit+xml"),
    };

/// `application/vnd.radisys.msml-conf+xml`
pub const APPLICATION_VND_RADISYS_MSML_CONF_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-conf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-conf+xml"),
    };

/// `application/vnd.radisys.msml-dialog-base+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_BASE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-base+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-base+xml"),
    };

/// `application/vnd.radisys.msml-dialog-fax-detect+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_FAX_DETECT_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-fax-detect+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-fax-detect+xml"),
    };

/// `application/vnd.radisys.msml-dialog-fax-sendrecv+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_FAX_SENDRECV_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-fax-sendrecv+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-fax-sendrecv+xml"),
    };

/// `application/vnd.radisys.msml-dialog-group+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_GROUP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-group+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-group+xml"),
    };

/// `application/vnd.radisys.msml-dialog-speech+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_SPEECH_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-speech+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-speech+xml"),
    };

/// `application/vnd.radisys.msml-dialog-transform+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_TRANSFORM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog-transform+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog-transform+xml"),
    };

/// `application/vnd.radisys.msml-dialog+xml`
pub const APPLICATION_VND_RADISYS_MSML_DIALOG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml-dialog+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml-dialog+xml"),
    };

/// `application/vnd.radisys.msml+xml`
pub const APPLICATION_VND_RADISYS_MSML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.radisys.msml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.radisys.msml+xml"),
    };

/// `application/vnd.rainstor.data`
pub const APPLICATION_VND_RAINSTOR_DATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.rainstor.data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.rainstor.data"),
    };

/// `application/vnd.rapid`
pub const APPLICATION_VND_RAPID: Essence<&'static str> =
    match Essence::new_const("application/vnd.rapid") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.rapid"),
    };

/// `application/vnd.rar`
pub const APPLICATION_VND_RAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.rar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.rar"),
    };

/// `application/vnd.realvnc.bed`
pub const APPLICATION_VND_REALVNC_BED: Essence<&'static str> =
    match Essence::new_const("application/vnd.realvnc.bed") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.realvnc.bed"),
    };

/// `application/vnd.recordare.musicxml`
pub const APPLICATION_VND_RECORDARE_MUSICXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.recordare.musicxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.recordare.musicxml"),
    };

/// `application/vnd.recordare.musicxml+xml`
pub const APPLICATION_VND_RECORDARE_MUSICXML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.recordare.musicxml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.recordare.musicxml+xml"),
    };

/// `application/vnd.RenLearn.rlprint`
pub const APPLICATION_VND_RENLEARN_RLPRINT: Essence<&'static str> =
    match Essence::new_const("application/vnd.RenLearn.rlprint") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.RenLearn.rlprint"),
    };

/// `application/vnd.resilient.logic`
pub const APPLICATION_VND_RESILIENT_LOGIC: Essence<&'static str> =
    match Essence::new_const("application/vnd.resilient.logic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.resilient.logic"),
    };

/// `application/vnd.restful+json`
pub const APPLICATION_VND_RESTFUL_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.restful+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.restful+json"),
    };

/// `application/vnd.rig.cryptonote`
pub const APPLICATION_VND_RIG_CRYPTONOTE: Essence<&'static str> =
    match Essence::new_const("application/vnd.rig.cryptonote") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.rig.cryptonote"),
    };

/// `application/vnd.route66.link66+xml`
pub const APPLICATION_VND_ROUTE66_LINK66_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.route66.link66+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.route66.link66+xml"),
    };

/// `application/vnd.rs-274x`
pub const APPLICATION_VND_RS_274X: Essence<&'static str> =
    match Essence::new_const("application/vnd.rs-274x") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.rs-274x"),
    };

/// `application/vnd.ruckus.download`
pub const APPLICATION_VND_RUCKUS_DOWNLOAD: Essence<&'static str> =
    match Essence::new_const("application/vnd.ruckus.download") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ruckus.download"),
    };

/// `application/vnd.s3sms`
pub const APPLICATION_VND_S3SMS: Essence<&'static str> =
    match Essence::new_const("application/vnd.s3sms") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.s3sms"),
    };

/// `application/vnd.sailingtracker.track`
pub const APPLICATION_VND_SAILINGTRACKER_TRACK: Essence<&'static str> =
    match Essence::new_const("application/vnd.sailingtracker.track") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sailingtracker.track"),
    };

/// `application/vnd.sar`
pub const APPLICATION_VND_SAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.sar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sar"),
    };

/// `application/vnd.sbm.cid`
pub const APPLICATION_VND_SBM_CID: Essence<&'static str> =
    match Essence::new_const("application/vnd.sbm.cid") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sbm.cid"),
    };

/// `application/vnd.sbm.mid2`
pub const APPLICATION_VND_SBM_MID2: Essence<&'static str> =
    match Essence::new_const("application/vnd.sbm.mid2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sbm.mid2"),
    };

/// `application/vnd.scribus`
pub const APPLICATION_VND_SCRIBUS: Essence<&'static str> =
    match Essence::new_const("application/vnd.scribus") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.scribus"),
    };

/// `application/vnd.sealed.3df`
pub const APPLICATION_VND_SEALED_3DF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.3df") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.3df"),
    };

/// `application/vnd.sealed.csf`
pub const APPLICATION_VND_SEALED_CSF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.csf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.csf"),
    };

/// `application/vnd.sealed.doc`
pub const APPLICATION_VND_SEALED_DOC: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.doc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.doc"),
    };

/// `application/vnd.sealed.eml`
pub const APPLICATION_VND_SEALED_EML: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.eml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.eml"),
    };

/// `application/vnd.sealed.mht`
pub const APPLICATION_VND_SEALED_MHT: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.mht") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.mht"),
    };

/// `application/vnd.sealed.net`
pub const APPLICATION_VND_SEALED_NET: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.net") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.net"),
    };

/// `application/vnd.sealed.ppt`
pub const APPLICATION_VND_SEALED_PPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.ppt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.ppt"),
    };

/// `application/vnd.sealed.tiff`
pub const APPLICATION_VND_SEALED_TIFF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.tiff") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.tiff"),
    };

/// `application/vnd.sealed.xls`
pub const APPLICATION_VND_SEALED_XLS: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealed.xls") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealed.xls"),
    };

/// `application/vnd.sealedmedia.softseal.html`
pub const APPLICATION_VND_SEALEDMEDIA_SOFTSEAL_HTML: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealedmedia.softseal.html") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealedmedia.softseal.html"),
    };

/// `application/vnd.sealedmedia.softseal.pdf`
pub const APPLICATION_VND_SEALEDMEDIA_SOFTSEAL_PDF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sealedmedia.softseal.pdf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sealedmedia.softseal.pdf"),
    };

/// `application/vnd.seemail`
pub const APPLICATION_VND_SEEMAIL: Essence<&'static str> =
    match Essence::new_const("application/vnd.seemail") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.seemail"),
    };

/// `application/vnd.seis+json`
pub const APPLICATION_VND_SEIS_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.seis+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.seis+json"),
    };

/// `application/vnd.sema`
pub const APPLICATION_VND_SEMA: Essence<&'static str> =
    match Essence::new_const("application/vnd.sema") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sema"),
    };

/// `application/vnd.semd`
pub const APPLICATION_VND_SEMD: Essence<&'static str> =
    match Essence::new_const("application/vnd.semd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.semd"),
    };

/// `application/vnd.semf`
pub const APPLICATION_VND_SEMF: Essence<&'static str> =
    match Essence::new_const("application/vnd.semf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.semf"),
    };

/// `application/vnd.shade-save-file`
pub const APPLICATION_VND_SHADE_SAVE_FILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.shade-save-file") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shade-save-file"),
    };

/// `application/vnd.shana.informed.formdata`
pub const APPLICATION_VND_SHANA_INFORMED_FORMDATA: Essence<&'static str> =
    match Essence::new_const("application/vnd.shana.informed.formdata") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shana.informed.formdata"),
    };

/// `application/vnd.shana.informed.formtemplate`
pub const APPLICATION_VND_SHANA_INFORMED_FORMTEMPLATE: Essence<&'static str> =
    match Essence::new_const("application/vnd.shana.informed.formtemplate") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shana.informed.formtemplate"),
    };

/// `application/vnd.shana.informed.interchange`
pub const APPLICATION_VND_SHANA_INFORMED_INTERCHANGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.shana.informed.interchange") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shana.informed.interchange"),
    };

/// `application/vnd.shana.informed.package`
pub const APPLICATION_VND_SHANA_INFORMED_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.shana.informed.package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shana.informed.package"),
    };

/// `application/vnd.shootproof+json`
pub const APPLICATION_VND_SHOOTPROOF_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.shootproof+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shootproof+json"),
    };

/// `application/vnd.shopkick+json`
pub const APPLICATION_VND_SHOPKICK_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.shopkick+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shopkick+json"),
    };

/// `application/vnd.shp`
pub const APPLICATION_VND_SHP: Essence<&'static str> =
    match Essence::new_const("application/vnd.shp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shp"),
    };

/// `application/vnd.shx`
pub const APPLICATION_VND_SHX: Essence<&'static str> =
    match Essence::new_const("application/vnd.shx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.shx"),
    };

/// `application/vnd.sigrok.session`
pub const APPLICATION_VND_SIGROK_SESSION: Essence<&'static str> =
    match Essence::new_const("application/vnd.sigrok.session") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sigrok.session"),
    };

/// `application/vnd.SimTech-MindMapper`
pub const APPLICATION_VND_SIMTECH_MINDMAPPER: Essence<&'static str> =
    match Essence::new_const("application/vnd.SimTech-MindMapper") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.SimTech-MindMapper"),
    };

/// `application/vnd.siren+json`
pub const APPLICATION_VND_SIREN_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.siren+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.siren+json"),
    };

/// `application/vnd.smaf`
pub const APPLICATION_VND_SMAF: Essence<&'static str> =
    match Essence::new_const("application/vnd.smaf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.smaf"),
    };

/// `application/vnd.smart.notebook`
pub const APPLICATION_VND_SMART_NOTEBOOK: Essence<&'static str> =
    match Essence::new_const("application/vnd.smart.notebook") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.smart.notebook"),
    };

/// `application/vnd.smart.teacher`
pub const APPLICATION_VND_SMART_TEACHER: Essence<&'static str> =
    match Essence::new_const("application/vnd.smart.teacher") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.smart.teacher"),
    };

/// `application/vnd.snesdev-page-table`
pub const APPLICATION_VND_SNESDEV_PAGE_TABLE: Essence<&'static str> =
    match Essence::new_const("application/vnd.snesdev-page-table") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.snesdev-page-table"),
    };

/// `application/vnd.software602.filler.form+xml`
pub const APPLICATION_VND_SOFTWARE602_FILLER_FORM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.software602.filler.form+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.software602.filler.form+xml"),
    };

/// `application/vnd.software602.filler.form-xml-zip`
pub const APPLICATION_VND_SOFTWARE602_FILLER_FORM_XML_ZIP: Essence<&'static str> =
    match Essence::new_const("application/vnd.software602.filler.form-xml-zip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.software602.filler.form-xml-zip"),
    };

/// `application/vnd.solent.sdkm+xml`
pub const APPLICATION_VND_SOLENT_SDKM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.solent.sdkm+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.solent.sdkm+xml"),
    };

/// `application/vnd.spotfire.dxp`
pub const APPLICATION_VND_SPOTFIRE_DXP: Essence<&'static str> =
    match Essence::new_const("application/vnd.spotfire.dxp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.spotfire.dxp"),
    };

/// `application/vnd.spotfire.sfs`
pub const APPLICATION_VND_SPOTFIRE_SFS: Essence<&'static str> =
    match Essence::new_const("application/vnd.spotfire.sfs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.spotfire.sfs"),
    };

/// `application/vnd.sqlite3`
pub const APPLICATION_VND_SQLITE3: Essence<&'static str> =
    match Essence::new_const("application/vnd.sqlite3") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sqlite3"),
    };

/// `application/vnd.sss-cod`
pub const APPLICATION_VND_SSS_COD: Essence<&'static str> =
    match Essence::new_const("application/vnd.sss-cod") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sss-cod"),
    };

/// `application/vnd.sss-dtf`
pub const APPLICATION_VND_SSS_DTF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sss-dtf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sss-dtf"),
    };

/// `application/vnd.sss-ntf`
pub const APPLICATION_VND_SSS_NTF: Essence<&'static str> =
    match Essence::new_const("application/vnd.sss-ntf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sss-ntf"),
    };

/// `application/vnd.stepmania.package`
pub const APPLICATION_VND_STEPMANIA_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.stepmania.package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.stepmania.package"),
    };

/// `application/vnd.stepmania.stepchart`
pub const APPLICATION_VND_STEPMANIA_STEPCHART: Essence<&'static str> =
    match Essence::new_const("application/vnd.stepmania.stepchart") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.stepmania.stepchart"),
    };

/// `application/vnd.street-stream`
pub const APPLICATION_VND_STREET_STREAM: Essence<&'static str> =
    match Essence::new_const("application/vnd.street-stream") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.street-stream"),
    };

/// `application/vnd.sun.wadl+xml`
pub const APPLICATION_VND_SUN_WADL_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.sun.wadl+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sun.wadl+xml"),
    };

/// `application/vnd.sus-calendar`
pub const APPLICATION_VND_SUS_CALENDAR: Essence<&'static str> =
    match Essence::new_const("application/vnd.sus-calendar") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sus-calendar"),
    };

/// `application/vnd.svd`
pub const APPLICATION_VND_SVD: Essence<&'static str> =
    match Essence::new_const("application/vnd.svd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.svd"),
    };

/// `application/vnd.swiftview-ics`
pub const APPLICATION_VND_SWIFTVIEW_ICS: Essence<&'static str> =
    match Essence::new_const("application/vnd.swiftview-ics") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.swiftview-ics"),
    };

/// `application/vnd.sycle+xml`
pub const APPLICATION_VND_SYCLE_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.sycle+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.sycle+xml"),
    };

/// `application/vnd.syft+json`
pub const APPLICATION_VND_SYFT_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.syft+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syft+json"),
    };

/// `application/vnd.syncml.dm.notification`
pub const APPLICATION_VND_SYNCML_DM_NOTIFICATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dm.notification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dm.notification"),
    };

/// `application/vnd.syncml.dmddf+xml`
pub const APPLICATION_VND_SYNCML_DMDDF_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dmddf+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dmddf+xml"),
    };

/// `application/vnd.syncml.dmtnds+wbxml`
pub const APPLICATION_VND_SYNCML_DMTNDS_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dmtnds+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dmtnds+wbxml"),
    };

/// `application/vnd.syncml.dmtnds+xml`
pub const APPLICATION_VND_SYNCML_DMTNDS_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dmtnds+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dmtnds+xml"),
    };

/// `application/vnd.syncml.dmddf+wbxml`
pub const APPLICATION_VND_SYNCML_DMDDF_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dmddf+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dmddf+wbxml"),
    };

/// `application/vnd.syncml.dm+wbxml`
pub const APPLICATION_VND_SYNCML_DM_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dm+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dm+wbxml"),
    };

/// `application/vnd.syncml.dm+xml`
pub const APPLICATION_VND_SYNCML_DM_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.dm+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.dm+xml"),
    };

/// `application/vnd.syncml.ds.notification`
pub const APPLICATION_VND_SYNCML_DS_NOTIFICATION: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml.ds.notification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml.ds.notification"),
    };

/// `application/vnd.syncml+xml`
pub const APPLICATION_VND_SYNCML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.syncml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.syncml+xml"),
    };

/// `application/vnd.tableschema+json`
pub const APPLICATION_VND_TABLESCHEMA_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.tableschema+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tableschema+json"),
    };

/// `application/vnd.tao.intent-module-archive`
pub const APPLICATION_VND_TAO_INTENT_MODULE_ARCHIVE: Essence<&'static str> =
    match Essence::new_const("application/vnd.tao.intent-module-archive") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tao.intent-module-archive"),
    };

/// `application/vnd.tcpdump.pcap`
pub const APPLICATION_VND_TCPDUMP_PCAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.tcpdump.pcap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tcpdump.pcap"),
    };

/// `application/vnd.think-cell.ppttc+json`
pub const APPLICATION_VND_THINK_CELL_PPTTC_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.think-cell.ppttc+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.think-cell.ppttc+json"),
    };

/// `application/vnd.tml`
pub const APPLICATION_VND_TML: Essence<&'static str> =
    match Essence::new_const("application/vnd.tml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tml"),
    };

/// `application/vnd.tmd.mediaflex.api+xml`
pub const APPLICATION_VND_TMD_MEDIAFLEX_API_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.tmd.mediaflex.api+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tmd.mediaflex.api+xml"),
    };

/// `application/vnd.tmobile-livetv`
pub const APPLICATION_VND_TMOBILE_LIVETV: Essence<&'static str> =
    match Essence::new_const("application/vnd.tmobile-livetv") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tmobile-livetv"),
    };

/// `application/vnd.tri.onesource`
pub const APPLICATION_VND_TRI_ONESOURCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.tri.onesource") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.tri.onesource"),
    };

/// `application/vnd.trid.tpt`
pub const APPLICATION_VND_TRID_TPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.trid.tpt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.trid.tpt"),
    };

/// `application/vnd.triscape.mxs`
pub const APPLICATION_VND_TRISCAPE_MXS: Essence<&'static str> =
    match Essence::new_const("application/vnd.triscape.mxs") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.triscape.mxs"),
    };

/// `application/vnd.trueapp`
pub const APPLICATION_VND_TRUEAPP: Essence<&'static str> =
    match Essence::new_const("application/vnd.trueapp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.trueapp"),
    };

/// `application/vnd.truedoc`
pub const APPLICATION_VND_TRUEDOC: Essence<&'static str> =
    match Essence::new_const("application/vnd.truedoc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.truedoc"),
    };

/// `application/vnd.ubisoft.webplayer`
pub const APPLICATION_VND_UBISOFT_WEBPLAYER: Essence<&'static str> =
    match Essence::new_const("application/vnd.ubisoft.webplayer") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ubisoft.webplayer"),
    };

/// `application/vnd.ufdl`
pub const APPLICATION_VND_UFDL: Essence<&'static str> =
    match Essence::new_const("application/vnd.ufdl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ufdl"),
    };

/// `application/vnd.uiq.theme`
pub const APPLICATION_VND_UIQ_THEME: Essence<&'static str> =
    match Essence::new_const("application/vnd.uiq.theme") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uiq.theme"),
    };

/// `application/vnd.umajin`
pub const APPLICATION_VND_UMAJIN: Essence<&'static str> =
    match Essence::new_const("application/vnd.umajin") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.umajin"),
    };

/// `application/vnd.unity`
pub const APPLICATION_VND_UNITY: Essence<&'static str> =
    match Essence::new_const("application/vnd.unity") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.unity"),
    };

/// `application/vnd.uoml+xml`
pub const APPLICATION_VND_UOML_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uoml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uoml+xml"),
    };

/// `application/vnd.uplanet.alert`
pub const APPLICATION_VND_UPLANET_ALERT: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.alert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.alert"),
    };

/// `application/vnd.uplanet.alert-wbxml`
pub const APPLICATION_VND_UPLANET_ALERT_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.alert-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.alert-wbxml"),
    };

/// `application/vnd.uplanet.bearer-choice`
pub const APPLICATION_VND_UPLANET_BEARER_CHOICE: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.bearer-choice") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.bearer-choice"),
    };

/// `application/vnd.uplanet.bearer-choice-wbxml`
pub const APPLICATION_VND_UPLANET_BEARER_CHOICE_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.bearer-choice-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.bearer-choice-wbxml"),
    };

/// `application/vnd.uplanet.cacheop`
pub const APPLICATION_VND_UPLANET_CACHEOP: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.cacheop") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.cacheop"),
    };

/// `application/vnd.uplanet.cacheop-wbxml`
pub const APPLICATION_VND_UPLANET_CACHEOP_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.cacheop-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.cacheop-wbxml"),
    };

/// `application/vnd.uplanet.channel`
pub const APPLICATION_VND_UPLANET_CHANNEL: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.channel") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.channel"),
    };

/// `application/vnd.uplanet.channel-wbxml`
pub const APPLICATION_VND_UPLANET_CHANNEL_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.channel-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.channel-wbxml"),
    };

/// `application/vnd.uplanet.list`
pub const APPLICATION_VND_UPLANET_LIST: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.list") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.list"),
    };

/// `application/vnd.uplanet.listcmd`
pub const APPLICATION_VND_UPLANET_LISTCMD: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.listcmd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.listcmd"),
    };

/// `application/vnd.uplanet.listcmd-wbxml`
pub const APPLICATION_VND_UPLANET_LISTCMD_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.listcmd-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.listcmd-wbxml"),
    };

/// `application/vnd.uplanet.list-wbxml`
pub const APPLICATION_VND_UPLANET_LIST_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.list-wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.list-wbxml"),
    };

/// `application/vnd.uri-map`
pub const APPLICATION_VND_URI_MAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.uri-map") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uri-map"),
    };

/// `application/vnd.uplanet.signal`
pub const APPLICATION_VND_UPLANET_SIGNAL: Essence<&'static str> =
    match Essence::new_const("application/vnd.uplanet.signal") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.uplanet.signal"),
    };

/// `application/vnd.valve.source.material`
pub const APPLICATION_VND_VALVE_SOURCE_MATERIAL: Essence<&'static str> =
    match Essence::new_const("application/vnd.valve.source.material") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.valve.source.material"),
    };

/// `application/vnd.vcx`
pub const APPLICATION_VND_VCX: Essence<&'static str> =
    match Essence::new_const("application/vnd.vcx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vcx"),
    };

/// `application/vnd.vd-study`
pub const APPLICATION_VND_VD_STUDY: Essence<&'static str> =
    match Essence::new_const("application/vnd.vd-study") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vd-study"),
    };

/// `application/vnd.vectorworks`
pub const APPLICATION_VND_VECTORWORKS: Essence<&'static str> =
    match Essence::new_const("application/vnd.vectorworks") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vectorworks"),
    };

/// `application/vnd.vel+json`
pub const APPLICATION_VND_VEL_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.vel+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vel+json"),
    };

/// `application/vnd.verimatrix.vcas`
pub const APPLICATION_VND_VERIMATRIX_VCAS: Essence<&'static str> =
    match Essence::new_const("application/vnd.verimatrix.vcas") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.verimatrix.vcas"),
    };

/// `application/vnd.veritone.aion+json`
pub const APPLICATION_VND_VERITONE_AION_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.veritone.aion+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.veritone.aion+json"),
    };

/// `application/vnd.veryant.thin`
pub const APPLICATION_VND_VERYANT_THIN: Essence<&'static str> =
    match Essence::new_const("application/vnd.veryant.thin") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.veryant.thin"),
    };

/// `application/vnd.ves.encrypted`
pub const APPLICATION_VND_VES_ENCRYPTED: Essence<&'static str> =
    match Essence::new_const("application/vnd.ves.encrypted") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.ves.encrypted"),
    };

/// `application/vnd.vidsoft.vidconference`
pub const APPLICATION_VND_VIDSOFT_VIDCONFERENCE: Essence<&'static str> =
    match Essence::new_const("application/vnd.vidsoft.vidconference") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vidsoft.vidconference"),
    };

/// `application/vnd.visio`
pub const APPLICATION_VND_VISIO: Essence<&'static str> =
    match Essence::new_const("application/vnd.visio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.visio"),
    };

/// `application/vnd.visionary`
pub const APPLICATION_VND_VISIONARY: Essence<&'static str> =
    match Essence::new_const("application/vnd.visionary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.visionary"),
    };

/// `application/vnd.vividence.scriptfile`
pub const APPLICATION_VND_VIVIDENCE_SCRIPTFILE: Essence<&'static str> =
    match Essence::new_const("application/vnd.vividence.scriptfile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vividence.scriptfile"),
    };

/// `application/vnd.vsf`
pub const APPLICATION_VND_VSF: Essence<&'static str> =
    match Essence::new_const("application/vnd.vsf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.vsf"),
    };

/// `application/vnd.wap.sic`
pub const APPLICATION_VND_WAP_SIC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wap.sic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wap.sic"),
    };

/// `application/vnd.wap.slc`
pub const APPLICATION_VND_WAP_SLC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wap.slc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wap.slc"),
    };

/// `application/vnd.wap.wbxml`
pub const APPLICATION_VND_WAP_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.wap.wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wap.wbxml"),
    };

/// `application/vnd.wap.wmlc`
pub const APPLICATION_VND_WAP_WMLC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wap.wmlc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wap.wmlc"),
    };

/// `application/vnd.wap.wmlscriptc`
pub const APPLICATION_VND_WAP_WMLSCRIPTC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wap.wmlscriptc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wap.wmlscriptc"),
    };

/// `application/vnd.webturbo`
pub const APPLICATION_VND_WEBTURBO: Essence<&'static str> =
    match Essence::new_const("application/vnd.webturbo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.webturbo"),
    };

/// `application/vnd.wfa.dpp`
pub const APPLICATION_VND_WFA_DPP: Essence<&'static str> =
    match Essence::new_const("application/vnd.wfa.dpp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wfa.dpp"),
    };

/// `application/vnd.wfa.p2p`
pub const APPLICATION_VND_WFA_P2P: Essence<&'static str> =
    match Essence::new_const("application/vnd.wfa.p2p") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wfa.p2p"),
    };

/// `application/vnd.wfa.wsc`
pub const APPLICATION_VND_WFA_WSC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wfa.wsc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wfa.wsc"),
    };

/// `application/vnd.windows.devicepairing`
pub const APPLICATION_VND_WINDOWS_DEVICEPAIRING: Essence<&'static str> =
    match Essence::new_const("application/vnd.windows.devicepairing") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.windows.devicepairing"),
    };

/// `application/vnd.wmc`
pub const APPLICATION_VND_WMC: Essence<&'static str> =
    match Essence::new_const("application/vnd.wmc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wmc"),
    };

/// `application/vnd.wmf.bootstrap`
pub const APPLICATION_VND_WMF_BOOTSTRAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.wmf.bootstrap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wmf.bootstrap"),
    };

/// `application/vnd.wolfram.mathematica`
pub const APPLICATION_VND_WOLFRAM_MATHEMATICA: Essence<&'static str> =
    match Essence::new_const("application/vnd.wolfram.mathematica") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wolfram.mathematica"),
    };

/// `application/vnd.wolfram.mathematica.package`
pub const APPLICATION_VND_WOLFRAM_MATHEMATICA_PACKAGE: Essence<&'static str> =
    match Essence::new_const("application/vnd.wolfram.mathematica.package") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wolfram.mathematica.package"),
    };

/// `application/vnd.wolfram.player`
pub const APPLICATION_VND_WOLFRAM_PLAYER: Essence<&'static str> =
    match Essence::new_const("application/vnd.wolfram.player") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wolfram.player"),
    };

/// `application/vnd.wordperfect`
pub const APPLICATION_VND_WORDPERFECT: Essence<&'static str> =
    match Essence::new_const("application/vnd.wordperfect") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wordperfect"),
    };

/// `application/vnd.wqd`
pub const APPLICATION_VND_WQD: Essence<&'static str> =
    match Essence::new_const("application/vnd.wqd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wqd"),
    };

/// `application/vnd.wrq-hp3000-labelled`
pub const APPLICATION_VND_WRQ_HP3000_LABELLED: Essence<&'static str> =
    match Essence::new_const("application/vnd.wrq-hp3000-labelled") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wrq-hp3000-labelled"),
    };

/// `application/vnd.wt.stf`
pub const APPLICATION_VND_WT_STF: Essence<&'static str> =
    match Essence::new_const("application/vnd.wt.stf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wt.stf"),
    };

/// `application/vnd.wv.csp+xml`
pub const APPLICATION_VND_WV_CSP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.wv.csp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wv.csp+xml"),
    };

/// `application/vnd.wv.csp+wbxml`
pub const APPLICATION_VND_WV_CSP_WBXML: Essence<&'static str> =
    match Essence::new_const("application/vnd.wv.csp+wbxml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wv.csp+wbxml"),
    };

/// `application/vnd.wv.ssp+xml`
pub const APPLICATION_VND_WV_SSP_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.wv.ssp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.wv.ssp+xml"),
    };

/// `application/vnd.xacml+json`
pub const APPLICATION_VND_XACML_JSON: Essence<&'static str> =
    match Essence::new_const("application/vnd.xacml+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xacml+json"),
    };

/// `application/vnd.xara`
pub const APPLICATION_VND_XARA: Essence<&'static str> =
    match Essence::new_const("application/vnd.xara") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xara"),
    };

/// `application/vnd.xfdl`
pub const APPLICATION_VND_XFDL: Essence<&'static str> =
    match Essence::new_const("application/vnd.xfdl") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xfdl"),
    };

/// `application/vnd.xfdl.webform`
pub const APPLICATION_VND_XFDL_WEBFORM: Essence<&'static str> =
    match Essence::new_const("application/vnd.xfdl.webform") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xfdl.webform"),
    };

/// `application/vnd.xmi+xml`
pub const APPLICATION_VND_XMI_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmi+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmi+xml"),
    };

/// `application/vnd.xmpie.cpkg`
pub const APPLICATION_VND_XMPIE_CPKG: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmpie.cpkg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmpie.cpkg"),
    };

/// `application/vnd.xmpie.dpkg`
pub const APPLICATION_VND_XMPIE_DPKG: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmpie.dpkg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmpie.dpkg"),
    };

/// `application/vnd.xmpie.plan`
pub const APPLICATION_VND_XMPIE_PLAN: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmpie.plan") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmpie.plan"),
    };

/// `application/vnd.xmpie.ppkg`
pub const APPLICATION_VND_XMPIE_PPKG: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmpie.ppkg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmpie.ppkg"),
    };

/// `application/vnd.xmpie.xlim`
pub const APPLICATION_VND_XMPIE_XLIM: Essence<&'static str> =
    match Essence::new_const("application/vnd.xmpie.xlim") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.xmpie.xlim"),
    };

/// `application/vnd.yamaha.hv-dic`
pub const APPLICATION_VND_YAMAHA_HV_DIC: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.hv-dic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.hv-dic"),
    };

/// `application/vnd.yamaha.hv-script`
pub const APPLICATION_VND_YAMAHA_HV_SCRIPT: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.hv-script") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.hv-script"),
    };

/// `application/vnd.yamaha.hv-voice`
pub const APPLICATION_VND_YAMAHA_HV_VOICE: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.hv-voice") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.hv-voice"),
    };

/// `application/vnd.yamaha.openscoreformat.osfpvg+xml`
pub const APPLICATION_VND_YAMAHA_OPENSCOREFORMAT_OSFPVG_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.openscoreformat.osfpvg+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.openscoreformat.osfpvg+xml"),
    };

/// `application/vnd.yamaha.openscoreformat`
pub const APPLICATION_VND_YAMAHA_OPENSCOREFORMAT: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.openscoreformat") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.openscoreformat"),
    };

/// `application/vnd.yamaha.remote-setup`
pub const APPLICATION_VND_YAMAHA_REMOTE_SETUP: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.remote-setup") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.remote-setup"),
    };

/// `application/vnd.yamaha.smaf-audio`
pub const APPLICATION_VND_YAMAHA_SMAF_AUDIO: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.smaf-audio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.smaf-audio"),
    };

/// `application/vnd.yamaha.smaf-phrase`
pub const APPLICATION_VND_YAMAHA_SMAF_PHRASE: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.smaf-phrase") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.smaf-phrase"),
    };

/// `application/vnd.yamaha.through-ngn`
pub const APPLICATION_VND_YAMAHA_THROUGH_NGN: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.through-ngn") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.through-ngn"),
    };

/// `application/vnd.yamaha.tunnel-udpencap`
pub const APPLICATION_VND_YAMAHA_TUNNEL_UDPENCAP: Essence<&'static str> =
    match Essence::new_const("application/vnd.yamaha.tunnel-udpencap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yamaha.tunnel-udpencap"),
    };

/// `application/vnd.yaoweme`
pub const APPLICATION_VND_YAOWEME: Essence<&'static str> =
    match Essence::new_const("application/vnd.yaoweme") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yaoweme"),
    };

/// `application/vnd.yellowriver-custom-menu`
pub const APPLICATION_VND_YELLOWRIVER_CUSTOM_MENU: Essence<&'static str> =
    match Essence::new_const("application/vnd.yellowriver-custom-menu") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.yellowriver-custom-menu"),
    };

/// `application/vnd.youtube.yt`
pub const APPLICATION_VND_YOUTUBE_YT: Essence<&'static str> =
    match Essence::new_const("application/vnd.youtube.yt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.youtube.yt"),
    };

/// `application/vnd.zul`
pub const APPLICATION_VND_ZUL: Essence<&'static str> =
    match Essence::new_const("application/vnd.zul") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.zul"),
    };

/// `application/vnd.zzazz.deck+xml`
pub const APPLICATION_VND_ZZAZZ_DECK_XML: Essence<&'static str> =
    match Essence::new_const("application/vnd.zzazz.deck+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vnd.zzazz.deck+xml"),
    };

/// `application/voicexml+xml`
pub const APPLICATION_VOICEXML_XML: Essence<&'static str> =
    match Essence::new_const("application/voicexml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/voicexml+xml"),
    };

/// `application/voucher-cms+json`
pub const APPLICATION_VOUCHER_CMS_JSON: Essence<&'static str> =
    match Essence::new_const("application/voucher-cms+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/voucher-cms+json"),
    };

/// `application/vq-rtcpxr`
pub const APPLICATION_VQ_RTCPXR: Essence<&'static str> =
    match Essence::new_const("application/vq-rtcpxr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/vq-rtcpxr"),
    };

/// `application/wasm`
pub const APPLICATION_WASM: Essence<&'static str> = match Essence::new_const("application/wasm") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/wasm"),
};

/// `application/watcherinfo+xml`
pub const APPLICATION_WATCHERINFO_XML: Essence<&'static str> =
    match Essence::new_const("application/watcherinfo+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/watcherinfo+xml"),
    };

/// `application/webpush-options+json`
pub const APPLICATION_WEBPUSH_OPTIONS_JSON: Essence<&'static str> =
    match Essence::new_const("application/webpush-options+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/webpush-options+json"),
    };

/// `application/whoispp-query`
pub const APPLICATION_WHOISPP_QUERY: Essence<&'static str> =
    match Essence::new_const("application/whoispp-query") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/whoispp-query"),
    };

/// `application/whoispp-response`
pub const APPLICATION_WHOISPP_RESPONSE: Essence<&'static str> =
    match Essence::new_const("application/whoispp-response") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/whoispp-response"),
    };

/// `application/widget`
pub const APPLICATION_WIDGET: Essence<&'static str> = match Essence::new_const("application/widget")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/widget"),
};

/// `application/wita`
pub const APPLICATION_WITA: Essence<&'static str> = match Essence::new_const("application/wita") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/wita"),
};

/// `application/wordperfect5.1`
pub const APPLICATION_WORDPERFECT5_1: Essence<&'static str> =
    match Essence::new_const("application/wordperfect5.1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/wordperfect5.1"),
    };

/// `application/wsdl+xml`
pub const APPLICATION_WSDL_XML: Essence<&'static str> =
    match Essence::new_const("application/wsdl+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/wsdl+xml"),
    };

/// `application/wspolicy+xml`
pub const APPLICATION_WSPOLICY_XML: Essence<&'static str> =
    match Essence::new_const("application/wspolicy+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/wspolicy+xml"),
    };

/// `application/x-pki-message`
pub const APPLICATION_X_PKI_MESSAGE: Essence<&'static str> =
    match Essence::new_const("application/x-pki-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x-pki-message"),
    };

/// `application/x-www-form-urlencoded`
pub const APPLICATION_X_WWW_FORM_URLENCODED: Essence<&'static str> =
    match Essence::new_const("application/x-www-form-urlencoded") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x-www-form-urlencoded"),
    };

/// `application/x-x509-ca-cert`
pub const APPLICATION_X_X509_CA_CERT: Essence<&'static str> =
    match Essence::new_const("application/x-x509-ca-cert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x-x509-ca-cert"),
    };

/// `application/x-x509-ca-ra-cert`
pub const APPLICATION_X_X509_CA_RA_CERT: Essence<&'static str> =
    match Essence::new_const("application/x-x509-ca-ra-cert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x-x509-ca-ra-cert"),
    };

/// `application/x-x509-next-ca-cert`
pub const APPLICATION_X_X509_NEXT_CA_CERT: Essence<&'static str> =
    match Essence::new_const("application/x-x509-next-ca-cert") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x-x509-next-ca-cert"),
    };

/// `application/x400-bp`
pub const APPLICATION_X400_BP: Essence<&'static str> =
    match Essence::new_const("application/x400-bp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/x400-bp"),
    };

/// `application/xacml+xml`
pub const APPLICATION_XACML_XML: Essence<&'static str> =
    match Essence::new_const("application/xacml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xacml+xml"),
    };

/// `application/xcap-att+xml`
pub const APPLICATION_XCAP_ATT_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-att+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-att+xml"),
    };

/// `application/xcap-caps+xml`
pub const APPLICATION_XCAP_CAPS_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-caps+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-caps+xml"),
    };

/// `application/xcap-diff+xml`
pub const APPLICATION_XCAP_DIFF_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-diff+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-diff+xml"),
    };

/// `application/xcap-el+xml`
pub const APPLICATION_XCAP_EL_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-el+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-el+xml"),
    };

/// `application/xcap-error+xml`
pub const APPLICATION_XCAP_ERROR_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-error+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-error+xml"),
    };

/// `application/xcap-ns+xml`
pub const APPLICATION_XCAP_NS_XML: Essence<&'static str> =
    match Essence::new_const("application/xcap-ns+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcap-ns+xml"),
    };

/// `application/xcon-conference-info-diff+xml`
pub const APPLICATION_XCON_CONFERENCE_INFO_DIFF_XML: Essence<&'static str> =
    match Essence::new_const("application/xcon-conference-info-diff+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcon-conference-info-diff+xml"),
    };

/// `application/xcon-conference-info+xml`
pub const APPLICATION_XCON_CONFERENCE_INFO_XML: Essence<&'static str> =
    match Essence::new_const("application/xcon-conference-info+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xcon-conference-info+xml"),
    };

/// `application/xenc+xml`
pub const APPLICATION_XENC_XML: Essence<&'static str> =
    match Essence::new_const("application/xenc+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xenc+xml"),
    };

/// `application/xfdf`
pub const APPLICATION_XFDF: Essence<&'static str> = match Essence::new_const("application/xfdf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/xfdf"),
};

/// `application/xhtml+xml`
pub const APPLICATION_XHTML_XML: Essence<&'static str> =
    match Essence::new_const("application/xhtml+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xhtml+xml"),
    };

/// `application/xliff+xml`
pub const APPLICATION_XLIFF_XML: Essence<&'static str> =
    match Essence::new_const("application/xliff+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xliff+xml"),
    };

/// `application/xml`
pub const APPLICATION_XML: Essence<&'static str> = match Essence::new_const("application/xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/xml"),
};

/// `application/xml-dtd`
pub const APPLICATION_XML_DTD: Essence<&'static str> =
    match Essence::new_const("application/xml-dtd") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xml-dtd"),
    };

/// `application/xml-external-parsed-entity`
pub const APPLICATION_XML_EXTERNAL_PARSED_ENTITY: Essence<&'static str> =
    match Essence::new_const("application/xml-external-parsed-entity") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xml-external-parsed-entity"),
    };

/// `application/xml-patch+xml`
pub const APPLICATION_XML_PATCH_XML: Essence<&'static str> =
    match Essence::new_const("application/xml-patch+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xml-patch+xml"),
    };

/// `application/xmpp+xml`
pub const APPLICATION_XMPP_XML: Essence<&'static str> =
    match Essence::new_const("application/xmpp+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xmpp+xml"),
    };

/// `application/xop+xml`
pub const APPLICATION_XOP_XML: Essence<&'static str> =
    match Essence::new_const("application/xop+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xop+xml"),
    };

/// `application/xslt+xml`
pub const APPLICATION_XSLT_XML: Essence<&'static str> =
    match Essence::new_const("application/xslt+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/xslt+xml"),
    };

/// `application/xv+xml`
pub const APPLICATION_XV_XML: Essence<&'static str> = match Essence::new_const("application/xv+xml")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/xv+xml"),
};

/// `application/yang`
pub const APPLICATION_YANG: Essence<&'static str> = match Essence::new_const("application/yang") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/yang"),
};

/// `application/yang-data+cbor`
pub const APPLICATION_YANG_DATA_CBOR: Essence<&'static str> =
    match Essence::new_const("application/yang-data+cbor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yang-data+cbor"),
    };

/// `application/yang-data+json`
pub const APPLICATION_YANG_DATA_JSON: Essence<&'static str> =
    match Essence::new_const("application/yang-data+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yang-data+json"),
    };

/// `application/yang-data+xml`
pub const APPLICATION_YANG_DATA_XML: Essence<&'static str> =
    match Essence::new_const("application/yang-data+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yang-data+xml"),
    };

/// `application/yang-patch+json`
pub const APPLICATION_YANG_PATCH_JSON: Essence<&'static str> =
    match Essence::new_const("application/yang-patch+json") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yang-patch+json"),
    };

/// `application/yang-patch+xml`
pub const APPLICATION_YANG_PATCH_XML: Essence<&'static str> =
    match Essence::new_const("application/yang-patch+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yang-patch+xml"),
    };

/// `application/yin+xml`
pub const APPLICATION_YIN_XML: Essence<&'static str> =
    match Essence::new_const("application/yin+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: application/yin+xml"),
    };

/// `application/zip`
pub const APPLICATION_ZIP: Essence<&'static str> = match Essence::new_const("application/zip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/zip"),
};

/// `application/zlib`
pub const APPLICATION_ZLIB: Essence<&'static str> = match Essence::new_const("application/zlib") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/zlib"),
};

/// `application/zstd`
pub const APPLICATION_ZSTD: Essence<&'static str> = match Essence::new_const("application/zstd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: application/zstd"),
};

/// `audio/1d-interleaved-parityfec`
pub const AUDIO_1D_INTERLEAVED_PARITYFEC: Essence<&'static str> =
    match Essence::new_const("audio/1d-interleaved-parityfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/1d-interleaved-parityfec"),
    };

/// `audio/32kadpcm`
pub const AUDIO_32KADPCM: Essence<&'static str> = match Essence::new_const("audio/32kadpcm") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/32kadpcm"),
};

/// `audio/3gpp`
pub const AUDIO_3GPP: Essence<&'static str> = match Essence::new_const("audio/3gpp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/3gpp"),
};

/// `audio/3gpp2`
pub const AUDIO_3GPP2: Essence<&'static str> = match Essence::new_const("audio/3gpp2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/3gpp2"),
};

/// `audio/aac`
pub const AUDIO_AAC: Essence<&'static str> = match Essence::new_const("audio/aac") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/aac"),
};

/// `audio/ac3`
pub const AUDIO_AC3: Essence<&'static str> = match Essence::new_const("audio/ac3") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ac3"),
};

/// `audio/AMR`
pub const AUDIO_AMR: Essence<&'static str> = match Essence::new_const("audio/AMR") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/AMR"),
};

/// `audio/AMR-WB`
pub const AUDIO_AMR_WB: Essence<&'static str> = match Essence::new_const("audio/AMR-WB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/AMR-WB"),
};

/// `audio/amr-wb+`
pub const AUDIO_AMR_WB_: Essence<&'static str> = match Essence::new_const("audio/amr-wb+") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/amr-wb+"),
};

/// `audio/aptx`
pub const AUDIO_APTX: Essence<&'static str> = match Essence::new_const("audio/aptx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/aptx"),
};

/// `audio/asc`
pub const AUDIO_ASC: Essence<&'static str> = match Essence::new_const("audio/asc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/asc"),
};

/// `audio/ATRAC-ADVANCED-LOSSLESS`
pub const AUDIO_ATRAC_ADVANCED_LOSSLESS: Essence<&'static str> =
    match Essence::new_const("audio/ATRAC-ADVANCED-LOSSLESS") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/ATRAC-ADVANCED-LOSSLESS"),
    };

/// `audio/ATRAC-X`
pub const AUDIO_ATRAC_X: Essence<&'static str> = match Essence::new_const("audio/ATRAC-X") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ATRAC-X"),
};

/// `audio/ATRAC3`
pub const AUDIO_ATRAC3: Essence<&'static str> = match Essence::new_const("audio/ATRAC3") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ATRAC3"),
};

/// `audio/basic`
pub const AUDIO_BASIC: Essence<&'static str> = match Essence::new_const("audio/basic") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/basic"),
};

/// `audio/BV16`
pub const AUDIO_BV16: Essence<&'static str> = match Essence::new_const("audio/BV16") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/BV16"),
};

/// `audio/BV32`
pub const AUDIO_BV32: Essence<&'static str> = match Essence::new_const("audio/BV32") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/BV32"),
};

/// `audio/clearmode`
pub const AUDIO_CLEARMODE: Essence<&'static str> = match Essence::new_const("audio/clearmode") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/clearmode"),
};

/// `audio/CN`
pub const AUDIO_CN: Essence<&'static str> = match Essence::new_const("audio/CN") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/CN"),
};

/// `audio/DAT12`
pub const AUDIO_DAT12: Essence<&'static str> = match Essence::new_const("audio/DAT12") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/DAT12"),
};

/// `audio/dls`
pub const AUDIO_DLS: Essence<&'static str> = match Essence::new_const("audio/dls") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/dls"),
};

/// `audio/dsr-es201108`
pub const AUDIO_DSR_ES201108: Essence<&'static str> = match Essence::new_const("audio/dsr-es201108")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/dsr-es201108"),
};

/// `audio/dsr-es202050`
pub const AUDIO_DSR_ES202050: Essence<&'static str> = match Essence::new_const("audio/dsr-es202050")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/dsr-es202050"),
};

/// `audio/dsr-es202211`
pub const AUDIO_DSR_ES202211: Essence<&'static str> = match Essence::new_const("audio/dsr-es202211")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/dsr-es202211"),
};

/// `audio/dsr-es202212`
pub const AUDIO_DSR_ES202212: Essence<&'static str> = match Essence::new_const("audio/dsr-es202212")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/dsr-es202212"),
};

/// `audio/DV`
pub const AUDIO_DV: Essence<&'static str> = match Essence::new_const("audio/DV") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/DV"),
};

/// `audio/DVI4`
pub const AUDIO_DVI4: Essence<&'static str> = match Essence::new_const("audio/DVI4") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/DVI4"),
};

/// `audio/eac3`
pub const AUDIO_EAC3: Essence<&'static str> = match Essence::new_const("audio/eac3") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/eac3"),
};

/// `audio/encaprtp`
pub const AUDIO_ENCAPRTP: Essence<&'static str> = match Essence::new_const("audio/encaprtp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/encaprtp"),
};

/// `audio/EVRC`
pub const AUDIO_EVRC: Essence<&'static str> = match Essence::new_const("audio/EVRC") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRC"),
};

/// `audio/EVRC-QCP`
pub const AUDIO_EVRC_QCP: Essence<&'static str> = match Essence::new_const("audio/EVRC-QCP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRC-QCP"),
};

/// `audio/EVRC0`
pub const AUDIO_EVRC0: Essence<&'static str> = match Essence::new_const("audio/EVRC0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRC0"),
};

/// `audio/EVRC1`
pub const AUDIO_EVRC1: Essence<&'static str> = match Essence::new_const("audio/EVRC1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRC1"),
};

/// `audio/EVRCB`
pub const AUDIO_EVRCB: Essence<&'static str> = match Essence::new_const("audio/EVRCB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCB"),
};

/// `audio/EVRCB0`
pub const AUDIO_EVRCB0: Essence<&'static str> = match Essence::new_const("audio/EVRCB0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCB0"),
};

/// `audio/EVRCB1`
pub const AUDIO_EVRCB1: Essence<&'static str> = match Essence::new_const("audio/EVRCB1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCB1"),
};

/// `audio/EVRCNW`
pub const AUDIO_EVRCNW: Essence<&'static str> = match Essence::new_const("audio/EVRCNW") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCNW"),
};

/// `audio/EVRCNW0`
pub const AUDIO_EVRCNW0: Essence<&'static str> = match Essence::new_const("audio/EVRCNW0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCNW0"),
};

/// `audio/EVRCNW1`
pub const AUDIO_EVRCNW1: Essence<&'static str> = match Essence::new_const("audio/EVRCNW1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCNW1"),
};

/// `audio/EVRCWB`
pub const AUDIO_EVRCWB: Essence<&'static str> = match Essence::new_const("audio/EVRCWB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCWB"),
};

/// `audio/EVRCWB0`
pub const AUDIO_EVRCWB0: Essence<&'static str> = match Essence::new_const("audio/EVRCWB0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCWB0"),
};

/// `audio/EVRCWB1`
pub const AUDIO_EVRCWB1: Essence<&'static str> = match Essence::new_const("audio/EVRCWB1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVRCWB1"),
};

/// `audio/EVS`
pub const AUDIO_EVS: Essence<&'static str> = match Essence::new_const("audio/EVS") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/EVS"),
};

/// `audio/example`
pub const AUDIO_EXAMPLE: Essence<&'static str> = match Essence::new_const("audio/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/example"),
};

/// `audio/flexfec`
pub const AUDIO_FLEXFEC: Essence<&'static str> = match Essence::new_const("audio/flexfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/flexfec"),
};

/// `audio/fwdred`
pub const AUDIO_FWDRED: Essence<&'static str> = match Essence::new_const("audio/fwdred") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/fwdred"),
};

/// `audio/G711-0`
pub const AUDIO_G711_0: Essence<&'static str> = match Essence::new_const("audio/G711-0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G711-0"),
};

/// `audio/G719`
pub const AUDIO_G719: Essence<&'static str> = match Essence::new_const("audio/G719") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G719"),
};

/// `audio/G7221`
pub const AUDIO_G7221: Essence<&'static str> = match Essence::new_const("audio/G7221") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G7221"),
};

/// `audio/G722`
pub const AUDIO_G722: Essence<&'static str> = match Essence::new_const("audio/G722") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G722"),
};

/// `audio/G723`
pub const AUDIO_G723: Essence<&'static str> = match Essence::new_const("audio/G723") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G723"),
};

/// `audio/G726-16`
pub const AUDIO_G726_16: Essence<&'static str> = match Essence::new_const("audio/G726-16") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G726-16"),
};

/// `audio/G726-24`
pub const AUDIO_G726_24: Essence<&'static str> = match Essence::new_const("audio/G726-24") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G726-24"),
};

/// `audio/G726-32`
pub const AUDIO_G726_32: Essence<&'static str> = match Essence::new_const("audio/G726-32") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G726-32"),
};

/// `audio/G726-40`
pub const AUDIO_G726_40: Essence<&'static str> = match Essence::new_const("audio/G726-40") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G726-40"),
};

/// `audio/G728`
pub const AUDIO_G728: Essence<&'static str> = match Essence::new_const("audio/G728") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G728"),
};

/// `audio/G729`
pub const AUDIO_G729: Essence<&'static str> = match Essence::new_const("audio/G729") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G729"),
};

/// `audio/G7291`
pub const AUDIO_G7291: Essence<&'static str> = match Essence::new_const("audio/G7291") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G7291"),
};

/// `audio/G729D`
pub const AUDIO_G729D: Essence<&'static str> = match Essence::new_const("audio/G729D") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G729D"),
};

/// `audio/G729E`
pub const AUDIO_G729E: Essence<&'static str> = match Essence::new_const("audio/G729E") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/G729E"),
};

/// `audio/GSM`
pub const AUDIO_GSM: Essence<&'static str> = match Essence::new_const("audio/GSM") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/GSM"),
};

/// `audio/GSM-EFR`
pub const AUDIO_GSM_EFR: Essence<&'static str> = match Essence::new_const("audio/GSM-EFR") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/GSM-EFR"),
};

/// `audio/GSM-HR-08`
pub const AUDIO_GSM_HR_08: Essence<&'static str> = match Essence::new_const("audio/GSM-HR-08") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/GSM-HR-08"),
};

/// `audio/iLBC`
pub const AUDIO_ILBC: Essence<&'static str> = match Essence::new_const("audio/iLBC") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/iLBC"),
};

/// `audio/ip-mr_v2.5`
pub const AUDIO_IP_MR_V2_5: Essence<&'static str> = match Essence::new_const("audio/ip-mr_v2.5") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ip-mr_v2.5"),
};

/// `audio/L8`
pub const AUDIO_L8: Essence<&'static str> = match Essence::new_const("audio/L8") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/L8"),
};

/// `audio/L16`
pub const AUDIO_L16: Essence<&'static str> = match Essence::new_const("audio/L16") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/L16"),
};

/// `audio/L20`
pub const AUDIO_L20: Essence<&'static str> = match Essence::new_const("audio/L20") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/L20"),
};

/// `audio/L24`
pub const AUDIO_L24: Essence<&'static str> = match Essence::new_const("audio/L24") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/L24"),
};

/// `audio/LPC`
pub const AUDIO_LPC: Essence<&'static str> = match Essence::new_const("audio/LPC") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/LPC"),
};

/// `audio/MELP`
pub const AUDIO_MELP: Essence<&'static str> = match Essence::new_const("audio/MELP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MELP"),
};

/// `audio/MELP600`
pub const AUDIO_MELP600: Essence<&'static str> = match Essence::new_const("audio/MELP600") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MELP600"),
};

/// `audio/MELP1200`
pub const AUDIO_MELP1200: Essence<&'static str> = match Essence::new_const("audio/MELP1200") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MELP1200"),
};

/// `audio/MELP2400`
pub const AUDIO_MELP2400: Essence<&'static str> = match Essence::new_const("audio/MELP2400") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MELP2400"),
};

/// `audio/mhas`
pub const AUDIO_MHAS: Essence<&'static str> = match Essence::new_const("audio/mhas") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/mhas"),
};

/// `audio/mobile-xmf`
pub const AUDIO_MOBILE_XMF: Essence<&'static str> = match Essence::new_const("audio/mobile-xmf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/mobile-xmf"),
};

/// `audio/MPA`
pub const AUDIO_MPA: Essence<&'static str> = match Essence::new_const("audio/MPA") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MPA"),
};

/// `audio/mp4`
pub const AUDIO_MP4: Essence<&'static str> = match Essence::new_const("audio/mp4") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/mp4"),
};

/// `audio/MP4A-LATM`
pub const AUDIO_MP4A_LATM: Essence<&'static str> = match Essence::new_const("audio/MP4A-LATM") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/MP4A-LATM"),
};

/// `audio/mpa-robust`
pub const AUDIO_MPA_ROBUST: Essence<&'static str> = match Essence::new_const("audio/mpa-robust") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/mpa-robust"),
};

/// `audio/mpeg`
pub const AUDIO_MPEG: Essence<&'static str> = match Essence::new_const("audio/mpeg") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/mpeg"),
};

/// `audio/mpeg4-generic`
pub const AUDIO_MPEG4_GENERIC: Essence<&'static str> =
    match Essence::new_const("audio/mpeg4-generic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/mpeg4-generic"),
    };

/// `audio/ogg`
pub const AUDIO_OGG: Essence<&'static str> = match Essence::new_const("audio/ogg") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ogg"),
};

/// `audio/opus`
pub const AUDIO_OPUS: Essence<&'static str> = match Essence::new_const("audio/opus") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/opus"),
};

/// `audio/parityfec`
pub const AUDIO_PARITYFEC: Essence<&'static str> = match Essence::new_const("audio/parityfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/parityfec"),
};

/// `audio/PCMA`
pub const AUDIO_PCMA: Essence<&'static str> = match Essence::new_const("audio/PCMA") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/PCMA"),
};

/// `audio/PCMA-WB`
pub const AUDIO_PCMA_WB: Essence<&'static str> = match Essence::new_const("audio/PCMA-WB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/PCMA-WB"),
};

/// `audio/PCMU`
pub const AUDIO_PCMU: Essence<&'static str> = match Essence::new_const("audio/PCMU") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/PCMU"),
};

/// `audio/PCMU-WB`
pub const AUDIO_PCMU_WB: Essence<&'static str> = match Essence::new_const("audio/PCMU-WB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/PCMU-WB"),
};

/// `audio/prs.sid`
pub const AUDIO_PRS_SID: Essence<&'static str> = match Essence::new_const("audio/prs.sid") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/prs.sid"),
};

/// `audio/QCELP`
pub const AUDIO_QCELP: Essence<&'static str> = match Essence::new_const("audio/QCELP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/QCELP"),
};

/// `audio/raptorfec`
pub const AUDIO_RAPTORFEC: Essence<&'static str> = match Essence::new_const("audio/raptorfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/raptorfec"),
};

/// `audio/RED`
pub const AUDIO_RED: Essence<&'static str> = match Essence::new_const("audio/RED") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/RED"),
};

/// `audio/rtp-enc-aescm128`
pub const AUDIO_RTP_ENC_AESCM128: Essence<&'static str> =
    match Essence::new_const("audio/rtp-enc-aescm128") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/rtp-enc-aescm128"),
    };

/// `audio/rtploopback`
pub const AUDIO_RTPLOOPBACK: Essence<&'static str> = match Essence::new_const("audio/rtploopback") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/rtploopback"),
};

/// `audio/rtp-midi`
pub const AUDIO_RTP_MIDI: Essence<&'static str> = match Essence::new_const("audio/rtp-midi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/rtp-midi"),
};

/// `audio/rtx`
pub const AUDIO_RTX: Essence<&'static str> = match Essence::new_const("audio/rtx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/rtx"),
};

/// `audio/scip`
pub const AUDIO_SCIP: Essence<&'static str> = match Essence::new_const("audio/scip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/scip"),
};

/// `audio/SMV`
pub const AUDIO_SMV: Essence<&'static str> = match Essence::new_const("audio/SMV") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/SMV"),
};

/// `audio/SMV0`
pub const AUDIO_SMV0: Essence<&'static str> = match Essence::new_const("audio/SMV0") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/SMV0"),
};

/// `audio/SMV-QCP`
pub const AUDIO_SMV_QCP: Essence<&'static str> = match Essence::new_const("audio/SMV-QCP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/SMV-QCP"),
};

/// `audio/sofa`
pub const AUDIO_SOFA: Essence<&'static str> = match Essence::new_const("audio/sofa") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/sofa"),
};

/// `audio/sp-midi`
pub const AUDIO_SP_MIDI: Essence<&'static str> = match Essence::new_const("audio/sp-midi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/sp-midi"),
};

/// `audio/speex`
pub const AUDIO_SPEEX: Essence<&'static str> = match Essence::new_const("audio/speex") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/speex"),
};

/// `audio/t140c`
pub const AUDIO_T140C: Essence<&'static str> = match Essence::new_const("audio/t140c") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/t140c"),
};

/// `audio/t38`
pub const AUDIO_T38: Essence<&'static str> = match Essence::new_const("audio/t38") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/t38"),
};

/// `audio/telephone-event`
pub const AUDIO_TELEPHONE_EVENT: Essence<&'static str> =
    match Essence::new_const("audio/telephone-event") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/telephone-event"),
    };

/// `audio/TETRA_ACELP`
pub const AUDIO_TETRA_ACELP: Essence<&'static str> = match Essence::new_const("audio/TETRA_ACELP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/TETRA_ACELP"),
};

/// `audio/TETRA_ACELP_BB`
pub const AUDIO_TETRA_ACELP_BB: Essence<&'static str> =
    match Essence::new_const("audio/TETRA_ACELP_BB") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/TETRA_ACELP_BB"),
    };

/// `audio/tone`
pub const AUDIO_TONE: Essence<&'static str> = match Essence::new_const("audio/tone") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/tone"),
};

/// `audio/TSVCIS`
pub const AUDIO_TSVCIS: Essence<&'static str> = match Essence::new_const("audio/TSVCIS") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/TSVCIS"),
};

/// `audio/UEMCLIP`
pub const AUDIO_UEMCLIP: Essence<&'static str> = match Essence::new_const("audio/UEMCLIP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/UEMCLIP"),
};

/// `audio/ulpfec`
pub const AUDIO_ULPFEC: Essence<&'static str> = match Essence::new_const("audio/ulpfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/ulpfec"),
};

/// `audio/usac`
pub const AUDIO_USAC: Essence<&'static str> = match Essence::new_const("audio/usac") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/usac"),
};

/// `audio/VDVI`
pub const AUDIO_VDVI: Essence<&'static str> = match Essence::new_const("audio/VDVI") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/VDVI"),
};

/// `audio/VMR-WB`
pub const AUDIO_VMR_WB: Essence<&'static str> = match Essence::new_const("audio/VMR-WB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/VMR-WB"),
};

/// `audio/vnd.3gpp.iufp`
pub const AUDIO_VND_3GPP_IUFP: Essence<&'static str> =
    match Essence::new_const("audio/vnd.3gpp.iufp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.3gpp.iufp"),
    };

/// `audio/vnd.4SB`
pub const AUDIO_VND_4SB: Essence<&'static str> = match Essence::new_const("audio/vnd.4SB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.4SB"),
};

/// `audio/vnd.audiokoz`
pub const AUDIO_VND_AUDIOKOZ: Essence<&'static str> = match Essence::new_const("audio/vnd.audiokoz")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.audiokoz"),
};

/// `audio/vnd.CELP`
pub const AUDIO_VND_CELP: Essence<&'static str> = match Essence::new_const("audio/vnd.CELP") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.CELP"),
};

/// `audio/vnd.cisco.nse`
pub const AUDIO_VND_CISCO_NSE: Essence<&'static str> =
    match Essence::new_const("audio/vnd.cisco.nse") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.cisco.nse"),
    };

/// `audio/vnd.cmles.radio-events`
pub const AUDIO_VND_CMLES_RADIO_EVENTS: Essence<&'static str> =
    match Essence::new_const("audio/vnd.cmles.radio-events") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.cmles.radio-events"),
    };

/// `audio/vnd.cns.anp1`
pub const AUDIO_VND_CNS_ANP1: Essence<&'static str> = match Essence::new_const("audio/vnd.cns.anp1")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.cns.anp1"),
};

/// `audio/vnd.cns.inf1`
pub const AUDIO_VND_CNS_INF1: Essence<&'static str> = match Essence::new_const("audio/vnd.cns.inf1")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.cns.inf1"),
};

/// `audio/vnd.dece.audio`
pub const AUDIO_VND_DECE_AUDIO: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dece.audio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dece.audio"),
    };

/// `audio/vnd.digital-winds`
pub const AUDIO_VND_DIGITAL_WINDS: Essence<&'static str> =
    match Essence::new_const("audio/vnd.digital-winds") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.digital-winds"),
    };

/// `audio/vnd.dlna.adts`
pub const AUDIO_VND_DLNA_ADTS: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dlna.adts") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dlna.adts"),
    };

/// `audio/vnd.dolby.heaac.1`
pub const AUDIO_VND_DOLBY_HEAAC_1: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.heaac.1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.heaac.1"),
    };

/// `audio/vnd.dolby.heaac.2`
pub const AUDIO_VND_DOLBY_HEAAC_2: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.heaac.2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.heaac.2"),
    };

/// `audio/vnd.dolby.mlp`
pub const AUDIO_VND_DOLBY_MLP: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.mlp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.mlp"),
    };

/// `audio/vnd.dolby.mps`
pub const AUDIO_VND_DOLBY_MPS: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.mps") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.mps"),
    };

/// `audio/vnd.dolby.pl2`
pub const AUDIO_VND_DOLBY_PL2: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.pl2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.pl2"),
    };

/// `audio/vnd.dolby.pl2x`
pub const AUDIO_VND_DOLBY_PL2X: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.pl2x") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.pl2x"),
    };

/// `audio/vnd.dolby.pl2z`
pub const AUDIO_VND_DOLBY_PL2Z: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.pl2z") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.pl2z"),
    };

/// `audio/vnd.dolby.pulse.1`
pub const AUDIO_VND_DOLBY_PULSE_1: Essence<&'static str> =
    match Essence::new_const("audio/vnd.dolby.pulse.1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.dolby.pulse.1"),
    };

/// `audio/vnd.dra`
pub const AUDIO_VND_DRA: Essence<&'static str> = match Essence::new_const("audio/vnd.dra") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.dra"),
};

/// `audio/vnd.dts`
pub const AUDIO_VND_DTS: Essence<&'static str> = match Essence::new_const("audio/vnd.dts") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.dts"),
};

/// `audio/vnd.dts.hd`
pub const AUDIO_VND_DTS_HD: Essence<&'static str> = match Essence::new_const("audio/vnd.dts.hd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.dts.hd"),
};

/// `audio/vnd.dts.uhd`
pub const AUDIO_VND_DTS_UHD: Essence<&'static str> = match Essence::new_const("audio/vnd.dts.uhd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.dts.uhd"),
};

/// `audio/vnd.dvb.file`
pub const AUDIO_VND_DVB_FILE: Essence<&'static str> = match Essence::new_const("audio/vnd.dvb.file")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.dvb.file"),
};

/// `audio/vnd.everad.plj`
pub const AUDIO_VND_EVERAD_PLJ: Essence<&'static str> =
    match Essence::new_const("audio/vnd.everad.plj") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.everad.plj"),
    };

/// `audio/vnd.hns.audio`
pub const AUDIO_VND_HNS_AUDIO: Essence<&'static str> =
    match Essence::new_const("audio/vnd.hns.audio") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.hns.audio"),
    };

/// `audio/vnd.lucent.voice`
pub const AUDIO_VND_LUCENT_VOICE: Essence<&'static str> =
    match Essence::new_const("audio/vnd.lucent.voice") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.lucent.voice"),
    };

/// `audio/vnd.ms-playready.media.pya`
pub const AUDIO_VND_MS_PLAYREADY_MEDIA_PYA: Essence<&'static str> =
    match Essence::new_const("audio/vnd.ms-playready.media.pya") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.ms-playready.media.pya"),
    };

/// `audio/vnd.nokia.mobile-xmf`
pub const AUDIO_VND_NOKIA_MOBILE_XMF: Essence<&'static str> =
    match Essence::new_const("audio/vnd.nokia.mobile-xmf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.nokia.mobile-xmf"),
    };

/// `audio/vnd.nortel.vbk`
pub const AUDIO_VND_NORTEL_VBK: Essence<&'static str> =
    match Essence::new_const("audio/vnd.nortel.vbk") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.nortel.vbk"),
    };

/// `audio/vnd.nuera.ecelp4800`
pub const AUDIO_VND_NUERA_ECELP4800: Essence<&'static str> =
    match Essence::new_const("audio/vnd.nuera.ecelp4800") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.nuera.ecelp4800"),
    };

/// `audio/vnd.nuera.ecelp7470`
pub const AUDIO_VND_NUERA_ECELP7470: Essence<&'static str> =
    match Essence::new_const("audio/vnd.nuera.ecelp7470") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.nuera.ecelp7470"),
    };

/// `audio/vnd.nuera.ecelp9600`
pub const AUDIO_VND_NUERA_ECELP9600: Essence<&'static str> =
    match Essence::new_const("audio/vnd.nuera.ecelp9600") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.nuera.ecelp9600"),
    };

/// `audio/vnd.octel.sbc`
pub const AUDIO_VND_OCTEL_SBC: Essence<&'static str> =
    match Essence::new_const("audio/vnd.octel.sbc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.octel.sbc"),
    };

/// `audio/vnd.presonus.multitrack`
pub const AUDIO_VND_PRESONUS_MULTITRACK: Essence<&'static str> =
    match Essence::new_const("audio/vnd.presonus.multitrack") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.presonus.multitrack"),
    };

/// `audio/vnd.rhetorex.32kadpcm`
pub const AUDIO_VND_RHETOREX_32KADPCM: Essence<&'static str> =
    match Essence::new_const("audio/vnd.rhetorex.32kadpcm") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.rhetorex.32kadpcm"),
    };

/// `audio/vnd.rip`
pub const AUDIO_VND_RIP: Essence<&'static str> = match Essence::new_const("audio/vnd.rip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.rip"),
};

/// `audio/vnd.sealedmedia.softseal.mpeg`
pub const AUDIO_VND_SEALEDMEDIA_SOFTSEAL_MPEG: Essence<&'static str> =
    match Essence::new_const("audio/vnd.sealedmedia.softseal.mpeg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vnd.sealedmedia.softseal.mpeg"),
    };

/// `audio/vnd.vmx.cvsd`
pub const AUDIO_VND_VMX_CVSD: Essence<&'static str> = match Essence::new_const("audio/vnd.vmx.cvsd")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vnd.vmx.cvsd"),
};

/// `audio/vorbis`
pub const AUDIO_VORBIS: Essence<&'static str> = match Essence::new_const("audio/vorbis") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: audio/vorbis"),
};

/// `audio/vorbis-config`
pub const AUDIO_VORBIS_CONFIG: Essence<&'static str> =
    match Essence::new_const("audio/vorbis-config") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: audio/vorbis-config"),
    };

/// `font/collection`
pub const FONT_COLLECTION: Essence<&'static str> = match Essence::new_const("font/collection") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/collection"),
};

/// `font/otf`
pub const FONT_OTF: Essence<&'static str> = match Essence::new_const("font/otf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/otf"),
};

/// `font/sfnt`
pub const FONT_SFNT: Essence<&'static str> = match Essence::new_const("font/sfnt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/sfnt"),
};

/// `font/ttf`
pub const FONT_TTF: Essence<&'static str> = match Essence::new_const("font/ttf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/ttf"),
};

/// `font/woff`
pub const FONT_WOFF: Essence<&'static str> = match Essence::new_const("font/woff") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/woff"),
};

/// `font/woff2`
pub const FONT_WOFF2: Essence<&'static str> = match Essence::new_const("font/woff2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: font/woff2"),
};

/// `image/aces`
pub const IMAGE_ACES: Essence<&'static str> = match Essence::new_const("image/aces") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/aces"),
};

/// `image/avci`
pub const IMAGE_AVCI: Essence<&'static str> = match Essence::new_const("image/avci") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/avci"),
};

/// `image/avcs`
pub const IMAGE_AVCS: Essence<&'static str> = match Essence::new_const("image/avcs") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/avcs"),
};

/// `image/avif`
pub const IMAGE_AVIF: Essence<&'static str> = match Essence::new_const("image/avif") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/avif"),
};

/// `image/bmp`
pub const IMAGE_BMP: Essence<&'static str> = match Essence::new_const("image/bmp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/bmp"),
};

/// `image/cgm`
pub const IMAGE_CGM: Essence<&'static str> = match Essence::new_const("image/cgm") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/cgm"),
};

/// `image/dicom-rle`
pub const IMAGE_DICOM_RLE: Essence<&'static str> = match Essence::new_const("image/dicom-rle") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/dicom-rle"),
};

/// `image/emf`
pub const IMAGE_EMF: Essence<&'static str> = match Essence::new_const("image/emf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/emf"),
};

/// `image/example`
pub const IMAGE_EXAMPLE: Essence<&'static str> = match Essence::new_const("image/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/example"),
};

/// `image/fits`
pub const IMAGE_FITS: Essence<&'static str> = match Essence::new_const("image/fits") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/fits"),
};

/// `image/g3fax`
pub const IMAGE_G3FAX: Essence<&'static str> = match Essence::new_const("image/g3fax") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/g3fax"),
};

/// `image/heic`
pub const IMAGE_HEIC: Essence<&'static str> = match Essence::new_const("image/heic") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/heic"),
};

/// `image/heic-sequence`
pub const IMAGE_HEIC_SEQUENCE: Essence<&'static str> =
    match Essence::new_const("image/heic-sequence") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/heic-sequence"),
    };

/// `image/heif`
pub const IMAGE_HEIF: Essence<&'static str> = match Essence::new_const("image/heif") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/heif"),
};

/// `image/heif-sequence`
pub const IMAGE_HEIF_SEQUENCE: Essence<&'static str> =
    match Essence::new_const("image/heif-sequence") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/heif-sequence"),
    };

/// `image/hej2k`
pub const IMAGE_HEJ2K: Essence<&'static str> = match Essence::new_const("image/hej2k") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/hej2k"),
};

/// `image/hsj2`
pub const IMAGE_HSJ2: Essence<&'static str> = match Essence::new_const("image/hsj2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/hsj2"),
};

/// `image/jls`
pub const IMAGE_JLS: Essence<&'static str> = match Essence::new_const("image/jls") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jls"),
};

/// `image/jp2`
pub const IMAGE_JP2: Essence<&'static str> = match Essence::new_const("image/jp2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jp2"),
};

/// `image/jph`
pub const IMAGE_JPH: Essence<&'static str> = match Essence::new_const("image/jph") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jph"),
};

/// `image/jphc`
pub const IMAGE_JPHC: Essence<&'static str> = match Essence::new_const("image/jphc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jphc"),
};

/// `image/jpm`
pub const IMAGE_JPM: Essence<&'static str> = match Essence::new_const("image/jpm") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jpm"),
};

/// `image/jpx`
pub const IMAGE_JPX: Essence<&'static str> = match Essence::new_const("image/jpx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jpx"),
};

/// `image/jxr`
pub const IMAGE_JXR: Essence<&'static str> = match Essence::new_const("image/jxr") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxr"),
};

/// `image/jxrA`
pub const IMAGE_JXRA: Essence<&'static str> = match Essence::new_const("image/jxrA") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxrA"),
};

/// `image/jxrS`
pub const IMAGE_JXRS: Essence<&'static str> = match Essence::new_const("image/jxrS") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxrS"),
};

/// `image/jxs`
pub const IMAGE_JXS: Essence<&'static str> = match Essence::new_const("image/jxs") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxs"),
};

/// `image/jxsc`
pub const IMAGE_JXSC: Essence<&'static str> = match Essence::new_const("image/jxsc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxsc"),
};

/// `image/jxsi`
pub const IMAGE_JXSI: Essence<&'static str> = match Essence::new_const("image/jxsi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxsi"),
};

/// `image/jxss`
pub const IMAGE_JXSS: Essence<&'static str> = match Essence::new_const("image/jxss") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/jxss"),
};

/// `image/ktx`
pub const IMAGE_KTX: Essence<&'static str> = match Essence::new_const("image/ktx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/ktx"),
};

/// `image/ktx2`
pub const IMAGE_KTX2: Essence<&'static str> = match Essence::new_const("image/ktx2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/ktx2"),
};

/// `image/naplps`
pub const IMAGE_NAPLPS: Essence<&'static str> = match Essence::new_const("image/naplps") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/naplps"),
};

/// `image/png`
pub const IMAGE_PNG: Essence<&'static str> = match Essence::new_const("image/png") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/png"),
};

/// `image/prs.btif`
pub const IMAGE_PRS_BTIF: Essence<&'static str> = match Essence::new_const("image/prs.btif") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/prs.btif"),
};

/// `image/prs.pti`
pub const IMAGE_PRS_PTI: Essence<&'static str> = match Essence::new_const("image/prs.pti") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/prs.pti"),
};

/// `image/pwg-raster`
pub const IMAGE_PWG_RASTER: Essence<&'static str> = match Essence::new_const("image/pwg-raster") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/pwg-raster"),
};

/// `image/svg+xml`
pub const IMAGE_SVG_XML: Essence<&'static str> = match Essence::new_const("image/svg+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/svg+xml"),
};

/// `image/t38`
pub const IMAGE_T38: Essence<&'static str> = match Essence::new_const("image/t38") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/t38"),
};

/// `image/tiff`
pub const IMAGE_TIFF: Essence<&'static str> = match Essence::new_const("image/tiff") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/tiff"),
};

/// `image/tiff-fx`
pub const IMAGE_TIFF_FX: Essence<&'static str> = match Essence::new_const("image/tiff-fx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/tiff-fx"),
};

/// `image/vnd.adobe.photoshop`
pub const IMAGE_VND_ADOBE_PHOTOSHOP: Essence<&'static str> =
    match Essence::new_const("image/vnd.adobe.photoshop") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.adobe.photoshop"),
    };

/// `image/vnd.airzip.accelerator.azv`
pub const IMAGE_VND_AIRZIP_ACCELERATOR_AZV: Essence<&'static str> =
    match Essence::new_const("image/vnd.airzip.accelerator.azv") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.airzip.accelerator.azv"),
    };

/// `image/vnd.cns.inf2`
pub const IMAGE_VND_CNS_INF2: Essence<&'static str> = match Essence::new_const("image/vnd.cns.inf2")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.cns.inf2"),
};

/// `image/vnd.dece.graphic`
pub const IMAGE_VND_DECE_GRAPHIC: Essence<&'static str> =
    match Essence::new_const("image/vnd.dece.graphic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.dece.graphic"),
    };

/// `image/vnd.djvu`
pub const IMAGE_VND_DJVU: Essence<&'static str> = match Essence::new_const("image/vnd.djvu") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.djvu"),
};

/// `image/vnd.dwg`
pub const IMAGE_VND_DWG: Essence<&'static str> = match Essence::new_const("image/vnd.dwg") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.dwg"),
};

/// `image/vnd.dxf`
pub const IMAGE_VND_DXF: Essence<&'static str> = match Essence::new_const("image/vnd.dxf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.dxf"),
};

/// `image/vnd.dvb.subtitle`
pub const IMAGE_VND_DVB_SUBTITLE: Essence<&'static str> =
    match Essence::new_const("image/vnd.dvb.subtitle") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.dvb.subtitle"),
    };

/// `image/vnd.fastbidsheet`
pub const IMAGE_VND_FASTBIDSHEET: Essence<&'static str> =
    match Essence::new_const("image/vnd.fastbidsheet") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.fastbidsheet"),
    };

/// `image/vnd.fpx`
pub const IMAGE_VND_FPX: Essence<&'static str> = match Essence::new_const("image/vnd.fpx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.fpx"),
};

/// `image/vnd.fst`
pub const IMAGE_VND_FST: Essence<&'static str> = match Essence::new_const("image/vnd.fst") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.fst"),
};

/// `image/vnd.fujixerox.edmics-mmr`
pub const IMAGE_VND_FUJIXEROX_EDMICS_MMR: Essence<&'static str> =
    match Essence::new_const("image/vnd.fujixerox.edmics-mmr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.fujixerox.edmics-mmr"),
    };

/// `image/vnd.fujixerox.edmics-rlc`
pub const IMAGE_VND_FUJIXEROX_EDMICS_RLC: Essence<&'static str> =
    match Essence::new_const("image/vnd.fujixerox.edmics-rlc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.fujixerox.edmics-rlc"),
    };

/// `image/vnd.globalgraphics.pgb`
pub const IMAGE_VND_GLOBALGRAPHICS_PGB: Essence<&'static str> =
    match Essence::new_const("image/vnd.globalgraphics.pgb") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.globalgraphics.pgb"),
    };

/// `image/vnd.microsoft.icon`
pub const IMAGE_VND_MICROSOFT_ICON: Essence<&'static str> =
    match Essence::new_const("image/vnd.microsoft.icon") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.microsoft.icon"),
    };

/// `image/vnd.mix`
pub const IMAGE_VND_MIX: Essence<&'static str> = match Essence::new_const("image/vnd.mix") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.mix"),
};

/// `image/vnd.ms-modi`
pub const IMAGE_VND_MS_MODI: Essence<&'static str> = match Essence::new_const("image/vnd.ms-modi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.ms-modi"),
};

/// `image/vnd.mozilla.apng`
pub const IMAGE_VND_MOZILLA_APNG: Essence<&'static str> =
    match Essence::new_const("image/vnd.mozilla.apng") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.mozilla.apng"),
    };

/// `image/vnd.net-fpx`
pub const IMAGE_VND_NET_FPX: Essence<&'static str> = match Essence::new_const("image/vnd.net-fpx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.net-fpx"),
};

/// `image/vnd.pco.b16`
pub const IMAGE_VND_PCO_B16: Essence<&'static str> = match Essence::new_const("image/vnd.pco.b16") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.pco.b16"),
};

/// `image/vnd.radiance`
pub const IMAGE_VND_RADIANCE: Essence<&'static str> = match Essence::new_const("image/vnd.radiance")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.radiance"),
};

/// `image/vnd.sealed.png`
pub const IMAGE_VND_SEALED_PNG: Essence<&'static str> =
    match Essence::new_const("image/vnd.sealed.png") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.sealed.png"),
    };

/// `image/vnd.sealedmedia.softseal.gif`
pub const IMAGE_VND_SEALEDMEDIA_SOFTSEAL_GIF: Essence<&'static str> =
    match Essence::new_const("image/vnd.sealedmedia.softseal.gif") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.sealedmedia.softseal.gif"),
    };

/// `image/vnd.sealedmedia.softseal.jpg`
pub const IMAGE_VND_SEALEDMEDIA_SOFTSEAL_JPG: Essence<&'static str> =
    match Essence::new_const("image/vnd.sealedmedia.softseal.jpg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.sealedmedia.softseal.jpg"),
    };

/// `image/vnd.svf`
pub const IMAGE_VND_SVF: Essence<&'static str> = match Essence::new_const("image/vnd.svf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.svf"),
};

/// `image/vnd.tencent.tap`
pub const IMAGE_VND_TENCENT_TAP: Essence<&'static str> =
    match Essence::new_const("image/vnd.tencent.tap") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.tencent.tap"),
    };

/// `image/vnd.valve.source.texture`
pub const IMAGE_VND_VALVE_SOURCE_TEXTURE: Essence<&'static str> =
    match Essence::new_const("image/vnd.valve.source.texture") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.valve.source.texture"),
    };

/// `image/vnd.wap.wbmp`
pub const IMAGE_VND_WAP_WBMP: Essence<&'static str> = match Essence::new_const("image/vnd.wap.wbmp")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.wap.wbmp"),
};

/// `image/vnd.xiff`
pub const IMAGE_VND_XIFF: Essence<&'static str> = match Essence::new_const("image/vnd.xiff") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/vnd.xiff"),
};

/// `image/vnd.zbrush.pcx`
pub const IMAGE_VND_ZBRUSH_PCX: Essence<&'static str> =
    match Essence::new_const("image/vnd.zbrush.pcx") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: image/vnd.zbrush.pcx"),
    };

/// `image/wmf`
pub const IMAGE_WMF: Essence<&'static str> = match Essence::new_const("image/wmf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: image/wmf"),
};

/// `message/CPIM`
pub const MESSAGE_CPIM: Essence<&'static str> = match Essence::new_const("message/CPIM") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/CPIM"),
};

/// `message/delivery-status`
pub const MESSAGE_DELIVERY_STATUS: Essence<&'static str> =
    match Essence::new_const("message/delivery-status") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/delivery-status"),
    };

/// `message/disposition-notification`
pub const MESSAGE_DISPOSITION_NOTIFICATION: Essence<&'static str> =
    match Essence::new_const("message/disposition-notification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/disposition-notification"),
    };

/// `message/example`
pub const MESSAGE_EXAMPLE: Essence<&'static str> = match Essence::new_const("message/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/example"),
};

/// `message/feedback-report`
pub const MESSAGE_FEEDBACK_REPORT: Essence<&'static str> =
    match Essence::new_const("message/feedback-report") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/feedback-report"),
    };

/// `message/global`
pub const MESSAGE_GLOBAL: Essence<&'static str> = match Essence::new_const("message/global") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/global"),
};

/// `message/global-delivery-status`
pub const MESSAGE_GLOBAL_DELIVERY_STATUS: Essence<&'static str> =
    match Essence::new_const("message/global-delivery-status") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/global-delivery-status"),
    };

/// `message/global-disposition-notification`
pub const MESSAGE_GLOBAL_DISPOSITION_NOTIFICATION: Essence<&'static str> =
    match Essence::new_const("message/global-disposition-notification") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/global-disposition-notification"),
    };

/// `message/global-headers`
pub const MESSAGE_GLOBAL_HEADERS: Essence<&'static str> =
    match Essence::new_const("message/global-headers") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/global-headers"),
    };

/// `message/http`
pub const MESSAGE_HTTP: Essence<&'static str> = match Essence::new_const("message/http") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/http"),
};

/// `message/imdn+xml`
pub const MESSAGE_IMDN_XML: Essence<&'static str> = match Essence::new_const("message/imdn+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/imdn+xml"),
};

/// `message/news`
pub const MESSAGE_NEWS: Essence<&'static str> = match Essence::new_const("message/news") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/news"),
};

/// `message/s-http`
pub const MESSAGE_S_HTTP: Essence<&'static str> = match Essence::new_const("message/s-http") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/s-http"),
};

/// `message/sip`
pub const MESSAGE_SIP: Essence<&'static str> = match Essence::new_const("message/sip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/sip"),
};

/// `message/sipfrag`
pub const MESSAGE_SIPFRAG: Essence<&'static str> = match Essence::new_const("message/sipfrag") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: message/sipfrag"),
};

/// `message/tracking-status`
pub const MESSAGE_TRACKING_STATUS: Essence<&'static str> =
    match Essence::new_const("message/tracking-status") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/tracking-status"),
    };

/// `message/vnd.si.simp`
pub const MESSAGE_VND_SI_SIMP: Essence<&'static str> =
    match Essence::new_const("message/vnd.si.simp") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/vnd.si.simp"),
    };

/// `message/vnd.wfa.wsc`
pub const MESSAGE_VND_WFA_WSC: Essence<&'static str> =
    match Essence::new_const("message/vnd.wfa.wsc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: message/vnd.wfa.wsc"),
    };

/// `model/3mf`
pub const MODEL_3MF: Essence<&'static str> = match Essence::new_const("model/3mf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/3mf"),
};

/// `model/e57`
pub const MODEL_E57: Essence<&'static str> = match Essence::new_const("model/e57") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/e57"),
};

/// `model/example`
pub const MODEL_EXAMPLE: Essence<&'static str> = match Essence::new_const("model/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/example"),
};

/// `model/gltf-binary`
pub const MODEL_GLTF_BINARY: Essence<&'static str> = match Essence::new_const("model/gltf-binary") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/gltf-binary"),
};

/// `model/gltf+json`
pub const MODEL_GLTF_JSON: Essence<&'static str> = match Essence::new_const("model/gltf+json") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/gltf+json"),
};

/// `model/iges`
pub const MODEL_IGES: Essence<&'static str> = match Essence::new_const("model/iges") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/iges"),
};

/// `model/mtl`
pub const MODEL_MTL: Essence<&'static str> = match Essence::new_const("model/mtl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/mtl"),
};

/// `model/obj`
pub const MODEL_OBJ: Essence<&'static str> = match Essence::new_const("model/obj") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/obj"),
};

/// `model/prc`
pub const MODEL_PRC: Essence<&'static str> = match Essence::new_const("model/prc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/prc"),
};

/// `model/step`
pub const MODEL_STEP: Essence<&'static str> = match Essence::new_const("model/step") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/step"),
};

/// `model/step+xml`
pub const MODEL_STEP_XML: Essence<&'static str> = match Essence::new_const("model/step+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/step+xml"),
};

/// `model/step+zip`
pub const MODEL_STEP_ZIP: Essence<&'static str> = match Essence::new_const("model/step+zip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/step+zip"),
};

/// `model/step-xml+zip`
pub const MODEL_STEP_XML_ZIP: Essence<&'static str> = match Essence::new_const("model/step-xml+zip")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/step-xml+zip"),
};

/// `model/stl`
pub const MODEL_STL: Essence<&'static str> = match Essence::new_const("model/stl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/stl"),
};

/// `model/u3d`
pub const MODEL_U3D: Essence<&'static str> = match Essence::new_const("model/u3d") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/u3d"),
};

/// `model/vnd.collada+xml`
pub const MODEL_VND_COLLADA_XML: Essence<&'static str> =
    match Essence::new_const("model/vnd.collada+xml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.collada+xml"),
    };

/// `model/vnd.dwf`
pub const MODEL_VND_DWF: Essence<&'static str> = match Essence::new_const("model/vnd.dwf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.dwf"),
};

/// `model/vnd.flatland.3dml`
pub const MODEL_VND_FLATLAND_3DML: Essence<&'static str> =
    match Essence::new_const("model/vnd.flatland.3dml") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.flatland.3dml"),
    };

/// `model/vnd.gdl`
pub const MODEL_VND_GDL: Essence<&'static str> = match Essence::new_const("model/vnd.gdl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.gdl"),
};

/// `model/vnd.gs-gdl`
pub const MODEL_VND_GS_GDL: Essence<&'static str> = match Essence::new_const("model/vnd.gs-gdl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.gs-gdl"),
};

/// `model/vnd.gtw`
pub const MODEL_VND_GTW: Essence<&'static str> = match Essence::new_const("model/vnd.gtw") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.gtw"),
};

/// `model/vnd.moml+xml`
pub const MODEL_VND_MOML_XML: Essence<&'static str> = match Essence::new_const("model/vnd.moml+xml")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.moml+xml"),
};

/// `model/vnd.mts`
pub const MODEL_VND_MTS: Essence<&'static str> = match Essence::new_const("model/vnd.mts") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.mts"),
};

/// `model/vnd.opengex`
pub const MODEL_VND_OPENGEX: Essence<&'static str> = match Essence::new_const("model/vnd.opengex") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.opengex"),
};

/// `model/vnd.parasolid.transmit.binary`
pub const MODEL_VND_PARASOLID_TRANSMIT_BINARY: Essence<&'static str> =
    match Essence::new_const("model/vnd.parasolid.transmit.binary") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.parasolid.transmit.binary"),
    };

/// `model/vnd.parasolid.transmit.text`
pub const MODEL_VND_PARASOLID_TRANSMIT_TEXT: Essence<&'static str> =
    match Essence::new_const("model/vnd.parasolid.transmit.text") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.parasolid.transmit.text"),
    };

/// `model/vnd.pytha.pyox`
pub const MODEL_VND_PYTHA_PYOX: Essence<&'static str> =
    match Essence::new_const("model/vnd.pytha.pyox") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.pytha.pyox"),
    };

/// `model/vnd.rosette.annotated-data-model`
pub const MODEL_VND_ROSETTE_ANNOTATED_DATA_MODEL: Essence<&'static str> =
    match Essence::new_const("model/vnd.rosette.annotated-data-model") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.rosette.annotated-data-model"),
    };

/// `model/vnd.sap.vds`
pub const MODEL_VND_SAP_VDS: Essence<&'static str> = match Essence::new_const("model/vnd.sap.vds") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.sap.vds"),
};

/// `model/vnd.usdz+zip`
pub const MODEL_VND_USDZ_ZIP: Essence<&'static str> = match Essence::new_const("model/vnd.usdz+zip")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.usdz+zip"),
};

/// `model/vnd.valve.source.compiled-map`
pub const MODEL_VND_VALVE_SOURCE_COMPILED_MAP: Essence<&'static str> =
    match Essence::new_const("model/vnd.valve.source.compiled-map") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/vnd.valve.source.compiled-map"),
    };

/// `model/vnd.vtu`
pub const MODEL_VND_VTU: Essence<&'static str> = match Essence::new_const("model/vnd.vtu") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/vnd.vtu"),
};

/// `model/x3d-vrml`
pub const MODEL_X3D_VRML: Essence<&'static str> = match Essence::new_const("model/x3d-vrml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/x3d-vrml"),
};

/// `model/x3d+fastinfoset`
pub const MODEL_X3D_FASTINFOSET: Essence<&'static str> =
    match Essence::new_const("model/x3d+fastinfoset") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: model/x3d+fastinfoset"),
    };

/// `model/x3d+xml`
pub const MODEL_X3D_XML: Essence<&'static str> = match Essence::new_const("model/x3d+xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: model/x3d+xml"),
};

/// `multipart/appledouble`
pub const MULTIPART_APPLEDOUBLE: Essence<&'static str> =
    match Essence::new_const("multipart/appledouble") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/appledouble"),
    };

/// `multipart/byteranges`
pub const MULTIPART_BYTERANGES: Essence<&'static str> =
    match Essence::new_const("multipart/byteranges") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/byteranges"),
    };

/// `multipart/encrypted`
pub const MULTIPART_ENCRYPTED: Essence<&'static str> =
    match Essence::new_const("multipart/encrypted") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/encrypted"),
    };

/// `multipart/example`
pub const MULTIPART_EXAMPLE: Essence<&'static str> = match Essence::new_const("multipart/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: multipart/example"),
};

/// `multipart/form-data`
pub const MULTIPART_FORM_DATA: Essence<&'static str> =
    match Essence::new_const("multipart/form-data") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/form-data"),
    };

/// `multipart/header-set`
pub const MULTIPART_HEADER_SET: Essence<&'static str> =
    match Essence::new_const("multipart/header-set") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/header-set"),
    };

/// `multipart/multilingual`
pub const MULTIPART_MULTILINGUAL: Essence<&'static str> =
    match Essence::new_const("multipart/multilingual") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/multilingual"),
    };

/// `multipart/related`
pub const MULTIPART_RELATED: Essence<&'static str> = match Essence::new_const("multipart/related") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: multipart/related"),
};

/// `multipart/report`
pub const MULTIPART_REPORT: Essence<&'static str> = match Essence::new_const("multipart/report") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: multipart/report"),
};

/// `multipart/signed`
pub const MULTIPART_SIGNED: Essence<&'static str> = match Essence::new_const("multipart/signed") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: multipart/signed"),
};

/// `multipart/vnd.bint.med-plus`
pub const MULTIPART_VND_BINT_MED_PLUS: Essence<&'static str> =
    match Essence::new_const("multipart/vnd.bint.med-plus") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/vnd.bint.med-plus"),
    };

/// `multipart/voice-message`
pub const MULTIPART_VOICE_MESSAGE: Essence<&'static str> =
    match Essence::new_const("multipart/voice-message") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/voice-message"),
    };

/// `multipart/x-mixed-replace`
pub const MULTIPART_X_MIXED_REPLACE: Essence<&'static str> =
    match Essence::new_const("multipart/x-mixed-replace") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: multipart/x-mixed-replace"),
    };

/// `text/1d-interleaved-parityfec`
pub const TEXT_1D_INTERLEAVED_PARITYFEC: Essence<&'static str> =
    match Essence::new_const("text/1d-interleaved-parityfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/1d-interleaved-parityfec"),
    };

/// `text/cache-manifest`
pub const TEXT_CACHE_MANIFEST: Essence<&'static str> =
    match Essence::new_const("text/cache-manifest") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/cache-manifest"),
    };

/// `text/calendar`
pub const TEXT_CALENDAR: Essence<&'static str> = match Essence::new_const("text/calendar") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/calendar"),
};

/// `text/cql`
pub const TEXT_CQL: Essence<&'static str> = match Essence::new_const("text/cql") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/cql"),
};

/// `text/cql-expression`
pub const TEXT_CQL_EXPRESSION: Essence<&'static str> =
    match Essence::new_const("text/cql-expression") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/cql-expression"),
    };

/// `text/cql-identifier`
pub const TEXT_CQL_IDENTIFIER: Essence<&'static str> =
    match Essence::new_const("text/cql-identifier") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/cql-identifier"),
    };

/// `text/css`
pub const TEXT_CSS: Essence<&'static str> = match Essence::new_const("text/css") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/css"),
};

/// `text/csv`
pub const TEXT_CSV: Essence<&'static str> = match Essence::new_const("text/csv") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/csv"),
};

/// `text/csv-schema`
pub const TEXT_CSV_SCHEMA: Essence<&'static str> = match Essence::new_const("text/csv-schema") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/csv-schema"),
};

/// `text/dns`
pub const TEXT_DNS: Essence<&'static str> = match Essence::new_const("text/dns") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/dns"),
};

/// `text/ecmascript`
pub const TEXT_ECMASCRIPT: Essence<&'static str> = match Essence::new_const("text/ecmascript") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/ecmascript"),
};

/// `text/encaprtp`
pub const TEXT_ENCAPRTP: Essence<&'static str> = match Essence::new_const("text/encaprtp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/encaprtp"),
};

/// `text/example`
pub const TEXT_EXAMPLE: Essence<&'static str> = match Essence::new_const("text/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/example"),
};

/// `text/fhirpath`
pub const TEXT_FHIRPATH: Essence<&'static str> = match Essence::new_const("text/fhirpath") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/fhirpath"),
};

/// `text/flexfec`
pub const TEXT_FLEXFEC: Essence<&'static str> = match Essence::new_const("text/flexfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/flexfec"),
};

/// `text/fwdred`
pub const TEXT_FWDRED: Essence<&'static str> = match Essence::new_const("text/fwdred") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/fwdred"),
};

/// `text/gff3`
pub const TEXT_GFF3: Essence<&'static str> = match Essence::new_const("text/gff3") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/gff3"),
};

/// `text/grammar-ref-list`
pub const TEXT_GRAMMAR_REF_LIST: Essence<&'static str> =
    match Essence::new_const("text/grammar-ref-list") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/grammar-ref-list"),
    };

/// `text/html`
pub const TEXT_HTML: Essence<&'static str> = match Essence::new_const("text/html") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/html"),
};

/// `text/javascript`
pub const TEXT_JAVASCRIPT: Essence<&'static str> = match Essence::new_const("text/javascript") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/javascript"),
};

/// `text/jcr-cnd`
pub const TEXT_JCR_CND: Essence<&'static str> = match Essence::new_const("text/jcr-cnd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/jcr-cnd"),
};

/// `text/markdown`
pub const TEXT_MARKDOWN: Essence<&'static str> = match Essence::new_const("text/markdown") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/markdown"),
};

/// `text/mizar`
pub const TEXT_MIZAR: Essence<&'static str> = match Essence::new_const("text/mizar") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/mizar"),
};

/// `text/n3`
pub const TEXT_N3: Essence<&'static str> = match Essence::new_const("text/n3") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/n3"),
};

/// `text/parameters`
pub const TEXT_PARAMETERS: Essence<&'static str> = match Essence::new_const("text/parameters") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/parameters"),
};

/// `text/parityfec`
pub const TEXT_PARITYFEC: Essence<&'static str> = match Essence::new_const("text/parityfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/parityfec"),
};

/// `text/provenance-notation`
pub const TEXT_PROVENANCE_NOTATION: Essence<&'static str> =
    match Essence::new_const("text/provenance-notation") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/provenance-notation"),
    };

/// `text/prs.fallenstein.rst`
pub const TEXT_PRS_FALLENSTEIN_RST: Essence<&'static str> =
    match Essence::new_const("text/prs.fallenstein.rst") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/prs.fallenstein.rst"),
    };

/// `text/prs.lines.tag`
pub const TEXT_PRS_LINES_TAG: Essence<&'static str> = match Essence::new_const("text/prs.lines.tag")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/prs.lines.tag"),
};

/// `text/prs.prop.logic`
pub const TEXT_PRS_PROP_LOGIC: Essence<&'static str> =
    match Essence::new_const("text/prs.prop.logic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/prs.prop.logic"),
    };

/// `text/raptorfec`
pub const TEXT_RAPTORFEC: Essence<&'static str> = match Essence::new_const("text/raptorfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/raptorfec"),
};

/// `text/RED`
pub const TEXT_RED: Essence<&'static str> = match Essence::new_const("text/RED") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/RED"),
};

/// `text/rfc822-headers`
pub const TEXT_RFC822_HEADERS: Essence<&'static str> =
    match Essence::new_const("text/rfc822-headers") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/rfc822-headers"),
    };

/// `text/rtf`
pub const TEXT_RTF: Essence<&'static str> = match Essence::new_const("text/rtf") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/rtf"),
};

/// `text/rtp-enc-aescm128`
pub const TEXT_RTP_ENC_AESCM128: Essence<&'static str> =
    match Essence::new_const("text/rtp-enc-aescm128") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/rtp-enc-aescm128"),
    };

/// `text/rtploopback`
pub const TEXT_RTPLOOPBACK: Essence<&'static str> = match Essence::new_const("text/rtploopback") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/rtploopback"),
};

/// `text/rtx`
pub const TEXT_RTX: Essence<&'static str> = match Essence::new_const("text/rtx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/rtx"),
};

/// `text/SGML`
pub const TEXT_SGML: Essence<&'static str> = match Essence::new_const("text/SGML") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/SGML"),
};

/// `text/shaclc`
pub const TEXT_SHACLC: Essence<&'static str> = match Essence::new_const("text/shaclc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/shaclc"),
};

/// `text/shex`
pub const TEXT_SHEX: Essence<&'static str> = match Essence::new_const("text/shex") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/shex"),
};

/// `text/spdx`
pub const TEXT_SPDX: Essence<&'static str> = match Essence::new_const("text/spdx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/spdx"),
};

/// `text/strings`
pub const TEXT_STRINGS: Essence<&'static str> = match Essence::new_const("text/strings") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/strings"),
};

/// `text/t140`
pub const TEXT_T140: Essence<&'static str> = match Essence::new_const("text/t140") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/t140"),
};

/// `text/tab-separated-values`
pub const TEXT_TAB_SEPARATED_VALUES: Essence<&'static str> =
    match Essence::new_const("text/tab-separated-values") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/tab-separated-values"),
    };

/// `text/troff`
pub const TEXT_TROFF: Essence<&'static str> = match Essence::new_const("text/troff") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/troff"),
};

/// `text/turtle`
pub const TEXT_TURTLE: Essence<&'static str> = match Essence::new_const("text/turtle") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/turtle"),
};

/// `text/ulpfec`
pub const TEXT_ULPFEC: Essence<&'static str> = match Essence::new_const("text/ulpfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/ulpfec"),
};

/// `text/uri-list`
pub const TEXT_URI_LIST: Essence<&'static str> = match Essence::new_const("text/uri-list") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/uri-list"),
};

/// `text/vcard`
pub const TEXT_VCARD: Essence<&'static str> = match Essence::new_const("text/vcard") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vcard"),
};

/// `text/vnd.a`
pub const TEXT_VND_A: Essence<&'static str> = match Essence::new_const("text/vnd.a") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.a"),
};

/// `text/vnd.abc`
pub const TEXT_VND_ABC: Essence<&'static str> = match Essence::new_const("text/vnd.abc") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.abc"),
};

/// `text/vnd.ascii-art`
pub const TEXT_VND_ASCII_ART: Essence<&'static str> = match Essence::new_const("text/vnd.ascii-art")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.ascii-art"),
};

/// `text/vnd.curl`
pub const TEXT_VND_CURL: Essence<&'static str> = match Essence::new_const("text/vnd.curl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.curl"),
};

/// `text/vnd.debian.copyright`
pub const TEXT_VND_DEBIAN_COPYRIGHT: Essence<&'static str> =
    match Essence::new_const("text/vnd.debian.copyright") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.debian.copyright"),
    };

/// `text/vnd.DMClientScript`
pub const TEXT_VND_DMCLIENTSCRIPT: Essence<&'static str> =
    match Essence::new_const("text/vnd.DMClientScript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.DMClientScript"),
    };

/// `text/vnd.dvb.subtitle`
pub const TEXT_VND_DVB_SUBTITLE: Essence<&'static str> =
    match Essence::new_const("text/vnd.dvb.subtitle") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.dvb.subtitle"),
    };

/// `text/vnd.esmertec.theme-descriptor`
pub const TEXT_VND_ESMERTEC_THEME_DESCRIPTOR: Essence<&'static str> =
    match Essence::new_const("text/vnd.esmertec.theme-descriptor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.esmertec.theme-descriptor"),
    };

/// `text/vnd.familysearch.gedcom`
pub const TEXT_VND_FAMILYSEARCH_GEDCOM: Essence<&'static str> =
    match Essence::new_const("text/vnd.familysearch.gedcom") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.familysearch.gedcom"),
    };

/// `text/vnd.ficlab.flt`
pub const TEXT_VND_FICLAB_FLT: Essence<&'static str> =
    match Essence::new_const("text/vnd.ficlab.flt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.ficlab.flt"),
    };

/// `text/vnd.fly`
pub const TEXT_VND_FLY: Essence<&'static str> = match Essence::new_const("text/vnd.fly") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.fly"),
};

/// `text/vnd.fmi.flexstor`
pub const TEXT_VND_FMI_FLEXSTOR: Essence<&'static str> =
    match Essence::new_const("text/vnd.fmi.flexstor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.fmi.flexstor"),
    };

/// `text/vnd.gml`
pub const TEXT_VND_GML: Essence<&'static str> = match Essence::new_const("text/vnd.gml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.gml"),
};

/// `text/vnd.graphviz`
pub const TEXT_VND_GRAPHVIZ: Essence<&'static str> = match Essence::new_const("text/vnd.graphviz") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.graphviz"),
};

/// `text/vnd.hans`
pub const TEXT_VND_HANS: Essence<&'static str> = match Essence::new_const("text/vnd.hans") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.hans"),
};

/// `text/vnd.hgl`
pub const TEXT_VND_HGL: Essence<&'static str> = match Essence::new_const("text/vnd.hgl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.hgl"),
};

/// `text/vnd.in3d.3dml`
pub const TEXT_VND_IN3D_3DML: Essence<&'static str> = match Essence::new_const("text/vnd.in3d.3dml")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.in3d.3dml"),
};

/// `text/vnd.in3d.spot`
pub const TEXT_VND_IN3D_SPOT: Essence<&'static str> = match Essence::new_const("text/vnd.in3d.spot")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.in3d.spot"),
};

/// `text/vnd.IPTC.NewsML`
pub const TEXT_VND_IPTC_NEWSML: Essence<&'static str> =
    match Essence::new_const("text/vnd.IPTC.NewsML") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.IPTC.NewsML"),
    };

/// `text/vnd.IPTC.NITF`
pub const TEXT_VND_IPTC_NITF: Essence<&'static str> = match Essence::new_const("text/vnd.IPTC.NITF")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.IPTC.NITF"),
};

/// `text/vnd.latex-z`
pub const TEXT_VND_LATEX_Z: Essence<&'static str> = match Essence::new_const("text/vnd.latex-z") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.latex-z"),
};

/// `text/vnd.motorola.reflex`
pub const TEXT_VND_MOTOROLA_REFLEX: Essence<&'static str> =
    match Essence::new_const("text/vnd.motorola.reflex") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.motorola.reflex"),
    };

/// `text/vnd.ms-mediapackage`
pub const TEXT_VND_MS_MEDIAPACKAGE: Essence<&'static str> =
    match Essence::new_const("text/vnd.ms-mediapackage") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.ms-mediapackage"),
    };

/// `text/vnd.net2phone.commcenter.command`
pub const TEXT_VND_NET2PHONE_COMMCENTER_COMMAND: Essence<&'static str> =
    match Essence::new_const("text/vnd.net2phone.commcenter.command") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.net2phone.commcenter.command"),
    };

/// `text/vnd.radisys.msml-basic-layout`
pub const TEXT_VND_RADISYS_MSML_BASIC_LAYOUT: Essence<&'static str> =
    match Essence::new_const("text/vnd.radisys.msml-basic-layout") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.radisys.msml-basic-layout"),
    };

/// `text/vnd.senx.warpscript`
pub const TEXT_VND_SENX_WARPSCRIPT: Essence<&'static str> =
    match Essence::new_const("text/vnd.senx.warpscript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.senx.warpscript"),
    };

/// `text/vnd.si.uricatalogue`
pub const TEXT_VND_SI_URICATALOGUE: Essence<&'static str> =
    match Essence::new_const("text/vnd.si.uricatalogue") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.si.uricatalogue"),
    };

/// `text/vnd.sun.j2me.app-descriptor`
pub const TEXT_VND_SUN_J2ME_APP_DESCRIPTOR: Essence<&'static str> =
    match Essence::new_const("text/vnd.sun.j2me.app-descriptor") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.sun.j2me.app-descriptor"),
    };

/// `text/vnd.sosi`
pub const TEXT_VND_SOSI: Essence<&'static str> = match Essence::new_const("text/vnd.sosi") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.sosi"),
};

/// `text/vnd.trolltech.linguist`
pub const TEXT_VND_TROLLTECH_LINGUIST: Essence<&'static str> =
    match Essence::new_const("text/vnd.trolltech.linguist") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.trolltech.linguist"),
    };

/// `text/vnd.wap.si`
pub const TEXT_VND_WAP_SI: Essence<&'static str> = match Essence::new_const("text/vnd.wap.si") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.wap.si"),
};

/// `text/vnd.wap.sl`
pub const TEXT_VND_WAP_SL: Essence<&'static str> = match Essence::new_const("text/vnd.wap.sl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.wap.sl"),
};

/// `text/vnd.wap.wml`
pub const TEXT_VND_WAP_WML: Essence<&'static str> = match Essence::new_const("text/vnd.wap.wml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vnd.wap.wml"),
};

/// `text/vnd.wap.wmlscript`
pub const TEXT_VND_WAP_WMLSCRIPT: Essence<&'static str> =
    match Essence::new_const("text/vnd.wap.wmlscript") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/vnd.wap.wmlscript"),
    };

/// `text/vtt`
pub const TEXT_VTT: Essence<&'static str> = match Essence::new_const("text/vtt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/vtt"),
};

/// `text/xml`
pub const TEXT_XML: Essence<&'static str> = match Essence::new_const("text/xml") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: text/xml"),
};

/// `text/xml-external-parsed-entity`
pub const TEXT_XML_EXTERNAL_PARSED_ENTITY: Essence<&'static str> =
    match Essence::new_const("text/xml-external-parsed-entity") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: text/xml-external-parsed-entity"),
    };

/// `video/1d-interleaved-parityfec`
pub const VIDEO_1D_INTERLEAVED_PARITYFEC: Essence<&'static str> =
    match Essence::new_const("video/1d-interleaved-parityfec") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/1d-interleaved-parityfec"),
    };

/// `video/3gpp`
pub const VIDEO_3GPP: Essence<&'static str> = match Essence::new_const("video/3gpp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/3gpp"),
};

/// `video/3gpp2`
pub const VIDEO_3GPP2: Essence<&'static str> = match Essence::new_const("video/3gpp2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/3gpp2"),
};

/// `video/3gpp-tt`
pub const VIDEO_3GPP_TT: Essence<&'static str> = match Essence::new_const("video/3gpp-tt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/3gpp-tt"),
};

/// `video/AV1`
pub const VIDEO_AV1: Essence<&'static str> = match Essence::new_const("video/AV1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/AV1"),
};

/// `video/BMPEG`
pub const VIDEO_BMPEG: Essence<&'static str> = match Essence::new_const("video/BMPEG") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/BMPEG"),
};

/// `video/BT656`
pub const VIDEO_BT656: Essence<&'static str> = match Essence::new_const("video/BT656") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/BT656"),
};

/// `video/CelB`
pub const VIDEO_CELB: Essence<&'static str> = match Essence::new_const("video/CelB") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/CelB"),
};

/// `video/DV`
pub const VIDEO_DV: Essence<&'static str> = match Essence::new_const("video/DV") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/DV"),
};

/// `video/encaprtp`
pub const VIDEO_ENCAPRTP: Essence<&'static str> = match Essence::new_const("video/encaprtp") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/encaprtp"),
};

/// `video/example`
pub const VIDEO_EXAMPLE: Essence<&'static str> = match Essence::new_const("video/example") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/example"),
};

/// `video/FFV1`
pub const VIDEO_FFV1: Essence<&'static str> = match Essence::new_const("video/FFV1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/FFV1"),
};

/// `video/flexfec`
pub const VIDEO_FLEXFEC: Essence<&'static str> = match Essence::new_const("video/flexfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/flexfec"),
};

/// `video/H261`
pub const VIDEO_H261: Essence<&'static str> = match Essence::new_const("video/H261") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H261"),
};

/// `video/H263`
pub const VIDEO_H263: Essence<&'static str> = match Essence::new_const("video/H263") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H263"),
};

/// `video/H263-1998`
pub const VIDEO_H263_1998: Essence<&'static str> = match Essence::new_const("video/H263-1998") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H263-1998"),
};

/// `video/H263-2000`
pub const VIDEO_H263_2000: Essence<&'static str> = match Essence::new_const("video/H263-2000") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H263-2000"),
};

/// `video/H264`
pub const VIDEO_H264: Essence<&'static str> = match Essence::new_const("video/H264") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H264"),
};

/// `video/H264-RCDO`
pub const VIDEO_H264_RCDO: Essence<&'static str> = match Essence::new_const("video/H264-RCDO") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H264-RCDO"),
};

/// `video/H264-SVC`
pub const VIDEO_H264_SVC: Essence<&'static str> = match Essence::new_const("video/H264-SVC") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H264-SVC"),
};

/// `video/H265`
pub const VIDEO_H265: Essence<&'static str> = match Essence::new_const("video/H265") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/H265"),
};

/// `video/iso.segment`
pub const VIDEO_ISO_SEGMENT: Essence<&'static str> = match Essence::new_const("video/iso.segment") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/iso.segment"),
};

/// `video/JPEG`
pub const VIDEO_JPEG: Essence<&'static str> = match Essence::new_const("video/JPEG") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/JPEG"),
};

/// `video/jpeg2000`
pub const VIDEO_JPEG2000: Essence<&'static str> = match Essence::new_const("video/jpeg2000") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/jpeg2000"),
};

/// `video/jxsv`
pub const VIDEO_JXSV: Essence<&'static str> = match Essence::new_const("video/jxsv") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/jxsv"),
};

/// `video/mj2`
pub const VIDEO_MJ2: Essence<&'static str> = match Essence::new_const("video/mj2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/mj2"),
};

/// `video/MP1S`
pub const VIDEO_MP1S: Essence<&'static str> = match Essence::new_const("video/MP1S") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/MP1S"),
};

/// `video/MP2P`
pub const VIDEO_MP2P: Essence<&'static str> = match Essence::new_const("video/MP2P") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/MP2P"),
};

/// `video/MP2T`
pub const VIDEO_MP2T: Essence<&'static str> = match Essence::new_const("video/MP2T") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/MP2T"),
};

/// `video/mp4`
pub const VIDEO_MP4: Essence<&'static str> = match Essence::new_const("video/mp4") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/mp4"),
};

/// `video/MP4V-ES`
pub const VIDEO_MP4V_ES: Essence<&'static str> = match Essence::new_const("video/MP4V-ES") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/MP4V-ES"),
};

/// `video/MPV`
pub const VIDEO_MPV: Essence<&'static str> = match Essence::new_const("video/MPV") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/MPV"),
};

/// `video/mpeg4-generic`
pub const VIDEO_MPEG4_GENERIC: Essence<&'static str> =
    match Essence::new_const("video/mpeg4-generic") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/mpeg4-generic"),
    };

/// `video/nv`
pub const VIDEO_NV: Essence<&'static str> = match Essence::new_const("video/nv") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/nv"),
};

/// `video/ogg`
pub const VIDEO_OGG: Essence<&'static str> = match Essence::new_const("video/ogg") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/ogg"),
};

/// `video/parityfec`
pub const VIDEO_PARITYFEC: Essence<&'static str> = match Essence::new_const("video/parityfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/parityfec"),
};

/// `video/pointer`
pub const VIDEO_POINTER: Essence<&'static str> = match Essence::new_const("video/pointer") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/pointer"),
};

/// `video/quicktime`
pub const VIDEO_QUICKTIME: Essence<&'static str> = match Essence::new_const("video/quicktime") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/quicktime"),
};

/// `video/raptorfec`
pub const VIDEO_RAPTORFEC: Essence<&'static str> = match Essence::new_const("video/raptorfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/raptorfec"),
};

/// `video/raw`
pub const VIDEO_RAW: Essence<&'static str> = match Essence::new_const("video/raw") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/raw"),
};

/// `video/rtp-enc-aescm128`
pub const VIDEO_RTP_ENC_AESCM128: Essence<&'static str> =
    match Essence::new_const("video/rtp-enc-aescm128") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/rtp-enc-aescm128"),
    };

/// `video/rtploopback`
pub const VIDEO_RTPLOOPBACK: Essence<&'static str> = match Essence::new_const("video/rtploopback") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/rtploopback"),
};

/// `video/rtx`
pub const VIDEO_RTX: Essence<&'static str> = match Essence::new_const("video/rtx") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/rtx"),
};

/// `video/scip`
pub const VIDEO_SCIP: Essence<&'static str> = match Essence::new_const("video/scip") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/scip"),
};

/// `video/smpte291`
pub const VIDEO_SMPTE291: Essence<&'static str> = match Essence::new_const("video/smpte291") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/smpte291"),
};

/// `video/SMPTE292M`
pub const VIDEO_SMPTE292M: Essence<&'static str> = match Essence::new_const("video/SMPTE292M") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/SMPTE292M"),
};

/// `video/ulpfec`
pub const VIDEO_ULPFEC: Essence<&'static str> = match Essence::new_const("video/ulpfec") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/ulpfec"),
};

/// `video/vc1`
pub const VIDEO_VC1: Essence<&'static str> = match Essence::new_const("video/vc1") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vc1"),
};

/// `video/vc2`
pub const VIDEO_VC2: Essence<&'static str> = match Essence::new_const("video/vc2") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vc2"),
};

/// `video/vnd.CCTV`
pub const VIDEO_VND_CCTV: Essence<&'static str> = match Essence::new_const("video/vnd.CCTV") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.CCTV"),
};

/// `video/vnd.dece.hd`
pub const VIDEO_VND_DECE_HD: Essence<&'static str> = match Essence::new_const("video/vnd.dece.hd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.dece.hd"),
};

/// `video/vnd.dece.mobile`
pub const VIDEO_VND_DECE_MOBILE: Essence<&'static str> =
    match Essence::new_const("video/vnd.dece.mobile") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.dece.mobile"),
    };

/// `video/vnd.dece.mp4`
pub const VIDEO_VND_DECE_MP4: Essence<&'static str> = match Essence::new_const("video/vnd.dece.mp4")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.dece.mp4"),
};

/// `video/vnd.dece.pd`
pub const VIDEO_VND_DECE_PD: Essence<&'static str> = match Essence::new_const("video/vnd.dece.pd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.dece.pd"),
};

/// `video/vnd.dece.sd`
pub const VIDEO_VND_DECE_SD: Essence<&'static str> = match Essence::new_const("video/vnd.dece.sd") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.dece.sd"),
};

/// `video/vnd.dece.video`
pub const VIDEO_VND_DECE_VIDEO: Essence<&'static str> =
    match Essence::new_const("video/vnd.dece.video") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.dece.video"),
    };

/// `video/vnd.directv.mpeg`
pub const VIDEO_VND_DIRECTV_MPEG: Essence<&'static str> =
    match Essence::new_const("video/vnd.directv.mpeg") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.directv.mpeg"),
    };

/// `video/vnd.directv.mpeg-tts`
pub const VIDEO_VND_DIRECTV_MPEG_TTS: Essence<&'static str> =
    match Essence::new_const("video/vnd.directv.mpeg-tts") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.directv.mpeg-tts"),
    };

/// `video/vnd.dlna.mpeg-tts`
pub const VIDEO_VND_DLNA_MPEG_TTS: Essence<&'static str> =
    match Essence::new_const("video/vnd.dlna.mpeg-tts") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.dlna.mpeg-tts"),
    };

/// `video/vnd.dvb.file`
pub const VIDEO_VND_DVB_FILE: Essence<&'static str> = match Essence::new_const("video/vnd.dvb.file")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.dvb.file"),
};

/// `video/vnd.fvt`
pub const VIDEO_VND_FVT: Essence<&'static str> = match Essence::new_const("video/vnd.fvt") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.fvt"),
};

/// `video/vnd.hns.video`
pub const VIDEO_VND_HNS_VIDEO: Essence<&'static str> =
    match Essence::new_const("video/vnd.hns.video") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.hns.video"),
    };

/// `video/vnd.iptvforum.1dparityfec-1010`
pub const VIDEO_VND_IPTVFORUM_1DPARITYFEC_1010: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.1dparityfec-1010") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.1dparityfec-1010"),
    };

/// `video/vnd.iptvforum.1dparityfec-2005`
pub const VIDEO_VND_IPTVFORUM_1DPARITYFEC_2005: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.1dparityfec-2005") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.1dparityfec-2005"),
    };

/// `video/vnd.iptvforum.2dparityfec-1010`
pub const VIDEO_VND_IPTVFORUM_2DPARITYFEC_1010: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.2dparityfec-1010") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.2dparityfec-1010"),
    };

/// `video/vnd.iptvforum.2dparityfec-2005`
pub const VIDEO_VND_IPTVFORUM_2DPARITYFEC_2005: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.2dparityfec-2005") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.2dparityfec-2005"),
    };

/// `video/vnd.iptvforum.ttsavc`
pub const VIDEO_VND_IPTVFORUM_TTSAVC: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.ttsavc") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.ttsavc"),
    };

/// `video/vnd.iptvforum.ttsmpeg2`
pub const VIDEO_VND_IPTVFORUM_TTSMPEG2: Essence<&'static str> =
    match Essence::new_const("video/vnd.iptvforum.ttsmpeg2") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.iptvforum.ttsmpeg2"),
    };

/// `video/vnd.motorola.video`
pub const VIDEO_VND_MOTOROLA_VIDEO: Essence<&'static str> =
    match Essence::new_const("video/vnd.motorola.video") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.motorola.video"),
    };

/// `video/vnd.motorola.videop`
pub const VIDEO_VND_MOTOROLA_VIDEOP: Essence<&'static str> =
    match Essence::new_const("video/vnd.motorola.videop") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.motorola.videop"),
    };

/// `video/vnd.mpegurl`
pub const VIDEO_VND_MPEGURL: Essence<&'static str> = match Essence::new_const("video/vnd.mpegurl") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.mpegurl"),
};

/// `video/vnd.ms-playready.media.pyv`
pub const VIDEO_VND_MS_PLAYREADY_MEDIA_PYV: Essence<&'static str> =
    match Essence::new_const("video/vnd.ms-playready.media.pyv") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.ms-playready.media.pyv"),
    };

/// `video/vnd.nokia.interleaved-multimedia`
pub const VIDEO_VND_NOKIA_INTERLEAVED_MULTIMEDIA: Essence<&'static str> =
    match Essence::new_const("video/vnd.nokia.interleaved-multimedia") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.nokia.interleaved-multimedia"),
    };

/// `video/vnd.nokia.mp4vr`
pub const VIDEO_VND_NOKIA_MP4VR: Essence<&'static str> =
    match Essence::new_const("video/vnd.nokia.mp4vr") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.nokia.mp4vr"),
    };

/// `video/vnd.nokia.videovoip`
pub const VIDEO_VND_NOKIA_VIDEOVOIP: Essence<&'static str> =
    match Essence::new_const("video/vnd.nokia.videovoip") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.nokia.videovoip"),
    };

/// `video/vnd.objectvideo`
pub const VIDEO_VND_OBJECTVIDEO: Essence<&'static str> =
    match Essence::new_const("video/vnd.objectvideo") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.objectvideo"),
    };

/// `video/vnd.radgamettools.bink`
pub const VIDEO_VND_RADGAMETTOOLS_BINK: Essence<&'static str> =
    match Essence::new_const("video/vnd.radgamettools.bink") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.radgamettools.bink"),
    };

/// `video/vnd.radgamettools.smacker`
pub const VIDEO_VND_RADGAMETTOOLS_SMACKER: Essence<&'static str> =
    match Essence::new_const("video/vnd.radgamettools.smacker") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.radgamettools.smacker"),
    };

/// `video/vnd.sealed.mpeg1`
pub const VIDEO_VND_SEALED_MPEG1: Essence<&'static str> =
    match Essence::new_const("video/vnd.sealed.mpeg1") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.sealed.mpeg1"),
    };

/// `video/vnd.sealed.mpeg4`
pub const VIDEO_VND_SEALED_MPEG4: Essence<&'static str> =
    match Essence::new_const("video/vnd.sealed.mpeg4") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.sealed.mpeg4"),
    };

/// `video/vnd.sealed.swf`
pub const VIDEO_VND_SEALED_SWF: Essence<&'static str> =
    match Essence::new_const("video/vnd.sealed.swf") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.sealed.swf"),
    };

/// `video/vnd.sealedmedia.softseal.mov`
pub const VIDEO_VND_SEALEDMEDIA_SOFTSEAL_MOV: Essence<&'static str> =
    match Essence::new_const("video/vnd.sealedmedia.softseal.mov") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.sealedmedia.softseal.mov"),
    };

/// `video/vnd.uvvu.mp4`
pub const VIDEO_VND_UVVU_MP4: Essence<&'static str> = match Essence::new_const("video/vnd.uvvu.mp4")
{
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.uvvu.mp4"),
};

/// `video/vnd.youtube.yt`
pub const VIDEO_VND_YOUTUBE_YT: Essence<&'static str> =
    match Essence::new_const("video/vnd.youtube.yt") {
        Ok(essence) => essence,
        Err(..) => panic!("invalid: video/vnd.youtube.yt"),
    };

/// `video/vnd.vivo`
pub const VIDEO_VND_VIVO: Essence<&'static str> = match Essence::new_const("video/vnd.vivo") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/vnd.vivo"),
};

/// `video/VP8`
pub const VIDEO_VP8: Essence<&'static str> = match Essence::new_const("video/VP8") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/VP8"),
};

/// `video/VP9`
pub const VIDEO_VP9: Essence<&'static str> = match Essence::new_const("video/VP9") {
    Ok(essence) => essence,
    Err(..) => panic!("invalid: video/VP9"),
};
